use std::{env, fs::File, io::Write, path::Path, process::exit, thread};

use anyhow::Result;
use cliclack::outro;
use cliclack::{input, intro, password};
use colored::Colorize;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use toml_edit::{value, DocumentMut};
use tracing::error;

use crate::{
    config::CLI,
    run_server::run_query_server,
    utils::{
        block_until_server_is_ready, check_port_usage, http_client, read_file_content,
        stop_query_server,
    },
};

#[derive(Deserialize, Serialize)]
struct Config {
    server: ServerOptions,
}

#[derive(Deserialize, Serialize)]
struct ServerOptions {
    url: String,
}

pub async fn command_settings() -> Result<()> {
    intro("Query - Set Server Settings".to_string().cyan().reversed())?;

    server_url_prompt().unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });

    get_user_token_value().await.unwrap_or_else(|err| {
        error!("{}", err);
        exit(1);
    });

    outro("Server Settings Configured".to_string().green().reversed())?;

    Ok(())
}

fn server_url_prompt() -> Result<()> {
    let config_file = &CLI::default().config_file_path;

    let default_url = "http://localhost:3000";
    let url: String = input("Server URL:")
        .placeholder(default_url)
        .default_input(default_url)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a URL.")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let content = if Path::new(config_file).exists() {
        let bytes = read_file_content(config_file)?;
        let content = String::from_utf8(bytes)?;
        let mut doc = content.parse::<DocumentMut>()?;

        doc["server"]["url"] = value(url);

        doc.to_string()
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
    let default_email = &env::var("QUERY_SERVER_ADMIN_EMAIL")?;
    let email: String = input("Email:")
        .placeholder(default_email)
        .default_input(default_email)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter an email.")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let password = password("Password:")
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

    match check_port_usage() {
        Ok(_) => (),
        Err(e) => {
            outro(e.to_string().yellow().reversed())?;
            exit(1);
        }
    }

    let server = thread::spawn(move || {
        run_query_server(false, true);
    });

    block_until_server_is_ready();

    match http_client("user/token/value", Some(&body), Method::POST).await {
        Ok(v) => {
            if v["data"][0].is_null() {
                stop_query_server();
                outro(
                    "There is an error retrieving the token. Please, review user and password."
                        .to_string()
                        .red()
                        .reversed(),
                )?;
                exit(1);
            } else {
                let token = v["data"][0]["token"].as_str().unwrap();

                let config_file = CLI::default().token_file_path;
                let mut file = File::create(config_file)?;
                file.write_all(format!("[default] {token}").as_bytes())?;
            }
        }
        Err(e) => {
            stop_query_server();
            outro(e.to_string().red().reversed())?;
            exit(1);
        }
    };

    let _ = server.join();

    stop_query_server();

    Ok(())
}
