/// Centralized help text constants for the UI.
/// All user-facing explanations live here, keeping UI code clean.

pub const SNAPSHOT: &str = "A snapshot is a point-in-time copy of your files. Each backup creates \
a new snapshot. Files are deduplicated — only changes since the last snapshot are stored, so \
snapshots are space-efficient.";

pub const RETENTION_POLICY: &str = "Retention controls how many old snapshots to keep. For example, \
'Keep daily: 7' keeps one snapshot per day for the last 7 days, automatically removing older ones. \
This prevents the repository from growing indefinitely.";

pub const KEEP_LAST: &str = "Keep the N most recent snapshots regardless of age.";

pub const KEEP_DAILY: &str =
    "Keep one snapshot per day for the last N days. Older duplicates within each day are removed.";

pub const KEEP_WEEKLY: &str =
    "Keep one snapshot per week for the last N weeks. Older duplicates within each week are removed.";

pub const KEEP_MONTHLY: &str = "Keep one snapshot per month for the last N months. Older duplicates \
within each month are removed.";

pub const REPO_PASSWORD: &str = "This password encrypts your backup data inside the repository. \
The repository has its own encryption layer — your backups stay protected even if the repository \
folder is copied or moved elsewhere.";

pub const SAVE_PASSWORD: &str = "The password file is stored unencrypted on disk — only enable \
this if the storage location is already protected (e.g., encrypted volume, private machine).";

pub const PROFILE: &str = "A profile defines what to back up: which folders, what to exclude, and \
how long to keep old snapshots. Create one profile per device or per use case.";

pub const TAGS: &str = "Tags help you organize and filter snapshots. For example, use your device \
name or 'documents' to easily find snapshots later.";

pub const EXCLUDE_PATTERNS: &str = "File or folder patterns to skip during backup. Use '*' as \
wildcard. Common excludes: node_modules, .cache, *.tmp, __pycache__, .git";

pub const PRUNE: &str = "After removing old snapshots with retention rules, pruning reclaims the \
disk space by cleaning up unused data chunks.";

pub const RESTORE: &str = "Copy all files from a snapshot back to a folder you choose. This does \
not modify the repository — you can restore the same snapshot multiple times.";

pub const WELCOME: &str = "Welcome to Rustic Vault! This tool helps you create encrypted, \
deduplicated backups of your files. Your backups are stored in a restic-compatible repository \
at a location you choose.";

pub const RUN_BACKUP_TOOLTIP: &str = "Create a new snapshot from this profile's folders";

pub const VIEW_SNAPSHOTS_TOOLTIP: &str = "Browse all saved snapshots";

pub const NEW_PROFILE_TOOLTIP: &str =
    "Create a new backup profile for a device or folder set";

pub const DELETE_SNAPSHOT_TOOLTIP: &str =
    "Permanently remove this snapshot. Pruning afterward will reclaim disk space.";

pub const RETENTION_PRUNE_TOOLTIP: &str =
    "Remove old snapshots according to retention rules and reclaim disk space";

pub const CHANGE_PASSWORD_HELP: &str =
    "Changes the encryption key for the repository. All existing snapshots remain accessible.";
