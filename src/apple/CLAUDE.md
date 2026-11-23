# Apple Applications

The `apple` subdirectory contains the source code for the Naked Time
applications and extensions targeting Apple platforms (macOS, iOS, iPadOS,
watchOS, tvOS, visionOS).

## Xcode workspace and project configuration

Naked Time uses [Tuist](https://tuist.dev) to manage the configuration of the
project using Swift Code. Tuist will generate the `.xcworkspace` and
`.xcodeproj` directories and files used by Xcode to build and develop the
applications in Xcode.

Tuist exposes four [manifest files](https://tuist.dev/en/docs/guides/features/projects/manifests):

* `Workspace.swift`: manifest for the workspace. The manifest documentation
  can be found [here](https://projectdescription.tuist.dev/documentation/projectdescription/workspace).
* `Tuist.swift`: configuration file that controls how Tuist generates the source
  code and Xcode files. The manifest documentation can be found
  [here](https://projectdescription.tuist.dev/documentation/projectdescription/tuist).
* `Project.swift`: manifest for each separate project in the workspace. The
  manifest documentation can be found
  [here](https://projectdescription.tuist.dev/documentation/projectdescription/project).
* `Tuist/Package.swift`: standard Swift Package Manager manifest file that
  contains the Swift package dependencies for the Naked Time product. Tuist will
  download and build the packages and will link them to the generated Xcode
  targets when the workspace is generated. See
  [https://tuist.dev/en/docs/guides/features/projects/dependencies] for
  information about how Tuist managed dependencies.

## Using Tuist

When any of the manifest files have changed, run the Tuist CLI to regenerate
the Xcode workspace, projects, `Info.plist`, and source code files:

```bash
tuist generate
```

If dependencies are added, updated, or removed, they must be reinstalled using
the Tuist CLI:

```bash
tuist install
```

## Directory layout

```plain
apple/                                  Directory containing Naked Time apps for Apple devices
|-- Projects/                           Directory that contains all of the projects, source code, and assets
|   |-- App/                            Directory that contains the source code for the main Naked Time application
|   |   |-- Resources/                  Directory that contains assets and other resources used by the Naked Time application
|   |   |   |-- Assets.xcassets/        Main asset catalog for the Naked Time application
|   |   |   |-- InfoPlist.xcstrings     String catalog that is used to localize the Info.plist string values
|   |   |-- Sources/                    Source code for the Naked Time application
|   |   |   |-- MainScene.swift         Main scene for the Naked Time application
|   |   |   |-- MainView.swift          Main Swift UI view that implements the Naked Time user experience
|   |   |   |-- TimeApp.swift           Main program for Naked Time and manages the application
|   |   |-- Project.swift               Project-level Tuist manifest for the Naked Time application.
|-- Tuist/                              Directory used by Tuist. Mainly stores Swift Package dependencies.
|   |-- Package.swift                   Swift Package Manager manifest that list the dependencies for the Naked Time product
|-- Tuist.swift                         Tuist manifest file containing configuration settings controlling how the Tuist CLI generates the source code
|-- Workspace.swift                     Workspace manifest file that defines the Xcode workspace for the Naked Time product
```
