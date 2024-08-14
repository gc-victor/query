use anyhow::Result;
use cliclack::{confirm, input, intro, outro, password};
use colored::Colorize;
use reqwest::Method;
use serde_json::json;

use crate::utils::{http_client, json_to_table};

use super::commands::{UserArgs, UserCommands};

pub async fn command_user(command: &UserArgs) -> Result<()> {
    match &command.command {
        UserCommands::Create => {
            intro("Create a User".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email")
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

            let admin = confirm("Is the user an admin user?")
                .initial_value(true)
                .interact()?;

            let active = confirm("Is the user active?")
                .initial_value(true)
                .interact()?;

            let body = json!({
                "email": email,
                "password": password,
                "admin": admin,
                "active": active,
            })
            .to_string();

            match http_client("user", Some(&body), Method::POST).await {
                Ok(_) => {
                    outro("User created".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        UserCommands::Delete => {
            intro("Delete a User".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email")
                .interact()?;

            let body = json!({"email": email }).to_string();

            match http_client("user", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    outro("User deleted".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
        UserCommands::List => {
            match http_client("user", None, Method::GET).await {
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
        UserCommands::Update => {
            intro("Update a User".to_string().cyan().reversed())?;

            let email: String = input("What is the user email?")
                .placeholder("Use the user email")
                .interact()?;

            let new_email: String = input("What is the user new email?")
                .placeholder("Use the user new email")
                .interact()?;

            let new_password = password("What is the user new password?")
                .mask('▪')
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a password.")
                    } else {
                        Ok(())
                    }
                })
                .interact()?;

            let admin = confirm("Is the user an admin user?")
                .initial_value(true)
                .interact()?;

            let active = confirm("Is the user active?")
                .initial_value(true)
                .interact()?;

            let body = json!({
                "email": email,
                "new_email": new_email,
                "new_password": new_password,
                "admin": admin,
                "active": active,
            })
            .to_string();

            match http_client("user", Some(&body), Method::PUT).await {
                Ok(_) => {
                    outro("User updated".to_string().green().reversed())?;
                }
                Err(err) => {
                    outro(err.to_string().red().reversed())?;
                }
            };

            Ok(())
        }
    }
}
