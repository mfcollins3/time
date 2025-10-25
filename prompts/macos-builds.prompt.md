---
mode: agent
model: Claude Sonnet 4.5 (copilot)
description: Create a GitHub Actions workflow that will build the Time programs for Apple macoS users.
---

Create a GitHub Actions workflow that will build the Time programs for Apple macOS users.

- The workflow should be triggered by a repository dispatch event with the type `build-macos-command`.
    - Update the `slash-command-dispatch.yml` workflow to add the `/build-macos` command.
- The workflow can also be triggered by a `workflow_dispatch` event.
- We are only supporting the ARM64 architecture on Apple macOS.
    - Use the ARM runners for GitHub Actions.
- Enable CGO for Go builds. We are using SQlite and this is necessary.
- Notarize the executable programs using Apple Notarization. Guide me through the steps required to set up Apple Notarization.
- Cache Go dependencies to improve build time.
- Produce debug and release builds for each architecture. The release builds should use the `strip` command or linker flags to remove debug symbols from the debug builds.
- Keep the output executable as `time`. Use output subdirectories to separate the debug and release builds for each architecture.
