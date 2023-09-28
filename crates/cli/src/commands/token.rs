use anyhow::Result;
use reqwest::Method;
use serde_json::json;
use tracing::{error, info};

use crate::{
    prompts::{
        confirm_optional_prompt, confirm_prompt, integer_optional_prompt, text_prompt,
        PROMPT_ACTIVE_USER_MESSAGE, PROMPT_EXPIRATION_DATE_MESSAGE, PROMPT_TOKEN_NAME_MESSAGE,
        PROMPT_WRITE_MESSAGE,
    },
    utils::{http_client, json_to_table, line_break},
};

use super::commands::{TokenArgs, TokenCommands};

pub async fn command_token(command: &TokenArgs) -> Result<()> {
    match &command.command {
        TokenCommands::Create => {
            let name = text_prompt(PROMPT_TOKEN_NAME_MESSAGE)?;
            let write = confirm_prompt(PROMPT_WRITE_MESSAGE)?;
            let active = confirm_prompt(PROMPT_ACTIVE_USER_MESSAGE)?;
            let expiration_date = integer_optional_prompt(PROMPT_EXPIRATION_DATE_MESSAGE)?;

            let body = json!({
                "name": name,
                "expiration_date": expiration_date,
                "active": active,
                "write": write,
            })
            .to_string();

            match http_client("token", Some(&body), Method::POST).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully token created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        TokenCommands::Delete => {
            let name = text_prompt(PROMPT_TOKEN_NAME_MESSAGE)?;

            let body = json!({
                "name": name
            })
            .to_string();

            match http_client("token", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully token deleted!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        TokenCommands::List => {
            match http_client("token", None, Method::GET).await {
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
        TokenCommands::Update => {
            let name = text_prompt(PROMPT_TOKEN_NAME_MESSAGE)?;
            let expiration_date = integer_optional_prompt(PROMPT_EXPIRATION_DATE_MESSAGE)?;
            let write = confirm_optional_prompt(PROMPT_WRITE_MESSAGE)?;
            let active = confirm_optional_prompt(PROMPT_ACTIVE_USER_MESSAGE)?;

            let body = json!({
                "name": name,
                "expiration_date": expiration_date,
                "active": active,
                "write": write,
            })
            .to_string();

            match http_client("token", Some(&body), Method::POST).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully token created!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            Ok(())
        }
        TokenCommands::Value => {
            let name = text_prompt(PROMPT_TOKEN_NAME_MESSAGE)?;

            let body = json!({
                "name": name,
            })
            .to_string();

            match http_client("token/value", Some(&body), Method::POST).await {
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
