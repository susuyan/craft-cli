use crate::cli::UploadArgs;
use crate::client::CraftClient;
use crate::error::{CliError, Result};
use serde_json::json;

pub async fn handle(client: &CraftClient, args: UploadArgs) -> Result<()> {
    let file_path = &args.file;

    if !file_path.exists() {
        return Err(CliError::Argument(format!(
            "File not found: {}",
            file_path.display()
        )));
    }

    // Determine target
    let target_id = match (args.date, args.page_id) {
        (Some(d), _) => {
            // Fetch the daily note to get its ID
            let daily_note = client.get_block(None, Some(&d), 0, false).await?;
            daily_note["id"]
                .as_str()
                .ok_or_else(|| CliError::Api {
                    code: 404,
                    message: "Daily note not found".to_string(),
                })?
                .to_string()
        }
        (None, Some(p)) => p,
        _ => {
            return Err(CliError::Argument(
                "Either --date or --page-id must be provided".to_string(),
            ))
        }
    };

    let result = client
        .upload_file(file_path, &target_id, &args.pos)
        .await?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
