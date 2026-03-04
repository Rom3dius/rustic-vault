use std::path::PathBuf;

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Task, Theme};

use crate::core::config::AppConfig;
use crate::core::profile::Profile;
use crate::core::repo::{BackupSummary, ForgetPruneSummary, RepoInfo, SnapshotInfo};
use crate::ui::widgets::theme_from_name;

// ── Screens ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Screen {
    FirstRun,
    Home,
    ProfileEditor,
    Backup,
    Snapshots,
    Settings,
}

// ── Messages ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    // Navigation
    GoHome,
    GoSettings,
    GoSnapshots,
    GoProfileEditor(Option<usize>), // None = new, Some(idx) = edit existing
    GoBackup(usize),                // profile index

    // First-run
    FirstRunPasswordChanged(String),
    FirstRunToggleCustomPassword(bool),
    FirstRunToggleSavePassword(bool),
    FirstRunGeneratePassword,
    FirstRunInit,
    FirstRunInitResult(Result<(), String>),

    // Home
    SelectProfile(usize),
    DeleteProfile(usize),
    DeleteProfileResult(Result<(), String>),

    // Profile editor
    ProfileNameChanged(String),
    ProfileAddFolder,
    ProfileFolderPicked(Option<PathBuf>),
    ProfileRemoveFolder(usize),
    ProfileExcludesChanged(String),
    ProfileTagsChanged(String),
    ProfileKeepLastChanged(String),
    ProfileKeepDailyChanged(String),
    ProfileKeepWeeklyChanged(String),
    ProfileKeepMonthlyChanged(String),
    ProfileSave,
    ProfileSaveResult(Result<(), String>),
    ProfileCancel,

    // Backup
    BackupStarted,
    BackupResult(Result<BackupSummary, String>),

    // Snapshots
    SnapshotsLoaded(Result<Vec<SnapshotInfo>, String>),
    SnapshotRestore(String),
    SnapshotRestoreFolderPicked(Option<PathBuf>),
    SnapshotRestoreResult(Result<(), String>),
    SnapshotDelete(String),
    SnapshotDeleteResult(Result<(), String>),
    SnapshotForgetPrune,
    SnapshotForgetPruneResult(Result<ForgetPruneSummary, String>),
    RefreshSnapshots,

    // Settings
    SettingsToggleSavePassword(bool),
    SettingsThemeSelected(String),
    SettingsPasswordChanged(String),
    SettingsChangePassword,
    SettingsChangePasswordResult(Result<(), String>),
    SettingsShowPassword,
    SettingsRepoInfoLoaded(Result<RepoInfo, String>),

    // Password prompt (when password not saved)
    PasswordPromptChanged(String),
    PasswordPromptSubmit,

    // Status
    StatusMessage(String),
    ClearStatus,
}

// ── App State ────────────────────────────────────────────────────────────────

pub struct App {
    pub base_path: PathBuf,
    pub config: AppConfig,
    pub screen: Screen,
    pub profiles: Vec<Profile>,
    pub selected_profile: Option<usize>,

    // Password management
    pub password: String,
    pub password_unlocked: bool,
    pub password_visible: bool,

    // First-run state
    pub first_run_password: String,
    pub first_run_custom_password: bool,
    pub first_run_save_password: bool,
    pub first_run_generated: String,

    // Profile editor state
    pub editor_profile: Option<Profile>,
    pub editor_excludes_text: String,
    pub editor_tags_text: String,
    pub editor_keep_last: String,
    pub editor_keep_daily: String,
    pub editor_keep_weekly: String,
    pub editor_keep_monthly: String,

    // Backup state
    pub backup_running: bool,
    pub backup_summary: Option<BackupSummary>,

    // Snapshots state
    pub snapshots: Vec<SnapshotInfo>,
    pub snapshots_loading: bool,
    pub restore_snapshot_id: Option<String>,

