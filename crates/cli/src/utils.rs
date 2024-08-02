use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    net::TcpStream,
    path::Path,
    process::{exit, Command, Output},
    time::Duration,
};

use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Method, Url,
};
use serde_json::{self, Value};
use tabled::{builder::Builder, settings::Style};

use crate::config::CONFIG;

pub fn read_file_content(file_path: &str) -> Result<Vec<u8>> {
    let file = File::open(file_path)?;

    let file = &file;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    Ok(content.as_bytes().to_vec())
}

pub fn line_break() {
    eprintln!("\n");
}

pub fn json_to_table(value: &Value) -> Result<String> {
    let mut builder = Builder::default();

    if value.as_array().is_none() {
        return Ok(String::new());
    }

    let array = value.as_array().unwrap();

    if array.first().is_none() {
        return Ok(String::new());
    }
    let first_object = array.first().unwrap();
    let keys = first_object.as_object().unwrap().keys();

    let keys = keys.map(|key| {
        let key = key.to_string();
        key.to_uppercase()
    });

    builder.push_record(keys);

    for object in array {
        let values = object.as_object().unwrap().values();
        let values = values.map(|value| value.to_string());
        builder.push_record(values);
    }

    let mut table = builder.build();

    table
        .with(Style::markdown().vertical(' ').remove_left().remove_right())
        .to_string();

    Ok(table.to_string())
}

pub async fn http_client(path: &str, body: Option<&String>, method: Method) -> Result<Value> {
    let config_url = &CONFIG.server.url;
    let config_url = if !config_url.ends_with('/') {
        format!("{}/", config_url)
    } else {
        config_url.to_owned()
    };
    let url = &format!("{}_/{}", config_url, path);
    let url = Url::parse(url)?;

    let token: &str = CONFIG.cli.token.as_str();

    let mut headers = HeaderMap::new();

    if !token.is_empty() {
        headers.insert(
            HeaderName::from_lowercase(b"authorization")?,
            HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
        );
    }

    let body = Body::from(match body {
        Some(body) => body.as_str().to_string(),
        None => String::new(),
    });

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()?;

    let response = client
        .request(method, url)
        .headers(headers)
        .body(body)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        if body.is_empty() {
            return Err(anyhow!("{}", status));
        } else {
            return Err(anyhow!("{}", body));
        }
    }

    if body.is_empty() {
        return Ok(Value::Null);
    }

    let value: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(_) => Value::String(body),
    };

    Ok(value)
}

// CREDIT: https://github.com/cloudflare/workers-sdk/blob/235c4398268322b6c0c13060bc3da91f52b4b066/packages/create-cloudflare/src/helpers/packageManagers.ts#L1
#[derive(Debug)]
pub enum PmName {
    Pnpm,
    Npm,
    Yarn,
    Bun,
}

#[derive(Debug)]
pub struct PackageManager {
    pub dlx: String,
    pub lock: String,
    pub name: PmName,
    pub npm: String,
    pub npx: String,
}

pub fn detect_package_manager() -> PackageManager {
    let pnpm_pm = PackageManager {
        dlx: "pnpm dlx".to_string(),
        lock: "pnpm-lock.yaml".to_string(),
        name: PmName::Pnpm,
        npm: "pnpm".to_string(),
        npx: "pnpm".to_string(),
    };
    let yarn_pm = PackageManager {
        dlx: "yarn dlx".to_string(),
        lock: "yarn.lock".to_string(),
        name: PmName::Yarn,
        npm: "yarn".to_string(),
        npx: "yarn".to_string(),
    };
    let bun_pm = PackageManager {
        dlx: "bunx".to_string(),
        lock: "bun.lockb".to_string(),
        name: PmName::Bun,
        npm: "bun".to_string(),
        npx: "bunx".to_string(),
    };
    let npm_pm = PackageManager {
        dlx: "npx".to_string(),
        lock: "package-lock.json".to_string(),
        name: PmName::Npm,
        npm: "npm".to_string(),
        npx: "npx".to_string(),
    };

    if let Ok(pm_info) = env::var("npm_config_user_agent") {
        let name = pm_info.split('/').next().unwrap_or("npm");

        match name {
            "pnpm" => return pnpm_pm,
            "yarn" => return yarn_pm,
            "bun" => return bun_pm,
            _ => return npm_pm,
        }
    }

    let current_dir = env::current_dir().unwrap();

    if current_dir.join(&npm_pm.lock).exists() && is_installed(&npm_pm.npm) {
        return npm_pm;
    }

    if current_dir.join(&yarn_pm.lock).exists() && is_installed(&yarn_pm.npm) {
        return yarn_pm;
    }

    if current_dir.join(&pnpm_pm.lock).exists() && is_installed(&pnpm_pm.npm) {
        return pnpm_pm;
    }

    if current_dir.join(&bun_pm.lock).exists() && is_installed(&bun_pm.npm) {
        return bun_pm;
    }

    npm_pm
}

pub fn has_node_modules_binary(package: &str) -> bool {
    Path::new("node_modules")
        .join(".bin")
        .join(package)
        .exists()
}

pub fn install_dependencies(dependencies: Vec<&str>) -> Result<Output, std::io::Error> {
    let pm = detect_package_manager();

    Command::new(pm.npm)
        .arg("install")
        .args(dependencies)
        .output()
}

#[cfg(not(windows))]
pub fn which(program: &str) -> Option<String> {
    if is_installed(program) {
        let output = match Command::new("which").arg(program).output() {
            Ok(output) => output,
            Err(_) => return None,
        };

        return match String::from_utf8(output.stdout) {
            Ok(output) => Some(output.trim().to_string()),
            Err(_) => None,
        };
    }

    None
}

#[cfg(windows)]
pub fn which(program: &str) -> Option<String> {
    if is_installed(program) {
        let output = match Command::new("cmd").args(&["/C", "where", program]).output() {
            Ok(output) => output,
            Err(_) => return None,
        };

        return match String::from_utf8(output.stdout) {
            Ok(output) => Some(output.trim().to_string()),
            Err(_) => None,
        };
    }

    None
}

#[cfg(not(windows))]
pub fn is_installed(program: &str) -> bool {
    Command::new("which")
        .arg(program)
        .output()
        .unwrap()
        .status
        .success()
}

#[cfg(windows)]
pub fn is_installed(program: &str) -> bool {
    Command::new("cmd")
        .args(&["/C", "where", program])
        .output()
        .unwrap()
        .status
        .success()
}

pub fn check_port_usage() -> Result<(), String> {
    let query_server_port: String = env::var("QUERY_SERVER_PORT").unwrap_or("3000".to_owned());
    if TcpStream::connect(format!("0.0.0.0:{}", query_server_port)).is_ok() {
        Err(format!(
            r#"Something is running on port {}. Please, stop it before running the command."#,
            query_server_port
        ))
    } else {
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub fn stop_query_server() {
    match Command::new("pkill").arg("-f").arg("query-server").spawn() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please, stop the query-server process manually");
            exit(1);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn stop_query_server() {
    match Command::new("taskkill")
        .arg("-f")
        .arg("-im")
        .arg("query-server*")
        .spawn()
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Please, stop the query-server process manually");
            exit(1);
        }
    }
}

pub fn block_until_server_is_ready() {
    let query_server_port: String = env::var("QUERY_SERVER_PORT").unwrap_or("3000".to_owned());
    for _ in 0..100 {
        if TcpStream::connect(format!("0.0.0.0:{}", query_server_port)).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
