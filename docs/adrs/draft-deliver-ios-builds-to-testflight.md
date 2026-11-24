---
status: "proposed"
date: 2026-11-23
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Deliver iOS builds to TestFlight from GitHub Actions

## Context and Problem Statement

The iOS app has no CI. Pull requests need a build and unit tests, and new builds
should reach internal testers and early adopters through TestFlight without
manual work. When should builds be delivered, and with what tooling?

## Decision Drivers

* Keep one CI system (GitHub Actions) and the existing Tuist workflow.
* release-please already groups changes into versioned releases.
* Internal TestFlight builds need no Beta App Review; external ones do.
* Reuse the App Store Connect API key already used for macOS notarization.

## Considered Options

* Every merge to `main` goes to internal testers; release-please releases also
  go to external testers
* Deliver only from a `release/*` branch
* Xcode Cloud
* Fastlane for the whole pipeline, including `match` for signing

## Decision Outcome

Chosen option: "Every merge to `main` goes to internal testers; release-please
releases also go to external testers", because the release-please release pull
request already is the gate that groups features, so a release branch would add
upkeep without adding value.

* **Pull requests** build and run the unit tests only, with no secrets.
* **Merges to `main`** touching `src/apple` upload to the `Internal` group.
* **release-please `ios/v*` releases** upload to `Internal` and `Early Adopters`
  and are submitted for Beta App Review with the release notes.
* **Tooling.** Tuist and `xcodebuild` build, test, archive, and export. Fastlane
  is used only for TestFlight upload and the build-number lookup. Signing stays
  cloud-managed (see [sign the iOS app automatically](draft-sign-ios-app-automatically.md)),
  so `match` is not needed.
* **Build number** is the latest TestFlight build number plus one, so the
  `main` and release workflows share one counter. Deliveries are serialized.

### Consequences

* Good, because testers get every merged change and delivery problems surface
  immediately.
* Good, because releases need no extra branch or process.
* Bad, because a hotfix to a shipped version while `main` has moved on needs a
  release branch cut from the tag later.
* Bad, because the pipeline depends on GitHub's macOS images shipping an Xcode
  that supports iOS 27.
* Bad, because internal builds do not wait for processing, so they carry no
  "What to Test" text.