    // Settings state
    pub settings_new_password: String,
    pub repo_info: Option<RepoInfo>,

    // Status bar
    pub status: String,
    pub busy: bool,
}

impl App {
    pub fn new(base_path: PathBuf) -> (Self, Task<Message>) {
        let config = AppConfig::load(&base_path);
        let profiles_dir = base_path.join("profiles");
        let profiles = Profile::load_all(&profiles_dir);
        let repo_path = config.resolve_repo_path(&base_path);

        // Check if repo exists
        let repo_exists = repo_path.join("config").exists();

        // Try to load password from file
        let (password, password_unlocked) =
            if let Some(pw_path) = config.resolve_password_path(&base_path) {
                if pw_path.exists() {
                    match std::fs::read_to_string(&pw_path) {
                        Ok(pw) => (pw.trim().to_string(), true),
                        Err(_) => (String::new(), false),
                    }
                } else {
                    (String::new(), false)
                }
            } else {
                (String::new(), false)
            };

        let screen = if !repo_exists {
            Screen::FirstRun
        } else if !password_unlocked {
            // Need password prompt - but we'll handle this as a special state
            Screen::Home
        } else {
            Screen::Home
        };

        // Generate a random password for first-run
        let generated = generate_password();

        let app = Self {
            base_path,
            config,
            screen,
            profiles,
            selected_profile: None,

            password,
            password_unlocked,
            password_visible: false,

            first_run_password: generated.clone(),
            first_run_custom_password: false,
            first_run_save_password: true,
            first_run_generated: generated,

            editor_profile: None,
            editor_excludes_text: String::new(),
            editor_tags_text: String::new(),
            editor_keep_last: String::new(),
            editor_keep_daily: String::new(),
            editor_keep_weekly: String::new(),
            editor_keep_monthly: String::new(),

            backup_running: false,
            backup_summary: None,

            snapshots: Vec::new(),
            snapshots_loading: false,
            restore_snapshot_id: None,

            settings_new_password: String::new(),
            repo_info: None,

            status: String::new(),
            busy: false,
        };

        (app, Task::none())
    }

    #[allow(dead_code)]
    pub fn title(&self) -> String {
        "Rustic Vault".to_string()
    }

    pub fn theme(&self) -> Theme {
        theme_from_name(&self.config.theme)
    }

    pub fn profiles_dir(&self) -> PathBuf {
        self.base_path.join("profiles")
    }

