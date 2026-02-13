use crate::client::CraftClient;
use crate::error::Result;

pub async fn handle_info(client: &CraftClient) -> Result<()> {
    let info = client.get_connection_info().await?;
    println!("{}", serde_json::to_string_pretty(&info)?);
    Ok(())
}
