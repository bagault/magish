# MagiSH

A cross-platform Rust utility to locate and run Bash scripts with a user-friendly interface.

## Features
- Detects OS and ensures Bash (or WSL2 on Windows) is available.
- Shows the current working folder and all `.sh` scripts in it.
- Allows changing the working folder with `-c <folder>`.
- Lets you select or auto-detect a Bash script to run.
- Executes each non-comment, non-empty line of the script in Bash (or WSL on Windows) in the selected folder.

---

## 📝 Usage

### Running MagiSH (after installation)
- On Linux, run:
  ```sh
  magish
  ```
- On Windows, use the desktop shortcut or run `magish.exe` from `Program Files\MagiSH`.

### How to Use
- On start, the program displays the current folder and lists all `.sh` files.
- To change the folder, enter:
  ```
  -c /path/to/your/folder
  ```
- To run a script, enter its filename (as shown in the list), or press Enter to auto-detect (it will pick the first `.sh` file found).
- The program will execute each command in the script in the selected folder.
- After execution, press Enter to exit.

---

## 🚀 Building and Running (for advanced users)

### Universal Build Script
To build all release artifacts for Linux (`.deb`, `.rpm`) and Windows (`.zip`), run:
```sh
./build.sh
```
Artifacts will be placed in `build/release/`.

### Manual Build (Linux)
```sh
cargo build --release
```
Binary: `target/release/magish`

### Manual Build (Windows)
```sh
cargo build --release --target x86_64-pc-windows-gnu
```
Binary: `target/x86_64-pc-windows-gnu/release/magish.exe`

### Installing on Linux
- `.deb`: `sudo dpkg -i build/release/magish_1.0.0_amd64.deb`
- `.rpm`: `sudo rpm -i build/release/magish-1.0.0-1.x86_64.rpm`

### Installing on Windows
- Download the latest `magish-windows-x86_64.zip` from [GitHub Releases](https://github.com/bagault/magish/releases).
- Run `windows_installer.bat` (it will always fetch and install the latest release).

---

## 📦 Packaging & Distribution
- All build artifacts are in `build/release/` for easy upload to GitHub Releases or distribution.
- The Windows installer batch file is universal and always fetches the latest release from GitHub.

---

## 📄 License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

The included favicon is licensed separately. See [assets/favicon_LICENSE.txt](assets/favicon_LICENSE.txt).
