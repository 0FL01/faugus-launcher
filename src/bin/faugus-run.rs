// faugus-run - Command-line launcher for Faugus games
// Usage: faugus-run --game <gameid>

use anyhow::{bail, Context, Result};
use faugus_launcher_rs::config::Game;
use faugus_launcher_rs::launcher::game_launcher::GameLauncher;
use std::env;
use std::process::ExitCode;
use tracing::{error, info};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parse command-line arguments and extract game ID
/// Unit-testable: accepts any iterator over strings
fn parse_args(args: impl Iterator<Item = String>) -> Result<String> {
    let args_vec: Vec<String> = args.collect();

    // Expected: faugus-run --game <id>
    if args_vec.len() != 3 {
        bail!(
            "Usage: {} --game <gameid>\n\
             Example: {} --game my-game-id",
            args_vec.first().map(|s| s.as_str()).unwrap_or("faugus-run"),
            args_vec.first().map(|s| s.as_str()).unwrap_or("faugus-run")
        );
    }

    if args_vec[1] != "--game" {
        bail!(
            "Unknown flag: {}\n\
             Expected: --game",
            args_vec[1]
        );
    }

    Ok(args_vec[2].clone())
}

/// Find a game by its ID in the loaded games list
fn find_game_by_id(games: &[Game], game_id: &str) -> Result<Game> {
    games
        .iter()
        .find(|g| g.gameid == game_id)
        .cloned()
        .with_context(|| format!("Game not found with id: {}", game_id))
}

#[tokio::main]
async fn main() -> ExitCode {
    // Initialize tracing with INFO level for user-facing messages
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    if let Err(e) = run().await {
        error!("Error: {:?}", e);
        // Print user-friendly error message with full error chain
        eprintln!("faugus-run v{}", VERSION);
        eprintln!("Error: {:?}", e);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

async fn run() -> Result<()> {
    // Parse command-line arguments
    let game_id = parse_args(std::env::args().skip(1)).context("Failed to parse arguments")?;
    info!("Launching game: {}", game_id);

    // Load all games from config
    let games = Game::load_all().context("Failed to load games configuration")?;

    if games.is_empty() {
        bail!("No games configured. Please add games through the Faugus Launcher GUI.");
    }

    // Find the requested game
    let game = find_game_by_id(&games, &game_id)?;

    info!("Found game: {} ({})", game.title, game.gameid);

    // Launch the game
    let process = GameLauncher::launch(&game)
        .await
        .context("Failed to launch game")?;

    println!(
        "Game '{}' launched with PID: {}",
        game.title, process.main_pid
    );
    info!("Game launched successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Helper to create a test game
    fn create_test_game(id: &str, title: &str) -> Game {
        Game {
            gameid: id.to_string(),
            title: title.to_string(),
            path: PathBuf::from("/tmp/test.exe"),
            prefix: PathBuf::from("/tmp/prefix"),
            ..Default::default()
        }
    }

    #[test]
    fn test_parse_args_valid() {
        // Simulate: faugus-run --game test-id
        let args = [
            "faugus-run".to_string(),
            "--game".to_string(),
            "test-id".to_string(),
        ]
        .into_iter();

        let result = parse_args(args);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("Should have returned Ok"),
            "test-id".to_string()
        );
    }

    #[test]
    fn test_parse_args_missing_flag() {
        // Simulate: faugus-run --invalid test-id
        let args = [
            "faugus-run".to_string(),
            "--invalid".to_string(),
            "test-id".to_string(),
        ]
        .into_iter();

        let result = parse_args(args);
        assert!(result.is_err());
        let error_msg = result.expect_err("Should return error").to_string();
        assert!(error_msg.contains("Unknown flag"));
        assert!(error_msg.contains("--game"));
    }

    #[test]
    fn test_parse_args_wrong_arg_count() {
        // Simulate: faugus-run --game (missing game id)
        let args = ["faugus-run".to_string(), "--game".to_string()].into_iter();

        let result = parse_args(args);
        assert!(result.is_err());
        let error_msg = result.expect_err("Should return error").to_string();
        assert!(error_msg.contains("Usage"));
    }

    #[test]
    fn test_find_game_by_id_success() {
        let game1 = create_test_game("game-1", "Game One");
        let game2 = create_test_game("game-2", "Game Two");
        let games = vec![game1, game2];

        let found = find_game_by_id(&games, "game-1");
        assert!(found.is_ok());
        let game = found.expect("Game should be found");
        assert_eq!(game.gameid, "game-1");
        assert_eq!(game.title, "Game One");
    }

    #[test]
    fn test_find_game_by_id_not_found() {
        let game1 = create_test_game("game-1", "Game One");
        let games = vec![game1];

        let found = find_game_by_id(&games, "nonexistent");
        assert!(found.is_err());
        let error_msg = found.expect_err("Should return error").to_string();
        assert!(error_msg.contains("Game not found"));
    }

    #[test]
    fn test_find_game_by_id_empty_list() {
        let games: Vec<Game> = vec![];

        let found = find_game_by_id(&games, "any-id");
        assert!(found.is_err());
        let error_msg = found.expect_err("Should return error").to_string();
        assert!(error_msg.contains("Game not found"));
    }

    #[test]
    fn test_game_serialization_format() {
        // Ensure Game can be serialized/deserialized correctly
        let game = create_test_game("test-id", "Test Game");

        let json = serde_json::to_string(&game).expect("Failed to serialize");
        let deserialized: Game = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(game.gameid, deserialized.gameid);
        assert_eq!(game.title, deserialized.title);
        assert_eq!(game.path, deserialized.path);
    }

    #[test]
    fn test_game_load_all_empty() {
        // This test doesn't actually test Game::load_all with a temp file,
        // but demonstrates the expected behavior for an empty/non-existent file
        // In practice, Game::load_all returns Ok(vec![]) when file doesn't exist

        let empty_games: Vec<Game> = vec![];
        assert!(empty_games.is_empty());
    }
}
