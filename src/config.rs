use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub log_level: String,
    pub timeout: u64,
    pub retry_count: Option<u64>,
    pub retry_delay: u64,
    /// Last loaded .usr file path — restored on startup
    pub last_usr_file: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            timeout: 5,
            retry_count: None,
            retry_delay: 500,
            last_usr_file: None,
        }
    }
}

impl AppSettings {
    pub fn cache_dir() -> Option<PathBuf> {
        ProjectDirs::from("rs", "cygnus", "cygnus-gui")
            .map(|dirs| dirs.cache_dir().to_path_buf())
    }

    pub fn cache_path() -> Option<PathBuf> {
        Self::cache_dir().map(|p| p.join("state.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::cache_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(settings) = serde_json::from_str(&content) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        if let Some(dir) = Self::cache_dir() {
            std::fs::create_dir_all(&dir)?;
            let content = serde_json::to_string_pretty(self).map_err(|e| {
                std::io::Error::other(format!("serde error: {}", e))
            })?;
            std::fs::write(dir.join("state.json"), content)?;
        }
        Ok(())
    }
}
