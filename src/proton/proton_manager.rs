// Proton manager
// Handles downloading and managing Proton versions

use anyhow::{Context, Result};
use futures::TryStreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};

use crate::config::paths::Paths;

/// Proton release information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonRelease {
    pub tag_name: String,
    pub name: String,
    pub html_url: String,
    pub assets: Vec<ProtonAsset>,
}

/// Proton asset (downloadable file)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

/// Proton configuration
#[derive(Debug, Clone)]
pub struct ProtonConfig {
    pub label: &'static str,
    pub dir: &'static str,
    pub api: &'static str,
    pub archive_ext: &'static str,
}

/// Available Proton configurations
pub const PROTON_CONFIGS: &[ProtonConfig] = &[
    ProtonConfig {
        label: "GE-Proton",
        dir: "GE-Proton Latest",
        api: "https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest",
        archive_ext: ".tar.gz",
    },
    ProtonConfig {
        label: "Proton-EM",
        dir: "Proton-EM Latest",
        api: "https://api.github.com/repos/Etaash-mathamsetty/Proton/releases/latest",
        archive_ext: ".tar.xz",
    },
];

/// Proton manager
#[derive(Clone)]
pub struct ProtonManager {
    client: Client,
    pub compat_dir: PathBuf,
}

impl ProtonManager {
    /// Create a new Proton manager
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .user_agent("Faugus-Launcher")
                .build()
                .unwrap_or_default(),
            compat_dir: Paths::steam_compat_tools_dir(),
        }
    }

    /// Get latest Proton release information
    pub async fn get_latest_release(&self, config: &ProtonConfig) -> Result<ProtonRelease> {
        info!("Fetching latest {} release", config.label);

        let response = self
            .client
            .get(config.api)
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .context("Failed to fetch release info")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch release: {}", response.status());
        }

        let release: ProtonRelease = response.json().await?;

        info!("Latest {} release: {}", config.label, release.tag_name);
        Ok(release)
    }

    /// Get all available Proton versions
    pub async fn get_available_versions(&self) -> Result<Vec<String>> {
        let mut versions = Vec::new();

        for config in PROTON_CONFIGS {
            if let Ok(release) = self.get_latest_release(config).await {
                versions.push(format!("{} {}", config.label, release.tag_name));
            }
        }

        // Add locally installed versions
        versions.extend(self.get_installed_versions());

        Ok(versions)
    }

    /// Get installed Proton versions
    pub fn get_installed_versions(&self) -> Vec<String> {
        let mut versions = Vec::new();

        if !self.compat_dir.exists() {
            return versions;
        }

        if let Ok(entries) = fs::read_dir(&self.compat_dir) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.starts_with("GE-Proton") || name.starts_with("Proton-") {
                        versions.push(name);
                    }
                }
            }
        }

        versions.sort();
        versions
    }

    /// Download a Proton version
    pub async fn download_proton(
        &self,
        config: &ProtonConfig,
        on_progress: impl Fn(u64, u64),
    ) -> Result<PathBuf> {
        let release = self.get_latest_release(config).await?;

        // Find the appropriate asset
        let asset = release
            .assets
            .iter()
            .find(|a| a.name.ends_with(config.archive_ext))
            .ok_or_else(|| anyhow::anyhow!("No suitable asset found in release"))?;

        info!("Downloading {} ({})", asset.name, format_size(asset.size));

        // Ensure compat directory exists
        fs::create_dir_all(&self.compat_dir).context("Failed to create compat tools directory")?;

        let download_path = self.compat_dir.join(&asset.name);

        // Download the file
        let response = self.client.get(&asset.browser_download_url).send().await?;

        let total_size = response.content_length().unwrap_or(asset.size);
        let mut downloaded = 0u64;

        let mut file = File::create(&download_path).await?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.try_next().await? {
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            on_progress(downloaded, total_size);
        }

        file.flush().await?;

        info!("Download complete: {}", download_path.display());

        // Extract the archive
        self.extract_archive(&download_path, config).await?;

        // Clean up archive
        let _ = fs::remove_file(&download_path);

        Ok(self.compat_dir.join(config.dir))
    }

    /// Extract downloaded Proton archive
    async fn extract_archive(&self, archive_path: &PathBuf, config: &ProtonConfig) -> Result<()> {
        info!("Extracting {}", archive_path.display());

        let compat_dir = &self.compat_dir;

        // Use tar command for extraction
        let status = tokio::process::Command::new("tar")
            .arg("-xf")
            .arg(archive_path)
            .arg("-C")
            .arg(compat_dir)
            .status()
            .await
            .context("Failed to extract archive (tar not found?)")?;

        if !status.success() {
            anyhow::bail!("Failed to extract archive");
        }

        info!("Extraction complete");

        Ok(())
    }

    /// Delete a Proton version
    pub fn delete_proton(&self, name: &str) -> Result<()> {
        let path = self.compat_dir.join(name);

        if !path.exists() {
            return Err(anyhow::anyhow!("Proton version not found: {}", name));
        }

        info!("Deleting Proton version: {}", name);

        fs::remove_dir_all(&path)
            .with_context(|| format!("Failed to delete Proton version: {}", name))?;

        Ok(())
    }

    /// Get default Proton runner
    pub fn get_default_runner() -> String {
        String::from("GE-Proton")
    }

    /// Check if a Proton version is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.compat_dir.join(name).exists()
    }
}

impl Default for ProtonManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_release() {
        let manager = ProtonManager::new();
        let config = &PROTON_CONFIGS[0];

        match manager.get_latest_release(config).await {
            Ok(release) => {
                println!("Latest release: {}", release.tag_name);
                assert!(!release.tag_name.is_empty());
            }
            Err(e) => {
                eprintln!("Error fetching release: {}", e);
            }
        }
    }

    #[test]
    fn test_get_installed_versions() {
        let manager = ProtonManager::new();
        let versions = manager.get_installed_versions();
        println!("Installed versions: {:?}", versions);
    }
}
