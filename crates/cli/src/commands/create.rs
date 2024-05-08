use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::exit,
    thread,
};

use anyhow::Result;
use cliclack::{confirm, input, intro, log, outro, select, spinner};
use colored::Colorize;
use openssl::rand::rand_bytes;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{
    config::CLI,
    run_server::run_query_server,
    utils::{
        block_until_server_is_ready, check_port_usage, detect_package_manager, http_client,
        stop_query_server,
    },
};

const APP: &str = "app";
const MINIMAL: &str = "minimal";

#[derive(Debug, Deserialize)]
pub struct Package {
    path: Option<String>,
    url: String,
}

pub async fn command_create() -> Result<()> {
    // ===
    // TODO: Get list of packages with GitHub API
    // https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-contents
    // curl -L \
    //     -H "Accept: application/vnd.github+json" \
    //     -H "X-GitHub-Api-Version: 2022-11-28" \
    //     https://api.github.com/repos/gc-victor/query/contents/packages
    // [{"name": "minimal", "path": "packages/minimal"}]
    // ===

    intro("Create a Query application".to_string().cyan().reversed())?;

    let application = select("Pick an application type")
        .item(APP, "Application", "")
        .item(MINIMAL, "Minimal", "")
        .interact()?;

    let default_path = &format!("./{}-{}", application, &generate_token(2)?);
    let dest: String = input("Where should we create your application?")
        .placeholder(default_path)
        .default_input(default_path)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a path.")
            } else if !input.starts_with("./") {
                Err("Please enter a relative path")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let install_dependencies = confirm("Install dependencies?")
        .initial_value(true)
        .interact()?;

    let install_git_repo = confirm("Initialize a new git repository?")
        .initial_value(true)
        .interact()?;

    fs::create_dir_all(&dest)?;

    let current_dir = std::env::current_dir()?;

    std::env::set_current_dir(&dest)?;

    let default_admin = "admin";
    let email: String = input("Admin email")
        .placeholder(default_admin)
        .default_input(default_admin)
        .interact()?;
    let password: String = input("Admin password")
        .placeholder(default_admin)
        .default_input(default_admin)
        .interact()?;

    // ==

    outro("Application configured")?;

    eprintln!("\n");

    // ===

    intro("Creating your application...".to_string().cyan().reversed())?;

    // ===

    let package = match application {
        MINIMAL => Package {
            path: None,
            url: "https://github.com/gc-victor/query-minimal.git".to_string(),
        },
        _ => Package {
            path: None,
            url: "https://github.com/gc-victor/query-app.git".to_string(),
        },
    };

    let copy_spinner = spinner();

    copy_spinner.start("Coping application...");

    match git_clone(package).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            exit(1);
        }
    };

    copy_spinner.stop("Application copied");

    set_env_vars(&email, &password)?;

    // ===

    let pm = detect_package_manager();
    let npm = pm.npm;

    if install_dependencies {
        let install_spinner = spinner();

        install_spinner.start("Installing dependencies...");

        match Command::new(&npm).arg("install").output().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        }

        install_spinner.stop("Installation completed");
    };

    // ===

    let is_port_used = check_port_usage().is_err();
    let mut has_user_token = false;
    let mut has_executed_create = false;

    if !is_port_used && install_dependencies {
        let admin_spinner = spinner();

        admin_spinner.start("Adding admin user...");

        let server = thread::spawn(move || {
            run_query_server(false, true);
        });

        let _ = server.join();

        let body = json!({
            "email": email,
            "password": password,
        })
        .to_string();

        block_until_server_is_ready();

        if let Ok(v) = http_client("user/token/value", Some(&body), Method::POST).await {
            if !v["data"][0].is_null() {
                let token = v["data"][0]["token"].as_str().unwrap();

                let config_file = CLI::default().token_file_path;
                let mut file = File::create(config_file)?;
                file.write_all(format!("[default] {token}").as_bytes())?;

                has_user_token = true;
            }
        }

        admin_spinner.stop("Admin user created");

        let task_spinner = spinner();

        task_spinner.start(format!(
            "{} Running tasks...",
            "Please wait".yellow().reversed()
        ));

        let package = current_dir
            .join(&dest)
            .join("node_modules")
            .join(".bin")
            .join("query");

        match Command::new(package)
            .args(["task", "create", "-y"])
            .output()
            .await
        {
            Ok(_) => has_executed_create = true,
            Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
        }

        task_spinner.stop("Create task completed");

        stop_query_server();
    }

    // ===

    if install_git_repo {
        match Command::new("git").arg("init").output().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        }

        match Command::new("git").arg("add").arg(".").output().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        }

        match Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Initial commit")
            .output()
            .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        }

        log::step("Git initialized")?;
    }

    // ===

    std::env::set_current_dir(current_dir)?;

    outro("Application created".to_string().green().reversed())?;

    eprintln!("\n");

    // ===

    let go_to = if dest != "./" {
        format!("{} Run `cd {}`\n", String::from('●').green(), dest)
    } else {
        "".to_string()
    };
    let npm_install = if !install_dependencies {
        format!("{} Run `{npm} install`\n", String::from('●').green())
    } else {
        "".to_string()
    };
    let npm_query_settings = if !has_user_token {
        format!("{} Run `{npm} query settings`\n", String::from('●').green())
    } else {
        "".to_string()
    };
    let npm_query_dev = format!("{} Run `{npm} query dev`. {}\n", String::from('●').green(), "It runs a local server in dev mode".to_string().cyan());
    let npm_query_create = if !has_executed_create {
        format!("{} Run in a new terminal `{npm} query create`. {}\n", String::from('●').green(), "It requires a local server running".to_string().yellow().reversed())
    } else {
        "".to_string()
    };
    let git_init = if !install_git_repo {
        format!("{} Run `git init && git add -A && git commit -m \"Initial commit\"` {}\n", String::from('●').green(), "(optional)".to_string().yellow())
    } else {
        "".to_string()
    };
    let visit_home = format!(
        "{} Visit Home: http://localhost:3000\n",
        String::from('●').green()
    );
    let visit_admin = format!(
        "{} Visit Admin: http://localhost:3000/admin\n",
        String::from('●').green()
    );

    let multiline_text = format!(
        "{}{}{}{}{}{}{}{}",
        go_to,
        npm_install,
        npm_query_settings,
        npm_query_dev,
        npm_query_create,
        git_init,
        visit_home,
        visit_admin,
    );

    eprintln!("{}", multiline_text);

    Ok(())
}

