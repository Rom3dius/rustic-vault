use std::path::{Path, PathBuf};

use rustic_backend::BackendOptions;
use rustic_core::{
    BackupOptions, ConfigOptions, Credentials, KeyOptions, LocalDestination, LsOptions,
    PruneOptions, Repository, RepositoryOptions, RestoreOptions, SnapshotOptions,
    repofile::SnapshotFile,
};

use crate::core::profile::RetentionPolicy;

/// Summary of a completed backup.
#[derive(Debug, Clone)]
pub struct BackupSummary {
    pub snapshot_id: String,
    pub files_new: u64,
    pub files_changed: u64,
    pub files_unmodified: u64,
    pub data_added: u64,
    pub total_bytes_processed: u64,
}

/// Summary of a forget+prune operation.
#[derive(Debug, Clone)]
pub struct ForgetPruneSummary {
    pub snapshots_removed: usize,
    pub snapshots_kept: usize,
}

/// Information about a snapshot for display.
#[derive(Debug, Clone)]
pub struct SnapshotInfo {
    pub id: String,
    pub short_id: String,
    pub time: String,
    pub hostname: String,
    pub paths: Vec<String>,
    pub tags: Vec<String>,
    pub summary_size: Option<u64>,
}

impl From<SnapshotFile> for SnapshotInfo {
    fn from(snap: SnapshotFile) -> Self {
        let id_str = snap.id.to_string();
        let short = if id_str.len() >= 8 {
            id_str[..8].to_string()
        } else {
            id_str.clone()
        };
        Self {
            id: id_str,
            short_id: short,
            time: snap.time.to_string(),
            hostname: snap.hostname.clone(),
            paths: snap.paths.iter().map(|s| s.to_string()).collect(),
            tags: snap.tags.iter().map(|s| s.to_string()).collect(),
            summary_size: snap.summary.as_ref().map(|s| s.total_bytes_processed),
        }
    }
}

/// Repository information for the settings screen.
#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub snapshot_count: usize,
}

fn backends(repo_path: &Path) -> Result<rustic_core::RepositoryBackends, String> {
    BackendOptions::default()
        .repository(repo_path.to_string_lossy().as_ref())
        .to_backends()
        .map_err(|e| format!("Failed to create backends: {e}"))
}

fn repo_opts() -> RepositoryOptions {
    RepositoryOptions::default()
}

/// Initialize a new repository at the given path with the given password.
pub fn init_repo(repo_path: &Path, password: &str) -> Result<(), String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be).map_err(|e| format!("Repository error: {e}"))?;
    let key_opts = KeyOptions::default();
    let config_opts = ConfigOptions::default();
    repo.init(&Credentials::password(password), &key_opts, &config_opts)
        .map_err(|e| format!("Init failed: {e}"))?;
    Ok(())
}

/// Open an existing repository and return snapshot count.
pub fn repo_info(repo_path: &Path, password: &str) -> Result<RepoInfo, String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?;

    let snaps = repo
        .get_all_snapshots()
        .map_err(|e| format!("Snapshots error: {e}"))?;

    Ok(RepoInfo {
        snapshot_count: snaps.len(),
    })
}

/// Run a backup with the given sources, excludes, and tags.
pub fn run_backup(
    repo_path: &Path,
    password: &str,
    sources: &[PathBuf],
    excludes: &[String],
    tags: &[String],
    hostname: Option<&str>,
) -> Result<BackupSummary, String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?
        .to_indexed_ids()
        .map_err(|e| format!("Index error: {e}"))?;

    let mut backup_opts = BackupOptions::default();
    for exc in excludes {
        backup_opts.excludes.globs.push(exc.clone());
    }

    let source_str = sources
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join(",");

    let source = rustic_core::PathList::from_string(&source_str)
        .map_err(|e| format!("Path error: {e}"))?
        .sanitize()
        .map_err(|e| format!("Path sanitize error: {e}"))?;

    let tags_str = tags.join(",");
    let mut snap_opts = SnapshotOptions::default();
    if !tags_str.is_empty() {
        snap_opts = snap_opts
            .add_tags(&tags_str)
            .map_err(|e| format!("Tags error: {e}"))?;
    }
    if let Some(h) = hostname {
        snap_opts = snap_opts.host(Some(h.to_string()));
    }
    let snap = snap_opts
        .to_snapshot()
        .map_err(|e| format!("Snapshot creation error: {e}"))?;

    let result = repo
        .backup(&backup_opts, &source, snap)
        .map_err(|e| format!("Backup failed: {e}"))?;

    let summary = result.summary.as_ref();
    Ok(BackupSummary {
        snapshot_id: result.id.to_string(),
        files_new: summary.map_or(0, |s| s.files_new),
        files_changed: summary.map_or(0, |s| s.files_changed),
        files_unmodified: summary.map_or(0, |s| s.files_unmodified),
        data_added: summary.map_or(0, |s| s.data_added),
        total_bytes_processed: summary.map_or(0, |s| s.total_bytes_processed),
    })
}

