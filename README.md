# MagiSH

A cross-platform Rust utility to locate and run Bash scripts with a user-friendly interface.

## Features
<<<<<<< HEAD
- Detects OS and ensures Bash (or WSL2 on Windows) is available.
- Shows the current working folder and all `.sh` scripts in it.
- Allows changing the working folder with `-c <folder>`.
- Lets you select or auto-detect a Bash script to run.
- Executes each non-comment, non-empty line of the script in Bash (or WSL on Windows) in the selected folder.
=======
- Detects OS and ensures Bash (or WSL2 on Windows) is available
- Shows the current working folder and all `.sh` scripts in it
- Enhanced navigation with `cd` and `ls` commands (supports both relative and absolute paths)
- Command history with arrow key navigation and persistent storage
- Tab completion for commands and paths
- Remembers last working directory between sessions
- Configuration file for customizing history size and other settings
- Color-coded directory listings (blue for directories, green for shell scripts)
- Executes each non-comment, non-empty line of the script in Bash (or WSL on Windows) in the selected folder
>>>>>>> 9692385 (Release v0.3.0)

---

## 📝 Usage

### Running MagiSH (after installation)
- On Linux, run:
  ```sh
  magish
  ```
- On Windows, use the desktop shortcut or run `magish.exe` from `Program Files\MagiSH`.

### How to Use
<<<<<<< HEAD
- On start, the program displays the current folder and lists all `.sh` files.
- To change the folder, enter:
  ```
  -c /path/to/your/folder
  ```
- To run a script, enter its filename (as shown in the list), or press Enter to auto-detect (it will pick the first `.sh` file found).
- The program will execute each command in the script in the selected folder.
- After execution, press Enter to exit.
=======
- On start, the program displays the current folder and lists all `.sh` files
- Navigate using standard commands:
  ```bash
  ls                 # List files in current directory
  ls <path>          # List files in specified directory
  cd                 # Go to home directory
  cd <path>          # Change to specified directory
  ```
- You can also navigate by simply entering a path:
  ```bash
  /absolute/path     # Navigate to absolute path
  relative/path      # Navigate to relative path
  ```
- To run a script:
  - Enter the script's path (relative or absolute)
  - Press Enter to auto-detect (it will pick the first `.sh` file found)
- Use arrow keys to navigate command history
- Use Tab for command/path completion
- After execution, press Enter to exit

### Configuration
The program creates two files next to the executable:
- `configs.json`: Stores last working directory and history settings
- `magish-history.txt`: Stores command history

You can modify `configs.json` to change:
- `history_limit`: Maximum number of commands to store (default: 100)
- `last_directory`: Last working directory to start from
>>>>>>> 9692385 (Release v0.3.0)

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
