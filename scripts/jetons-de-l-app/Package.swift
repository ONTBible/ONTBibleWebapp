// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "extraire",
    platforms: [.macOS("14.0")],
    dependencies: [
        .package(path: "../../../ONTBibleApp/app/Packages/ONTDesignSystem")
    ],
    targets: [
        .executableTarget(
            name: "extraire",
            dependencies: [.product(name: "ONTDesignSystem", package: "ONTDesignSystem")]
        )
    ]
)
