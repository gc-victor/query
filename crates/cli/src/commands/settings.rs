use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
};

use anyhow::Result;
use inquire::{Confirm, Text};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::{
    config::CLI,
    prompts::{password_prompt, text_prompt, PROMPT_EMAIL_MESSAGE},
    utils::{http_client, read_file_content},
};

#[derive(Deserialize, Serialize)]
struct Config {
    server: ServerOptions,
}

#[derive(Deserialize, Serialize)]
struct ServerOptions {
    url: String,
}

pub async fn command_settings() {
    server_url_prompt().unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });

    get_user_token_value().await.unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });

    save_history_prompt().unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });
}

fn server_url_prompt() -> Result<()> {
    let config_file = &CLI::default().config_file_path;

    let url = Text::new("What is the server URL?").prompt();
    let url = match url {
        Ok(s) => {
            if s.is_empty() {
                return Ok(());
            }

            s
        }
        Err(_) => {
            eprintln!("An error happened when asking for the URL, try again.");
            exit(1)
        }
    };

    let content = if Path::new(config_file).exists() {
        let bytes = read_file_content(config_file)?;
        let content = String::from_utf8(bytes)?;
        let mut config: Config = toml::from_str(&content)?;

        config.server.url = url;

        toml::to_string(&config)?
    } else {
        let s = format!(
            r#"
                [server]
                url = "{url}"
            "#
        );

        // Remove indentation
        s.trim()
            .lines()
            .map(|line| line.trim_start())
            .collect::<Vec<&str>>()
            .join("\n")
    };

    let mut file = File::create(config_file)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

async fn get_user_token_value() -> Result<()> {
    eprintln!("You need to log in to get the token.");

    let email = text_prompt(PROMPT_EMAIL_MESSAGE)?;
    let password = password_prompt()?;

    if email.is_empty() || password.is_empty() {
        return Ok(());
    }

    let body = json!({
        "email": email,
        "password": password,
    })
    .to_string();

    match http_client("user/token/value", Some(&body), Method::POST).await {
        Ok(v) => {
            if v["data"][0].is_null() {
                eprintln!("Error: no data returned. This user doesn't have a token assigned.");
            } else {
                let token = v["data"][0]["token"].as_str().unwrap();

                let config_file = CLI::default().token_file_path;
                let mut file = File::create(config_file)?;
                file.write_all(format!("[default] {token}").as_bytes())?;

                eprintln!("Token has been saved as a default token.");
            }
        }
        Err(err) => error!("{}", err),
    };

    Ok(())
}

pub fn save_history_prompt() -> Result<()> {
    let save_history = Confirm::new("Do you want to save the history of your shell?")
        .with_default(true)
        .prompt()?;

    if save_history {
        File::create(CLI::default().history_file_path)?;
    } else {
        fs::remove_file(CLI::default().history_file_path).ok();
    }

    Ok(())
}
