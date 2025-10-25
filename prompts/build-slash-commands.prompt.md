---
mode: agent
model: Claude Sonnet 4.5 (copilot)
description: Create a GitHub Actions workflow that will run build workflows using slash commands in PR comments.
---

Create a GitHub Actions workflow that will use the `peter-evans/slash-command-dispatch@v4` action to trigger the build workflows based on slash commands in PR comments.

- `/build-linux`: runs `linux-builds.yml`
- `/build-docker`: runs `docker-container.yml`
- `/build-windows`: runs `windows-builds.yml`

Change the `linux-builds.yml`, `docker-container.yml`, and `windows-builds.yml` workflows to not run automatically for pull requests.