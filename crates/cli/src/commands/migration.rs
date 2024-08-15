use std::fs;

use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::{config::CONFIG, utils::http_client};

use super::commands::MigrationArgs;

pub async fn command_migration(command: &MigrationArgs) {
    let db_name = &command.db_name;
    let path = &command.path;
    let path = path.to_string();
    let query = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => {
            panic!(r#"The migration file "{}" doesn't exists"#, path);
        }
    };

    let body = json!({
        "db_name": db_name,
        "query": query
    })
    .to_string();

    match http_client("migration", Some(&body), Method::POST).await {
        Ok(_) => {
            let path = path.replace(&CONFIG.structure.migrations_folder.clone(), "");

            println!("{} Migration executed: {path}", String::from('●').green());
        }
        Err(e) => eprintln!("{} {}", String::from('●').red(), e),
    };
}
