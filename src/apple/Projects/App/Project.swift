// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

import ProjectDescription

/// Build number (`CURRENT_PROJECT_VERSION`). CI sets `TUIST_BUILD_NUMBER`;
/// local builds default to 1.
let buildNumber = Environment.buildNumber.getString(default: "1")

let project = Project(
    name: "Time",
    organizationName: "Michael Collins",
    settings: .settings(
        base: SettingsDictionary()
            .automaticCodeSigning(devTeam: "WTG7RTG947")
            .appleGenericVersioningSystem()
            .currentProjectVersion(buildNumber)
            .merging([
                "ASSETCATALOG_COMPILER_GENERATE_SWIFT_ASSET_SYMBOL_EXTENSIONS": true,
                "ENABLE_USER_SCRIPT_SANDBOXING": true,
                "MARKETING_VERSION": "0.0.0", // x-release-please-version
                "STRING_CATALOG_GENERATE_SYMBOLS": true
            ]) { _, new in new },
        configurations: [
            .debug(name: .debug),
            .release(name: .release)
        ]
    ),
    targets: [
        .target(
            name: "Time",
            destinations: [
                .iPad,
                .iPhone
            ],
            product: .app,
            bundleId: "software.naked.time",
            deploymentTargets: .iOS("27.0"),
            infoPlist: .extendingDefault(with: [
                "CFBundleShortVersionString": "$(MARKETING_VERSION)",
                "CFBundleVersion": "$(CURRENT_PROJECT_VERSION)"
            ]),
            sources: [
                "Sources/**"
            ],
            resources: [
                "Resources/**"
            ]
        ),
        .target(
            name: "TimeTests",
            destinations: [
                .iPad,
                .iPhone
            ],
            product: .unitTests,
            bundleId: "software.naked.time.tests",
            sources: [
                "Tests/**"
            ]
        )
    ]
)
