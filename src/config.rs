use crate::error::{CliError, Result};
use crate::cli::Cli;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(default)]
    pub connections: HashMap<String, Connection>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            default: None,
            connections: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedConnection {
    pub url: String,
    pub secret_key: Option<String>,
}

/// Find config file in priority order
pub fn find_config_file(specified: Option<&PathBuf>) -> Option<PathBuf> {
    // 1. Explicitly specified
    if let Some(path) = specified {
        if path.exists() {
            return Some(path.clone());
        }
    }

    // 2. Current directory: ./craft-cli.toml
    let current_dir = env::current_dir().ok()?;
    let local_config = current_dir.join("craft-cli.toml");
    if local_config.exists() {
        return Some(local_config);
    }

    // 3. User config directory: ~/.config/craft-cli/config.toml
    if let Some(config_dir) = directories::ProjectDirs::from("", "", "craft-cli") {
        let user_config = config_dir.config_dir().join("config.toml");
        if user_config.exists() {
            return Some(user_config);
        }
    }

    None
}

/// Load config from file
pub fn load_config_file(path: &Path) -> Result<ConfigFile> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| CliError::Config(format!("Failed to read config file: {}", e)))?;

    let config: ConfigFile = toml::from_str(&content)
        .map_err(|e| CliError::Config(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}

/// Save config to file
pub fn save_config_file(path: &Path, config: &ConfigFile) -> Result<()> {
    let content = toml::to_string_pretty(config)
        .map_err(|e| CliError::Config(format!("Failed to serialize config: {}", e)))?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| CliError::Config(format!("Failed to create config directory: {}", e)))?;
    }

    std::fs::write(path, content)
        .map_err(|e| CliError::Config(format!("Failed to write config file: {}", e)))?;

    Ok(())
}

/// Get default config file path for initialization
pub fn default_config_path() -> Result<PathBuf> {
    let config_dir = directories::ProjectDirs::from("", "", "craft-cli")
        .ok_or_else(|| CliError::Config("Cannot determine config directory".to_string()))?;

    Ok(config_dir.config_dir().join("config.toml"))
}

/// Resolve connection from CLI args, env vars, or config file
pub fn resolve_connection(cli: &Cli) -> Result<ResolvedConnection> {
    // Priority 1: --url parameter
    if let Some(url) = &cli.url {
        return Ok(ResolvedConnection {
            url: url.clone(),
            secret_key: cli.key.clone(),
        });
    }

    // Priority 2: CRAFT_API_URL env var
    if let Ok(url) = env::var("CRAFT_API_URL") {
        let key = cli.key.clone().or_else(|| env::var("CRAFT_API_KEY").ok());
        return Ok(ResolvedConnection { url, secret_key: key });
    }

    // Priority 3: Config file
    if let Some(config_path) = find_config_file(cli.config.as_ref()) {
        let config = load_config_file(&config_path)?;

        // Select connection name
        let conn_name = cli.conn.as_ref()
            .or(config.default.as_ref())
            .ok_or_else(|| CliError::Config(
                "No connection specified. Use --conn, set default in config, or use --url/CRAFT_API_URL".to_string()
            ))?;

        let connection = config.connections.get(conn_name)
            .ok_or_else(|| CliError::Config(
                format!("Connection '{}' not found in config file", conn_name)
            ))?;

        return Ok(ResolvedConnection {
            url: connection.url.clone(),
            secret_key: connection.secret_key.clone().or_else(|| cli.key.clone()),
        });
    }

    Err(CliError::Config(
        "No API URL configured. Use --url, CRAFT_API_URL env, or create a config file".to_string()
    ))
}

/// Initialize empty config file
pub fn init_config() -> Result<PathBuf> {
    let path = default_config_path()?;

    if path.exists() {
        return Ok(path); // Already exists, skip
    }

    let config = ConfigFile::default();
    save_config_file(&path, &config)?;

    Ok(path)
}

/// Add connection to config
pub fn add_connection(name: &str, url: &str, key: Option<&str>) -> Result<()> {
    let path = default_config_path()?;

    let mut config = if path.exists() {
        load_config_file(&path)?
    } else {
        ConfigFile::default()
    };

    config.connections.insert(name.to_string(), Connection {
        url: url.to_string(),
        secret_key: key.map(|s| s.to_string()),
    });

    save_config_file(&path, &config)?;
    Ok(())
}

/// Remove connection from config
pub fn remove_connection(name: &str) -> Result<()> {
    let path = default_config_path()?;

    let mut config = load_config_file(&path)?;

    if config.connections.remove(name).is_none() {
        return Err(CliError::Config(format!("Connection '{}' not found", name)));
    }

    // If removing default, clear it
    if config.default.as_ref() == Some(&name.to_string()) {
        config.default = None;
    }

    save_config_file(&path, &config)?;
    Ok(())
}

/// List all connections
pub fn list_connections() -> Result<Vec<(String, Connection)>> {
    let path = default_config_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let config = load_config_file(&path)?;
    let mut connections: Vec<_> = config.connections.into_iter().collect();
    connections.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(connections)
}

/// Set default connection
pub fn set_default_connection(name: &str) -> Result<()> {
    let path = default_config_path()?;

    let mut config = load_config_file(&path)?;

    if !config.connections.contains_key(name) {
        return Err(CliError::Config(format!("Connection '{}' not found", name)));
    }

    config.default = Some(name.to_string());
    save_config_file(&path, &config)?;

    Ok(())
}
