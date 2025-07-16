//! `magish` - cross-platform Rust utility to locate and run Bash scripts.
//!
//! Features:
//! - Detects OS and on Windows ensures WSL2 is available (or directs to install it).
//! - Prompts for a Bash script path, or auto-discovers `.sh` files in current directory.
//! - Executes each line of the script in Bash/WSL sequentially.
//! - Retry up to 3 times for missing scripts, then exits.

<<<<<<< HEAD
use std::fs;
use std::io::{self, Write};
=======
mod config;
mod history;

use config::Config;
use history::CommandHistory;
use std::fs;
use std::io;
>>>>>>> 9692385 (Release v0.3.0)
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!(r#"
 ██████   ██████                     ███   █████████  █████   █████
░░██████ ██████                     ░░░   ███░░░░░███░░███   ░░███ 
 ░███░█████░███   ██████    ███████ ████ ░███    ░░░  ░███    ░███ 
 ░███░░███ ░███  ░░░░░███  ███░░███░░███ ░░█████████  ░███████████ 
 ░███ ░░░  ░███   ███████ ░███ ░███ ░███  ░░░░░░░░███ ░███░░░░░███ 
 ░███      ░███  ███░░███ ░███ ░███ ░███  ███    ░███ ░███    ░███ 
 █████     █████░░████████░░███████ █████░░█████████  █████   █████
░░░░░     ░░░░░  ░░░░░░░░  ░░░░░███░░░░░  ░░░░░░░░░  ░░░░░   ░░░░░ 
                           ███ ░███                                
                          ░░██████                                 
                           ░░░░░░ 

v{}
A cross-platform Rust utility to locate and run Bash scripts.
"#, env!("CARGO_PKG_VERSION"));
    // 1) OS Detection and Preparation
    if cfg!(target_os = "windows") {
        println!("Windows detected.");
        if !check_wsl2() {
            println!("WSL2 is not installed or not enabled.");
            open_edge_install_link();
            return;
        }
    } else if cfg!(unix) {
        println!("POSIX-compatible OS detected.");
        if which::which("bash").is_err() {
            eprintln!("Bash not found on this system. Unsupported OS.");
            return;
        }
    } else {
        eprintln!("Unsupported OS detected. Exiting.");
        return;
    }

<<<<<<< HEAD
    let mut current_dir = if cfg!(target_os = "windows") {
        // Default to Desktop on Windows
        dirs::desktop_dir().unwrap_or_else(|| std::env::current_dir().unwrap())
    } else {
        std::env::current_dir().unwrap()
    };
=======
    let mut config = Config::load();
    let mut current_dir = config.last_directory.clone();
    if !current_dir.exists() {
        current_dir = if cfg!(target_os = "windows") {
            dirs::desktop_dir().unwrap_or_else(|| std::env::current_dir().unwrap())
        } else {
            std::env::current_dir().unwrap()
        };
    }

    let mut history = CommandHistory::new(config.history_limit);
    
>>>>>>> 9692385 (Release v0.3.0)
    loop {
        println!("\nCurrent folder: {}", current_dir.display());
        let bash_files = list_bash_files(&current_dir);
        if bash_files.is_empty() {
            println!("No .sh files found in this folder.");
        } else {
            println!("Available Bash scripts:");
            for (i, file) in bash_files.iter().enumerate() {
                println!("  [{}] {}", i + 1, file.display());
            }
        }
<<<<<<< HEAD
        print!("Enter path to Bash script, or -c <folder> to change directory (Enter to auto-detect): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.starts_with("-c ") {
            let new_dir = input[3..].trim();
            let new_path = Path::new(new_dir);
            if new_path.is_dir() {
                current_dir = new_path.canonicalize().unwrap();
                continue;
            } else {
                println!("Provided folder does not exist.");
                continue;
            }
        }
        let script_path = if !input.is_empty() {
            let pb = current_dir.join(input);
            if pb.exists() && pb.is_file() {
                Some(pb)
            } else {
                println!("Provided path is invalid or not a file.");
                continue;
            }
        } else {
            auto_discover_script(&current_dir)
        };
        if let Some(script_path) = script_path {
            println!("Using script: {}", script_path.display());
            if let Ok(contents) = fs::read_to_string(&script_path) {
                for line in contents.lines() {
                    let cmd = line.trim();
                    if cmd.is_empty() || cmd.starts_with('#') {
                        continue;
                    }
                    println!("Executing: {}", cmd);
                    let mut command = if cfg!(target_os = "windows") {
                        let mut c = Command::new("wsl");
                        c.arg("bash").arg("-c").arg(cmd);
                        c
                    } else {
                        let mut c = Command::new("bash");
                        c.arg("-c").arg(cmd);
                        c
                    };
                    command.current_dir(&current_dir);
                    let mut child = command
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("Failed to spawn shell process");
                    let _ = child.wait();
                    thread::sleep(Duration::from_millis(100));
                }
                println!("All commands executed.");
            } else {
                eprintln!("Failed to read script file. Exiting.");
            }
            break;
        } else {
            println!("No scripts found in current directory.");
        }
    }
=======

        let prompt = format!("{}> ", current_dir.display());
        let input = match history.readline(&prompt) {
            Ok(line) => line.trim().to_string(),
            Err(_) => break,
        };

        if input.is_empty() {
            if let Some(script) = auto_discover_script(&current_dir) {
                execute_script(&script, &current_dir);
                break;
            }
            continue;
        }

        match input.as_str() {
            "ls" => list_directory(&current_dir),
            input if input.starts_with("ls ") => {
                let path = input[3..].trim();
                let target_path = resolve_path(&current_dir, path);
                if target_path.exists() && target_path.is_dir() {
                    list_directory(&target_path);
                } else {
                    println!("Invalid path: {}", path);
                }
            }
            "cd" => {
                if let Some(home) = dirs::home_dir() {
                    current_dir = home;
                    config.last_directory = current_dir.clone();
                    config.save().unwrap_or_else(|e| eprintln!("Failed to save config: {}", e));
                }
            }
            input if input.starts_with("cd ") => {
                let path = input[3..].trim();
                let target_path = resolve_path(&current_dir, path);
                if target_path.exists() && target_path.is_dir() {
                    current_dir = target_path;
                    config.last_directory = current_dir.clone();
                    config.save().unwrap_or_else(|e| eprintln!("Failed to save config: {}", e));
                } else {
                    println!("Invalid directory: {}", path);
                }
            }
            input => {
                let target_path = resolve_path(&current_dir, input);
                if target_path.exists() {
                    if target_path.is_dir() {
                        current_dir = target_path;
                        config.last_directory = current_dir.clone();
                        config.save().unwrap_or_else(|e| eprintln!("Failed to save config: {}", e));
                    } else if target_path.extension().and_then(|s| s.to_str()) == Some("sh") {
                        execute_script(&target_path, &current_dir);
                        break;
                    } else {
                        println!("Not a directory or shell script: {}", input);
                    }
                } else {
                    println!("Path does not exist: {}", input);
                }
            }
        }
    }

    history.save_history().unwrap_or_else(|e| eprintln!("Failed to save history: {}", e));
