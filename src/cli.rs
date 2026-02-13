use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "craft-cli")]
#[command(about = "Craft.do API CLI for Agents")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// API endpoint URL (or CRAFT_API_URL env)
    #[arg(long, env = "CRAFT_API_URL")]
    pub url: Option<String>,

    /// API secret key (or CRAFT_API_KEY env)
    #[arg(long, env = "CRAFT_API_KEY")]
    pub key: Option<String>,

    /// Config file path
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Connection name from config file
    #[arg(long)]
    pub conn: Option<String>,

    /// Verbose output for debugging
    #[arg(long, short)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configuration management
    Config(ConfigArgs),

    /// Block operations
    Blocks(BlocksArgs),

    /// Task management
    Tasks(TasksArgs),

    /// Search across daily notes
    Search(SearchArgs),

    /// Collection operations
    Collections(CollectionsArgs),

    /// Connection information
    Connection(ConnectionArgs),

    /// Upload file (image, video, document)
    Upload(UploadArgs),
}

#[derive(Parser)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand, Clone)]
pub enum ConfigAction {
    /// Initialize config file
    Init,
    /// Add a connection
    Add {
        /// Connection name
        name: String,
        /// API URL
        url: String,
        /// Secret key (optional)
        #[arg(long)]
        key: Option<String>,
    },
    /// Remove a connection
    Remove {
        /// Connection name
        name: String,
    },
    /// List all connections
    List,
    /// Set default connection
    Default {
        /// Connection name
        name: String,
    },
}

#[derive(Parser)]
pub struct BlocksArgs {
    #[command(subcommand)]
    pub action: BlocksAction,
}

#[derive(Subcommand, Clone)]
pub enum BlocksAction {
    /// Get blocks content
    Get {
        /// Date (today, yesterday, tomorrow, or YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Block ID
        #[arg(long)]
        id: Option<String>,
        /// Max depth to fetch
        #[arg(long, default_value = "-1")]
        depth: i32,
        /// Include metadata
        #[arg(long)]
        metadata: bool,
    },
    /// Insert blocks
    Insert {
        /// Date (today, yesterday, tomorrow, or YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Page ID (alternative to date)
        #[arg(long)]
        page_id: Option<String>,
        /// Position: start or end
        #[arg(long, default_value = "end")]
        pos: String,
        /// Markdown content (or use --stdin)
        markdown: Option<String>,
        /// Read from stdin
        #[arg(long)]
        stdin: bool,
    },
    /// Update blocks
    Update {
        /// Block ID
        id: String,
        /// New markdown content (or use --stdin)
        markdown: Option<String>,
        /// Read from stdin
        #[arg(long)]
        stdin: bool,
    },
    /// Delete blocks
    Delete {
        /// Block IDs to delete
        ids: Vec<String>,
    },
    /// Move blocks
    Move {
        /// Block IDs to move
        ids: Vec<String>,
        /// Target date
        #[arg(long)]
        date: Option<String>,
        /// Target page ID
        #[arg(long)]
        page_id: Option<String>,
        /// Position: start or end
        #[arg(long, default_value = "end")]
        pos: String,
    },
    /// Search within a date
    Search {
        /// Search pattern
        pattern: String,
        /// Date to search in
        #[arg(long)]
        date: Option<String>,
        /// Case sensitive
        #[arg(long)]
        case_sensitive: bool,
        /// Blocks before match
        #[arg(long)]
        before: Option<i32>,
        /// Blocks after match
        #[arg(long)]
        after: Option<i32>,
    },
}

#[derive(Parser)]
pub struct TasksArgs {
    #[command(subcommand)]
    pub action: TasksAction,
}

#[derive(Subcommand, Clone)]
pub enum TasksAction {
    /// List tasks
    List {
        /// Scope: active, inbox, upcoming, logbook
        scope: String,
    },
    /// Add a task
    Add {
        /// Task text
        text: String,
        /// Schedule date
        #[arg(long)]
        schedule: Option<String>,
        /// Deadline date
        #[arg(long)]
        deadline: Option<String>,
        /// Target: inbox or daily
        #[arg(long, default_value = "inbox")]
        to: String,
        /// Date (for daily target)
        #[arg(long)]
        date: Option<String>,
    },
    /// Update a task
    Update {
        /// Task ID
        id: String,
        /// New text
        #[arg(long)]
        text: Option<String>,
        /// New schedule date
        #[arg(long)]
        schedule: Option<String>,
        /// New deadline
        #[arg(long)]
        deadline: Option<String>,
        /// New state
        #[arg(long)]
        state: Option<String>,
    },
    /// Mark tasks as done
    Done {
        /// Task IDs
        ids: Vec<String>,
    },
    /// Delete tasks
    Delete {
        /// Task IDs
        ids: Vec<String>,
    },
}

#[derive(Parser, Clone)]
pub struct SearchArgs {
    /// Search query
    pub query: String,
    /// Start date
    #[arg(long)]
    pub from: Option<String>,
    /// End date
    #[arg(long)]
    pub to: Option<String>,
    /// Use regex
    #[arg(long)]
    pub regex: bool,
    /// Include metadata
    #[arg(long)]
    pub metadata: bool,
}

#[derive(Parser)]
pub struct CollectionsArgs {
    #[command(subcommand)]
    pub action: CollectionsAction,
}

#[derive(Subcommand, Clone)]
pub enum CollectionsAction {
    /// List collections
    List {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    /// Get collection schema
    Schema {
        /// Collection ID
        id: String,
        /// Format: schema or json-schema-items
        #[arg(long, default_value = "json-schema-items")]
        format: String,
    },
    /// Get collection items
    Items {
        /// Collection ID
        id: String,
        /// Max depth
        #[arg(long, default_value = "-1")]
        depth: i32,
    },
    /// Add item to collection
    AddItem {
        /// Collection ID
        id: String,
        /// Item title
        #[arg(long)]
        title: String,
        /// Properties (key=value format)
        #[arg(long = "prop", value_parser = parse_key_val)]
        props: Vec<(String, String)>,
    },
    /// Update collection item
    UpdateItem {
        /// Collection ID
        id: String,
        /// Item ID
        item_id: String,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// Properties (key=value format)
        #[arg(long = "prop", value_parser = parse_key_val)]
        props: Vec<(String, String)>,
    },
    /// Delete collection items
    DeleteItem {
        /// Collection ID
        id: String,
        /// Item IDs to delete
        item_ids: Vec<String>,
    },
}

#[derive(Parser)]
pub struct ConnectionArgs {
    #[command(subcommand)]
    pub action: Option<ConnectionAction>,
}

#[derive(Subcommand)]
pub enum ConnectionAction {
    /// Show connection info
    Info,
}

#[derive(Parser, Clone)]
pub struct UploadArgs {
    /// File path to upload
    pub file: PathBuf,
    /// Target date
    #[arg(long)]
    pub date: Option<String>,
    /// Target page ID
    #[arg(long)]
    pub page_id: Option<String>,
    /// Position: start or end
    #[arg(long, default_value = "end")]
    pub pos: String,
}

/// Parse key=value pair
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
