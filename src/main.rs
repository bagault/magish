//! `magish` - cross-platform Rust utility to locate and run Bash scripts.
//!
//! Features:
//! - Detects OS and on Windows ensures WSL2 is available (or directs to install it).
//! - Prompts for a Bash script path, or auto-discovers `.sh` files in current directory.
//! - Executes each line of the script in Bash/WSL sequentially.
//! - Retry up to 3 times for missing scripts, then exits.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!("
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
v1.0.0 \n
A cross-platform Rust utility to locate and run Bash scripts.\n");                           
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

    let mut current_dir = if cfg!(target_os = "windows") {
        // Default to Desktop on Windows
        dirs::desktop_dir().unwrap_or_else(|| std::env::current_dir().unwrap())
    } else {
        std::env::current_dir().unwrap()
    };
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
    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

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

