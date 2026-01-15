// GUI module
// Main graphical user interface using Iced

pub mod add_game_dialog;
pub mod confirmation_dialog;
pub mod log_viewer_dialog;
pub mod main_window;
pub mod proton_manager_dialog;
pub mod settings_dialog;

pub use add_game_dialog::{AddGameDialog, AddGameMessage};
pub use confirmation_dialog::ConfirmationDialog;
pub use log_viewer_dialog::{LogViewerDialog, LogViewerMessage};
pub use main_window::MainWindow;
pub use proton_manager_dialog::{ProtonManagerDialog, ProtonManagerMessage};
pub use settings_dialog::{SettingsDialog, SettingsMessage};
