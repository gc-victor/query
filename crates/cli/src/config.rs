use std::{
    collections::HashMap,
    env,
    fs::{self, metadata, File},
    io::prelude::*,
    process::exit,
};

#[allow(unused_imports)]
use lazy_static::lazy_static;
#[allow(unused_imports)]
use once_cell::sync::Lazy;
use regex::Regex;
use serde_derive::Deserialize;
use toml;
use tracing::{error, info};

use crate::utils::read_file_content;

#[cfg(not(test))]
lazy_static! {
    pub static ref CONFIG: Config = config();
}

#[cfg(test)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    cli: CLI::default(),
    current_exe: String::new(),
    esbuild: HashMap::new(),
    server: Server::default(),
    structure: Structure::default(),
});

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cli: CLI,
    pub current_exe: String,
    pub esbuild: HashMap<String, String>,
    pub server: Server,
    pub structure: Structure,
}

#[derive(Debug, Deserialize)]
pub struct CLI {
    pub cache_file_path: String,
    pub config_file_path: String,
    pub history_file_path: String,
    pub plugin_file_path: String,
    pub token_file_path: String,
    pub token: String,
}

impl Default for CLI {
    fn default() -> Self {
        CLI {
            cache_file_path: ".query/.cache".to_string(),
            config_file_path: ".query/Query.toml".to_string(),
            history_file_path: ".query/.history".to_string(),
            plugin_file_path: ".query/plugins.toml".to_string(),
            token_file_path: ".query/.token".to_string(),
            token: String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            url: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Structure {
    pub functions_folder: String,
    pub migrations_folder: String,
    pub templates_folder: String,
    pub plugins_folder: String,
}

impl Default for Structure {
    fn default() -> Self {
        Structure {
            functions_folder: "src/functions".to_string(),
            migrations_folder: "src/migrations".to_string(),
            templates_folder: "templates".to_string(),
            plugins_folder: "plugins".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct InnerConfig {
    pub cli: Option<InnerCLI>,
    pub esbuild: Option<HashMap<String, String>>,
    pub server: InnerServer,
    pub structure: Option<InnerStructure>,
}

#[derive(Debug, Default, Deserialize)]
struct InnerCLI {
    pub cache_file_path: Option<String>,
    pub history_file_path: Option<String>,
    pub plugin_file_path: Option<String>,
    pub token_file_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InnerServer {
    pub url: String,
}

impl Default for InnerServer {
    fn default() -> Self {
        InnerServer {
            url: "".to_string(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct InnerStructure {
    pub functions_folder: Option<String>,
    pub migrations_folder: Option<String>,
    pub templates_folder: Option<String>,
    pub plugins_folder: Option<String>,
}

pub fn config() -> Config {
    let config_file_path = CLI::default().config_file_path;

    let contents = match fs::read_to_string(&config_file_path) {
        Ok(contents) => contents,
        Err(_) => {
            info!("No config file found");
            exit(1);
        }
    };

    let inner_config: InnerConfig = match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    };

    let inner_config_cli = inner_config.cli.unwrap_or_default();
    let token_file_path = inner_config_cli
        .token_file_path
        .unwrap_or(CLI::default().token_file_path);
    let token = env::var("QUERY_PRIVATE_TOKEN").unwrap_or_else(|_| "".to_string());
    let cli = CLI {
        cache_file_path: inner_config_cli
            .cache_file_path
            .unwrap_or(CLI::default().cache_file_path),
        config_file_path,
        history_file_path: inner_config_cli
            .history_file_path
            .unwrap_or(CLI::default().history_file_path),
        plugin_file_path: inner_config_cli
            .plugin_file_path
            .unwrap_or(CLI::default().plugin_file_path),
        token_file_path: token_file_path.to_owned(),
        token: if !token.is_empty() {
            token
        } else {
            match read_file_content(&token_file_path.to_string()) {
                Ok(token) => {
                    let token: &str = std::str::from_utf8(&token).unwrap();
                    let re = Regex::new(r"\[.*\](.*)").unwrap();
                    let caps = re.captures(token).unwrap();
                    caps.get(1).unwrap().as_str().trim().to_string()
                }
                Err(_) => "".to_string(),
            }
        },
    };

    if metadata(&token_file_path).is_err() {
        let mut file = File::create(&token_file_path).unwrap();
        file.write_all(b"[default]").unwrap();
    }

    let inner_config_server = inner_config.server;
    let server = Server {
        url: inner_config_server.url,
    };

    let inner_config_structure = inner_config.structure.unwrap_or_default();
    let structure = Structure {
        functions_folder: inner_config_structure
            .functions_folder
            .unwrap_or(Structure::default().functions_folder),
        migrations_folder: inner_config_structure
            .migrations_folder
            .unwrap_or(Structure::default().migrations_folder),
        templates_folder: inner_config_structure
            .templates_folder
            .unwrap_or(Structure::default().templates_folder),
        plugins_folder: inner_config_structure
            .plugins_folder
            .unwrap_or(Structure::default().plugins_folder),
    };

    let esbuild = inner_config.esbuild.unwrap_or_default();

    let mut current_exe = env::current_exe().unwrap();
    current_exe.pop();
    let current_exe = current_exe.display().to_string();

    Config {
        cli,
        current_exe,
        esbuild,
        server,
        structure,
    }
}
