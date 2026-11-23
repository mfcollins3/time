// Copyright 2026 Michael F. Collins, III
// Licensed under the Naked Time license. See LICENSE.md for details.

import ProjectDescription

let project = Project(
    name: "Time",
    organizationName: "Michael Collins",
    settings: .settings(
        base: [
            "ASSETCATALOG_COMPILER_GENERATE_SWIFT_ASSET_SYMBOL_EXTENSIONS": true,
            "ENABLE_USER_SCRIPT_SANDBOXING": true,
            "MARKETING_VERSION": "0.0.0", // x-release-please-version
            "STRING_CATALOG_GENERATE_SYMBOLS": true
        ],
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
            infoPlist: .extendingDefault(with: [:]),
            sources: [
                "Sources/**"
            ],
            resources: [
                "Resources/**"
            ]
        )
    ]
)
