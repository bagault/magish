use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub last_directory: PathBuf,
    pub history_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_directory: dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
            history_limit: 100,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if let Ok(contents) = fs::read_to_string(&config_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            let config = Config::default();
            let _ = config.save();
            config
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let config_path = Self::get_config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, contents)
    }

    fn get_config_path() -> PathBuf {
        let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        exe_path.parent().unwrap_or(Path::new(".")).join("configs.json")
    }
}
