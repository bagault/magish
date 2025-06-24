#!/bin/bash
set -e

# Build Linux binary
cargo build --release

# Build Windows binary
cargo build --release --target x86_64-pc-windows-gnu

# Prepare directories
mkdir -p build/release

# Build .deb package
mkdir -p build/deb/usr/bin build/deb/usr/share/icons/hicolor/256x256/apps build/deb/usr/share/applications build/deb/DEBIAN
cp target/release/magish build/deb/usr/bin/
cp assets/favicon-256.png build/deb/usr/share/icons/hicolor/256x256/apps/magish.png
cp magish.desktop build/deb/usr/share/applications/
cp build/deb/DEBIAN/control build/deb/DEBIAN/control
X-Age-Rating: 0+
dpkg-deb --build build/deb build/release/magish_1.0.0_amd64.deb

# Build .rpm package (using fpm)
fpm -s dir -t rpm -n magish -v 1.0.0 --license MIT --url https://github.com/bagault/magish --description "Cross-platform Bash script runner with user-friendly interface. The newest release is always available at https://github.com/bagault/magish/releases" --maintainer "Daniel <your@email.com>" --depends bash --category utils --architecture amd64 target/release/magish=/usr/bin/magish magish.desktop=/usr/share/applications/magish.desktop assets/favicon-256.png=/usr/share/icons/hicolor/256x256/apps/magish.png -p build/release/magish-1.0.0-1.x86_64.rpm

# Build Windows zip
zip -j build/release/magish-windows-x86_64.zip target/x86_64-pc-windows-gnu/release/magish.exe assets/favicon.ico windows_installer.bat

echo "All builds complete. Artifacts are in build/release/"
