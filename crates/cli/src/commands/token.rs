use anyhow::Result;
use cliclack::{confirm, input, intro, outro};
use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::{
    prompts::expiration_date,
    utils::{http_client, json_to_table},
};

use super::commands::{TokenArgs, TokenCommands};

pub async fn command_token(command: &TokenArgs) -> Result<()> {
    match &command.command {
        TokenCommands::Create => {
            intro("Create a Token".to_string().cyan().reversed())?;

            let name: String = input("What is the token name?")
                .placeholder("Give a name to the token")
                .interact()?;

            let write = confirm("Should the token be granted with write permissions?")
                .initial_value(true)
                .interact()?;

            let active = confirm("Is it an active token?")
                .initial_value(true)
                .interact()?;

            let expiration_date = expiration_date()?;

            let body = json!({
                "name": name,
                "expiration_date": expiration_date,
                "active": active,
                "write": write,
            })
            .to_string();

            match http_client("token", Some(&body), Method::POST).await {
                Ok(_) => {
                    outro("Token created".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        TokenCommands::Delete => {
            intro("Delete a Token".to_string().cyan().reversed())?;

            let name: String = input("What is the token name?")
                .placeholder("Give a name to the token")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a token name.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let body = json!({
                "name": name
            })
            .to_string();

            match http_client("token", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    outro("Token deleted".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        TokenCommands::List => {
            match http_client("token", None, Method::GET).await {
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
        TokenCommands::Update => {
            intro("Update a Token".to_string().cyan().reversed())?;

            let name: String = input("What is the token name?")
                .placeholder("Give a name to the token")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a token name.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let write = confirm("Should have write permissions?")
                .initial_value(true)
                .interact()?;

            let active = confirm("Is it an active token?")
                .initial_value(true)
                .interact()?;

            let expiration_date = expiration_date()?;

            let body = json!({
                "name": name,
                "expiration_date": expiration_date,
                "active": active,
                "write": write,
            })
            .to_string();

            match http_client("token", Some(&body), Method::PUT).await {
                Ok(_) => {
                    outro("Token updated".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        TokenCommands::Value => {
            intro("Get Token".to_string().cyan().reversed())?;

            let name: String = input("What is the token name?")
                .placeholder("Give a name to the token")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a token name.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let path = format!("{}{}", "token/value", "?name=".to_string() + &name);

            match http_client(&path, None, Method::GET).await {
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
