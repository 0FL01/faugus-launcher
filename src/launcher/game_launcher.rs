// Game launcher module
// Handles launching games with UMU-Launcher and Proton

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use sysinfo::{Pid, System};
use tokio::process::Command as AsyncCommand;
use tracing::{debug, info, warn};

use crate::config::app_config::AppConfig;
use crate::config::envar;
use crate::config::paths::Paths;
use crate::config::Game;
use crate::proton::runner_resolver;

/// Process information for running games
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcess {
    pub game_title: String,
    pub main_pid: u32,
    pub umu_pid: Option<u32>,
}

/// Game launcher
pub struct GameLauncher;

impl GameLauncher {
    /// Launch a game
    pub async fn launch(game: &Game) -> Result<GameProcess> {
        info!("Launching game: {}", game.title);

        // Ensure required directories exist
        Self::ensure_directories(game)?;

        // Build the command
        let umu_run = Self::get_umu_run()?;

        // Check if GameMode is enabled - wrap with gamemoderun prefix if so
        let mut cmd = if game.gamemode {
            if let Some(gamemoderun) = Paths::gamemoderun() {
                info!("Enabling GameMode as command prefix");
                let mut cmd = AsyncCommand::new(&gamemoderun);
                cmd.arg(&umu_run);
                cmd
            } else {
                // GameMode requested but not installed - proceed without it
                info!("GameMode requested but not found, proceeding without it");
                AsyncCommand::new(&umu_run)
            }
        } else {
            AsyncCommand::new(&umu_run)
        };

        // Set environment variables
        Self::setup_environment(&mut cmd, game)?;

        // Set up arguments
        let args = Self::build_arguments(game);
        cmd.args(&args);

        // Spawn the process
        let child = cmd
            .spawn()
            .with_context(|| format!("Failed to launch game: {}", game.title))?;

        let pid = child.id().unwrap_or(0);
        info!("Game {} launched with PID: {}", game.title, pid);

        Ok(GameProcess {
            game_title: game.title.clone(),
            main_pid: pid,
            umu_pid: None,
        })
    }

    /// Ensure required directories exist
    fn ensure_directories(game: &Game) -> Result<()> {
        // Create prefix if it doesn't exist
        if !game.prefix.exists() {
            std::fs::create_dir_all(&game.prefix)
                .with_context(|| format!("Failed to create prefix: {:?}", game.prefix))?;
        }

        // Create logs directory
        let logs_dir = Paths::logs_dir();
        if !logs_dir.exists() {
            std::fs::create_dir_all(&logs_dir)
                .with_context(|| format!("Failed to create logs directory: {:?}", logs_dir))?;
        }

        Ok(())
    }

    /// Get umu-run binary
    fn get_umu_run() -> Result<PathBuf> {
        let umu_run = Paths::umu_run();

        if !umu_run.exists() {
            // Try to find in PATH
            if let Some(path) = Paths::find_binary("umu-run") {
                return Ok(path);
            }

            return Err(anyhow::anyhow!(
                "umu-run not found. Please install UMU-Launcher."
            ));
        }

        Ok(umu_run)
    }

