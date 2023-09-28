use std::process::exit;

use anyhow::Result;
use inquire::{Password, PasswordDisplayMode};
use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{
    prompts::{
        confirm_optional_prompt, confirm_prompt, new_password_prompt, password_prompt, text_prompt,
        PROMPT_ACTIVE_USER_MESSAGE, PROMPT_ADMIN_MESSAGE, PROMPT_EMAIL_MESSAGE,
        PROMPT_NEW_EMAIL_MESSAGE,
    },
    utils::{http_client, json_to_table, line_break},
};

use super::commands::{UserArgs, UserCommands};

pub async fn command_user(command: &UserArgs) -> Result<()> {
    match &command.command {
        UserCommands::Create => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
            let password = password_prompt()?;
            let admin = confirm_prompt(PROMPT_ADMIN_MESSAGE)?;
            let active = confirm_prompt(PROMPT_ACTIVE_USER_MESSAGE)?;

            let body = json!({
                "email": email,
                "password": password,
                "admin": admin,
                "active": active,
            })
            .to_string();

            match http_client("user", Some(&body), Method::POST).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserCommands::Delete => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;

            let body = json!({"email": email }).to_string();

            match http_client("user", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user deleted!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserCommands::List => {
            match http_client("user", None, Method::GET).await {
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
        UserCommands::Password => {
            let password = Password::new("What is your new password?")
                .without_confirmation()
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_formatter(&|s| {
                    if s.is_empty() {
                        return String::new();
                    }
                    String::from("Input received")
                })
                .prompt();

            if password.is_err() {
                error!("An error happened when asking for the password, try again.");
                exit(1);
            };

            let body = json!({ "password": password? }).to_string();

            match http_client("user", Some(&body), Method::PUT).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user password updated!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserCommands::Update => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
            let new_email = text_prompt(PROMPT_NEW_EMAIL_MESSAGE)?;
            let new_password = new_password_prompt()?;
            let admin = confirm_optional_prompt(PROMPT_ADMIN_MESSAGE)?;
            let active = confirm_optional_prompt(PROMPT_ACTIVE_USER_MESSAGE)?;

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
                    line_break();
                    info!("Successfully user updated!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
    }
}
