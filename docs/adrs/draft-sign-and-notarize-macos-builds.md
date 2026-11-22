---
status: "proposed"
date: 2026-11-21
decision-makers: Michael Collins
---

<!--
Copyright 2026 Michael F. Collins, III
Licensed under the Naked Time license. See LICENSE.md for details.
-->

# Sign and notarize macOS builds with Developer ID

## Context and Problem Statement

The release build for macOS (`aarch64-apple-darwin`) produces an unsigned
`.tar.gz`. When a user downloads it with a browser, macOS quarantines the file
and Gatekeeper refuses to run the binary because it is neither signed with a
Developer ID certificate nor notarized by Apple. How do we ship a macOS build
that users can download and run?

This extends [Build and publish release binaries on native runners for five
targets](draft-release-build-targets.md), which defines the build workflow and
the macOS target.

## Decision Drivers

* Downloaded builds must run without Gatekeeper warnings.
* Signing and notarization must be automated in the existing `build.yml`.
* Notarization must not depend on a personal Apple ID or two-factor
  authentication.
* Signing secrets must not be exposed to pull requests from forks.
* Installation should work offline where possible.

## Considered Options

* Sign and notarize the binary; ship a `.tar.gz` and a stapled `.pkg`
* Sign and notarize the binary; ship only the `.tar.gz`
* Ship a stapled `.dmg` in addition to the `.tar.gz`

## Decision Outcome

Chosen option: "Sign and notarize the binary; ship a `.tar.gz` and a stapled
`.pkg`", because a notarization ticket can be stapled to a `.pkg` (but not to a
bare binary or a `.tar.gz`), so the installer works offline, while the archive
remains available for users who prefer it.

* **Signing.** The binary is signed with a *Developer ID Application*
  certificate using `codesign --options runtime --timestamp`. The hardened
  runtime is required for notarization; audio output through CoreAudio needs no
  entitlements. Signing happens before packaging, so the `.tar.gz` contains the
  signed binary.
* **Installer.** `pkgbuild` creates `naked-time-<version>-aarch64-apple-darwin.pkg`
  (identifier `software.naked.naked-time`), which installs
  `/usr/local/bin/naked-time`. It is signed with a *Developer ID Installer*
  certificate.
* **Notarization.** The `.pkg` is submitted once with `xcrun notarytool submit
  --wait` and the ticket is stapled with `xcrun stapler staple`. Notarizing the
  `.pkg` also registers the signed binary inside it, so the `.tar.gz` passes
  Gatekeeper's online check on first launch (no ticket can be stapled to it).
* **Authentication.** `notarytool` authenticates with an App Store Connect API
  key (`.p8`, key ID, issuer ID).
* **When it runs.** The reusable `build.yml` takes a `sign_macos` input.
  `ci.yml` sets it for pushes to `main` only, and `release-please.yml` always
  sets it. Pull request builds remain unsigned because repository secrets are
  not available to forks. Both callers use `secrets: inherit`.
* **Secrets handling.** Certificates are imported into a temporary keychain
  created in `$RUNNER_TEMP`, with a random password. The keychain and the API
  key file are deleted in a final `always()` step.

### Required repository secrets

| Secret                                 | Contents                                          |
|----------------------------------------|---------------------------------------------------|
| `MACOS_APP_CERTIFICATE_P12`            | Base64 of the Developer ID Application `.p12`     |
| `MACOS_APP_CERTIFICATE_PASSWORD`       | Password of that `.p12`                           |
| `MACOS_INSTALLER_CERTIFICATE_P12`      | Base64 of the Developer ID Installer `.p12`       |
| `MACOS_INSTALLER_CERTIFICATE_PASSWORD` | Password of that `.p12`                           |
| `APPLE_API_KEY_P8`                     | Base64 of the App Store Connect API key (`.p8`)   |
| `APPLE_API_KEY_ID`                     | App Store Connect API key ID                      |
| `APPLE_API_ISSUER_ID`                  | App Store Connect issuer ID                       |

Export each certificate with its private key from Keychain Access as a `.p12`,
encode it with `base64 -i file | pbcopy`, and store it with `gh secret set`. Create
the API key in App Store Connect under Users and Access > Integrations > Team
Keys with the Developer role.

### Consequences

* Good, because users can install the `.pkg` without warnings, even offline.
* Good, because only one notarization submission is needed per build.
* Good, because the API key avoids personal Apple ID credentials and 2FA.
* Bad, because the build depends on Apple's notary service, which can be slow or
  unavailable, and failures block the macOS job.
* Bad, because the first launch of the extracted `.tar.gz` binary needs an
  internet connection.
* Bad, because the Developer ID certificates expire and must be renewed and the
  secrets updated.
* Bad, because the `.pkg` installs into `/usr/local/bin` and must be uninstalled
  manually.

### Confirmation

On a push to `main`, the macOS job log must show `codesign --verify` passing,
a notarytool status of `Accepted`, a successful `stapler validate`, and `spctl`
reporting `accepted source=Notarized Developer ID`. On a Mac, a `.pkg` and a
`.tar.gz` downloaded through a browser must install and run without a Gatekeeper
warning.

## Pros and Cons of the Options

### Sign and notarize the binary; ship a `.tar.gz` and a stapled `.pkg`

* Good, because the stapled ticket makes the installer work offline.
* Good, because users have both an installer and a plain archive.
* Bad, because it requires an additional Developer ID Installer certificate.

### Sign and notarize the binary; ship only the `.tar.gz`

* Good, because it is the simplest workflow and needs one certificate.
* Bad, because a ticket cannot be stapled, so first launch needs the internet.

### Ship a stapled `.dmg` in addition to the `.tar.gz`

* Good, because it needs only the Application certificate.
* Bad, because users must copy the binary onto their `PATH` themselves.

## More Information

* [Notarizing macOS software before distribution](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)
* [Customizing the notarization workflow](https://developer.apple.com/documentation/security/customizing-the-notarization-workflow)
