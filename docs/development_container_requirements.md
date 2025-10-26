# Software Requirements for Development Container Development

This document lists the additional software that needs to be installed for developing Time using a [development container](https://containers.dev). The use of the development container is the preferred development platform for building and working with Time. The development container provides a stable Linux-based environment that includes all development tools, programming language compilers, and libraries installed and configured for use.

The development container is compatible with:

- [Visual Studio Code](https://code.visualstudio.com)
- [JetBrains IDEs](https://jetbrains.com)
- [GitHub Codepsaces](https://github.com/features/codespaces)

This document will describe the requirements to use Visual Studio Code for hosting and running the development container.

The following software is required to run the development container using Visual Studio Code:

1. [Visual Studio Code](#visual-studio-code)
1. [Remote Development Extension Pack for Visual Studio Code](#remote-development-extension-pack-for-visual-studio-code)

## Visual Studio Code

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Visual Studio Code](https://code.visualstudio.com) is an open source text editor and development environment for software developers. Visual Studio Code is a highly extensible platform and boasts a large ecosystem of extensions that add support for programming languages, software development tools, and other tools and applications.

To install Visual Studio Code, download and run the installer from the [website](https://code.visualstudio.com).

## Remote Development Extension Pack for Visual Studio Code

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

The [Remote Development Extension Pack for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) is a set of extensions that support development on remote machines or in virtual environments hosted on local machines. The Remote Development Extension Pack includes support for building and running development containers using [Docker Desktop](software_requirements.md#docker-desktop).

The Remote Development Extension Pack can be installed from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack).
