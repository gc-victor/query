use anyhow::Result;
use cliclack::{input, intro, outro};
use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::utils::{http_client, json_to_table};

use super::commands::{BranchArgs, BranchCommands};

pub const PROMPT_BRANCH_DB_NAME_MESSAGE: &str =
    "Which database would you like to use for creating a branch?";
pub const PROMPT_BRANCH_DB_NAME_DELETE_MESSAGE: &str =
    "Which branch database would you like to delete?";
pub const PROMPT_BRANCH_NAME_MESSAGE: &str = "What is the branch name?";

pub async fn command_branch(command: &BranchArgs) -> Result<()> {
    match &command.command {
        BranchCommands::Create => {
            intro("Create a Branch".to_string().cyan().reversed())?;

            let db_name: String = input("What database would you like to use to create a branch?")
                .placeholder("Use the database you want to create a branch from.")
                .interact()?;
            let branch_name: String = input("What is the branch name?")
                .placeholder("Give a name to the branch.")
                .interact()?;

            let body = json!({
                "db_name": db_name,
                "branch_name": branch_name
            })
            .to_string();

            match http_client("branch", Some(&body), Method::POST).await {
                Ok(_) => {
                    outro("Branch created".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        BranchCommands::Delete => {
            intro("Delete a Branch".to_string().cyan().reversed())?;

            let db_name: String = input("What database branch would you like to delete?")
                .placeholder("Use the database you want to delete.")
                .interact()?;

            let body = json!({
                "db_name": db_name,
            })
            .to_string();

            match http_client("branch", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    outro("Branch deleted".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        BranchCommands::List => {
            match http_client("branch", None, Method::GET).await {
                Ok(v) => {
                    let is_empty = match v["data"].as_array() {
                        Some(v) => v.is_empty(),
                        None => true,
                    };

                    if is_empty {
                        eprintln!("{} No data returned", String::from('●').red());
                    } else {
                        println!("{}", json_to_table(&v["data"])?);
                    }
                }
                Err(e) => eprintln!("{} {}", String::from('●').red(), e),
            };

            Ok(())
        }
    }
}
