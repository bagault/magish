#!/bin/bash
<<<<<<< HEAD
set -e

# Get version from Cargo.toml
toml_version=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)

# Update version in DEBIAN/control
sed -i "s/^Version: .*/Version: $toml_version/" build/deb/DEBIAN/control

# Build Linux binary
cargo build --release

# Build Windows binary
cargo build --release --target x86_64-pc-windows-gnu

# Prepare directories
mkdir -p build/release

# Build .deb package
=======

# MagiSH build script with dependency checks
set -e

REQUIRED_CMDS=(rustc cargo)
REQUIRED_CRATES=(which dirs rustyline serde serde_json glob)

missing_cmds=()
missing_crates=()

# Check for required system commands
for cmd in "${REQUIRED_CMDS[@]}"; do
    if ! command -v "$cmd" &>/dev/null; then
        missing_cmds+=("$cmd")
    fi
done

# Check for required crates in Cargo.toml
for crate in "${REQUIRED_CRATES[@]}"; do
    if ! grep -q "^$crate" Cargo.toml; then
        missing_crates+=("$crate")
    fi
done

if [ ${#missing_cmds[@]} -eq 0 ] && [ ${#missing_crates[@]} -eq 0 ]; then
    echo "All dependencies are satisfied."
else
    echo "Missing dependencies detected:"
    if [ ${#missing_cmds[@]} -gt 0 ]; then
        echo "  System commands: ${missing_cmds[*]}"
    fi
    if [ ${#missing_crates[@]} -gt 0 ]; then
        echo "  Cargo crates: ${missing_crates[*]}"
    fi
    echo
    read -p "Would you like to install missing dependencies automatically? (y/n): " choice
    if [[ "$choice" =~ ^[Yy]$ ]]; then
        if [ ${#missing_cmds[@]} -gt 0 ]; then
            echo "Please install the following system commands manually: ${missing_cmds[*]}"
        fi
        if [ ${#missing_crates[@]} -gt 0 ]; then
            for crate in "${missing_crates[@]}"; do
                echo "Installing crate: $crate"
                cargo add "$crate"
            done
        fi
    else
        echo "Please install the missing dependencies manually before building."
        exit 1
    fi
fi

echo "Building MagiSH..."
cargo build --release

# Get version from Cargo.toml
toml_version=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)

# Prepare directories
mkdir -p build/release
>>>>>>> 9692385 (Release v0.3.0)
mkdir -p build/deb/usr/bin build/deb/usr/share/icons/hicolor/256x256/apps build/deb/usr/share/applications build/deb/DEBIAN
cp target/release/magish build/deb/usr/bin/
cp assets/favicon-256.png build/deb/usr/share/icons/hicolor/256x256/apps/magish.png
cp magish.desktop build/deb/usr/share/applications/
<<<<<<< HEAD
dpkg-deb --build build/deb build/release/magish_${toml_version}_amd64.deb

# Build .rpm package (using fpm)
fpm -s dir -t rpm -n magish -v $toml_version --license MIT --url https://github.com/bagault/magish --description "Cross-platform Bash script runner with user-friendly interface. The newest release is always available at https://github.com/bagault/magish/releases" --maintainer "Daniel <your@email.com>" --depends bash --category utils --architecture amd64 -p build/release/magish-${toml_version}-1.x86_64.rpm target/release/magish=/usr/bin/magish magish.desktop=/usr/share/applications/magish.desktop assets/favicon-256.png=/usr/share/icons/hicolor/256x256/apps/magish.png

# Build Windows zip
zip -j build/release/magish-windows-x86_64.zip target/x86_64-pc-windows-gnu/release/magish.exe assets/favicon.ico windows_installer.bat
=======

# Create default DEBIAN/control file if missing
if [ ! -f build/deb/DEBIAN/control ]; then
cat <<EOL > build/deb/DEBIAN/control
Package: magish
Version: $toml_version
Section: utils
Priority: optional
Architecture: amd64
Depends: bash
Maintainer: Daniel <your@email.com>
Description: Cross-platform Bash script runner with user-friendly interface.
EOL
fi

# Update version in DEBIAN/control (after control file exists)
sed -i "s/^Version: .*/Version: $toml_version/" build/deb/DEBIAN/control

# Build .deb package
dpkg-deb --build build/deb build/release/magish_${toml_version}_amd64.deb

# Build .rpm package (using fpm)
if command -v fpm &>/dev/null; then
    fpm -s dir -t rpm -n magish -v $toml_version --license MIT --url https://github.com/bagault/magish --description "Cross-platform Bash script runner with user-friendly interface. The newest release is always available at https://github.com/bagault/magish/releases" --maintainer "Daniel <your@email.com>" --depends bash --category utils --architecture amd64 -p build/release/magish-${toml_version}-1.x86_64.rpm target/release/magish=/usr/bin/magish magish.desktop=/usr/share/applications/magish.desktop assets/favicon-256.png=/usr/share/icons/hicolor/256x256/apps/magish.png
else
    echo "Warning: fpm not found. Skipping RPM build. Install with: sudo gem install fpm"
fi

# Build Windows zip
win_bin="target/x86_64-pc-windows-gnu/release/magish.exe"
if [ -f "$win_bin" ]; then
    zip -j build/release/magish-windows-x86_64.zip "$win_bin" assets/favicon.ico windows_installer.bat
else
    echo "Warning: Windows binary not found at $win_bin. Skipping Windows zip."
    zip -j build/release/magish-windows-x86_64.zip assets/favicon.ico windows_installer.bat
fi
>>>>>>> 9692385 (Release v0.3.0)

echo "All builds complete. Artifacts are in build/release/"
