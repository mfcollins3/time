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
