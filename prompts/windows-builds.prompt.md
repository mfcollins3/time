---
mode: agent
model: Claude Sonnet 4.5 (copilot)
description: Creates the GitHub Actions workflow to generate Time builds for Microsoft Windows
---

Create a GitHub Actions workflow to produce Linux builds of Time.

- The workflow should have the following triggers:
    - `push` events on the `main` branch
    - pull requests for the `main` branch
    - `workflow_dispatch` events
- The workflow should build the Windows binaries for both ARM64 and x64 architectures
- ARM64 and x64 builds should run on separate jobs to run concurrently
- The workflow should cache Go dependencies to improve build times
- The Time product uses SQLite, so CGO should be enabled
- The workflow should produce both debug and smaller release builds of the Time programs
    - Release builds should use the `strip` command to remove debug symbols from the debug builds
- The workflow should build for x64 only using the `windows-2025` runner.
- Produce debug and release builds for each architecture. The release builds should use the `strip` command or linker flags to remove debug symbols from the debug builds.
- Keep the output executable as `time`. Use output subdirectories to separate the debug and release builds for each architecture.
