// Game configuration structure
// Represents a single game entry in the launcher

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::config::paths::Paths;

/// Represents a single game in the launcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for the game
    pub gameid: String,

    /// Display title
    pub title: String,

    /// Path to the game executable
    pub path: PathBuf,

    /// Wine prefix path
    pub prefix: PathBuf,

    /// Additional launch arguments
    pub launch_arguments: String,

    /// Game-specific arguments
    pub game_arguments: String,

    /// Enable MangoHud
    pub mangohud: bool,

    /// Enable GameMode
    pub gamemode: bool,

    /// Disable hidraw support
    pub disable_hidraw: bool,

    /// Proton fix configuration
    pub protonfix: String,

    /// Proton runner to use
    pub runner: String,

    /// AddApp checkbox state
    pub addapp_checkbox: bool,

    /// AddApp configuration
    pub addapp: String,

    /// AddApp batch file
    pub addapp_bat: String,

    /// Path to banner image
    #[serde(default)]
    pub banner: Option<PathBuf>,

    /// Lossless scaling enabled
    pub lossless_enabled: bool,

    /// Lossless scaling multiplier
    pub lossless_multiplier: u32,

    /// Lossless scaling flow
    pub lossless_flow: bool,

    /// Lossless scaling performance mode
    pub lossless_performance: bool,

    /// Lossless scaling HDR
    pub lossless_hdr: bool,

    /// Total playtime in seconds
    pub playtime: u64,

    /// Hidden from library
    pub hidden: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            gameid: uuid::Uuid::new_v4().to_string(),
            title: String::new(),
            path: PathBuf::new(),
            prefix: Paths::default_prefix(),
            launch_arguments: String::new(),
            game_arguments: String::new(),
            mangohud: false,
            gamemode: false,
            disable_hidraw: false,
            protonfix: String::new(),
            runner: String::from("GE-Proton"),
            addapp_checkbox: false,
            addapp: String::new(),
            addapp_bat: String::new(),
            banner: None,
            lossless_enabled: false,
            lossless_multiplier: 2,
            lossless_flow: false,
            lossless_performance: false,
            lossless_hdr: false,
            playtime: 0,
            hidden: false,
        }
    }
}

impl Game {
    /// Load all games from the games.json file
    pub fn load_all() -> Result<Vec<Self>> {
        let games_path = Paths::games_json();

        if !games_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&games_path)
            .with_context(|| format!("Failed to read games file: {:?}", games_path))?;

        let games: Vec<Self> =
            serde_json::from_str(&content).with_context(|| "Failed to parse games JSON")?;

