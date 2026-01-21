// Environment variable parsing from envar.txt
// Legacy compatibility with Python-era envar.txt format

use std::collections::HashMap;
use tracing::warn;

use crate::config::paths::Paths;

/// Parse environment variable content from a string
/// Format: KEY=value
/// - Empty lines and comments (# or ; at start) are ignored
/// - Whitespace around key and value is trimmed
/// - Keys must be valid env var identifiers: [A-Za-z_][A-Za-z0-9_]*
///
/// Returns a tuple of (valid_vars, warnings) - valid vars are always collected,
/// even if some lines have parsing errors.
pub fn parse_envar_content(content: &str) -> (HashMap<String, String>, Vec<String>) {
    let mut env_vars = HashMap::new();
    let mut warnings = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Skip comments
        if trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        // Parse KEY=value
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            let value = trimmed[eq_pos + 1..].trim();

            // Validate key format
            if is_valid_env_var_key(key) {
                env_vars.insert(key.to_string(), value.to_string());
            } else {
                warnings.push(format!(
                    "Line {}: Invalid env var key '{}': must match [A-Za-z_][A-Za-z0-9_]*",
                    line_num + 1,
                    key
                ));
            }
        } else {
            warnings.push(format!(
                "Line {}: No '=' separator found in '{}'",
                line_num + 1,
                trimmed
            ));
        }
    }

    (env_vars, warnings)
}

