use crate::cli::SearchArgs;
use crate::client::CraftClient;
use crate::error::Result;

pub async fn handle(client: &CraftClient, args: SearchArgs) -> Result<()> {
    let result = client
        .search(&args.query, args.from.as_deref(), args.to.as_deref(), args.regex)
        .await?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
