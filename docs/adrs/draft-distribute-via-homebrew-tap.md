---
status: "proposed"
date: 2026-11-21
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Distribute Naked Time through a personal Homebrew tap

## Context and Problem Statement

Naked Time is published as GitHub release assets, which users must download and
install by hand. Homebrew is the common way to install command-line tools on
macOS and Linux. How do we make Naked Time installable with `brew install`, in a
way that can also serve other applications later?

This builds on [Build and publish release binaries on native runners for five
targets](draft-release-build-targets.md) and [Sign and notarize macOS builds
with Developer ID](draft-sign-and-notarize-macos-builds.md).

## Decision Drivers

* Installing and upgrading should be a single `brew` command.
* The signed and notarized macOS binary should be the one users run.
* Releases should stay hands-off.
* The tap should be reusable for future applications.
* Homebrew core submission is not an option for now because of the custom
  license and the single-maintainer project.

## Considered Options

* A personal tap (`mfcollins3/homebrew-tap`) with a formula that installs
  prebuilt binaries
* A personal tap with a formula that builds from source with `cargo`
* A personal tap with a cask that installs the notarized `.pkg`

## Decision Outcome

Chosen option: "A personal tap with a formula that installs prebuilt
binaries", because it installs quickly on macOS and Linux, uses the signed
release binary, and needs no Rust toolchain.

* The formula is generated from `packaging/homebrew/naked-time.rb.tmpl` by
  `scripts/update-homebrew-formula.sh` and pushed to the tap by a `homebrew`
  job in `release-please.yml` after each release, using a `HOMEBREW_TAP_TOKEN`
  secret scoped to the tap repository.
* The tap repository runs `brew style`, `brew audit`, and an install test on
  each change as the safety net for the direct push.
* macOS is Apple silicon only, matching the build matrix.
* On Linux the binary uses the system ALSA library; the formula shows a caveat
  instead of a `depends_on`, because Homebrew would not link a non-bottled
  binary against it.

### Consequences

* Good, because users get `brew install mfcollins3/tap/naked-time` and
  `brew upgrade`.
* Good, because releases update the tap with no manual steps.
* Bad, because Intel Macs are unsupported until an `x86_64-apple-darwin` build
  is added.
* Bad, because a leaked tap token could modify the formula, so it must be
  scoped to the tap repository only.
* Bad, because a failed release build leaves the formula on the prior version,
  which is intended, but a bad push is only caught after the fact by tap CI.

## Pros and Cons of the Options

### Formula with prebuilt binaries

* Good, because installs are fast and use the notarized binary.
* Good, because it covers macOS and Linux.
* Bad, because every release must regenerate checksums.

### Formula that builds from source

* Good, because it works on any architecture.
* Bad, because installs are slow and need the Rust toolchain.
* Bad, because the result is not the signed release binary.

### Cask for the `.pkg`

* Good, because it installs the notarized installer.
* Bad, because it is macOS only and requires `sudo`.
* Bad, because it installs outside Homebrew's prefix.
