// Tray Icon Handler
// Manages the system tray icon

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

use crate::config::paths::Paths;

/// Tray icon manager
pub struct TrayIcon {
    icon_path: PathBuf,
}

impl TrayIcon {
    /// Create a new tray icon manager
    pub fn new(custom_path: Option<PathBuf>) -> Result<Self> {
        let icon_path = if let Some(path) = custom_path {
            if path.exists() {
                path
            } else {
                warn!("Custom tray icon not found: {:?}, using default", path);
                Self::find_default_icon()?
            }
        } else {
            Self::find_default_icon()?
        };

        info!("Using tray icon: {:?}", icon_path);

        Ok(Self { icon_path })
    }

    /// Get the icon path
    pub fn path(&self) -> &PathBuf {
        &self.icon_path
    }

    /// Find the default Faugus Launcher icon
    fn find_default_icon() -> Result<PathBuf> {
        // Try system icons first
        let icon = Paths::get_icon("faugus-launcher.png");
        if icon.exists() {
            return Ok(icon);
        }

        // Try common installation paths
        let possible_paths = vec![
            PathBuf::from("/usr/share/icons/hicolor/256x256/apps/faugus-launcher.png"),
            PathBuf::from("/usr/share/pixmaps/faugus-launcher.png"),
            PathBuf::from("/usr/local/share/icons/hicolor/256x256/apps/faugus-launcher.png"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        // Fallback to config icons directory
        let config_icon = Paths::icons_dir().join("faugus-launcher.png");
        if config_icon.exists() {
            return Ok(config_icon);
        }

        // Last resort: use assets directory (for development)
        let asset_paths = vec![
            PathBuf::from("assets/faugus-launcher.png"),
            PathBuf::from("../assets/faugus-launcher.png"),
        ];

        for path in asset_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(anyhow::anyhow!("No suitable tray icon found"))
    }

    /// Load icon as tray_icon::Icon
    pub fn load_icon(&self) -> Result<tray_icon::Icon> {
        let img = image::open(&self.icon_path).context("Failed to open tray icon image")?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let rgba_raw = rgba.into_raw();

        tray_icon::Icon::from_rgba(rgba_raw, width, height)
            .context("Failed to create tray icon from RGBA")
    }
}