pub async fn git_clone(package: Package) -> Result<()> {
    let Package { path, url } = package;

    let error_handler = |e| {
        eprintln!("{}", format!("Error: {e}").red());
        exit(1)
    };

    let response = match reqwest::get(&url).await.map_err(error_handler) {
        Ok(response) => response,
        Err(_) => {
            eprintln!("{}", format!("Error: {} could not be reached", url).red());
            exit(1);
        }
    };

    let status = response.status();

    if !status.is_success() {
        eprintln!("{}", format!("Error: {} {}", url, status).red());
        exit(1);
    }

    Command::new("git").arg("init").output().await?;
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(url)
        .output()
        .await?;

    if path.is_some() {
        Command::new("git")
            .arg("config")
            .arg("core.sparsecheckout")
            .arg("true")
            .output()
            .await?;

        let path = path.as_ref().unwrap();
        let mut file = tokio::fs::File::create(".git/info/sparse-checkout").await?;
        file.write_all(path.as_bytes()).await?;
    }

    Command::new("git")
        .arg("pull")
        .arg("--depth=1")
        .arg("origin")
        .arg("main")
        .output()
        .await?;

    fs::remove_dir_all(".git")?;

    Ok(())
}

fn set_env_vars(new_email: &str, new_password: &str) -> Result<()> {
    if Path::new(".env.dist").exists() {
        fs::copy(".env.dist", ".env")?;
    }

    if Path::new(".env").exists() {
        let contents = fs::read_to_string(".env")?;

        // TODO: remove once the minimal project is moved
        let contents = contents.replace(
            "QUERY_SERVER_TOKEN_SECRET=15acfcfcd8810ea2bbfb1b3bbaff9e9",
            "QUERY_SERVER_TOKEN_SECRET=",
        );
        let contents = contents.replace(
            "QUERY_SERVER_TOKEN_SECRET=",
            &format!("QUERY_SERVER_TOKEN_SECRET={}", generate_token(16)?),
        );
        // TODO: remove once the minimal project is moved
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_EMAIL=admin",
            "QUERY_SERVER_ADMIN_EMAIL=",
        );
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_EMAIL=",
            &format!("QUERY_SERVER_ADMIN_EMAIL={}", new_email),
        );
        // TODO: remove once the minimal project is moved
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_PASSWORD=admin",
            "QUERY_SERVER_ADMIN_PASSWORD=",
        );
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_PASSWORD=",
            &format!("QUERY_SERVER_ADMIN_PASSWORD={}", new_password),
        );

        let mut file = fs::File::create(".env")?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(())
}

fn generate_token(length: usize) -> Result<String> {
    let mut buffer = vec![0; length];
    rand_bytes(&mut buffer)?;

    let token = buffer
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("");

    Ok(token)
}
