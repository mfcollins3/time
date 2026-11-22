// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

//! Persistence for Pomodoros in a local SQLite database.

use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;

use rusqlite::Connection;
use uuid::Uuid;

/// SQL expression producing the current UTC time as an ISO-8601 string with
/// millisecond precision, e.g. `2026-11-21T14:03:07.123Z`.
const NOW: &str = "strftime('%Y-%m-%dT%H:%M:%fZ', 'now')";

/// Schema migrations, applied in order. The number of applied migrations is
/// stored in `PRAGMA user_version`. Never edit an existing entry; append a new
/// one instead.
const MIGRATIONS: &[&str] = &[
    // 1: Pomodoros. `end_time` is NULL if the Pomodoro was cancelled or the
    // program was terminated before it finished.
    "CREATE TABLE pomodoros (
         id          INTEGER PRIMARY KEY AUTOINCREMENT,
         public_id   TEXT NOT NULL UNIQUE,
         start_time  TEXT NOT NULL,
         end_time    TEXT
     )",
];

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens the database at `$HOME/.naked-time/data.db` (the user profile
    /// directory on Windows), creating the directory and schema if needed.
    pub fn open_default() -> Result<Self, Box<dyn Error>> {
        let home = std::env::home_dir().ok_or("unable to determine the home directory")?;
        let dir = home.join(".naked-time");
        fs::create_dir_all(&dir)?;
        Ok(Self::open(&dir.join("data.db"))?)
    }

    /// Opens the database at `path` and brings its schema up to date.
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let mut conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        // Returns the new mode as a row, so it can't go through `execute`.
        conn.query_row("PRAGMA journal_mode = WAL", [], |_| Ok(()))?;
        migrate(&mut conn)?;
        Ok(Self { conn })
    }

    /// Records a new Pomodoro starting now and returns its row id.
    pub fn start_pomodoro(&self) -> rusqlite::Result<i64> {
        self.conn.execute(
            &format!("INSERT INTO pomodoros (public_id, start_time) VALUES (?1, {NOW})"),
            [Uuid::now_v7().to_string()],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Records that the Pomodoro `id` finished now.
    pub fn complete_pomodoro(&self, id: i64) -> rusqlite::Result<()> {
        self.conn.execute(
            &format!("UPDATE pomodoros SET end_time = {NOW} WHERE id = ?1"),
            [id],
        )?;
        Ok(())
    }
}

fn migrate(conn: &mut Connection) -> rusqlite::Result<()> {
    let current: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
    for (index, sql) in MIGRATIONS.iter().enumerate().skip(current as usize) {
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute_batch(&format!("PRAGMA user_version = {}", index + 1))?;
        tx.commit()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memory() -> Database {
        Database::open(Path::new(":memory:")).unwrap()
    }

    fn row(db: &Database, id: i64) -> (String, String, Option<String>) {
        db.conn
            .query_row(
                "SELECT public_id, start_time, end_time FROM pomodoros WHERE id = ?1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap()
    }

    fn is_timestamp(s: &str) -> bool {
        // 2026-11-21T14:03:07.123Z
        let b = s.as_bytes();
        b.len() == 24
            && b[23] == b'Z'
            && b[10] == b'T'
            && b[19] == b'.'
            && [4, 7].iter().all(|&i| b[i] == b'-')
            && [13, 16].iter().all(|&i| b[i] == b':')
            && b.iter()
                .enumerate()
                .all(|(i, c)| [4, 7, 10, 13, 16, 19, 23].contains(&i) || c.is_ascii_digit())
    }

    #[test]
    fn migrations_set_user_version_and_are_idempotent() {
        let path = std::env::temp_dir().join(format!("naked-time-test-{}.db", Uuid::now_v7()));
        {
            let db = Database::open(&path).unwrap();
            db.start_pomodoro().unwrap();
        }
        let db = Database::open(&path).unwrap();
        let version: i64 = db
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, MIGRATIONS.len() as i64);
        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM pomodoros", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
        drop(db);
        for suffix in ["", "-wal", "-shm"] {
            let _ = fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }

    #[test]
    fn start_pomodoro_records_start_only() {
        let db = memory();
        let id = db.start_pomodoro().unwrap();
        let (public_id, start, end) = row(&db, id);
        assert_eq!(Uuid::parse_str(&public_id).unwrap().get_version_num(), 7);
        assert!(is_timestamp(&start), "bad timestamp: {start}");
        assert_eq!(end, None);
    }

    #[test]
    fn complete_pomodoro_sets_end_time() {
        let db = memory();
        let id = db.start_pomodoro().unwrap();
        db.complete_pomodoro(id).unwrap();
        let (_, start, end) = row(&db, id);
        let end = end.expect("end_time should be set");
        assert!(is_timestamp(&end), "bad timestamp: {end}");
        assert!(end >= start);
    }

    #[test]
    fn pomodoros_get_increasing_ids_and_distinct_public_ids() {
        let db = memory();
        let a = db.start_pomodoro().unwrap();
        let b = db.start_pomodoro().unwrap();
        assert!(b > a);
        assert_ne!(row(&db, a).0, row(&db, b).0);
    }
}
