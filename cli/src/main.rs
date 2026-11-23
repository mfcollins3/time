// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

mod commands;
mod db;

use std::error::Error;

use clap::{Parser, Subcommand};

/// Naked Time helps you plan your day and track the time you spend on
/// important activities.
#[derive(Parser)]
#[command(name = "naked-time", version, about, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Work with Pomodoros
    #[command(subcommand, arg_required_else_help = true)]
    Pomodoro(PomodoroCommand),
}

#[derive(Subcommand)]
enum PomodoroCommand {
    /// Start a 25-minute Pomodoro and record it
    Start,
}

fn main() -> Result<(), Box<dyn Error>> {
    match Cli::parse().command {
        Command::Pomodoro(PomodoroCommand::Start) => commands::pomodoro::start(),
    }
}
