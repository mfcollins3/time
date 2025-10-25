---
mode: agent
model: Claude Sonnet 4.5 (copilot)
description: Creates a GitHub Actions workflow to build and publish a Docker container containing Time.
---

Create a GitHub Actions workflow that builds and publishes a Docker container containing the Time executable.

- The workflow should use the release artifacts produced by the `linux-builds.yml` workflow. Make any changes necessary to make `linux-builds.yml` reusable.
- The workflow should produce Docker container images for both ARM64 and AMD64 architectures, and then produce a multi-platform container image supporting both platforms.
- Set the following environment variables in the `Dockerfile`:
    - `TIME_NO_SOUND=1`: disables playing the alarm when the pomodoro completes
    - `TIME_NO_DESKTOP_NOTIFICATION=1`: disables desktop notifications
- Use the `gcr.io/distroless/base-debian12` distroless base image for the container image.
- Create a `.dockerignore` file to ignore all files that are not needed in the container image.
- Store the `time` executable in the `/opt/time/bin` directory.
- Set the `/opt/time/bin/time` executable as the entry point for the container.
- Set the `/opt/time` directory as the working directory for the container.
