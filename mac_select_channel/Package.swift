// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "mac_select_channel",
    products: [
        .library(
            name: "mac_select_channel",
            type: .static,
            targets: ["mac_select_channel"]),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "mac_select_channel",
            dependencies: [],
            path: "src"),
    ]
)
