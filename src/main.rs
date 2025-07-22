//! `magish` - cross-platform Rust utility to locate and run Bash scripts.
//!
//! Features:
//! - Detects OS and on Windows ensures WSL2 is available (or directs to install it).
//! - Prompts for a Bash script path, or auto-discovers `.sh` files in current directory.
//! - Executes each line of the script in Bash/WSL sequentially.
//! - Retry up to 3 times for missing scripts, then exits.

mod config;
mod history;

use config::Config;
use history::CommandHistory;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;

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

Write help to see available commands.
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
                let target_path = match path {
                    "." => current_dir.to_path_buf(),
                    ".." => current_dir.parent().unwrap_or(&current_dir).to_path_buf(),
                    _ => resolve_path(&current_dir, path)
                };
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
                let target_path = match path {
                    "." => current_dir.to_path_buf(),
                    ".." => current_dir.parent().unwrap_or(&current_dir).to_path_buf(),
                    _ => resolve_path(&current_dir, path)
                };
                if target_path.exists() && target_path.is_dir() {
                    current_dir = target_path;
                    config.last_directory = current_dir.clone();
                    config.save().unwrap_or_else(|e| eprintln!("Failed to save config: {}", e));
                } else {
                    println!("Invalid directory: {}", path);
                }
            }
            "help" => {
                println!("\nAvailable commands:");
                println!("  ls                    List files in current directory");
                println!("  ls <path>             List files in specified directory");
                println!("  cd                    Go to home directory");
                println!("  cd <path>             Change to specified directory");
                println!("  scan                  Search for all bash scripts in the system");
                println!("  scan -o               Same as scan but also saves to magish_scripts.txt");
                println!("  help                  Show this help message");
                println!("  quit, exit            Exit the program");
                println!("\nOther features:");
                println!("  - Enter a number to run the corresponding script from the list");
                println!("  - Enter a path to navigate to that directory");
                println!("  - Enter a script path to run it");
                println!("  - Press Enter with no input to auto-discover and run a script");
                println!("  - Use arrow keys for command history");
            },
            "quit" | "exit" => {
                history.save_history().unwrap_or_else(|e| eprintln!("Failed to save history: {}", e));
                return;
            }
            "scan" => {
                let scripts = scan_filesystem(false);
                for (i, script) in scripts.iter().enumerate() {
                    println!("  [{}] {}", i + 1, script.display());
                }
                println!("\nEnter a number to run a script, or press Enter to continue.");
                let mut choice = String::new();
                if io::stdin().read_line(&mut choice).is_ok() {
                    if let Ok(num) = choice.trim().parse::<usize>() {
                        if num > 0 && num <= scripts.len() {
                            execute_script(&scripts[num - 1], &current_dir);
                            break;
                        } else {
                            println!("Invalid script number.");
                        }
                    }
                }
            }
            input if input.starts_with("scan -o") => {
                let scripts = scan_filesystem(true);
                println!("Scan complete. Found {} scripts.", scripts.len());
            }
            input => {
                // Try to parse as a number first
                if let Ok(num) = input.parse::<usize>() {
                    let bash_files = list_bash_files(&current_dir);
                    if num > 0 && num <= bash_files.len() {
                        execute_script(&bash_files[num - 1], &current_dir);
                        break;
                    } else {
                        println!("Invalid script number. Please choose between 1 and {}", if bash_files.is_empty() { 1 } else { bash_files.len() });
                    }
                } else {
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
    }

    history.save_history().unwrap_or_else(|e| eprintln!("Failed to save history: {}", e));
    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

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
        let mut working_dir = current_dir.to_path_buf();
        for line in contents.lines() {
            let cmd = line.trim();
            if cmd.is_empty() || cmd.starts_with('#') {
                continue;
            }
            println!("Executing: {}", cmd);
            
            // If this is a cd command, update the working directory
            if cmd.starts_with("cd ") {
                let dir = cmd[3..].trim();
                let new_dir = if dir.starts_with('/') {
                    PathBuf::from(dir)
                } else {
                    working_dir.join(dir)
                };
                if new_dir.exists() && new_dir.is_dir() {
                    working_dir = new_dir;
                }
            }
            
            let mut command = if cfg!(target_os = "windows") {
                let mut c = Command::new("wsl");
                c.arg("bash").arg("-c").arg(cmd);
                c
            } else {
                let mut c = Command::new("bash");
                c.arg("-c").arg(cmd);
                c
            };
            command.current_dir(&working_dir);
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

/// Recursively scans the filesystem for bash scripts.
/// If save_to_file is true, saves the list to "magish_scripts.txt".
fn scan_filesystem(save_to_file: bool) -> Vec<PathBuf> {
    use std::collections::HashSet;
    use std::io::Write;
    let mut scripts = HashSet::new();
    
    // Start from common directories
    let start_dirs = vec![
        dirs::home_dir(),
        Some(PathBuf::from("/")),
        dirs::document_dir(),
        dirs::desktop_dir(),
    ];

    println!("Scanning filesystem for bash scripts...");
    let start_time = std::time::Instant::now();
    let mut last_update = std::time::Instant::now();
    let mut files_scanned = 0;
    let mut scripts_found = 0;
    let mut loading_chars = ['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'].iter().cycle();

    // Start scanning
    for start_dir in start_dirs.into_iter().flatten() {
        // Define a closure for updating the loading bar
        let mut update_progress = |current_scripts: usize| {
            files_scanned += 1;
            scripts_found = current_scripts;
            if last_update.elapsed() >= std::time::Duration::from_millis(100) {
                print!("\r{} Files scanned: {}, Scripts found: {}, Time: {:?}    ", 
                    loading_chars.next().unwrap(),
                    files_scanned,
                    scripts_found,
                    start_time.elapsed());
                let _ = std::io::stdout().flush();
                last_update = std::time::Instant::now();
            }
        };
        
        scan_dir_recursively(&start_dir, &mut scripts, &mut update_progress);
    }

    println!("\nScan complete! Found {} scripts in {:?}", scripts.len(), start_time.elapsed());
    
    let mut scripts: Vec<_> = scripts.into_iter().collect();
    scripts.sort();

    if save_to_file {
        if let Ok(mut file) = fs::File::create("magish_scripts.txt") {
            use std::io::Write;
            for (i, script) in scripts.iter().enumerate() {
                if let Err(e) = writeln!(file, "[{}] {}", i + 1, script.display()) {
                    eprintln!("Error writing to file: {}", e);
                    break;
                }
            }
            println!("Script list saved to magish_scripts.txt");
        }
    }

    scripts
}

/// Helper function for scanning directories recursively
fn scan_dir_recursively(dir: &Path, scripts: &mut HashSet<PathBuf>, update_progress: &mut dyn FnMut(usize)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                // Skip hidden directories and certain system paths
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if !name.starts_with('.') && 
                       !name.contains("node_modules") && 
                       !name.contains("target") &&
                       !name.contains("vendor") {
                        scan_dir_recursively(&path, scripts, update_progress);
                    }
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("sh") {
                scripts.insert(path.clone());
            }
            update_progress(scripts.len());
        }
    }
}

