use crate::cli::BlocksAction;
use crate::client::CraftClient;
use crate::error::{CliError, Result};
use serde_json::json;
use std::io::{self, Read};

pub async fn handle(client: &CraftClient, action: BlocksAction) -> Result<()> {
    match action {
        BlocksAction::Get {
            date,
            id,
            depth,
            metadata,
        } => {
            let block = client
                .get_block(id.as_deref(), date.as_deref(), depth, metadata)
                .await?;
            println!("{}", serde_json::to_string_pretty(&block)?);
            Ok(())
        }
        BlocksAction::Insert {
            date,
            page_id,
            pos,
            markdown,
            stdin,
        } => {
            let content = if stdin {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else if let Some(md) = markdown {
                md
            } else {
                return Err(CliError::Argument(
                    "Either markdown content or --stdin must be provided".to_string(),
                ));
            };

            // If date is provided, first fetch the daily note to get its block ID
            let target_id = match (date, page_id) {
                (Some(d), _) => {
                    // Fetch the daily note to get its ID
                    let daily_note = client.get_block(None, Some(&d), 0, false).await?;
                    daily_note["id"].as_str()
                        .ok_or_else(|| CliError::Api { code: 404, message: "Daily note not found".to_string() })?
                        .to_string()
                }
                (None, Some(p)) => p,
                _ => {
                    return Err(CliError::Argument(
                        "Either --date or --page-id must be provided".to_string(),
                    ))
                }
            };

            let blocks = vec![json!({
                "type": "text",
                "markdown": content
            })];

            let position = json!({
                "position": pos,
                "pageId": target_id
            });

            let result = client.insert_blocks(blocks, position).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        BlocksAction::Update { id, markdown, stdin } => {
            let content = if stdin {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else if let Some(md) = markdown {
                md
            } else {
                return Err(CliError::Argument(
                    "Either markdown content or --stdin must be provided".to_string(),
                ));
            };

            let blocks = vec![json!({
                "id": id,
                "markdown": content
            })];

            let result = client.update_blocks(blocks).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        BlocksAction::Delete { ids } => {
            let result = client.delete_blocks(ids).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        BlocksAction::Move {
            ids,
            date,
            page_id,
            pos,
        } => {
            let target_id = match (date, page_id) {
                (Some(d), _) => d,
                (None, Some(p)) => p,
                _ => {
                    return Err(CliError::Argument(
                        "Either --date or --page-id must be provided".to_string(),
                    ))
                }
            };

            let position = json!({
                "position": pos,
                "pageId": target_id
            });

            let result = client.move_blocks(ids, position).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        BlocksAction::Search {
            pattern,
            date,
            case_sensitive,
            before,
            after,
        } => {
            let result = client
                .search_blocks(date.as_deref(), &pattern, case_sensitive, before, after)
                .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
    }
}
