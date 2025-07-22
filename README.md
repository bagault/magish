# MagiSH

A cross-platform Rust utility to locate and run Bash scripts with a user-friendly interface.

## Features
- Detects OS and ensures Bash (or WSL2 on Windows) is available
- Shows the current working folder and all `.sh` scripts in it
- Enhanced navigation with `cd` and `ls` commands (supports both relative and absolute paths)
- Command history with arrow key navigation and persistent storage
- Tab completion for commands and paths
- Remembers last working directory between sessions
- Configuration file for customizing history size and other settings
- Color-coded directory listings (blue for directories, green for shell scripts)
- Executes each non-comment, non-empty line of the script in Bash (or WSL on Windows) in the selected folder
- Script selection by number (just type the number shown next to the script)
- System-wide script scanning with `scan` command to find all available bash scripts

---

## 📝 Usage

### Running MagiSH (after installation)
- On Linux, run:
  ```sh
  magish
  ```
- On Windows, use the desktop shortcut or run `magish.exe` from `Program Files\MagiSH`.

### How to Use
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
- To scan for bash scripts:
  ```bash
  scan              # Show all bash scripts found in the system
  scan -o           # Same as above but also saves the list to magish_scripts.txt
  ```
- To select a script:
  ```bash
  1                 # Run the first script from the list
  2                 # Run the second script from the list
  # ... and so on
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

---

## 🚀 Building and Running (for advanced users)

### Universal Build Script
To build all release artifacts for Linux (`.deb`, `.rpm`) and Windows (`.zip`), run:
```sh
./build.sh
```
Artifacts will be placed in `build/release/`.

---

## 📦 Packaging & Distribution
- All build artifacts are in `build/release/` for easy upload to GitHub Releases or distribution.
- The Windows installer batch file is universal and always fetches the latest release from GitHub.

---

## 📄 License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

The included favicon is licensed separately. See [assets/favicon_LICENSE.txt](assets/favicon_LICENSE.txt).
