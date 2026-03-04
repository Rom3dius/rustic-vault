use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub repo_path: PathBuf,
    pub password_file: Option<PathBuf>,
    pub theme: String,
    pub save_password: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("../repo"),
            password_file: Some(PathBuf::from("../.password")),
            theme: String::from("Dark"),
            save_password: true,
        }
    }
}

impl AppConfig {
    pub fn config_path(base: &Path) -> PathBuf {
        base.join("config.json")
    }

    pub fn load(base: &Path) -> Self {
        let path = Self::config_path(base);
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    pub fn save(&self, base: &Path) -> Result<(), String> {
        let path = Self::config_path(base);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())
    }

    /// Resolve the repo path relative to the project base directory.
    pub fn resolve_repo_path(&self, base: &Path) -> PathBuf {
        if self.repo_path.is_absolute() {
            self.repo_path.clone()
        } else {
            base.join(&self.repo_path)
        }
    }

    /// Resolve the password file path relative to the project base directory.
    pub fn resolve_password_path(&self, base: &Path) -> Option<PathBuf> {
        self.password_file.as_ref().map(|p| {
            if p.is_absolute() {
                p.clone()
            } else {
                base.join(p)
            }
        })
    }
}
