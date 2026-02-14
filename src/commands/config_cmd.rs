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
        ConfigAction::Add { name, api, key } => {
            let api = api.ok_or_else(|| {
                crate::error::CliError::Config("--api is required".to_string())
            })?;
            let key = key.ok_or_else(|| {
                crate::error::CliError::Config("--key is required".to_string())
            })?;
            config::add_api(&name, &api, &key)?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("API '{}' added", name),
                    "name": name,
                    "api": api
                })
            );
            Ok(())
        }
        ConfigAction::Remove { name } => {
            config::remove_api(&name)?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("API '{}' removed", name)
                })
            );
            Ok(())
        }
        ConfigAction::List => {
            let apis = config::list_apis()?;
            let current = config::get_current_api().unwrap_or_default();
            let items: Vec<_> = apis
                .into_iter()
                .map(|(name, entry)| {
                    json!({
                        "name": name,
                        "api": entry.api,
                        "current": name == current
                    })
                })
                .collect();
            println!("{}", json!({ "items": items }));
            Ok(())
        }
        ConfigAction::Use { name } => {
            config::set_current_api(&name)?;
            println!(
                "{}",
                json!({
                    "success": true,
                    "message": format!("'{}' set as current API", name)
                })
            );
            Ok(())
        }
    }
}
