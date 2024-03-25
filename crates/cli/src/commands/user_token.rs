use anyhow::Result;
use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{
    prompts::{
        confirm_optional_prompt, confirm_prompt, integer_optional_prompt, password_prompt,
        text_prompt, PROMPT_EMAIL_MESSAGE, PROMPT_EXPIRATION_DATE_MESSAGE, PROMPT_WRITE_MESSAGE,
    },
    utils::{http_client, json_to_table, line_break},
};

use super::commands::{UserTokenArgs, UserTokenCommands};

pub async fn command_user_token(command: &UserTokenArgs) -> Result<()> {
    match &command.command {
        UserTokenCommands::Create => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
            let write = confirm_prompt(PROMPT_WRITE_MESSAGE)?;
            let expiration_date = integer_optional_prompt(PROMPT_EXPIRATION_DATE_MESSAGE)?;

            let body = json!({
                "email": email,
                "expiration_date": expiration_date,
                "write": write,
            })
            .to_string();

            match http_client("user/token", Some(&body), Method::POST).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user token created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserTokenCommands::Delete => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;

            let body = json!({"email": email }).to_string();

            match http_client("user/token", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user token deleted!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserTokenCommands::List => {
            match http_client("user/token", None, Method::GET).await {
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
        UserTokenCommands::Update => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
            let write = confirm_optional_prompt(PROMPT_WRITE_MESSAGE)?;
            let expiration_date = integer_optional_prompt(PROMPT_EXPIRATION_DATE_MESSAGE)?;

            let body = json!({
                "email": email,
                "expiration_date": expiration_date,
                "write": write,
            })
            .to_string();

            match http_client("user/token", Some(&body), Method::PUT).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully user token created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        UserTokenCommands::Value => {
            let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
            let password = password_prompt()?;

            let body = json!({
                "email": email,
                "password": password,
            })
            .to_string();

            match http_client("user/token/value", Some(&body), Method::POST).await {
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
