mod cli;
mod client;
mod commands;
mod config;
mod error;
mod logger;

use clap::Parser;
use cli::{Cli, Commands};
use client::CraftClient;
use config::resolve_connection;
use logger::set_verbose;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Set verbose mode
    set_verbose(cli.verbose);

    match run(cli).await {
        Ok(_) => std::process::exit(0),
        Err(e) => e.exit(),
    }
}

async fn run(cli: Cli) -> error::Result<()> {
    match &cli.command {
        Commands::Config(args) => {
            commands::config_cmd::handle(args.action.clone()).await?;
        }
        _ => {
            // Other commands need a client
            let conn = resolve_connection(&cli)?;
            let client = CraftClient::new(conn.url, conn.secret_key);

            match &cli.command {
                Commands::Blocks(args) => {
                    commands::blocks::handle(&client, args.action.clone()).await?;
                }
                Commands::Tasks(args) => {
                    commands::tasks::handle(&client, args.action.clone()).await?;
                }
                Commands::Search(args) => {
                    commands::search::handle(&client, args.clone()).await?;
                }
                Commands::Collections(args) => {
                    commands::collections::handle(&client, args.action.clone()).await?;
                }
                Commands::Connection(args) => {
                    match args.action.as_ref() {
                        Some(cli::ConnectionAction::Info) | None => {
                            commands::connection::handle_info(&client).await?;
                        }
                    }
                }
                Commands::Upload(args) => {
                    commands::upload::handle(&client, args.clone()).await?;
                }
                Commands::Config(_) => unreachable!(),
            }
        }
    }

    Ok(())
}
