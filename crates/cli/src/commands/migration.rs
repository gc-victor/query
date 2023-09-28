use std::{fs, process::exit};

use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{utils::http_client, utils::line_break};

use super::commands::MigrationArgs;

pub async fn command_migration(command: &MigrationArgs) {
    let db_name = &command.db_name;
    let path = &command.path;
    let path = path.to_string();
    let query = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => {
            error!(r#"The migration file "{}" doesn't exists"#, path);
            exit(1);
        }
    };

    let body = json!({
        "db_name": db_name,
        "query": query
    })
    .to_string();

    match http_client("migration", Some(&body), Method::POST).await {
        Ok(_) => {
            line_break();
            info!("Migrated {path}!!!");
            line_break();
        }
        Err(err) => error!("{}", err),
    };
}
