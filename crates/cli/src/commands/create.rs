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
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client, Method,
};
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

use super::commands::CreateArgs;

#[derive(Debug, Deserialize)]
pub struct Package {
    name: String,
    path: String,
}

type Packages = Vec<Package>;

static PROJECTS_FOLDER: &str = "examples";

pub async fn command_create(command: &CreateArgs) -> Result<()> {
    let repo_url = command.repo_url.clone();

    // Init prompt

    intro("Create a Query application".to_string().cyan().reversed())?;

    let packages: Packages = if repo_url.is_none() {
        fetch_packages().await?
    } else {
        vec![]
    };
    let application = if let Some(url) = &repo_url {
        url.split('/').last().unwrap().to_string()
    } else {
        let items: Vec<(String, String, String)> = packages
            .iter()
            .map(|p| (p.name.clone(), p.name.clone(), "".to_string()))
            .collect();
        let items = &items[..];

        select("Pick an application type").items(items).interact()?
    };

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

    let initialize_git_repo = confirm("Initialize a new git repository?")
        .initial_value(true)
        .interact()?;

    let default_admin = "admin";
    let email: String = input("Admin email")
        .placeholder(default_admin)
        .default_input(default_admin)
        .interact()?;
    let password: String = input("Admin password")
        .placeholder(default_admin)
        .default_input(default_admin)
        .interact()?;

    outro("Application configured")?;

    eprintln!("\n");

    // Create the destination directory

    fs::create_dir_all(&dest)?;

    let current_dir = std::env::current_dir()?;

    std::env::set_current_dir(&dest)?;

    // Pull the selected repository

    intro("Creating your application...".to_string().cyan().reversed())?;

    let copy_spinner = spinner();

    copy_spinner.start("Cloning application...");

    if let Some(url) = repo_url {
        match git_clone_full(&url).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        };
    } else {
        let package = packages.iter().find(|p| p.name == application).unwrap();
        match git_clone(package).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                exit(1);
            }
        };
    }

    copy_spinner.stop("Application cloned");

    // Set environment variables

    let new_email: &str = &email;
    let new_password: &str = &password;
    if Path::new(".env.dist").exists() {
        fs::copy(".env.dist", ".env")?;
    }

    if Path::new(".env").exists() {
        let contents = fs::read_to_string(".env")?;

        let contents = contents.replace(
            "QUERY_SERVER_TOKEN_SECRET=",
            &format!("QUERY_SERVER_TOKEN_SECRET={}", generate_token(16)?),
        );
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_EMAIL=",
            &format!("QUERY_SERVER_ADMIN_EMAIL={}", new_email),
        );
        let contents = contents.replace(
            "QUERY_SERVER_ADMIN_PASSWORD=",
            &format!("QUERY_SERVER_ADMIN_PASSWORD={}", new_password),
        );

        let mut file = fs::File::create(".env")?;
        file.write_all(contents.as_bytes())?;
    }

    // Install dependencies

    let pm = detect_package_manager();
    let npm = pm.npm;
    let npx = pm.npx;

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

    // Get user token

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

    // Initialize git repository

    if initialize_git_repo {
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

    // Go back to the original directory

    std::env::set_current_dir(current_dir)?;

    outro("Application created".to_string().green().reversed())?;

    eprintln!("\n");

    // Notify user about next steps

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
        format!("{} Run `{npx} query settings`\n", String::from('●').green())
    } else {
        "".to_string()
    };
    let npm_query_dev = format!(
        "{} Run `{npx} query dev`. {}\n",
        String::from('●').green(),
        "It runs a local server in dev mode".to_string().cyan()
    );
    let npm_query_create = if !has_executed_create {
        format!(
            "{} Run in a new terminal `{npx} query task create -y`. {}\n",
            String::from('●').green(),
            "It requires a local server running"
                .to_string()
                .yellow()
                .reversed()
        )
    } else {
        "".to_string()
    };
    let git_init = if !initialize_git_repo {
        format!(
            "{} Run `git init && git add -A && git commit -m \"Initial commit\"` {}\n",
            String::from('●').green(),
            "(optional)".to_string().yellow()
        )
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

async fn fetch_packages() -> Result<Packages> {
    // NOTICE: Possible rate limit issues if more than 60 requests per hour
    // curl 'https://api.github.com/rate_limit'
    // @see: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28#staying-under-the-rate-limit
    let url = "https://api.github.com/repos/gc-victor/query/contents/".to_owned() + PROJECTS_FOLDER;

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("X-GitHub-Api-Version: 2022-11-28"),
    );

    let client = Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    let text = response.text().await?;
    let json: Packages = serde_json::from_str(&text)?;

    let packages: Packages = json
        .iter()
        .filter(|github_package| github_package.name != "proxy")
        .map(|github_package: &Package| Package {
            name: github_package.name.clone(),
            path: github_package.path.clone(),
        })
        .collect();

    Ok(packages)
}

async fn git_clone(package: &Package) -> Result<()> {
    let Package { name: _, path } = package;
    let url = "https://github.com/gc-victor/query.git";

    let error_handler = |e| {
        eprintln!("{}", format!("Error: {e}").red());
        exit(1)
    };

    let response = match reqwest::get(url).await.map_err(error_handler) {
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

    Command::new("git")
        .arg("config")
        .arg("core.sparsecheckout")
        .arg("true")
        .output()
        .await?;

    let mut file = tokio::fs::File::create(".git/info/sparse-checkout").await?;
    file.write_all(path.as_bytes()).await?;

    Command::new("git")
        .arg("pull")
        .arg("--depth=1")
        .arg("origin")
        .arg("main")
        .output()
        .await?;

    fs::remove_dir_all(".git")?;

    let from_dir = Path::new(path);
    let to_dir = Path::new(".");
    let entries = fs::read_dir(from_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        // If the path is a normal file, this is the file name.
        // If it's the path of a directory, this is the directory name
        let file_name = match path.file_name() {
            Some(file_name) => file_name,
            None => continue,
        };

        let mut new_path = to_dir.to_path_buf();
        new_path.push(file_name);

        fs::rename(path, new_path)?;
    }

    fs::remove_dir_all(PROJECTS_FOLDER)?;

    Ok(())
}

async fn git_clone_full(repo_url: &str) -> Result<()> {
    let error_handler = |e| {
        eprintln!("{}", format!("Error: {e}").red());
        exit(1)
    };

    let response = match reqwest::get(repo_url).await.map_err(error_handler) {
        Ok(response) => response,
        Err(_) => {
            eprintln!(
                "{}",
                format!("Error: {} could not be reached", repo_url).red()
            );
            exit(1);
        }
    };

    let status = response.status();

    if !status.is_success() {
        eprintln!("{}", format!("Error: {} {}", repo_url, status).red());
        exit(1);
    }

    Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--single-branch",
            "--branch",
            "main",
            repo_url,
            ".",
        ])
        .output()
        .await?;

    fs::remove_dir_all(".git")?;

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
