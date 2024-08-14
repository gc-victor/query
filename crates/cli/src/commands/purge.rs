use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::utils::http_client;

pub async fn command_purge() {
    let body = json!({
        "db_name": "query_cache_function.sql",
        "query": "DELETE FROM cache_function;"
    })
    .to_string();

    match http_client("query", Some(&body), Method::POST).await {
        Ok(_) => {
            eprintln!(
                "{} Successfully cache function purged!!!!",
                String::from('●').green()
            );
        }
        Err(e) => eprintln!("{} {}", String::from('●').red(), e),
    };
}