        Ok(games)
    }

    /// Save all games to the games.json file
    pub fn save_all(games: &[Self]) -> Result<()> {
        let games_path = Paths::games_json();

        // Ensure parent directory exists
        if let Some(parent) = games_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create games directory: {:?}", parent))?;
        }

        let content =
            serde_json::to_string_pretty(games).with_context(|| "Failed to serialize games")?;

        fs::write(&games_path, content)
            .with_context(|| format!("Failed to write games file: {:?}", games_path))?;

        Ok(())
    }

    /// Save this game to the games list
    pub fn save(&self) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        // Find and update existing game or add new one
        if let Some(existing) = games.iter().position(|g| g.gameid == self.gameid) {
            games[existing] = self.clone();
        } else {
            games.push(self.clone());
        }

        Self::save_all(&games)
    }

    /// Delete this game from the games list
    pub fn delete(&self) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        games.retain(|g| g.gameid != self.gameid);

        Self::save_all(&games)
    }

    /// Format playtime as human-readable string
    pub fn format_playtime(&self) -> String {
        let hours = self.playtime / 3600;
        let minutes = (self.playtime % 3600) / 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    /// Increment playtime by given seconds
    pub fn add_playtime(&mut self, seconds: u64) {
        self.playtime += seconds;
    }

    /// Update the hidden state of this game
    pub fn update_hidden(&self, hidden: bool) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        // Find and update existing game
        if let Some(existing) = games.iter().position(|g| g.gameid == self.gameid) {
            games[existing].hidden = hidden;
            Self::save_all(&games)?;
        }

        Ok(())
    }

    /// Duplicate this game with a new ID and (Copy) suffix
    pub fn duplicate(&self) -> Self {
        Self {
            gameid: uuid::Uuid::new_v4().to_string(),
            title: format!("{} (Copy)", self.title),
            path: self.path.clone(),
            prefix: self.prefix.clone(),
            launch_arguments: self.launch_arguments.clone(),
            game_arguments: self.game_arguments.clone(),
            mangohud: self.mangohud,
            gamemode: self.gamemode,
            disable_hidraw: self.disable_hidraw,
            protonfix: self.protonfix.clone(),
            runner: self.runner.clone(),
            addapp_checkbox: self.addapp_checkbox,
            addapp: self.addapp.clone(),
            addapp_bat: self.addapp_bat.clone(),
            banner: self.banner.clone(),
            lossless_enabled: self.lossless_enabled,
            lossless_multiplier: self.lossless_multiplier,
            lossless_flow: self.lossless_flow,
            lossless_performance: self.lossless_performance,
            lossless_hdr: self.lossless_hdr,
            playtime: 0,
            hidden: false,
        }
    }
}

/// Game configuration for creation/editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub gameid: Option<String>,
    pub title: String,
    pub path: PathBuf,
    pub prefix: PathBuf,
    pub launch_arguments: String,
    pub game_arguments: String,
    pub mangohud: bool,
    pub gamemode: bool,
    pub disable_hidraw: bool,
    pub protonfix: String,
    pub runner: String,
    pub addapp_checkbox: bool,
    pub addapp: String,
    pub addapp_bat: String,
    #[serde(default)]
    pub banner: Option<PathBuf>,
    pub lossless_enabled: bool,
    pub lossless_multiplier: u32,
    pub lossless_flow: bool,
    pub lossless_performance: bool,
    pub lossless_hdr: bool,
}

impl From<GameConfig> for Game {
    fn from(config: GameConfig) -> Self {
        Self {
            gameid: config
                .gameid
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            title: config.title,
            path: config.path,
            prefix: config.prefix,
            launch_arguments: config.launch_arguments,
            game_arguments: config.game_arguments,
            mangohud: config.mangohud,
            gamemode: config.gamemode,
            disable_hidraw: config.disable_hidraw,
            protonfix: config.protonfix,
            runner: config.runner,
            addapp_checkbox: config.addapp_checkbox,
            addapp: config.addapp,
            addapp_bat: config.addapp_bat,
            banner: config.banner,
            lossless_enabled: config.lossless_enabled,
            lossless_multiplier: config.lossless_multiplier,
            lossless_flow: config.lossless_flow,
            lossless_performance: config.lossless_performance,
            lossless_hdr: config.lossless_hdr,
            playtime: 0,
            hidden: false,
        }
    }
}

impl From<Game> for GameConfig {
    fn from(game: Game) -> Self {
        Self {
            gameid: Some(game.gameid),
            title: game.title,
            path: game.path,
            prefix: game.prefix,
            launch_arguments: game.launch_arguments,
            game_arguments: game.game_arguments,
            mangohud: game.mangohud,
            gamemode: game.gamemode,
            disable_hidraw: game.disable_hidraw,
            protonfix: game.protonfix,
            runner: game.runner,
            addapp_checkbox: game.addapp_checkbox,
            addapp: game.addapp,
            addapp_bat: game.addapp_bat,
            banner: game.banner,
            lossless_enabled: game.lossless_enabled,
            lossless_multiplier: game.lossless_multiplier,
            lossless_flow: game.lossless_flow,
            lossless_performance: game.lossless_performance,
            lossless_hdr: game.lossless_hdr,
        }
    }
}
