---
status: "proposed"
date: 2026-11-23
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Localize the CLI with Project Fluent

## Context and Problem Statement

All text shown by `naked-time` was hard-coded in English, in Rust source and in
clap's generated help. We want to provide translations of everything shown to
the user. How should user-facing text be stored, selected, and looked up?

## Decision Drivers

* Translators should work on plain resource files, not Rust code.
* Translations must work with plurals, gender, and grammar that differ between
  languages.
* Missing or misspelled message IDs should be caught at build time.
* No files to install at runtime and no system libraries on any of the five
  release targets (see [Build and publish release binaries on native runners
  for five targets](draft-release-build-targets.md)).
* clap's help output is part of the user interface and must be localizable.

## Considered Options

* Project Fluent with `i18n-embed` and `i18n-embed-fl`
* `rust-i18n`
* gettext (`gettext-rs`)

## Decision Outcome

Chosen option: "Project Fluent with `i18n-embed`", because Fluent handles
grammatical variation well, `fl!` checks message IDs and arguments at compile
time, and resources are embedded in the binary with `rust-embed`.

* Resources live in `cli/i18n/<language>/time.ftl`. `en-US` is the fallback.
* The language comes from `NAKED_TIME_LANG` or the operating system, resolved
  by `i18n-embed`'s desktop requester. `NAKED_TIME_PSEUDO=1` pseudo-localizes
  all text to expose hard-coded strings.
* clap's derive API is used for parsing only. Descriptions, headings, and the
  help and version flag text are applied at runtime from the resources. clap's
  help subcommand is disabled because its text cannot be overridden;
  `--help` works on every command.
* clap's parse error messages remain in English for now.
* Unicode bidi isolation marks are disabled because terminals can render them
  as stray characters.
* Accepted answers to the end-Pomodoro prompt are the localized yes and no
  values and always the English `y` and `n`.
* Only `en-US` ships initially.

### Consequences

* Good, because adding a language needs only a new `.ftl` file.
* Good, because a missing ID or argument is a compile error.
* Good, because resources are embedded, so nothing extra is packaged.
* Bad, because clap's parse errors are still English.
* Bad, because the help template and flags are rebuilt by hand, which must be
  maintained as clap evolves.
* Bad, because several dependencies are added.

### Confirmation

Unit tests render the help for each command and check the localized text, check
that arguments are substituted, and check that pseudo-localization changes text.
Running with `NAKED_TIME_PSEUDO=1` should show no English text that we own.

## Pros and Cons of the Options

### Project Fluent with `i18n-embed`

* Good, because it supports plurals, gender, and parameterized messages.
* Good, because of compile-time checking with `fl!`.
* Bad, because it adds more dependencies than the alternatives.

### `rust-i18n`

* Good, because it is the simplest to wire up.
* Bad, because its plural and grammar support is weaker.

### gettext

* Good, because translators and tools know the `.po` format.
* Bad, because it needs a system library or build step, which complicates
  cross-platform builds.
