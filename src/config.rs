use crate::error::{CliError, Result};
use crate::cli::Cli;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiEntry {
    pub api: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub current: String,
    pub apis: HashMap<String, ApiEntry>,
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

    // 2. Current directory: ./craft-cli.json
    let current_dir = env::current_dir().ok()?;
    let local_config = current_dir.join("craft-cli.json");
    if local_config.exists() {
        return Some(local_config);
    }

    // 3. User config directory: ~/.config/craft-cli/config.json
    if let Some(config_dir) = directories::ProjectDirs::from("", "", "craft-cli") {
        let user_config = config_dir.config_dir().join("config.json");
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

    let config: ConfigFile = serde_json::from_str(&content)
        .map_err(|e| CliError::Config(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}

/// Save config to file
pub fn save_config_file(path: &Path, config: &ConfigFile) -> Result<()> {
    let content = serde_json::to_string_pretty(config)
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

    Ok(config_dir.config_dir().join("config.json"))
}

/// Resolve connection from CLI args, env vars, or config file
pub fn resolve_connection(cli: &Cli) -> Result<ResolvedConnection> {
    // Priority 1: --api and --key parameters
    if let (Some(api), Some(key)) = (&cli.api, &cli.key) {
        return Ok(ResolvedConnection {
            url: api.clone(),
            secret_key: Some(key.clone()),
        });
    }

    // Priority 2: CRAFT_API and CRAFT_KEY env vars
    if let (Ok(api), Ok(key)) = (env::var("CRAFT_API"), env::var("CRAFT_KEY")) {
        return Ok(ResolvedConnection {
            url: api,
            secret_key: Some(key),
        });
    }

    // Priority 3: Config file (read current API)
    if let Some(config_path) = find_config_file(cli.config.as_ref()) {
        let config = load_config_file(&config_path)?;

        let entry = config.apis.get(&config.current)
            .ok_or_else(|| CliError::Config(
                format!("Current API '{}' not found in config file", config.current)
            ))?;

        return Ok(ResolvedConnection {
            url: entry.api.clone(),
            secret_key: Some(entry.key.clone()),
        });
    }

    Err(CliError::Config(
        "No API configured. Use --api/--key, CRAFT_API/CRAFT_KEY env, or create a config file".to_string()
    ))
}

/// Initialize empty config file
pub fn init_config() -> Result<PathBuf> {
    let path = default_config_path()?;

    if path.exists() {
        return Ok(path); // Already exists, skip
    }

    let config = ConfigFile {
        current: "default".to_string(),
        apis: HashMap::new(),
    };
    save_config_file(&path, &config)?;

    Ok(path)
}

/// Add API to config
pub fn add_api(name: &str, api: &str, key: &str) -> Result<()> {
    let path = default_config_path()?;

    let mut config = if path.exists() {
        load_config_file(&path)?
    } else {
        ConfigFile {
            current: name.to_string(),
            apis: HashMap::new(),
        }
    };

    config.apis.insert(name.to_string(), ApiEntry {
        api: api.to_string(),
        key: key.to_string(),
    });

    save_config_file(&path, &config)?;
    Ok(())
}

/// Remove API from config
pub fn remove_api(name: &str) -> Result<()> {
    let path = default_config_path()?;

    let mut config = load_config_file(&path)?;

    if config.apis.remove(name).is_none() {
        return Err(CliError::Config(format!("API '{}' not found", name)));
    }

    // If removing current, switch to another one if available
    if config.current == name {
        config.current = config.apis.keys().next()
            .cloned()
            .unwrap_or_else(|| "default".to_string());
    }

    save_config_file(&path, &config)?;
    Ok(())
}

/// List all APIs
pub fn list_apis() -> Result<Vec<(String, ApiEntry)>> {
    let path = default_config_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let config = load_config_file(&path)?;
    let mut apis: Vec<_> = config.apis.into_iter().collect();
    apis.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(apis)
}

/// Get current API name
pub fn get_current_api() -> Result<String> {
    let path = default_config_path()?;

    if !path.exists() {
        return Err(CliError::Config("Config file not found".to_string()));
    }

    let config = load_config_file(&path)?;
    Ok(config.current)
}

/// Set current API
pub fn set_current_api(name: &str) -> Result<()> {
    let path = default_config_path()?;

    let mut config = load_config_file(&path)?;

    if !config.apis.contains_key(name) {
        return Err(CliError::Config(format!("API '{}' not found", name)));
    }

    config.current = name.to_string();
    save_config_file(&path, &config)?;

    Ok(())
}
