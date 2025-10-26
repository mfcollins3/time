# Software Requirements for Local Development

This document lists the additional software that needs to be installed to support local development of Time on a development laptop, desktop, or virtial machine environment. 

The preferred way to build, run, and develop for the Time product is to use the [development container](). The development container is a stable software development environment that includes the correct versions of all software development tools, programming language compilers, and libraries installed and ready for use.

The following software packages are required for local development:

1. [Go](#go)

## Go

__Required Version__: 1.25.3

| Operating System | Required? |
|------------------|-----------|
| Apple macOS | :white_check_mark: |
| Linux | :white_check_mark: |
| Microsoft Windows | :white_check_mark: |

[Go](https://go.dev) is a general purpose programming language for application development. Go was created and is maintained by [Google](https://google.com) and is used to build many popular software development tools including [Kubernetes](https://kubernetes.io) and [Docker](https://docker.com). Go is a popular programming language and has a large and growing ecosystem.

- __Apple macOS or Linux__: [Go] can be installed using [Homebrew](software_requirements.md#homebrew). In a terminal, run:

```shell
brew install go@1.25.3
```

- __Microsoft Windows__: [Go] can be installed on Microsoft Windows by downloading the required version from the [Go download page](https://go.dev/dl/).
