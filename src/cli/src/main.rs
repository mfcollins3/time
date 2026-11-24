// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

mod commands;
mod db;
mod i18n;

use std::process::ExitCode;

use clap::builder::styling::Style;
use clap::{
    Arg, ArgAction, Command as ClapCommand, CommandFactory, FromArgMatches, Parser, Subcommand,
};

use crate::commands::pomodoro::POMODORO_SECONDS;
use crate::i18n::fl;

// User-facing text is not set here. It is applied from the localized
// resources in `localized_command`.
#[derive(Parser)]
#[command(name = "naked-time", version, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(subcommand, arg_required_else_help = true)]
    Pomodoro(PomodoroCommand),
}

#[derive(Subcommand)]
enum PomodoroCommand {
    Start,
}

/// Builds the command line definition with all text localized, including
/// clap's own help scaffolding.
fn localized_command() -> ClapCommand {
    localize(Cli::command(), true)
}

fn localize(cmd: ClapCommand, root: bool) -> ClapCommand {
    let about = match cmd.get_name() {
        "naked-time" => Some(fl!("app-about")),
        "pomodoro" => Some(fl!("cmd-pomodoro-about")),
        "start" => {
            let minutes: u64 = POMODORO_SECONDS / 60;
            Some(fl!("cmd-pomodoro-start-about", minutes = minutes))
        }
        _ => None,
    };

    let header = Style::new().bold().underline();
    let template = format!(
        "{{about-with-newline}}\n{header}{}{header:#} {{usage}}\n\n{{all-args}}{{after-help}}",
        fl!("help-usage-heading")
    );

    let names: Vec<String> = cmd
        .get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect();

    let mut cmd = cmd
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .subcommand_help_heading(fl!("help-commands-heading"))
        .next_help_heading(fl!("help-options-heading"))
        .subcommand_value_name(fl!("help-command-placeholder"))
        .help_template(template)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::Help)
                .global(false)
                .help(fl!("help-flag-help")),
        );
    if let Some(about) = about {
        cmd = cmd.about(about);
    }
    if root {
        cmd = cmd.disable_version_flag(true).arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(ArgAction::Version)
                .help(fl!("help-flag-version")),
        );
    }
    for name in names {
        cmd = cmd.mut_subcommand(name, |sub| localize(sub, false));
    }
    cmd
}

fn main() -> ExitCode {
    i18n::init();

    let matches = localized_command().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    let result = match cli.command {
        Command::Pomodoro(PomodoroCommand::Start) => commands::pomodoro::start(),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", fl!("error-prefix", message = error.to_string()));
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn help(args: &[&str]) -> String {
        localized_command()
            .try_get_matches_from(args)
            .expect_err("help should exit early")
            .to_string()
    }

    #[test]
    fn command_definition_is_valid() {
        localized_command().debug_assert();
    }

    #[test]
    fn root_help_is_localized() {
        let text = help(&["naked-time", "--help"]);
        assert!(text.contains("Usage:"), "{text}");
        assert!(text.contains("Commands:"), "{text}");
        assert!(text.contains("Options:"), "{text}");
        assert!(text.contains("Work with Pomodoros"), "{text}");
        assert!(text.contains("Print help"), "{text}");
        assert!(text.contains("Print version"), "{text}");
        assert!(!text.contains("Print this message"), "{text}");
    }

    #[test]
    fn subcommand_help_is_localized() {
        let text = help(&["naked-time", "pomodoro", "start", "--help"]);
        assert!(
            text.contains("Start a 25-minute Pomodoro and record it"),
            "{text}"
        );
        assert!(text.contains("Print help"), "{text}");
    }
}
