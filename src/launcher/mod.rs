// Launcher module
// Handles game launching and process management

mod game_launcher;
mod launch_controller;

pub use launch_controller::{GameLaunchController, LaunchMessage, LaunchStatus};
