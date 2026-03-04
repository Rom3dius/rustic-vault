use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_last: Option<u32>,
    pub keep_daily: Option<u32>,
    pub keep_weekly: Option<u32>,
    pub keep_monthly: Option<u32>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_last: Some(5),
            keep_daily: Some(7),
            keep_weekly: Some(4),
            keep_monthly: Some(12),
        }
    }
}

impl RetentionPolicy {
    /// Estimate the approximate number of snapshots this policy will keep.
    pub fn estimated_count(&self) -> u32 {
        self.keep_last.unwrap_or(0)
            + self.keep_daily.unwrap_or(0)
            + self.keep_weekly.unwrap_or(0)
            + self.keep_monthly.unwrap_or(0)
    }

    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if let Some(n) = self.keep_last {
            parts.push(format!("{n} most recent"));
        }
        if let Some(n) = self.keep_daily {
            parts.push(format!("{n} daily"));
        }
        if let Some(n) = self.keep_weekly {
            parts.push(format!("{n} weekly"));
        }
        if let Some(n) = self.keep_monthly {
            parts.push(format!("{n} monthly"));
        }
        if parts.is_empty() {
            "No retention policy set".to_string()
        } else {
            format!(
                "Keep ~{} snapshots: {}",
                self.estimated_count(),
                parts.join(" + ")
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub paths: Vec<PathBuf>,
    pub excludes: Vec<String>,
    pub tags: Vec<String>,
    pub retention: RetentionPolicy,
    pub last_backup: Option<DateTime<Utc>>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            paths: Vec::new(),
            excludes: vec![
                "node_modules".into(),
                ".cache".into(),
                "__pycache__".into(),
                "*.tmp".into(),
                ".git".into(),
            ],
            tags: Vec::new(),
            retention: RetentionPolicy::default(),
            last_backup: None,
        }
    }

    fn file_path(profiles_dir: &Path, id: &str) -> PathBuf {
        profiles_dir.join(format!("{id}.json"))
    }

    pub fn save(&self, profiles_dir: &Path) -> Result<(), String> {
        fs::create_dir_all(profiles_dir).map_err(|e| e.to_string())?;
        let path = Self::file_path(profiles_dir, &self.id);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn delete(profiles_dir: &Path, id: &str) -> Result<(), String> {
        let path = Self::file_path(profiles_dir, id);
        if path.exists() {
            fs::remove_file(path).map_err(|e| e.to_string())
        } else {
            Ok(())
        }
    }

    pub fn load_all(profiles_dir: &Path) -> Vec<Profile> {
        let mut profiles = Vec::new();
        if let Ok(entries) = fs::read_dir(profiles_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "json") {
                    if let Ok(data) = fs::read_to_string(&path) {
                        if let Ok(profile) = serde_json::from_str::<Profile>(&data) {
                            profiles.push(profile);
                        }
                    }
                }
            }
        }
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        profiles
    }
}