>>>>>>> 9692385 (Release v0.3.0)
    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

<<<<<<< HEAD
=======
fn resolve_path(current_dir: &Path, path: &str) -> PathBuf {
    if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        current_dir.join(path)
    }
}

fn list_directory(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            if path.is_dir() {
                println!("\x1b[1;34m{}/\x1b[0m", file_name);
            } else if path.extension().and_then(|s| s.to_str()) == Some("sh") {
                println!("\x1b[1;32m{}\x1b[0m", file_name);
            } else {
                println!("{}", file_name);
            }
        }
    } else {
        println!("Cannot read directory");
    }
}

fn execute_script(script_path: &Path, current_dir: &Path) {
    println!("Using script: {}", script_path.display());
    if let Ok(contents) = fs::read_to_string(script_path) {
        for line in contents.lines() {
            let cmd = line.trim();
            if cmd.is_empty() || cmd.starts_with('#') {
                continue;
            }
            println!("Executing: {}", cmd);
            let mut command = if cfg!(target_os = "windows") {
                let mut c = Command::new("wsl");
                c.arg("bash").arg("-c").arg(cmd);
                c
            } else {
                let mut c = Command::new("bash");
                c.arg("-c").arg(cmd);
                c
            };
            command.current_dir(current_dir);
            let mut child = command
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to spawn shell process");
            let _ = child.wait();
            thread::sleep(Duration::from_millis(100));
        }
        println!("All commands executed.");
    } else {
        eprintln!("Failed to read script file.");
    }
}

>>>>>>> 9692385 (Release v0.3.0)
fn list_bash_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sh") && path.is_file() {
                files.push(path);
            }
        }
    }
    files
}

fn auto_discover_script(dir: &Path) -> Option<PathBuf> {
    let names = ["base.sh", "index.sh", "script.sh"];
    for name in &names {
        let pb = dir.join(name);
        if pb.exists() && pb.is_file() {
            println!("Auto-detected script: {}", pb.display());
            return Some(pb);
        }
    }
    let bash_files = list_bash_files(dir);
    if !bash_files.is_empty() {
        println!("Auto-detected script: {}", bash_files[0].display());
        return Some(bash_files[0].clone());
    }
    None
}

/// Checks if WSL2 is installed and available.
fn check_wsl2() -> bool {
    if let Ok(output) = Command::new("wsl").arg("-l").arg("-v").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        // We look for a version column containing '2'.
        return text.lines().any(|l| l.contains("2"));
    }
    false
}

/// Opens Microsoft Edge to the WSL install documentation.
fn open_edge_install_link() {
    let url = "https://docs.microsoft.com/windows/wsl/install";
    let _ = Command::new("cmd")
        .args(&["/C", "start", "ms-edge:" , url])
        .spawn();
    println!("Opening Edge to guide for WSL2 installation...");
}

