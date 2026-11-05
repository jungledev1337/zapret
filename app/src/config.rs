use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub discord: bool,
    pub telegram: bool,
    pub youtube: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            discord: true,
            telegram: true,
            youtube: true,
        }
    }
}

impl AppConfig {
    pub fn load_from(path: &Path) -> Self {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(cfg) = serde_json::from_str(&data) {
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save_to(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }
}
