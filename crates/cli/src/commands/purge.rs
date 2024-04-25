use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{utils::http_client, utils::line_break};

pub async fn command_purge() {
    let body = json!({
        "db_name": "query_cache_function.sql",
        "query": "DELETE FROM cache_function;"
    })
    .to_string();

    match http_client("query", Some(&body), Method::POST).await {
        Ok(_) => {
            line_break();
            info!("Successfully cache function purged!!!!");
            line_break();
        }
        Err(err) => error!("{}", err),
    };
}
