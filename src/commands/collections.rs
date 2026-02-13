use crate::cli::CollectionsAction;
use crate::client::CraftClient;
use crate::error::Result;
use serde_json::json;

pub async fn handle(client: &CraftClient, action: CollectionsAction) -> Result<()> {
    match action {
        CollectionsAction::List { from, to } => {
            let result = client.list_collections(from.as_deref(), to.as_deref()).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        CollectionsAction::Schema { id, format } => {
            let result = client.get_collection_schema(&id, &format).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        CollectionsAction::Items { id, depth } => {
            let result = client.get_collection_items(&id, depth).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        CollectionsAction::AddItem { id, title, props } => {
            let mut properties = json!({});
            for (key, val) in props {
                properties[key] = json!(val);
            }

            let items = vec![json!({
                "title": title,
                "properties": properties
            })];

            let result = client.add_collection_items(&id, items).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        CollectionsAction::UpdateItem {
            id,
            item_id,
            title,
            props,
        } => {
            let mut update = json!({"id": item_id});

            if let Some(title) = title {
                update["title"] = json!(title);
            }

            if !props.is_empty() {
                let mut properties = json!({});
                for (key, val) in props {
                    properties[key] = json!(val);
                }
                update["properties"] = properties;
            }

            let items = vec![update];
            let result = client.update_collection_items(&id, items).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        CollectionsAction::DeleteItem { id, item_ids } => {
            let result = client.delete_collection_items(&id, item_ids).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
    }
}
