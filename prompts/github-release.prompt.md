---
mode: agent
model: Claude Sonnet 4.5 (copilot)
description: Generates a GitHub Actions workflow to generate a GitHub release for the Time product.
---

Create a GitHub Actions workflow to automate the generation of a GitHub release for the Time product.

- Trigger on the creation of and changes to a pull request for a release branch.
    - A release branch has the prefix `release/`
    - A release branch includes the version number for the release. For example, `release/1.2.3` is for version `1.2.3`
        - The version number is a semantic version.
        - The first component `1` is the major version number.
        - The second component `2` is the minor version number.
        - The third component `3` is the patch version number.
- When the pull request for the release branch is created, create a draft GitHub release.
- When the pull request for the release branch is merged into the `main` branch, create a final GitHub release.
- On each push to the release branch:
    - Build the product
        - Use the `linux-builds.yml` workflow to build the Linux executables
        - Use the `macos-builds.yml` workflow to build the macOS executable
        - Use the `windows-builds.yml` workflow to build the Windows executable
        - Use the `docker-container.yml` workflow to build and publish the container image to GitHub Packages
    - Update the GitHub release
        - Update the release notes
        - Upload the binaries to the release
           - Use the format `time-<version>-<platform>-<architecture>` for release binary archives.
           - Use the format `time-<version>-<platform>-<architecture>-debug` for debug binary archives.
           - Distribute the binaries as a `.tar.gz` for Linux or `.zip` for Windows and Apple macOS
