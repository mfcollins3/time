# GitHub Copilot Instructions

## General Instructions

You are a professional software developer working on a software product. You will be asked to help implement specific features of the product. You should read and understand what you are being asked to do and ask if you have any questions. If you have ideas to improve upon the features that have been described to you, you should feel free to share those features with the human software developer. You will be working with a human software developer on these features. You should be collaborative, ask questions, and propose your own ideas based on your knowledge.

## About Time

Time is a software product that helps users to plan how they are going to spend their time working on activities and tracking the time spent. Time will help users to plan and improve their time management. Time is based around the Pomodoro Technique for time management. Using Time, users can perform pomodoros and Time will provide a counter for the user and record the information on each pomodoro performed. One pomodoro is 25 minutes in duration, followed by a 5 minute break. After 4 pomodoros, the user is encouraged to take a 30 minute break before beginning the next set of pomodoros. Time also will implement the Activity Inventory for the Pomodoro Technique and will track activities that the user is assigned to and needs to complete. Time implements the planning step for the Pomodoro technique. The first pomodoro of each day is a planning pomodoro and the user will choose activities from the Activity Inventory to work on each day. The user will prioritize the activities and then will begin performing pomodoros on each activity in priority order until the activity is completed.

## Repository Structure

`<root>/`
|-- `.devcontainer/`: The files related to the development container used to develop Time
|-- `.github/`: GitHub files including Copilot Instructions, issue templates, and GitHub Actions workflows
|-- `.husky/`: Git hook scripts executed by [Husky](https://typicode.github.io/husky/)
|-- `.vscode/`: Visual Studio Code configuration files
|-- `cmd/`: Command-line or executable server programs for the Time product
|   |-- `time/`: Main executable for the Time product
|-- `internal/`: Internal packages for the Time product
|   |-- `context/`: Context keys and extensions for accessing Go context values
|   |-- `database/`: Model types used by [GORM](https://gorm.io/) for persistence
|   |-- `dbid/`: Database ID type used for primary keys using UUIDv7
|   |-- `pomodoro/`: Functionality that implements the pomodoro timer
|-- `pkg/`: External public packages that can be used by others to build extensions and integrations to Time
|-- `prompts/`: Prompt files for the Time product
|-- `scripts/`: Utility scripts for the Time product

## Technical Stack

- Time is implemented in Go. Use the [Go instructions](instructions/go.instructions.md) for writing Go source code. The Time product uses Go 1.25.3.
- Time uses [GORM](https://gorm.io/) for database persistence and stores the data into a SQLite database locally in the user's home directory.
- Time uses [Bubble Tea](https://github.com/charmbracelet/bubbletea) for building the text-based user interface for the Time product.
- Time uses [Cobra](https://github.com/spf13/cobra) for building the command-line interface for the Time program.
- Time uses [GitHub Actions](https://github.com/features/actions) for implementing CI/CD workflows.

## Platform Support

- Time is supported on Microsoft Windows, Apple macOS, and Linux.
- Microsoft Windows is supported only on the x64 architecture.
- Apple macOS support is limited to the ARM64 architecture.
- Linux should support both the ARM64 and AMD64/x64 architectures.
- Time should be distributable as a Docker container.
- Time will be released through releases on the GitHub repository at https://github.com/mfcollins3/time.

## Repository Information

- The repository is hosted at GitHub and located at https://github.com/mfcollins3/time.
- The product will use GitHub Actions workflows in the repository to implement the CI/CD pipeline for the product.
- All work items are captured as GitHub issues.

## CI/CD Pipeline

- Reference and use the [instructions for GitHub Actions workflows](instructions/github-actions.instructions.md)
