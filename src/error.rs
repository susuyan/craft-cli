use serde_json::json;
use std::process::exit;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("API error: {code} - {message}")]
    Api { code: u16, message: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Argument error: {0}")]
    Argument(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("JSON error: {0}")]
    Json(String),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Api { .. } => 1,
            CliError::Config(_) => 2,
            CliError::Argument(_) => 3,
            CliError::Network(_) => 10,
            CliError::Io(_) => 4,
            CliError::Parse(_) => 5,
            CliError::Json(_) => 6,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            CliError::Api { .. } => "api_error",
            CliError::Config(_) => "config_error",
            CliError::Argument(_) => "argument_error",
            CliError::Network(_) => "network_error",
            CliError::Io(_) => "io_error",
            CliError::Parse(_) => "parse_error",
            CliError::Json(_) => "json_error",
        }
    }

    pub fn print_json(&self) {
        let error_json = json!({
            "error": self.kind(),
            "code": self.exit_code(),
            "message": self.to_string()
        });
        eprintln!("{}", serde_json::to_string(&error_json).unwrap());
    }

    pub fn exit(&self) -> ! {
        self.print_json();
        exit(self.exit_code());
    }
}

pub type Result<T> = std::result::Result<T, CliError>;

/// Helper to convert reqwest errors
pub fn map_reqwest_error(e: reqwest::Error) -> CliError {
    if e.is_timeout() {
        CliError::Network("Request timeout".to_string())
    } else if e.is_connect() {
        CliError::Network("Connection failed".to_string())
    } else if let Some(status) = e.status() {
        CliError::Api {
            code: status.as_u16(),
            message: e.to_string(),
        }
    } else {
        CliError::Network(e.to_string())
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::Json(e.to_string())
    }
}
