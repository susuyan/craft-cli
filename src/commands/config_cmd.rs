use crate::cli::ConfigAction;
use crate::config;
use crate::error::Result;
use serde_json::json;

pub async fn handle(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Init => {
            let path = config::init_config()?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": "Config file initialized",
                    "path": path.to_string_lossy().to_string()
                })
            );
            Ok(())
        }
        ConfigAction::Add { name, url, key } => {
            config::add_connection(&name, &url, key.as_deref())?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("Connection '{}' added", name),
                    "name": name,
                    "url": url
                })
            );
            Ok(())
        }
        ConfigAction::Remove { name } => {
            config::remove_connection(&name)?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("Connection '{}' removed", name)
                })
            );
            Ok(())
        }
        ConfigAction::List => {
            let connections = config::list_connections()?;
            let items: Vec<_> = connections
                .into_iter()
                .map(|(name, conn)| {
                    json!({
                        "name": name,
                        "url": conn.url,
                        "has_key": conn.secret_key.is_some()
                    })
                })
                .collect();
            println!("{}", json!({ "items": items }));
            Ok(())
        }
        ConfigAction::Default { name } => {
            config::set_default_connection(&name)?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("'{}' set as default connection", name)
                })
            );
            Ok(())
        }
    }
}
