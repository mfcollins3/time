---
status: "proposed"
date: 2026-11-21
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Build and publish release binaries on native runners for five targets

## Context and Problem Statement

Naked Time is a Rust program that plays an alarm through `rodio`, which uses
`cpal` and, on Linux, ALSA (`alsa-sys`). Releases are managed by
release-please, but no workflow builds the program or attaches binaries to a
release. Which platforms do we support, how do we build them, and how do we
package and name the results?

## Decision Drivers

* Support Windows (x64, ARM64), macOS (ARM64 only) and Linux (x64, ARM64).
* Avoid cross-compiling, because `alsa-sys` needs target-specific ALSA headers.
* Audio must work on typical Linux desktops (PipeWire/PulseAudio via ALSA).
* Builds must run on pull requests and `main`, and release builds must be
  attached to the GitHub release created by release-please.
* Linux users should be able to install with their package manager.

## Considered Options

* Native GitHub-hosted runners per target, glibc on Linux
* Cross-compile all targets from one runner
* Add musl-based static Linux builds

## Decision Outcome

Chosen option: "Native GitHub-hosted runners per target, glibc on Linux",
because it avoids cross-compilation and gives working audio on desktop Linux.

* **Runners** (the repository is public, so ARM runners are free):

  | Target                      | Runner             |
  |-----------------------------|--------------------|
  | `x86_64-pc-windows-msvc`    | `windows-latest`   |
  | `aarch64-pc-windows-msvc`   | `windows-11-arm`   |
  | `aarch64-apple-darwin`      | `macos-latest`     |
  | `x86_64-unknown-linux-gnu`  | `ubuntu-22.04`     |
  | `aarch64-unknown-linux-gnu` | `ubuntu-22.04-arm` |

* **glibc, not musl.** ALSA loads its plugins (including the PipeWire and
  PulseAudio bridges) with `dlopen`, which a fully static musl binary cannot do.
  Building on Ubuntu 22.04 (glibc 2.35) keeps the binary compatible with current
  mainstream distributions. musl can be revisited if Alpine support is requested.
* **Linux packages.** Besides tarballs, Linux builds produce `.deb`
  (`cargo-deb`) and `.rpm` (`cargo-generate-rpm`) packages that declare the
  ALSA runtime dependency.
* **Binary name.** The binary is named `naked-time` (`[[bin]]` in `Cargo.toml`)
  because `time` conflicts with `/usr/bin/time` from the GNU `time` package and
  with the shell `time` keyword. The crate name remains `time`.
* **Workflow structure.** A reusable workflow, `build.yml`, performs the build.
  `ci.yml` calls it (plus fmt and clippy) for pull requests and pushes to
  `main`, uploading workflow artifacts. `release-please.yml` calls it with the
  release tag when release-please creates a release, and the build uploads the
  archives, packages and SHA-256 checksums to that release.

### Consequences

* Good, because there is no cross-compilation toolchain to maintain.
* Good, because the same build definition is used for CI and releases.
* Good, because Linux users get dependency-aware packages.
* Bad, because the Linux binaries are not static and need `libasound.so.2`.
* Bad, because Alpine/musl users have no prebuilt binary.
* Bad, because the build relies on GitHub's ARM runner images being available.

### Confirmation

Pull request builds must pass on all five targets. After a release, the release
page must show five archives plus two `.deb` and two `.rpm` files, each with a
`.sha256` file. The `.deb` and `.rpm` can be installed in Ubuntu and Fedora
containers to confirm `naked-time` is placed on the `PATH`.

## Pros and Cons of the Options

### Native GitHub-hosted runners per target, glibc on Linux

* Good, because the build is the same as a developer's local build.
* Good, because audio works with the system sound server.
* Bad, because it needs one matrix job per target.

### Cross-compile all targets from one runner

* Good, because fewer runner types are used.
* Bad, because cross-compiling ALSA and the MSVC targets is fragile.

### Add musl-based static Linux builds

* Good, because the binary has no glibc version requirement.
* Bad, because static linking prevents ALSA plugin loading, so audio may fail.
* Bad, because a static `libasound` would have to be built.

## More Information

* [release-please action](https://github.com/googleapis/release-please-action)
* [Sign and notarize macOS builds with Developer ID](draft-sign-and-notarize-macos-builds.md)
  extends this decision for the macOS target.