/// Check if a string is a valid environment variable key
/// Valid format: [A-Za-z_][A-Za-z0-9_]*
fn is_valid_env_var_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    let mut chars = key.chars();

    // First character must be letter or underscore
    if let Some(first) = chars.next() {
        if !first.is_ascii_alphabetic() && first != '_' {
            return false;
        }
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Load environment variables from envar.txt file
/// Returns an empty map if file doesn't exist or can't be read
pub fn load_envar_txt() -> HashMap<String, String> {
    let envar_path = Paths::envar_txt();

    if !envar_path.exists() {
        return HashMap::new();
    }

    match std::fs::read_to_string(&envar_path) {
        Ok(content) => {
            let (env_vars, warnings) = parse_envar_content(&content);

            // Log warnings but still return valid vars
            for warning in &warnings {
                warn!("{}: {}", envar_path.display(), warning);
            }

            if !env_vars.is_empty() {
                tracing::debug!(
                    "Loaded {} environment variables from {}",
                    env_vars.len(),
                    envar_path.display()
                );
            }

            env_vars
        }
        Err(e) => {
            warn!("Failed to read {}: {}", envar_path.display(), e);
            HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_content() {
        let (vars, warnings) = parse_envar_content("");
        assert!(vars.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_simple_key_value() {
        let (vars, warnings) = parse_envar_content("KEY=value");
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_parse_multiple_vars() {
        let content = r#"
PATH=/usr/bin:/usr/local/bin
HOME=/home/user
EDITOR=vim
"#;
        let (vars, warnings) = parse_envar_content(content);
        assert_eq!(vars.len(), 3);
        assert_eq!(
            vars.get("PATH"),
            Some(&"/usr/bin:/usr/local/bin".to_string())
        );
        assert_eq!(vars.get("HOME"), Some(&"/home/user".to_string()));
        assert_eq!(vars.get("EDITOR"), Some(&"vim".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_trim_whitespace() {
        let (vars, warnings) = parse_envar_content("  KEY  =  value  ");
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_skip_comments_hash() {
        let (vars, warnings) = parse_envar_content("# This is a comment\nKEY=value");
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_skip_comments_semicolon() {
        let (vars, warnings) = parse_envar_content("; This is a comment\nKEY=value");
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_skip_empty_lines() {
        let (vars, warnings) = parse_envar_content("\n\nKEY=value\n\n");
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_invalid_key_starts_with_number() {
        let (vars, warnings) = parse_envar_content("1INVALID=value");
        assert!(vars.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("Invalid env var key"));
        assert!(warnings[0].contains("1INVALID"));
    }

    #[test]
    fn test_invalid_key_with_dash() {
        let (vars, warnings) = parse_envar_content("INVALID-KEY=value");
        assert!(vars.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("INVALID-KEY"));
    }

    #[test]
    fn test_valid_key_with_numbers() {
        let (vars, warnings) = parse_envar_content("KEY123=value");
        assert_eq!(vars.get("KEY123"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_valid_key_with_underscore() {
        let (vars, warnings) = parse_envar_content("KEY_WITH_UNDERSCORE=value");
        assert_eq!(vars.get("KEY_WITH_UNDERSCORE"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_valid_key_all_caps() {
        let (vars, warnings) = parse_envar_content("ALLCAPSKEY=value");
        assert_eq!(vars.get("ALLCAPSKEY"), Some(&"value".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_value_with_equals() {
        let (vars, warnings) = parse_envar_content("KEY=value=with=equals");
        assert_eq!(vars.get("KEY"), Some(&"value=with=equals".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_empty_value() {
        let (vars, warnings) = parse_envar_content("KEY=");
        assert_eq!(vars.get("KEY"), Some(&"".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_no_separator() {
        let (vars, warnings) = parse_envar_content("INVALID_LINE");
        assert!(vars.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("No '=' separator found"));
    }

    #[test]
    fn test_partial_parse_with_errors() {
        let content = r#"
VALID_KEY=value
1INVALID=value
ANOTHER_VALID=value
"#;
        let (vars, warnings) = parse_envar_content(content);
        assert_eq!(vars.len(), 2); // Two valid entries
        assert_eq!(warnings.len(), 1); // One invalid entry
        assert!(warnings[0].contains("1INVALID"));
        assert!(vars.contains_key("VALID_KEY"));
        assert!(vars.contains_key("ANOTHER_VALID"));
    }

    #[test]
    fn test_is_valid_env_var_key() {
        assert!(is_valid_env_var_key("KEY"));
        assert!(is_valid_env_var_key("key"));
        assert!(is_valid_env_var_key("_KEY"));
        assert!(is_valid_env_var_key("KEY123"));
        assert!(is_valid_env_var_key("KEY_WITH_UNDERSCORE"));
        assert!(!is_valid_env_var_key(""));
        assert!(!is_valid_env_var_key("1KEY"));
        assert!(!is_valid_env_var_key("KEY-WITH-DASH"));
        assert!(!is_valid_env_var_key("KEY.WITH.DOT"));
        assert!(!is_valid_env_var_key("KEY WITH SPACE"));
    }

    #[test]
    fn test_wine_prefix_env_var() {
        let content = r#"
# Wine prefix setup
WINEPREFIX=/home/user/.wine
WINEDLLOVERRIDES=n,b;v
"#;
        let (vars, warnings) = parse_envar_content(content);
        assert_eq!(
            vars.get("WINEPREFIX"),
            Some(&"/home/user/.wine".to_string())
        );
        assert_eq!(vars.get("WINEDLLOVERRIDES"), Some(&"n,b;v".to_string()));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_realistic_envar_content() {
        let content = r#"
# Faugus Launcher Environment Variables

# Wine configuration
WINEPREFIX=/home/user/.wine
WINEDLLOVERRIDES=n,b;v

# Proton settings
PROTON_NO_ESYNC=1
PROTON_NO_FSYNC=0

# Graphics
MANGOHUD_CONFIG=cpu_stats,gpu_stats

# Game-specific
GAME_LAUNCH_OPTIONS=-windowed -dx11
"#;
        let (vars, warnings) = parse_envar_content(content);
        assert_eq!(vars.len(), 6);
        assert!(vars.contains_key("WINEPREFIX"));
        assert!(vars.contains_key("PROTON_NO_ESYNC"));
        assert!(vars.contains_key("MANGOHUD_CONFIG"));
        assert!(warnings.is_empty());
    }
}
