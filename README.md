# Naked Time

1. [About Naked Time](#about-naked-time)
1. [Installing](#installing)
1. [Project layout](#project-layout)
1. [Getting started](#getting-started)
1. [License](#license)

## About Naked Time

Naked Time is a time planning and tracking application. Naked Time's goal is to
make you more productive in completing value-building activities by allowing you
to hyperfocus on what is important and tracking how much time you spend
completing activities.

Naked Time implements the
[Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique), a
popular time management technique where work is broken down into 25-minute
intervals followed by a short break. The Pomodoro Technique is intended to help
you to avoid burnout by time-boxing work into smaller intervals and
incrementally completing assigned tasks. The Pomodoro Technique helps you to
channel your thoughts and focus on working interruption free on completing
activities. The Pomodoro Technique measures and records the results of your
work, allowing you to review your performance and optimize or improve your use
of time.

Naked Time is targeted for use by:

* students;
* individuals;
* solo software developers;
* independent contractors, freelancers, or gig workers who charge based on time
  worked;
* small software teams;
* businesses of all sizes that need to track their employee's time.

## Installing

Download the package for your platform from the
[releases page](https://github.com/mfcollins3/time/releases). Each file has a
matching `.sha256` checksum.

* **Windows (x64, ARM64)**: download and extract the `.zip` archive.
* **macOS (Apple silicon)**: open the signed and notarized `.pkg` installer,
  which installs `naked-time` into `/usr/local/bin`. You can also extract the
  `.tar.gz` archive, which contains the same signed and notarized binary; the
  first launch of the extracted binary requires an internet connection so that
  macOS can verify the notarization. To uninstall the package, run
  `sudo rm /usr/local/bin/naked-time && sudo pkgutil --forget software.naked.naked-time`.
* **Linux (x64, ARM64)**: install the `.deb`
  (`sudo apt install ./naked-time-*.deb`) or the `.rpm`
  (`sudo dnf install ./naked-time-*.rpm`), which also installs the ALSA audio
  library. You can also extract the `.tar.gz` archive, which requires
  `libasound2` (ALSA).

Run the program with `naked-time`.

## Project layout

```plain
(root)/                             Root directory of the Naked Time repository
|-- .claude/                        Claude code settings and customizations
|   |-- settings.json               Custom settings for Claude Code when working with the Naked Time project
|-- .github/                        GitHub-specific customizations and GitHub Actions workflows
|   |-- workflows/                  GitHub Actions workflows
|   |   |-- build.yml               Reusable workflow that builds and packages Naked Time for all supported platforms
|   |   |-- ci.yml                  Runs formatting, lint, and build checks for pull requests and pushes to main
|   |   |-- release-please.yml      GitHub Actions workflow for managing releases using release-please
|-- .vscode/                        Settings and customizations for Visual Studio Code
|   |-- settings.json               Custom settings for the Naked Time workspace in Visual Studio Code
|-- assets/                         Assets and resources for the Naked Time application
|   |-- alarm.mp3                   Alarm sound that plays when a Pomodoro completes
|-- docs/                           Project documentation
|   |-- adrs/                       Architectural decision records
|-- src/                            Source code for the Naked Time application
|   |-- main.rs                     Main program for Naked Time's CLI/TUI application
|-- .gitattributes                  Attributes that controls how Git treats files in the repository
|-- .gitignore                      Specifies which files and directories Git should not include in the repository
|-- .release-please-manifest.json   Stores the current component version numbers; managed by release-please
|-- Cargo.lock                      Stores information on Rust crates used by Naked Time and what versions are currently in use
|-- Cargo.toml                      Rust manifest for the Naked Time product
|-- CLAUDE.md                       Instructions that are used to customize how Claude Code works for Naked Time
|-- LICENSE.md                      The license agreement for the Naked Time product and the source code
|-- README.md                       (This document) Introductory documentation about Naked Time and how to get started working with the source code and running Naked Time
|-- release-please-config.json      Configuration settings for release-please to control how releases are managed and generated
```

## Getting started

This section has instructions on how to clone the repository and build and run
the Naked Time application.

### Prerequisites

* [Git](https://git-scm.com)
* [GitHub CLI](https://cli.github.com)
* [Rust](https://rust-lang.org/tools/install/)

### Clone the repository

The first step to work with Naked Time's source code is to clone the
[Git repository](https://github.com/mfcollins3/time) from 
[GitHub](https://github.com) onto your computer. Open a terminal, navigate to
the location where you want to host the repository, and run:

```bash
gh repo clone mfcollins3/time time/main
cd time/main
```

This command will clone the Naked Time Git repository and will store it in the
`src/main` subdirectory. Git will then check out the `main` branch.

### Build and run Naked Time

To build and run Naked Time, use the same terminal or open a new terminal and
navigate to the directory where the Git repository was cloned to and run:

```bash
cd time/main      # if necessary
cargo run
```

This command will run Naked Time (the `naked-time` binary) and will begin to record a Pomodoro. When the
Pomodoro is complete, an alarm will play.

## License

Please be sure to review the [license](LICENSE.md). Naked Time is being released
under a _source available_ license and not an open source license. Naked Time is
a commercial product that requires users to have a valid license to use the
product. Licensed users may also use Naked Time source code to customize the
product for their own use or use the source code to build integrations and
extensions for the Naked Time product.
