---
title: Installation
description: Install Naked Time on macOS, Linux, or Windows.
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

## Homebrew (macOS and Linux)

On macOS (Apple silicon) and Linux (x64, ARM64), install with
[Homebrew](https://brew.sh):

```sh
brew install mfcollins3/tap/naked-time
```

On Linux, Naked Time also needs the ALSA library (`libasound2`) from your
system package manager.

## Download a release

Download the package for your platform from the
[releases page](https://github.com/mfcollins3/time/releases). Each file has a
matching `.sha256` checksum.

### Windows (x64, ARM64)

Download and extract the `.zip` archive.

### macOS (Apple silicon)

Open the signed and notarized `.pkg` installer, which installs `naked-time`
into `/usr/local/bin`. You can also extract the `.tar.gz` archive, which
contains the same signed and notarized binary. The first launch of the
extracted binary requires an internet connection so that macOS can verify the
notarization.

To uninstall the package, run:

```sh
sudo rm /usr/local/bin/naked-time && sudo pkgutil --forget software.naked.naked-time
```

### Linux (x64, ARM64)

Install the `.deb`:

```sh
sudo apt install ./naked-time-*.deb
```

or the `.rpm`:

```sh
sudo dnf install ./naked-time-*.rpm
```

Both packages also install the ALSA audio library. You can also extract the
`.tar.gz` archive, which requires `libasound2` (ALSA).

## Run Naked Time

Run the program with:

```sh
naked-time
```
