// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

//! The `pomodoro` commands.

use std::error::Error;
use std::io::{self, Cursor, Write, stdout};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use crate::db;
use crate::i18n::fl;

pub(crate) const POMODORO_SECONDS: u64 = 25 * 60;
const ALARM: &[u8] = include_bytes!("../../assets/alarm.mp3");

/// Something the user did that the countdown needs to react to.
enum Event {
    /// The user pressed CTRL+C.
    Interrupt,
    /// The user entered a line of text and pressed Enter.
    Line(String),
    /// Standard input was closed.
    InputClosed,
}

/// How the countdown finished.
enum Outcome {
    Completed,
    Ended,
}

/// Starts a Pomodoro, records it, and plays an alarm when it completes.
pub fn start() -> Result<(), Box<dyn Error>> {
    let database = db::Database::open_default()?;
    let pomodoro_id = database.start_pomodoro()?;

    let (tx, rx) = mpsc::channel();

    let interrupt_tx = tx.clone();
    ctrlc::set_handler(move || {
        let _ = interrupt_tx.send(Event::Interrupt);
    })?;

    // Read standard input on its own thread so that the countdown keeps
    // running while the user decides how to answer the prompt.
    thread::spawn(move || {
        let stdin = io::stdin();
        loop {
            let mut line = String::new();
            let event = match stdin.read_line(&mut line) {
                Ok(0) | Err(_) => Event::InputClosed,
                Ok(_) => Event::Line(line),
            };
            let closed = matches!(event, Event::InputClosed);
            if tx.send(event).is_err() || closed {
                break;
            }
        }
    });

    match countdown(POMODORO_SECONDS, &rx)? {
        Outcome::Completed => {
            database.complete_pomodoro(pomodoro_id)?;
            play_alarm()?;
        }
        Outcome::Ended => {}
    }
    Ok(())
}

/// How the user answered the confirmation prompt.
#[derive(Debug, PartialEq)]
enum Answer {
    Yes,
    No,
    Invalid,
}

/// Interprets a line typed at the prompt. The localized answers are accepted
/// along with the English `y` and `n`, so a partially translated locale stays
/// usable. An empty answer (just Enter) selects the default, no.
fn classify_answer(line: &str, yes: &str, no: &str) -> Answer {
    let answer = line.trim().to_lowercase();
    if answer == yes || answer == "y" {
        Answer::Yes
    } else if answer.is_empty() || answer == no || answer == "n" {
        Answer::No
    } else {
        Answer::Invalid
    }
}

/// Counts down from `total` seconds, showing the time remaining once per
/// second.
///
/// If the user presses CTRL+C, a confirmation prompt is shown below the time
/// while the countdown continues. Answering `y` ends the Pomodoro; answering
/// `n` removes the prompt and the countdown carries on.
fn countdown(total: u64, events: &Receiver<Event>) -> io::Result<Outcome> {
    let start = Instant::now();
    let mut out = stdout().lock();
    let mut prompting = false;
    // Fluent trims trailing whitespace, so the space after the prompt is added
    // here.
    let prompt = format!("{} ", fl!("pomodoro-end-prompt"));
    let yes = fl!("pomodoro-answer-yes").to_lowercase();
    let no = fl!("pomodoro-answer-no").to_lowercase();

    for elapsed in 0..=total {
        let remaining = total - elapsed;
        // "\x1b[K" clears to end of line so a shorter time (e.g. "9:59" after
        // "10:00") doesn't leave a stale trailing character.
        if prompting {
            // The time is on the line above the prompt. Save the cursor, which
            // is wherever the user is typing, redraw the time, and restore it.
            write!(
                out,
                "\x1b7\x1b[1A\r{}:{:02}\x1b[K\x1b8",
                remaining / 60,
                remaining % 60
            )?;
        } else {
            write!(out, "\r{}:{:02}\x1b[K", remaining / 60, remaining % 60)?;
        }
        out.flush()?;

        if remaining == 0 {
            break;
        }

        // Wait for the next whole-second tick (avoiding cumulative drift)
        // while handling anything the user does in the meantime.
        let next_tick = start + Duration::from_secs(elapsed + 1);
        loop {
            let timeout = next_tick.saturating_duration_since(Instant::now());
            let event = match events.recv_timeout(timeout) {
                Ok(event) => event,
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => {
                    thread::sleep(timeout);
                    break;
                }
            };

            match event {
                Event::Interrupt if !prompting => {
                    prompting = true;
                    write!(out, "\n{prompt}")?;
                    out.flush()?;
                }
                Event::Interrupt => {}
                Event::Line(_) if !prompting => {}
                Event::Line(line) => match classify_answer(&line, &yes, &no) {
                    Answer::Yes => {
                        // Move up over the time and prompt and clear them.
                        write!(out, "\x1b[2A\r\x1b[J")?;
                        out.flush()?;
                        return Ok(Outcome::Ended);
                    }
                    // An empty answer (just Enter) selects the default, "n".
                    Answer::No => {
                        prompting = false;
                        // Clear the prompt, leaving the cursor on the time.
                        write!(out, "\x1b[1A\r\x1b[J\x1b[1A\r")?;
                        out.flush()?;
                        // Redraw the time immediately on its own line.
                        let remaining = total.saturating_sub(start.elapsed().as_secs());
                        write!(out, "{}:{:02}\x1b[K", remaining / 60, remaining % 60)?;
                        out.flush()?;
                    }
                    Answer::Invalid => {
                        // Invalid answer: replace it with a fresh prompt.
                        write!(out, "\x1b[1A\r\x1b[J{prompt}")?;
                        out.flush()?;
                    }
                },
                Event::InputClosed => {
                    // No way to answer, so carry on as if the user said no.
                    if prompting {
                        prompting = false;
                        write!(out, "\r\x1b[J\x1b[1A\r")?;
                        out.flush()?;
                    }
                }
            }
        }
    }

    if prompting {
        // The Pomodoro finished while the prompt was showing; remove it.
        write!(out, "\r\x1b[J")?;
    } else {
        writeln!(out)?;
    }
    Ok(Outcome::Completed)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_answers() {
        assert_eq!(classify_answer("y\n", "y", "n"), Answer::Yes);
        assert_eq!(classify_answer(" N ", "y", "n"), Answer::No);
        assert_eq!(classify_answer("\n", "y", "n"), Answer::No);
        assert_eq!(classify_answer("maybe", "y", "n"), Answer::Invalid);
    }

    #[test]
    fn accepts_localized_and_english_answers() {
        assert_eq!(classify_answer("Oui", "oui", "non"), Answer::Yes);
        assert_eq!(classify_answer("non", "oui", "non"), Answer::No);
        assert_eq!(classify_answer("y", "oui", "non"), Answer::Yes);
    }
}
