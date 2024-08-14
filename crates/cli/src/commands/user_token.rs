use anyhow::Result;
use cliclack::{confirm, input, intro, outro, password};
use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::{
    prompts::expiration_date,
    utils::{http_client, json_to_table},
};

use super::commands::{UserTokenArgs, UserTokenCommands};

pub async fn command_user_token(command: &UserTokenArgs) -> Result<()> {
    match &command.command {
        UserTokenCommands::Create => {
            intro("Create a User Token".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email to create a token")
                .interact()?;
            let write = confirm("Should the token be granted with write permissions?")
                .initial_value(true)
                .interact()?;
            let expiration_date = expiration_date()?;

            let body = json!({
                "email": email,
                "expiration_date": expiration_date,
                "write": write,
            })
            .to_string();

            match http_client("user/token", Some(&body), Method::POST).await {
                Ok(_) => {
                    outro("Token created".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        UserTokenCommands::Delete => {
            intro("Delete a Token".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email to delete a token")
                .interact()?;

            let body = json!({"email": email }).to_string();

            match http_client("user/token", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    outro("Token deleted".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        UserTokenCommands::List => {
            match http_client("user/token", None, Method::GET).await {
                Ok(v) => {
                    let is_empty = match v["data"].as_array() {
                        Some(v) => v.is_empty(),
                        None => true,
                    };

                    if is_empty {
                        eprintln!("{} No data returned", String::from('●').red());
                    } else {
                        eprintln!("{}", json_to_table(&v["data"])?);
                    }
                }
                Err(err) => {
                    eprintln!("{} {}", String::from('●').red(), err);
                }
            };

            Ok(())
        }
        UserTokenCommands::Update => {
            intro("Update a User Token".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email to create a token")
                .interact()?;
            let write = confirm("Should the token be granted with write permissions?")
                .initial_value(true)
                .interact()?;
            let expiration_date = expiration_date()?;

            let body = json!({
                "email": email,
                "expiration_date": expiration_date,
                "write": write,
            })
            .to_string();

            match http_client("user/token", Some(&body), Method::PUT).await {
                Ok(_) => {
                    outro("Token updated".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        UserTokenCommands::Value => {
            let email: String = input("What is the user email?")
                .placeholder("Use the user email to get the token")
                .interact()?;
            let password = password("What is the user password?")
                .mask('▪')
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a password.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let body = json!({
                "email": email,
                "password": password,
            })
            .to_string();

            match http_client("user/token/value", Some(&body), Method::POST).await {
                Ok(v) => {
                    if v["data"][0].is_null() {
                        outro("No data returned".to_string().red().reversed())?;
                    } else {
                        let toke = match v["data"][0]["token"] {
                            serde_json::Value::String(ref toke) => toke.to_string(),
                            _ => {
                                outro("No data returned".to_string().red().reversed())?;
                                return Ok(());
                            }
                        };
                        outro(format!(r#"token: "{}""#, toke).green().reversed())?;
                    }
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
    }
}
