// Faugus Launcher - Library exports for external binaries
// This library exposes core functionality needed by faugus-run and other tools

// Re-export core modules
pub mod config;
pub mod launcher;
pub mod proton;

// Re-export commonly used types
pub use config::game_config::Game;
pub use launcher::game_launcher::GameLauncher;
