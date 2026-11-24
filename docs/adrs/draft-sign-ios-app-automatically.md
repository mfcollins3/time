---
status: "proposed"
date: 2026-11-23
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Sign the iOS app automatically and derive the build number from CI

## Context and Problem Statement

The Tuist manifest for the iOS app had no signing or build number settings, so
the app could not be run on a device or archived for App Store Connect. How
should the app be signed, and how should its version and build number be set?

## Decision Drivers

* Local development should work without manual profile management.
* The marketing version is already managed by release-please.
* CI must be able to set a monotonically increasing build number.
* Signing configuration should live in the Tuist manifest, not in Xcode.

## Considered Options

* Automatic signing with the team in the manifest; build number from an
  environment variable
* Automatic for Debug, manual signing for Release with imported certificates
  and profiles
* Build number from the Git commit count

## Decision Outcome

Chosen option: "Automatic signing with the team in the manifest; build number
from an environment variable", because it needs the least secret handling and
works the same locally and in CI.

* **Signing.** `Project.swift` sets `CODE_SIGN_STYLE = Automatic` and
  `DEVELOPMENT_TEAM = NZXG7K5N83`. CI archives use `-allowProvisioningUpdates`
  with an App Store Connect API key.
* **Marketing version.** `MARKETING_VERSION` stays owned by release-please via
  the `x-release-please-version` marker.
* **Build number.** `CURRENT_PROJECT_VERSION` is read from `TUIST_BUILD_NUMBER`
  at generation time and defaults to `1`. CI passes `github.run_number`.
  Apple generic versioning is enabled, and `Info.plist` references both
  settings.

### Consequences

* Good, because no certificates or profiles are stored in the repository or CI.
* Good, because local builds need no extra setup.
* Bad, because CI depends on Xcode's cloud signing and an API key with
  sufficient role.
* Bad, because build numbers must be passed explicitly when generating the
  project for a release.
