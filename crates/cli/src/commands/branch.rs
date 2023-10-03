use anyhow::Result;
use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{
    prompts::{
        text_prompt, PROMPT_BRANCH_DB_NAME_DELETE_MESSAGE, PROMPT_BRANCH_DB_NAME_MESSAGE,
        PROMPT_BRANCH_NAME_MESSAGE,
    },
    utils::{http_client, json_to_table, line_break},
};

use super::commands::{BranchArgs, BranchCommands};

pub async fn command_branch(command: &BranchArgs) -> Result<()> {
    match &command.command {
        BranchCommands::Create => {
            let db_name = text_prompt(PROMPT_BRANCH_DB_NAME_MESSAGE)?;
            let branch_name = text_prompt(PROMPT_BRANCH_NAME_MESSAGE)?;

            let body = json!({
                "db_name": db_name,
                "branch_name": branch_name
            })
            .to_string();

            match http_client("branch", Some(&body), Method::POST).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully branch created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        BranchCommands::Delete => {
            let db_name = text_prompt(PROMPT_BRANCH_DB_NAME_DELETE_MESSAGE)?;

            let body = json!({
                "db_name": db_name,
            })
            .to_string();

            match http_client("branch", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully branch deleted!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        BranchCommands::List => {
            match http_client("branch", None, Method::GET).await {
                Ok(v) => {
                    if v["data"][0].is_null() {
                        line_break();
                        info!("No data returned.");
                        line_break();
                    } else {
                        line_break();
                        eprintln!("{}", json_to_table(&v["data"])?);
                        line_break();
                    }
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
    }
}
