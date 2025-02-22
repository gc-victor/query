use std::{
    env,
    io::{self, BufRead},
    process::{exit, Command, Stdio},
    thread,
};

use anyhow::Result;
use cliclack::outro;
use cliclack::{input, intro, password};
use colored::Colorize;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::http_client;

use super::commands::DeployArgs;

#[derive(Deserialize, Serialize)]
struct Config {
    server: ServerOptions,
}

#[derive(Deserialize, Serialize)]
struct ServerOptions {
    url: String,
}

const QUERY_DEPLOY_URL: &str = "QUERY_DEPLOY_URL";
const QUERY_DEPLOY_TOKEN: &str = "QUERY_DEPLOY_TOKEN";
const QUERY_DEPLOY_EMAIL: &str = "QUERY_DEPLOY_EMAIL";
const QUERY_DEPLOY_PASSWORD: &str = "QUERY_DEPLOY_PASSWORD";
const QUERY_SERVER_ADMIN_EMAIL: &str = "QUERY_SERVER_ADMIN_EMAIL";
const QUERY_SERVER_PORT: &str = "QUERY_SERVER_PORT";

pub async fn command_deploy(command: &DeployArgs) -> Result<()> {
    let env_deploy_url = env::var(QUERY_DEPLOY_URL).unwrap_or("".to_string());
    let env_deploy_token = env::var(QUERY_DEPLOY_TOKEN).unwrap_or("".to_string());
    let env_deploy_email = env::var(QUERY_DEPLOY_EMAIL).unwrap_or("".to_string());
    let env_deploy_password = env::var(QUERY_DEPLOY_PASSWORD).unwrap_or("".to_string());

    let has_url = env_deploy_url.is_empty();
    let has_token = env_deploy_token.is_empty();
    let has_email = env_deploy_email.is_empty();
    let has_password = env_deploy_password.is_empty();

    let command_env = command.env;
    if command_env {
        if has_url {
            eprintln!("Please, set the environment variable QUERY_DEPLOY_URL.");
            exit(1);
        }

        if has_token && has_email {
            eprintln!("Please, set the environment variable QUERY_DEPLOY_EMAIL.");
            exit(1);
        }

        if has_token && has_password {
            eprintln!("Please, set the environment variable QUERY_DEPLOY_PASSWORD.");
            exit(1);
        }
    }

    let is_prompt_required = (has_url && has_token) || (has_url && has_email && has_password);

    if is_prompt_required {
        intro("Query Deploy".to_string().cyan().reversed())?;
    } else {
        eprintln!("{} Query Deploying...", String::from('●').cyan());
    }

    if has_url {
        let url = &server_url_prompt().unwrap_or_else(|err| {
            eprintln!("{} {}", String::from('●').red(), err);
            exit(1);
        });

        unsafe {
            env::set_var(QUERY_DEPLOY_URL, url);
        }
    };

    if has_token {
        let token = get_user_token_value().await.unwrap_or_else(|err| {
            eprintln!("{} {}", String::from('●').red(), err);
            exit(1);
        });
        
        unsafe {
            env::set_var(QUERY_DEPLOY_TOKEN, token);
        }
    };

    let url = env::var(QUERY_DEPLOY_URL).unwrap_or("".to_string());
    let token = env::var(QUERY_DEPLOY_TOKEN).unwrap_or("".to_string());

    if command.no_cache {
        eprintln!("{} Cache removed", String::from('●').cyan());
        execute_command("rm -rf .query/.cache")?;
    }

    let current_exe = env::current_exe()?;

    execute_command(&format!(
        "export QUERY_DEPLOY_TOKEN={} && export QUERY_DEPLOY_URL={} && {} task deploy",
        token,
        url,
        current_exe.display()
    ))?;

    if is_prompt_required {
        outro("Deploy completed".to_string().cyan().reversed())?;
    } else {
        eprintln!("{} Deploy completed", String::from('●').cyan());
    }

    Ok(())
}

fn server_url_prompt() -> Result<String> {
    let port = env::var(QUERY_SERVER_PORT).unwrap_or("3000".to_string());
    let default_local_url = &format!("http://localhost:{}", port);
    let url: String = input("Server URL:")
        .placeholder(default_local_url)
        .default_input(default_local_url)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a URL.")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(url)
}

async fn get_user_token_value() -> Result<String> {
    let env_deploy_email = env::var(QUERY_DEPLOY_EMAIL).unwrap_or("".to_string());
    let default_email = &env::var(QUERY_SERVER_ADMIN_EMAIL).unwrap_or("".to_string());
    let email: String = if !env_deploy_email.is_empty() {
        env_deploy_email.clone()
    } else {
        input("Email:")
            .placeholder(default_email)
            .default_input(default_email)
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter an email.")
                } else {
                    Ok(())
                }
            })
            .interact()?
    };

    let env_deploy_password = env::var(QUERY_DEPLOY_PASSWORD).unwrap_or("".to_string());
    let password = if !env_deploy_password.is_empty() {
        env_deploy_password.clone()
    } else {
        password("Password:")
            .mask('▪')
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a password.")
                } else {
                    Ok(())
                }
            })
            .interact()?
    };

    let body = json!({
        "email": email,
        "password": password,
    })
    .to_string();

    let token = match http_client("user/token/value", Some(&body), Method::POST).await {
        Ok(v) => {
            if v["data"][0].is_null() {
                outro(
                    "There is an error retrieving the token. Please, review user and password."
                        .to_string()
                        .red()
                        .reversed(),
                )?;
                exit(1);
            } else {
                let token: &str = v["data"][0]["token"].as_str().unwrap();

                token.to_string()
            }
        }
        Err(e) => {
            outro(e.to_string().red().reversed())?;
            exit(1);
        }
    };

    Ok(token.to_string())
}

fn execute_command(command: &str) -> Result<()> {
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }?;

    let stdout_thread = spawn_output_thread(child.stdout.take().unwrap());
    let stderr_thread = spawn_output_thread(child.stderr.take().unwrap());

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}

fn spawn_output_thread(output: impl io::Read + Send + 'static) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let reader = io::BufReader::new(output);
        for line in reader.lines().map_while(Result::ok) {
            let message = line
                .trim()
                .trim_matches('"')
                .replace('●', "")
                .replace("\\n\\n", "\n")
                .replace("\\n", "\n")
                .trim()
                .to_string();
            if !message.is_empty() {
                eprintln!("{} {}", String::from('●').cyan(), message);
            }
        }
    })
}
