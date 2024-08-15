pub mod cache;
pub mod commands;
pub mod config;
pub mod prompts;
pub mod run_server;
pub mod utils;

use std::env;

use clap::{command, Parser};
use colored::Colorize;
use dotenv::dotenv;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use commands::{
    asset::command_asset, branch::command_branch, commands::Commands, create::command_create,
    dev::command_dev, function::command_function, generate::command_generate,
    migration::command_migration, plugin::command_plugin, purge::command_purge,
    settings::command_settings, shell::command_shell, task::command_task, token::command_token,
    user::command_user, user_token::command_user_token,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "Query")]
#[command(version = VERSION)]
#[command(about = "The CLI to manage your Query Server instance", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    let _ = tracing::subscriber::set_default(subscriber);

    std::panic::set_hook(Box::new(tracing_error));

    let args = Cli::parse();
    let command = args.command;

    // Ex. `export QUERY_CLI_DEV=true && ../../query/target/debug/query dev`
    if env::var("QUERY_CLI_DEV").is_ok() {
        println!("{}", "[WARNING] A QUERY_CLI_DEV is being used".yellow())
    };

    match &command {
        Commands::Asset(command) => command_asset(command).await.unwrap(),
        Commands::Branch(command) => command_branch(command).await.unwrap(),
        Commands::Create(command) => command_create(command).await.unwrap(),
        Commands::Dev(command) => command_dev(command).await.unwrap(),
        Commands::Function(command) => command_function(command).await.unwrap(),
        Commands::Generate(command) => command_generate(command).await.unwrap(),
        Commands::Migration(command) => command_migration(command).await,
        Commands::Settings => command_settings().await.unwrap(),
        Commands::Plugin(command) => command_plugin(command).await,
        Commands::Purge => command_purge().await,
        Commands::Shell(command) => command_shell(command).await.unwrap(),
        Commands::Task(command) => command_task(command).unwrap(),
        Commands::Token(command) => command_token(command).await.unwrap(),
        Commands::User(command) => command_user(command).await.unwrap(),
        Commands::UserToken(command) => command_user_token(command).await.unwrap(),
    }
}

fn tracing_error(panic_info: &std::panic::PanicInfo) {
    let debug = env::var("DEBUG").unwrap_or("false".to_string());

    if debug != "true" {
        return;
    }

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

    eprintln!("{}", location);
    eprintln!("{}", payload.unwrap());
}
