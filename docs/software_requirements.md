# Software Requirements

Please read this document completely before cloning the repository and working with the Time source code. This document describes what software you need to have installed in order to build, run, debug, and develop the Time product.

There are two supported methods of development for the Time product:

1. Development on your local machine
2. Using a development container (recommended)

Time uses a development container to provide a stable and standardized development environment that is pre-configured with all software development tools, programming language compilers, and libraries or frameworks needed to build and run the Time software. The development container is recommended for developing the CLI tools, services, and APIs.

The software requirements are divided into the groups:

1. [Required Software](#required-software)
1. [Software Required for Local Development](local_development_requirements.md)
1. [Software Required for Development Container Development]()

## Development Platforms

Time supports developers working on Microsoft Windows, Linux, and Apple macOS. Not all software is required or compatible with all platforms. Each software package includes a table that indicates whether the software package is required for the platform. The following symbols are used to indicate whether or not a software package is required:

- :white_check_mark:: The software package is required on this operating system
- :grey_question:: The software package is optional on this operating system
- :x:: The software package is not supported or compatible with this operating system

> :exclamation: Windows Subsystem for Linux 2 on Microsoft Windows platforms is considered Linux. All requirements for Linux should be installed in the WSL2 deployment.

## Required Software

The following software is required for all Time developers:

1. [Homebrew](#homebrew)
1. [Git](#git)
1. [Git LFS](#git-lfs)
1. [GitHub CLI](#github-cli)
1. [Fast Node Manager](#fast-node-manager)
1. [Node.js](#nodejs)
1. [Docker Desktop](#docker-desktop)
1. [Visual Studio Code]()
1. [Remote Development Extension Pack for Visual Studio Code]()

### Homebrew

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :x: |

[Homebrew](https://brew.sh) is a package management tool for Apple macOS and Linux. Homebrew can be used to install many popular applications and libraries and keep them up-to-date as new changes are released. The Time development team uses Homebrew as much as possible to manage external dependencies.

To install Homebrew, open a terminal and run:

    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

After installing Homebrew, you will need to close and restart your terminal for the environment changes to take effect.

### Git

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Git](https://git-scm.com) is a distributed version control system. Git is used to manage the source code for the Time product and to track changes over time. Git supports several different software development workflows and can help to facilitate collaboration between developers. As a distributed version control system, each developer maintains their own clone of the Git repository. All development is performed and persisted locally. Developers can share changes with each other either directly, or by using a shared repository such as the [GitHub repository](https://github.com/mfcollins3/time).

- __Apple macOS and Linux__: Git can be installed using [Homebrew](#homebrew). To install Git, open a terminal and run:

```shell
brew install git
```

- __Microsoft Windows__: Git can be installed using [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/). To install Git, open the Windows Terminal and run:

```powershell
winget install --id Git.Git -e --source winget
```

### Git LFS

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Git LFS](https://git-lfs.com) is an extension for [Git](#git) that supports storing large, binary, unversionable resources outside of the Git repository in storage optimized for storing binary objects. By storing unversionable binary resources in LFS, the size of the Git repository can be more maintainable and performant for developers.

- __Apple macOS or Linux__: Git LFS can be installed using [Homebrew](#homebrew). In a terminal, run:

```shell
brew install git-lfs
```

- __Microsoft Windows__: Git LFS is installed automatically with [Git](#git) on Microsoft Windows. You do not need to install Git LFS separately.

### GitHub CLI

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[GitHub CLI](https://cli.github.com) is a command line interface for GitHub. GitHub CLI wraps and exposes GitHub features through its API as command line tools. GitHub CLI can be used to clone repositories, interact with projects and repositories, or to automate administrative tasks. The Time team recommends using GitHub CLI to clone the repository and uses GitHub CLI for various administrative tasks.

- __Apple macOS or Linux__: GitHub CLI can be installed using [Homebrew](#homebrew). In a terminal, run:

```shell
brew install gh
```

- __Microsoft Windows__: GitHub CLI can be installed using [WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/). To install Git, open the Windows Terminal and run:

```powershell
winget install --id GitHub.cli
```

### Fast Node Manager

| Operating System | Required? |
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Fast Node Manager](https://github.com/Schniz/fnm) is a version manager for [Node.js](https://nodejs.org). Fast Node Manager can be used to install multiple versions of Node.js required for different projects and make it easy to switch between the different versions. Fast Node Manager can be integrated into the shell and will automatically switch the correct version of Node.js if it finds a [`.node-version`](../.node-version) in the root subdirectory of a project.

- __Apple macOS or Linux__: Fast Node Manager can be installed using [Homebrew](#homebrew). Tn a terminal, run:

```shell
curl -fsSL https://fnm.vercel.app/install | bash
```

- __Microsoft Windows__: Fast Node Manager can be installed using WinGet](https://learn.microsoft.com/en-us/windows/package-manager/winget/). To install Git, open the Windows Terminal and run:

```powershell
winget install Schniz.fnm
```

### Node.js

| Operating System | Required? |
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Node.js](https://nodejs.org) is an interpreter and runtime engine for JavaScript applications. Node.js makes it possible to use JavaScript to build command line tools, desktop applications, and service applications. The Time product depends on Node.js to execute some development tools including Git hook event handlers.

Node.js is installed and managed by [Fast Node Manager](#fast-node-manager). If Fast Node Manager is integrated into the shell, navigating to the Time project directory will select the correct version of Node. To ensure the correct version is being used, navigate to the root directory of the project in a terminal and run:

    fnm use

If the correct version of Node.js is not installed, you will be prompted for permission to download, install, and activate the correct Node.js version.

### Docker Desktop

| Operating System | Required? |
| Apple macOS | :grey_question: |
| Linux | :grey_question: |
| Microsoft Windows | :grey_question: |

[Docker Desktop] is a set of tools for building and running container images. Docker Desktop can be used to run the development container on the developer's machine for developing Time in the standardized development environment. Time is also packaged as a container image for redistribution and use by consumers.

Docker Desktop can be installed by downloading the installer from the [Docker Desktop website](https://www.docker.com/products/docker-desktop/) and running the installer.

## Visual Studio Code

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :grey_question: |
| Linux | :grey_question: |
| Microsoft Windows | :grey_question: |

[Visual Studio Code](https://code.visualstudio.com) is an open source text editor and development environment for software developers. Visual Studio Code is a highly extensible platform and boasts a large ecosystem of extensions that add support for programming languages, software development tools, and other tools and applications.

To install Visual Studio Code, download and run the installer from the [website](https://code.visualstudio.com).

## Remote Development Extension Pack for Visual Studio Code

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :grey_question: |
| Linux | :grey_question: |
| Microsoft Windows | :grey_question: |

The [Remote Development Extension Pack for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) is a set of extensions that support development on remote machines or in virtual environments hosted on local machines. The Remote Development Extension Pack includes support for building and running development containers using [Docker Desktop](software_requirements.md#docker-desktop).

The Remote Development Extension Pack can be installed from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack).

:exclamation: If you are doing local development on Microsoft Windows, but using Windows Subsystem for Linux 2, you will need to install the Remote Development Extension Pack to use Visual Studio Code with your Linux environment.