    pub fn repo_path(&self) -> PathBuf {
        self.config.resolve_repo_path(&self.base_path)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // ── Navigation ───────────────────────────────────────────────
            Message::GoHome => {
                self.screen = Screen::Home;
                self.backup_summary = None;
                Task::none()
            }
            Message::GoSettings => {
                self.screen = Screen::Settings;
                self.settings_new_password.clear();
                self.password_visible = false;
                // Load repo info
                let repo_path = self.repo_path();
                let password = self.password.clone();
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::repo::repo_info(&repo_path, &password)
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Message::SettingsRepoInfoLoaded,
                )
            }
            Message::GoSnapshots => {
                self.screen = Screen::Snapshots;
                self.load_snapshots()
            }
            Message::GoProfileEditor(idx) => {
                self.screen = Screen::ProfileEditor;
                if let Some(i) = idx {
                    if let Some(p) = self.profiles.get(i) {
                        self.editor_excludes_text = p.excludes.join(", ");
                        self.editor_tags_text = p.tags.join(", ");
                        self.editor_keep_last = p
                            .retention
                            .keep_last
                            .map_or(String::new(), |n| n.to_string());
                        self.editor_keep_daily = p
                            .retention
                            .keep_daily
                            .map_or(String::new(), |n| n.to_string());
                        self.editor_keep_weekly = p
                            .retention
                            .keep_weekly
                            .map_or(String::new(), |n| n.to_string());
                        self.editor_keep_monthly = p
                            .retention
                            .keep_monthly
                            .map_or(String::new(), |n| n.to_string());
                        self.editor_profile = Some(p.clone());
                    }
                } else {
                    let new_profile = Profile::new("New Profile".to_string());
                    self.editor_excludes_text = new_profile.excludes.join(", ");
                    self.editor_tags_text = new_profile.tags.join(", ");
                    self.editor_keep_last = new_profile
                        .retention
                        .keep_last
                        .map_or(String::new(), |n| n.to_string());
                    self.editor_keep_daily = new_profile
                        .retention
                        .keep_daily
                        .map_or(String::new(), |n| n.to_string());
                    self.editor_keep_weekly = new_profile
                        .retention
                        .keep_weekly
                        .map_or(String::new(), |n| n.to_string());
                    self.editor_keep_monthly = new_profile
                        .retention
                        .keep_monthly
                        .map_or(String::new(), |n| n.to_string());
                    self.editor_profile = Some(new_profile);
                }
                Task::none()
            }
            Message::GoBackup(idx) => {
                self.screen = Screen::Backup;
                self.backup_running = true;
                self.backup_summary = None;
                let profile = self.profiles[idx].clone();
                let repo_path = self.repo_path();
                let password = self.password.clone();
                let hostname = hostname::get()
                    .ok()
                    .and_then(|h| h.into_string().ok());
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::repo::run_backup(
                                &repo_path,
                                &password,
                                &profile.paths,
                                &profile.excludes,
                                &profile.tags,
                                hostname.as_deref(),
                            )
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Message::BackupResult,
                )
            }

            // ── First Run ────────────────────────────────────────────────
            Message::FirstRunPasswordChanged(pw) => {
                self.first_run_password = pw;
                Task::none()
            }
            Message::FirstRunToggleCustomPassword(custom) => {
                self.first_run_custom_password = custom;
                if !custom {
                    self.first_run_password = self.first_run_generated.clone();
                } else {
                    self.first_run_password.clear();
                }
                Task::none()
            }
            Message::FirstRunToggleSavePassword(save) => {
                self.first_run_save_password = save;
                Task::none()
            }
            Message::FirstRunGeneratePassword => {
                self.first_run_generated = generate_password();
                if !self.first_run_custom_password {
                    self.first_run_password = self.first_run_generated.clone();
                }
                Task::none()
            }
            Message::FirstRunInit => {
                self.busy = true;
                self.status = "Initializing repository...".into();
                let repo_path = self.repo_path();
                let password = self.first_run_password.clone();
                let save = self.first_run_save_password;
                let base = self.base_path.clone();

                // Save password to file if requested
                if save {
                    let pw_path = base.join("../.password");
                    let _ = std::fs::write(&pw_path, &password);
                }

                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::repo::init_repo(&repo_path, &password)
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Message::FirstRunInitResult,
                )
            }
            Message::FirstRunInitResult(result) => {
                self.busy = false;
                match result {
                    Ok(()) => {
                        self.password = self.first_run_password.clone();
                        self.password_unlocked = true;
                        self.config.save_password = self.first_run_save_password;
                        if self.first_run_save_password {
                            self.config.password_file = Some(PathBuf::from("../.password"));
                        } else {
                            self.config.password_file = None;
                        }
                        let _ = self.config.save(&self.base_path);
                        self.screen = Screen::Home;
                        self.status = "Repository initialized successfully!".into();
                    }
                    Err(e) => {
                        self.status = format!("Init failed: {e}");
                    }
                }
                Task::none()
            }

            // ── Home ─────────────────────────────────────────────────────
            Message::SelectProfile(idx) => {
                self.selected_profile = Some(idx);
                Task::none()
            }
            Message::DeleteProfile(idx) => {
                if let Some(profile) = self.profiles.get(idx) {
                    let profiles_dir = self.profiles_dir();
                    let id = profile.id.clone();
                    self.profiles.remove(idx);
                    if self.selected_profile == Some(idx) {
                        self.selected_profile = None;
                    }
                    Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                Profile::delete(&profiles_dir, &id)
                            })
                            .await
                            .unwrap_or_else(|e| Err(e.to_string()))
                        },
                        Message::DeleteProfileResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::DeleteProfileResult(result) => {
                if let Err(e) = result {
                    self.status = format!("Delete failed: {e}");
                }
                Task::none()
            }

            // ── Profile Editor ───────────────────────────────────────────
            Message::ProfileNameChanged(name) => {
                if let Some(ref mut p) = self.editor_profile {
                    p.name = name;
                }
                Task::none()
            }
            Message::ProfileAddFolder => {
                Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select folder to back up")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    Message::ProfileFolderPicked,
                )
            }
            Message::ProfileFolderPicked(path) => {
                if let (Some(path), Some(profile)) = (path, &mut self.editor_profile) {
                    if !profile.paths.contains(&path) {
                        profile.paths.push(path);
                    }
                }
                Task::none()
            }
            Message::ProfileRemoveFolder(idx) => {
                if let Some(ref mut p) = self.editor_profile {
                    if idx < p.paths.len() {
                        p.paths.remove(idx);
                    }
                }
                Task::none()
            }
            Message::ProfileExcludesChanged(s) => {
                self.editor_excludes_text = s;
                Task::none()
            }
            Message::ProfileTagsChanged(s) => {
                self.editor_tags_text = s;
                Task::none()
            }
            Message::ProfileKeepLastChanged(s) => {
                self.editor_keep_last = s;
                Task::none()
            }
            Message::ProfileKeepDailyChanged(s) => {
                self.editor_keep_daily = s;
                Task::none()
            }
            Message::ProfileKeepWeeklyChanged(s) => {
                self.editor_keep_weekly = s;
                Task::none()
            }
            Message::ProfileKeepMonthlyChanged(s) => {
                self.editor_keep_monthly = s;
                Task::none()
            }
            Message::ProfileSave => {
                let profiles_dir = self.profiles_dir();
                if let Some(ref mut profile) = self.editor_profile {
                    // Parse fields
                    profile.excludes = self
                        .editor_excludes_text
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    profile.tags = self
                        .editor_tags_text
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    profile.retention.keep_last = self.editor_keep_last.parse().ok();
                    profile.retention.keep_daily = self.editor_keep_daily.parse().ok();
                    profile.retention.keep_weekly = self.editor_keep_weekly.parse().ok();
                    profile.retention.keep_monthly = self.editor_keep_monthly.parse().ok();

                    let p = profile.clone();

                    // Update or add to local list
                    if let Some(pos) = self.profiles.iter().position(|x| x.id == p.id) {
                        self.profiles[pos] = p.clone();
                    } else {
                        self.profiles.push(p.clone());
                    }

                    Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || p.save(&profiles_dir))
                                .await
                                .unwrap_or_else(|e| Err(e.to_string()))
                        },
                        Message::ProfileSaveResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::ProfileSaveResult(result) => {
                match result {
                    Ok(()) => {
                        self.screen = Screen::Home;
                        self.status = "Profile saved.".into();
                    }
                    Err(e) => {
                        self.status = format!("Save failed: {e}");
                    }
                }
                Task::none()
            }
            Message::ProfileCancel => {
                self.screen = Screen::Home;
                self.editor_profile = None;
                Task::none()
            }

            // ── Backup ───────────────────────────────────────────────────
            Message::BackupStarted => Task::none(),
            Message::BackupResult(result) => {
                self.backup_running = false;
                match result {
                    Ok(summary) => {
                        // Update last_backup on the profile
                        let profiles_dir = self.profiles_dir();
                        if let Some(idx) = self.selected_profile {
                            if let Some(p) = self.profiles.get_mut(idx) {
                                p.last_backup = Some(chrono::Utc::now());
                                let _ = p.save(&profiles_dir);
                            }
                        }
                        self.status = format!(
                            "Backup complete! {} new, {} changed, {} unchanged",
                            summary.files_new, summary.files_changed, summary.files_unmodified
                        );
                        self.backup_summary = Some(summary);
                    }
                    Err(e) => {
                        self.status = format!("Backup failed: {e}");
                    }
                }
                Task::none()
            }

            // ── Snapshots ────────────────────────────────────────────────
            Message::SnapshotsLoaded(result) => {
                self.snapshots_loading = false;
                match result {
                    Ok(snaps) => self.snapshots = snaps,
                    Err(e) => self.status = format!("Failed to load snapshots: {e}"),
                }
                Task::none()
            }
            Message::RefreshSnapshots => self.load_snapshots(),
            Message::SnapshotRestore(id) => {
                self.restore_snapshot_id = Some(id);
                Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select restore destination")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    Message::SnapshotRestoreFolderPicked,
                )
            }
            Message::SnapshotRestoreFolderPicked(path) => {
                if let (Some(target), Some(snap_id)) = (path, self.restore_snapshot_id.take()) {
                    self.busy = true;
                    self.status = "Restoring...".into();
                    let repo_path = self.repo_path();
                    let password = self.password.clone();
                    Task::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                crate::core::repo::restore_snapshot(
                                    &repo_path,
                                    &password,
                                    &snap_id,
                                    &target,
                                )
                            })
                            .await
                            .unwrap_or_else(|e| Err(e.to_string()))
                        },
                        Message::SnapshotRestoreResult,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SnapshotRestoreResult(result) => {
                self.busy = false;
                match result {
                    Ok(()) => self.status = "Restore complete!".into(),
                    Err(e) => self.status = format!("Restore failed: {e}"),
                }
                Task::none()
            }
            Message::SnapshotDelete(id) => {
                self.busy = true;
                self.status = "Deleting snapshot...".into();
                let repo_path = self.repo_path();
                let password = self.password.clone();
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::repo::delete_snapshot(&repo_path, &password, &id)
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Message::SnapshotDeleteResult,
                )
            }
            Message::SnapshotDeleteResult(result) => {
                self.busy = false;
                match result {
                    Ok(()) => {
                        self.status = "Snapshot deleted.".into();
                        return self.load_snapshots();
                    }
                    Err(e) => self.status = format!("Delete failed: {e}"),
                }
                Task::none()
            }
            Message::SnapshotForgetPrune => {
                // Use retention from selected profile, or defaults
                let retention = self
                    .selected_profile
                    .and_then(|i| self.profiles.get(i))
                    .map(|p| p.retention.clone())
                    .unwrap_or_default();

                self.busy = true;
                self.status = "Applying retention policy and pruning...".into();
                let repo_path = self.repo_path();
                let password = self.password.clone();
                Task::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            crate::core::repo::forget_and_prune(&repo_path, &password, &retention)
                        })
                        .await
                        .unwrap_or_else(|e| Err(e.to_string()))
                    },
                    Message::SnapshotForgetPruneResult,
                )
            }
            Message::SnapshotForgetPruneResult(result) => {
                self.busy = false;
                match result {
                    Ok(summary) => {
                        self.status = format!(
                            "Pruned: {} removed, {} kept",
                            summary.snapshots_removed, summary.snapshots_kept
                        );
                        return self.load_snapshots();
                    }
                    Err(e) => self.status = format!("Prune failed: {e}"),
                }
                Task::none()
            }

            // ── Settings ─────────────────────────────────────────────────
            Message::SettingsToggleSavePassword(save) => {
                self.config.save_password = save;
                if save {
                    self.config.password_file = Some(PathBuf::from("../.password"));
                    let pw_path = self.base_path.join("../.password");
                    let _ = std::fs::write(&pw_path, &self.password);
                } else {
                    self.config.password_file = None;
                    let pw_path = self.base_path.join("../.password");
                    let _ = std::fs::remove_file(&pw_path);
                }
                let _ = self.config.save(&self.base_path);
                Task::none()
            }
            Message::SettingsThemeSelected(theme) => {
                self.config.theme = theme;
                let _ = self.config.save(&self.base_path);
                Task::none()
            }
            Message::SettingsPasswordChanged(pw) => {
                self.settings_new_password = pw;
                Task::none()
            }
            Message::SettingsChangePassword => {
                // TODO: rustic_core key change not trivially exposed.
                // For now, update the stored password file.
                self.password = self.settings_new_password.clone();
                if self.config.save_password {
                    let pw_path = self.base_path.join("../.password");
                    let _ = std::fs::write(&pw_path, &self.password);
                }
                self.status = "Password updated (stored locally).".into();
                self.settings_new_password.clear();
                Task::none()
            }
            Message::SettingsChangePasswordResult(_) => Task::none(),
            Message::SettingsShowPassword => {
                self.password_visible = !self.password_visible;
                Task::none()
            }
            Message::SettingsRepoInfoLoaded(result) => {
                match result {
                    Ok(info) => self.repo_info = Some(info),
                    Err(e) => self.status = format!("Repo info error: {e}"),
                }
                Task::none()
            }

            // ── Password Prompt ──────────────────────────────────────────
            Message::PasswordPromptChanged(pw) => {
                self.password = pw;
                Task::none()
            }
            Message::PasswordPromptSubmit => {
                self.password_unlocked = true;
                Task::none()
            }

            // ── Status ───────────────────────────────────────────────────
            Message::StatusMessage(msg) => {
                self.status = msg;
                Task::none()
            }
            Message::ClearStatus => {
                self.status.clear();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // If password not unlocked and repo exists, show password prompt
        if !self.password_unlocked && !matches!(self.screen, Screen::FirstRun) {
            return self.view_password_prompt();
        }

        let content: Element<Message> = match self.screen {
            Screen::FirstRun => crate::ui::screen_home::view_first_run(self),
            Screen::Home => crate::ui::screen_home::view(self),
            Screen::ProfileEditor => crate::ui::screen_profile::view(self),
            Screen::Backup => crate::ui::screen_backup::view(self),
            Screen::Snapshots => crate::ui::screen_snapshots::view(self),
            Screen::Settings => crate::ui::screen_settings::view(self),
        };

        // Wrap with status bar at bottom
        let mut main = column![content].spacing(0).width(Length::Fill);

        if !self.status.is_empty() {
            let status_bar = container(
                row![
                    text(&self.status).size(13),
                    iced::widget::space::horizontal(),
                    button(text("x").size(12))
                        .on_press(Message::ClearStatus)
                        .style(button::text),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            )
            .padding(8)
            .width(Length::Fill)
            .style(container::rounded_box);

            main = main.push(status_bar);
        }

        container(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_password_prompt(&self) -> Element<'_, Message> {
        let content = column![
            text("Rustic Vault").size(28),
            text("Enter your repository password to continue.").size(14),
            iced::widget::text_input("Password...", &self.password)
                .on_input(Message::PasswordPromptChanged)
                .on_submit(Message::PasswordPromptSubmit)
                .secure(true)
                .width(300),
            button(text("Unlock")).on_press(Message::PasswordPromptSubmit),
        ]
        .spacing(16)
        .align_x(iced::Alignment::Center);

        container(content)
            .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn load_snapshots(&mut self) -> Task<Message> {
        self.snapshots_loading = true;
        let repo_path = self.repo_path();
        let password = self.password.clone();
        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    crate::core::repo::list_snapshots(&repo_path, &password)
                })
                .await
                .unwrap_or_else(|e| Err(e.to_string()))
            },
            Message::SnapshotsLoaded,
        )
    }
}

/// Generate a random alphanumeric password.
fn generate_password() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%&*";
    let mut rng = rand::rng();
    (0..24)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
