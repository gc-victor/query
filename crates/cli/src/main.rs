pub mod commands;
pub mod config;
pub mod prompts;
pub mod utils;

use std::fs;

use clap::{command, Parser};
use commands::{
    commands::Commands, migration::command_migration, settings::command_settings,
    shell::command_shell, token::command_token, user::command_user, user_token::command_user_token,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "Query")]
#[command(version = "0.1.0")]
#[command(about = "The CLI to manage your Query Server instance", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Cli::parse();
    let command = args.command;

    fs::create_dir_all(".query").unwrap();

    match &command {
        Commands::Migration(command) => command_migration(command).await,
        Commands::Settings => command_settings().await,
        Commands::Shell(command) => command_shell(command).await.unwrap(),
        Commands::Token(command) => command_token(command).await.unwrap(),
        Commands::User(command) => command_user(command).await.unwrap(),
        Commands::UserToken(command) => command_user_token(command).await.unwrap(),
    }
}
