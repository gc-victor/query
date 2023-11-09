pub mod cache;
pub mod commands;
pub mod config;
pub mod prompts;
pub mod utils;

use std::fs;

use clap::{command, Parser};
use commands::{
    branch::command_branch, commands::Commands, function::command_function,
    migration::command_migration, settings::command_settings, shell::command_shell,
    token::command_token, user::command_user, user_token::command_user_token,
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "Query")]
#[command(version = "0.3.0")]
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

    std::panic::set_hook(Box::new(tracing_error));

    match &command {
        Commands::Branch(command) => command_branch(command).await.unwrap(),
        Commands::Function(command) => command_function(command).await.unwrap(),
        Commands::Migration(command) => command_migration(command).await,
        Commands::Settings => command_settings().await,
        Commands::Shell(command) => command_shell(command).await.unwrap(),
        Commands::Token(command) => command_token(command).await.unwrap(),
        Commands::User(command) => command_user(command).await.unwrap(),
        Commands::UserToken(command) => command_user_token(command).await.unwrap(),
    }
}

fn tracing_error(panic_info: &std::panic::PanicInfo) {
    // @see: https://github.com/LukeMathWalker/tracing-panic/blob/main/src/lib.rs#L52
    let payload = panic_info.payload();

    #[allow(clippy::manual_map)]
    let payload = if let Some(s) = payload.downcast_ref::<&str>() {
        Some(&**s)
    } else if let Some(s) = payload.downcast_ref::<String>() {
        Some(s.as_str())
    } else {
        None
    };

    let location = panic_info
        .location()
        .map(|l| l.to_string())
        .unwrap_or_default();

    tracing::error!("{}", location);
    tracing::error!("{}", payload.unwrap());
}
