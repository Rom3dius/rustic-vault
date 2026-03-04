use semver::Version;
use serde::Deserialize;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub owner/repo for release checks.
const GITHUB_REPO: &str = "romedius/rustic-vault";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

/// Result of a version check.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
    pub url: String,
}

/// Check GitHub for the latest release. Returns `Some(UpdateInfo)` if a newer
/// version exists, `None` if already up to date.
pub async fn check_for_update() -> Option<UpdateInfo> {
    let url = format!(
        "https://api.github.com/repos/{GITHUB_REPO}/releases/latest"
    );

    let client = reqwest::Client::builder()
        .user_agent(format!("rustic-vault/{CURRENT_VERSION}"))
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let resp = client.get(&url).send().await.ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let release: GitHubRelease = resp.json().await.ok()?;

    let latest_tag = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);
    let latest = Version::parse(latest_tag).ok()?;
    let current = Version::parse(CURRENT_VERSION).ok()?;

    if latest > current {
        Some(UpdateInfo {
            current: current.to_string(),
            latest: latest.to_string(),
            url: release.html_url,
        })
    } else {
        None
    }
}
