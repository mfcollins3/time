// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

use std::error::Error;
use std::io::{Cursor, Write, stdout};
use std::thread;
use std::time::{Duration, Instant};

const POMODORO_SECONDS: u64 = 25 * 60;
const ALARM: &[u8] = include_bytes!("../assets/alarm.mp3");

fn main() -> Result<(), Box<dyn Error>> {
    countdown(POMODORO_SECONDS)?;
    play_alarm()?;
    Ok(())
}

/// Counts down from `total` seconds, overwriting the time remaining on the
/// current terminal line once per second.
fn countdown(total: u64) -> std::io::Result<()> {
    let start = Instant::now();
    let mut out = stdout().lock();

    for elapsed in 0..=total {
        let remaining = total - elapsed;
        // "\x1b[K" clears to end of line so a shorter time (e.g. "9:59" after
        // "10:00") doesn't leave a stale trailing character.
        write!(out, "\r{}:{:02}\x1b[K", remaining / 60, remaining % 60)?;
        out.flush()?;

        if remaining == 0 {
            break;
        }

        // Sleep until the next whole-second tick to avoid cumulative drift.
        let next_tick = start + Duration::from_secs(elapsed + 1);
        thread::sleep(next_tick.saturating_duration_since(Instant::now()));
    }

    writeln!(out)?;
    Ok(())
}

/// Plays the embedded alarm sound and blocks until it finishes.
fn play_alarm() -> Result<(), Box<dyn Error>> {
    let mut sink = rodio::DeviceSinkBuilder::open_default_sink()?;
    // We block until playback finishes, so dropping the sink afterward is safe.
    sink.log_on_drop(false);
    let player = rodio::Player::connect_new(sink.mixer());
    player.append(rodio::Decoder::try_from(Cursor::new(ALARM))?);
    player.sleep_until_end();
    Ok(())
}