/// List all snapshots in the repository.
pub fn list_snapshots(repo_path: &Path, password: &str) -> Result<Vec<SnapshotInfo>, String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?;

    let snaps = repo
        .get_all_snapshots()
        .map_err(|e| format!("Snapshots error: {e}"))?;

    Ok(snaps.into_iter().map(SnapshotInfo::from).collect())
}

/// Restore a snapshot to a target directory.
pub fn restore_snapshot(
    repo_path: &Path,
    password: &str,
    snapshot_id: &str,
    target: &Path,
) -> Result<(), String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?
        .to_indexed()
        .map_err(|e| format!("Index error: {e}"))?;

    let node = repo
        .node_from_snapshot_path(snapshot_id, |_| true)
        .map_err(|e| format!("Snapshot lookup error: {e}"))?;

    let ls_opts = LsOptions::default();
    let ls = repo
        .ls(&node, &ls_opts)
        .map_err(|e| format!("Ls error: {e}"))?;

    let dest = LocalDestination::new(
        target.to_string_lossy().as_ref(),
        true,
        !node.is_dir(),
    )
    .map_err(|e| format!("Destination error: {e}"))?;

    let opts = RestoreOptions::default();
    let plan = repo
        .prepare_restore(&opts, ls.clone(), &dest, false)
        .map_err(|e| format!("Restore plan error: {e}"))?;

    repo.restore(plan, &opts, ls, &dest)
        .map_err(|e| format!("Restore failed: {e}"))?;

    Ok(())
}

/// Delete a single snapshot by ID.
pub fn delete_snapshot(repo_path: &Path, password: &str, snapshot_id: &str) -> Result<(), String> {
    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?;

    let snaps = repo
        .get_snapshots(&[snapshot_id])
        .map_err(|e| format!("Snapshot lookup error: {e}"))?;

    let ids: Vec<_> = snaps.iter().map(|s| s.id).collect();
    repo.delete_snapshots(&ids)
        .map_err(|e| format!("Delete failed: {e}"))?;

    Ok(())
}

/// Apply retention policy and prune unreferenced data.
pub fn forget_and_prune(
    repo_path: &Path,
    password: &str,
    keep: &RetentionPolicy,
) -> Result<ForgetPruneSummary, String> {
    use jiff::Zoned;
    use rustic_core::{ForgetGroups, Grouped, KeepOptions, SnapshotGroupCriterion};

    let be = backends(repo_path)?;
    let repo = Repository::new(&repo_opts(), &be)
        .map_err(|e| format!("Repository error: {e}"))?
        .open(&Credentials::password(password))
        .map_err(|e| format!("Open failed: {e}"))?;

    let mut keep_opts = KeepOptions::default();
    if let Some(n) = keep.keep_last {
        keep_opts = keep_opts.keep_last(n as i32);
    }
    if let Some(n) = keep.keep_daily {
        keep_opts = keep_opts.keep_daily(n as i32);
    }
    if let Some(n) = keep.keep_weekly {
        keep_opts = keep_opts.keep_weekly(n as i32);
    }
    if let Some(n) = keep.keep_monthly {
        keep_opts = keep_opts.keep_monthly(n as i32);
    }

    let group_by = SnapshotGroupCriterion::default();
    let snaps = repo
        .get_all_snapshots()
        .map_err(|e| format!("Snapshots error: {e}"))?;
    let total = snaps.len();
    let grouped = Grouped::from_items(snaps, group_by);

    let forget_groups =
        ForgetGroups::from_grouped_snapshots_with_retention(grouped, &keep_opts, &Zoned::now())
            .map_err(|e| format!("Forget error: {e}"))?;

    let ids_to_delete = forget_groups.into_forget_ids();
    let removed = ids_to_delete.len();

    if !ids_to_delete.is_empty() {
        repo.delete_snapshots(&ids_to_delete)
            .map_err(|e| format!("Delete failed: {e}"))?;
    }

    // Prune unreferenced data
    let prune_opts = PruneOptions::default();
    let prune_plan = repo
        .prune_plan(&prune_opts)
        .map_err(|e| format!("Prune plan error: {e}"))?;
    repo.prune(&prune_opts, prune_plan)
        .map_err(|e| format!("Prune failed: {e}"))?;

    Ok(ForgetPruneSummary {
        snapshots_removed: removed,
        snapshots_kept: total - removed,
    })
}
