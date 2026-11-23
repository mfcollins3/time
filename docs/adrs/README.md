<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Architectural Decision Records

| ADR | Status | Description |
|-----|--------|-------------|
| [Build and publish release binaries on native runners for five targets](draft-release-build-targets.md) | proposed | Native GitHub runners per target, glibc (no musl), `.deb`/`.rpm` packages, `naked-time` binary name, reusable build workflow. |
| [Sign and notarize macOS builds with Developer ID](draft-sign-and-notarize-macos-builds.md) | proposed | Developer ID signing, notarized `.tar.gz` plus stapled `.pkg`, App Store Connect API key, signing only on `main` and releases. |
| [Distribute Naked Time through a personal Homebrew tap](draft-distribute-via-homebrew-tap.md) | proposed | `mfcollins3/homebrew-tap` formula installing prebuilt binaries, updated automatically from release CI, Apple silicon only on macOS. |
| [Publish product documentation with Astro Starlight on GitHub Pages](draft-publish-docs-with-astro-starlight.md) | proposed | Docs site in `website/` built with Astro Starlight and pnpm, deployed to GitHub Pages at `/time/` by a `docs` workflow that builds on PRs and deploys from `main`. |
| [Record Pomodoros in a local SQLite database](draft-record-pomodoros-in-sqlite.md) | proposed | Bundled SQLite at `~/.naked-time/data.db`, integer key plus UUIDv7 public id, UTC ISO-8601 timestamps, NULL end time for cancelled or killed Pomodoros. |
| [Use clap for CLI parsing and subcommands](draft-use-clap-for-cli-subcommands.md) | proposed | `clap` derive API, noun-verb commands (`naked-time pomodoro start`), bare invocation prints help. |
| [Localize the CLI with Project Fluent](draft-localize-cli-with-fluent.md) | proposed | Fluent `.ftl` files embedded via `i18n-embed`, compile-time checked `fl!`, runtime-localized clap help, `NAKED_TIME_LANG` and pseudo-locale, `en-US` only initially. |
