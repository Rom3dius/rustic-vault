use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    pub id: String,
    pub name: String,
    pub repo_path: PathBuf,
    pub password_file: Option<PathBuf>,
    pub save_password: bool,
}

impl RepoConfig {
    pub fn new(name: String, repo_path: PathBuf) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Self {
            id,
            name,
            repo_path,
            password_file: None,
            save_password: true,
        }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub repos: Vec<RepoConfig>,
    pub current_repo: Option<String>,

    // Legacy fields — kept for migration from old config format
    #[serde(default, skip_serializing)]
    pub repo_path: Option<PathBuf>,
    #[serde(default, skip_serializing)]
    pub password_file: Option<PathBuf>,
    #[serde(default, skip_serializing)]
    pub save_password: Option<bool>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: String::from("Dark"),
            repos: Vec::new(),
            current_repo: None,
            repo_path: None,
            password_file: None,
            save_password: None,
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
                Ok(data) => {
                    let mut config: Self =
                        serde_json::from_str(&data).unwrap_or_default();
                    config.migrate(base);
                    config
                }
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Migrate from legacy single-repo config to multi-repo format.
    fn migrate(&mut self, base: &Path) {
        if self.repos.is_empty() {
            if let Some(ref repo_path) = self.repo_path.take() {
                let save = self.save_password.take().unwrap_or(true);
                let pw_file = self.password_file.take();
                let mut repo = RepoConfig::new("Default".into(), repo_path.clone());
                repo.save_password = save;
                repo.password_file = pw_file;
                self.current_repo = Some(repo.id.clone());
                self.repos.push(repo);
                let _ = self.save(base);
            }
        }
    }

    pub fn save(&self, base: &Path) -> Result<(), String> {
        let path = Self::config_path(base);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())
    }

    /// Get the current repo config, if any.
    pub fn current_repo_config(&self) -> Option<&RepoConfig> {
        let id = self.current_repo.as_ref()?;
        self.repos.iter().find(|r| &r.id == id)
    }

    /// Get a mutable reference to the current repo config.
    pub fn current_repo_config_mut(&mut self) -> Option<&mut RepoConfig> {
        let id = self.current_repo.as_ref()?.clone();
        self.repos.iter_mut().find(|r| r.id == id)
    }

    /// Add a new repo and set it as current.
    pub fn add_repo(&mut self, repo: RepoConfig) {
        self.current_repo = Some(repo.id.clone());
        self.repos.push(repo);
    }

    /// Remove a repo by id. If it was current, clear current_repo.
    pub fn remove_repo(&mut self, id: &str) {
        self.repos.retain(|r| r.id != id);
        if self.current_repo.as_deref() == Some(id) {
            self.current_repo = self.repos.first().map(|r| r.id.clone());
        }
    }
}
