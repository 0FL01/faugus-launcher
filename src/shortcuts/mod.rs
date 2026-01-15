// Desktop shortcuts module
// Handles creation and management of desktop application shortcuts

mod desktop_entry;

pub use desktop_entry::DesktopShortcutManager;

/// Shortcut location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutLocation {
    Desktop,
    Applications,
    Both,
}