    /// Set up environment variables for the game
    fn setup_environment(cmd: &mut tokio::process::Command, game: &Game) -> Result<()> {
        // Load envar.txt first (global env vars, overridden by game-specific later)
        let global_env_vars = envar::load_envar_txt();
        if !global_env_vars.is_empty() {
            debug!(
                "Applying {} environment variables from envar.txt",
                global_env_vars.len()
            );
            for (key, value) in &global_env_vars {
                cmd.env(key, value);
            }
        }

        // Wine prefix (game-specific, overrides envar.txt if set there)
        cmd.env("WINEPREFIX", &game.prefix);

        // Proton runner
        runner_resolver::validate_runner(&game.runner)?;
        let runner = runner_resolver::resolve_runner(&game.runner)?;

        if !runner.is_empty() {
            cmd.env("PROTONPATH", &runner);
        }

        // Game ID for UMU
        cmd.env("GAMEID", &game.gameid);

        // MangoHud
        if game.mangohud {
            if let Some(_mangohud) = Paths::mangohud() {
                info!("Enabling MangoHud");
                cmd.env("MANGOHUD", "1");
            }
        }

        // Disable hidraw
        if game.disable_hidraw {
            cmd.env("WINE_DISABLE_HIDRAW", "1");
        }

        // Load config.ini using structured AppConfig
        let app_config = match AppConfig::load() {
            Ok(config) => config,
            Err(e) => {
                warn!("Failed to load config.ini, using defaults: {}", e);
                AppConfig::default()
            }
        };

        // Wayland driver
        if app_config.wayland_driver {
            cmd.env("PROTON_ENABLE_WAYLAND", "1");
        }

        // HDR
        if app_config.enable_hdr {
            cmd.env("ENABLE_HDR", "1");
        }

        // WOW64
        if app_config.enable_wow64 {
            cmd.env("PROTON_USE_WOW64", "1");
        }

        // Lossless scaling - uses LSFG_* environment variables (Linux native)
        // See: https://github.com/Fir3element/Lossless-Scaling
        if game.lossless_enabled {
            info!("Enabling Lossless Scaling via LSFG environment variables");
            // Enable legacy mode for Wine/Proton compatibility
            cmd.env("LSFG_LEGACY", "1");

            // Map multiplier if > 0
            if game.lossless_multiplier > 0 {
                cmd.env("LSFG_MULTIPLIER", game.lossless_multiplier.to_string());
            }

            // Map performance mode (1 for enabled, 0 for disabled)
            cmd.env(
                "LSFG_PERFORMANCE_MODE",
                if game.lossless_performance { "1" } else { "0" },
            );

            // Map HDR mode (1 for enabled, 0 for disabled)
            cmd.env("LSFG_HDR_MODE", if game.lossless_hdr { "1" } else { "0" });

            // Conservative mapping for flow scale:
            // Only set LSFG_FLOW_SCALE to 1.0 when explicitly enabled, otherwise omit.
            // This follows the principle of minimal environment variable pollution.
            if game.lossless_flow {
                cmd.env("LSFG_FLOW_SCALE", "1.0");
            }
            // Note: Not overriding WINEDLLOVERRIDES to preserve user configurations
        }

        // Discrete GPU
        if app_config.discrete_gpu {
            cmd.env("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
        }

        // Proton fixes
        if !game.protonfix.is_empty() {
            cmd.env("PROTON_NO_FSYNC", "1");
            cmd.env("PROTON_NO_ESYNC", "1");
        }

        // Logging
        if app_config.enable_logging {
            let _log_file = Paths::logs_dir().join(format!("{}.log", game.gameid));
            cmd.env("WINEDEBUG", "+all");
            cmd.env("WINE_MONO_TRACE", "E:System.Windows.Forms");
        }

        Ok(())
    }

    /// Build command arguments
    fn build_arguments(game: &Game) -> Vec<String> {
        let mut args = Vec::new();

        // Game executable path
        args.push(game.path.to_string_lossy().to_string());

        // Launch arguments (e.g., -no-dwrite)
        if !game.launch_arguments.is_empty() {
            for arg in game.launch_arguments.split_whitespace() {
                args.push(arg.to_string());
            }
        }

        // Game-specific arguments
        if !game.game_arguments.is_empty() {
            for arg in game.game_arguments.split_whitespace() {
                args.push(arg.to_string());
            }
        }

        args
    }

    /// Check if a process is running
    pub fn is_process_running(pid: u32) -> bool {
        let mut sys = System::new();
        sys.refresh_processes();

        let pid = Pid::from_u32(pid);
        sys.processes().contains_key(&pid)
    }

    /// Terminate a game process
    pub fn terminate(pid: u32) -> Result<()> {
        info!("Terminating process tree: {}", pid);
        Self::terminate_tree(pid);
        Ok(())
    }

    /// Terminate a process tree recursively
    pub fn terminate_tree(pid: u32) {
        let mut sys = System::new();
        sys.refresh_processes();
        Self::terminate_tree_recursive(&mut sys, pid);
    }

    fn terminate_tree_recursive(sys: &mut System, pid: u32) {
        let target_pid = Pid::from_u32(pid);

        // Find children
        let mut children = Vec::new();
        for (p, proc) in sys.processes() {
            if let Some(parent) = proc.parent() {
                if parent == target_pid {
                    children.push(p.as_u32());
                }
            }
        }

        // Kill children first
        for child_pid in children {
            Self::terminate_tree_recursive(sys, child_pid);
        }

        // Kill the parent
        info!("Killing process: {}", pid);
        #[cfg(unix)]
        {
            use nix::sys::signal::{self, Signal};
            use nix::unistd::Pid as NixPid;
            let _ = signal::kill(NixPid::from_raw(pid as i32), Signal::SIGKILL);
        }
        #[cfg(windows)]
        {
            let _ = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }
    }

    /// Kill all common Wine processes
    pub fn kill_all_wine_processes() {
        info!("Killing all common Wine processes");
        let wine_binaries = [
            "wineserver",
            "wine64-preloader",
            "winedevice.exe",
            "plugplay.exe",
            "services.exe",
            "explorer.exe",
        ];

        for bin in wine_binaries {
            let _ = Command::new("pkill").args(["-9", bin]).output();
        }
    }

    /// Get game process by title
    /// TODO: Use for game process monitoring, status display
    #[allow(dead_code)]
    pub fn get_game_process(title: &str) -> Option<GameProcess> {
        let running_games = Paths::running_games_json();

        if !running_games.exists() {
            return None;
        }

        let content = std::fs::read_to_string(running_games).ok()?;
        let processes: Vec<GameProcess> = serde_json::from_str(&content).ok()?;

        processes.into_iter().find(|p| p.game_title == title)
    }

    /// Save running game process
    pub fn save_process(process: &GameProcess) -> Result<()> {
        let running_games = Paths::running_games_json();

        let mut processes: Vec<GameProcess> = if running_games.exists() {
            let content = std::fs::read_to_string(&running_games)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        processes.push(process.clone());

        let content = serde_json::to_string_pretty(&processes)?;
        std::fs::write(&running_games, content)?;

        Ok(())
    }

    /// Remove game process from running games
    pub fn remove_process(title: &str) -> Result<()> {
        let running_games = Paths::running_games_json();

        if !running_games.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&running_games)?;
        let mut processes: Vec<GameProcess> = serde_json::from_str(&content)?;

        processes.retain(|p| p.game_title != title);

        let content = serde_json::to_string_pretty(&processes)?;
        std::fs::write(&running_games, content)?;

        Ok(())
    }

    /// Update latest games file
    pub fn update_latest_games(title: &str) -> Result<()> {
        let latest_games = Paths::latest_games_txt();

        let content = if latest_games.exists() {
            std::fs::read_to_string(&latest_games).unwrap_or_default()
        } else {
            String::new()
        };

        let mut games: Vec<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        // Remove if already exists
        games.retain(|g| g != title);

        // Add to front
        games.insert(0, title.to_string());

        // Keep only last 10
        games.truncate(10);

        std::fs::write(&latest_games, games.join("\n") + "\n")?;

        Ok(())
    }
}
