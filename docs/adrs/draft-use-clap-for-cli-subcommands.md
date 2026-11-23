---
status: "proposed"
date: 2026-11-23
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Use clap for CLI parsing and subcommands

## Context and Problem Statement

The `naked-time` binary hand-parsed `--version` and otherwise started a
Pomodoro immediately. As Naked Time gains more features (history, reports,
other activities), it needs a structured command line with subcommands, help
text, and consistent error handling. How should the CLI be parsed and organized?

## Decision Drivers

* Support multiple subcommands without writing argument parsing by hand.
* Generated `--help`, `--version`, and usage errors.
* Works on all five release targets with no extra runtime dependencies.
* Keep `naked-time --version` output compatible with the Homebrew formula test.

## Considered Options

* `clap` with the derive API
* `clap` with the builder API
* Hand-written argument parsing

## Decision Outcome

Chosen option: "`clap` with the derive API", because it is the de facto standard
for Rust CLIs, generates help and errors, and keeps the command structure
declarative.

* Commands follow a noun-verb structure: `naked-time pomodoro start`. The
  previous behavior of recording a Pomodoro moved to `pomodoro start`.
* Running `naked-time` or `naked-time pomodoro` with no subcommand prints help
  and exits with a non-zero status (`arg_required_else_help`).
* The clap definitions live in `src/main.rs`; each command's implementation lives
  in `src/commands/`.

### Consequences

* Good, because new commands are added by declaring an enum variant and a handler.
* Good, because help, version, and usage errors are consistent.
* Bad, because `naked-time` alone no longer starts a Pomodoro, a breaking change
  for existing users.
* Bad, because clap adds compile time and some binary size.

### Confirmation

CI builds and lints the binary; manual verification of `--help`, `--version`,
and `pomodoro start`. The Homebrew formula test runs `naked-time --version`.

## Pros and Cons of the Options

### `clap` with the derive API

* Good, because it is concise and type-checked.
* Bad, because it relies on proc macros, which lengthen the build.

### `clap` with the builder API

* Good, because it avoids proc macros.
* Bad, because it is more verbose and loses compile-time typing.

### Hand-written argument parsing

* Good, because there is no dependency.
* Bad, because help, errors, and subcommands must all be implemented and
  maintained by hand.
