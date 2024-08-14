use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::exit,
    sync::LazyLock,
    vec,
};

use anyhow::Result;
use openssl::sha::Sha256;
use reqwest::{Method, StatusCode};
use serde::Deserialize;
use serde_json::json;
use toml_edit::{value, ArrayOfTables, DocumentMut, Item, Table};

use crate::{
    cache::{Cache, CacheItem},
    config::CONFIG,
    utils::http_client,
};

use super::commands::{PluginArgs, PluginPushArgs};

static PLUGINS_TOML_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    env::current_dir()
        .unwrap()
        .join(&CONFIG.cli.plugin_file_path)
});

static PLUGINS_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    env::current_dir()
        .unwrap()
        .join(&CONFIG.structure.plugins_folder)
});

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiAsset {
    pub browser_download_url: String,
    pub content_type: String,
    pub name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
// https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28
pub struct GitHubApiRelease {
    pub assets: Vec<GitHubApiAsset>,
    pub tag_name: String,
    pub url: String,
    pub prerelease: bool,
    pub published_at: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct ReleaseInfo {
    pub name: String,
    pub tag: String,
    pub url: String,
    pub sha256_url: Option<String>,
    pub repo_url: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginDoc {
    pub name: String,
    pub tag: String,
    pub url: String,
    pub sha256: Option<String>,
    pub published_at: Option<String>,
}

pub async fn command_plugin(command: &PluginArgs) {
    let command = &command.command;

    match command {
        super::commands::PluginCommands::Install(args) => {
            let releases_info = get_release_info(args.github_repo_url.as_str(), vec![])
                .await
                .unwrap_or_else(|err| {
                    eprintln!("Error: Failed to get the release info: {}", err);
                    exit(1);
                });

            match update_plugins(releases_info).await {
                Ok(_) => eprintln!("Successfully installed the plugin"),
                Err(err) => eprintln!("Error: Failed to install the plugin: {}", err),
            };
        }
        super::commands::PluginCommands::Update => {
            update_command().await.unwrap_or_else(|err| {
                eprintln!("Error: Failed to update the plugins: {}", err);
            });
        }
        super::commands::PluginCommands::Push(args) => {
            match push_command(args).await {
                Ok(_) => (),
                Err(err) => eprintln!("Error: Failed to push the plugin: {}", err),
            };
        }
    }
}

// CREDIT: https://github.com/moonrepo/proto/blob/1b1dabb440089dc899aca107bba71fa6888015ef/crates/warpgate/src/loader.rs#L336
// @see: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-the-latest-release
// @see: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-a-release-by-tag-name
async fn get_release_info(github_repo_url: &str, exclude: Vec<String>) -> Result<Vec<ReleaseInfo>> {
    let repo_slug = github_repo_url.split("/").collect::<Vec<&str>>();
    let repo_slug = repo_slug[repo_slug.len() - 2..].join("/");
    let tag = github_repo_url.split("@").nth(1).unwrap_or("").to_string();

    let url = if tag.is_empty() {
        format!("https://api.github.com/repos/{}/releases/latest", repo_slug)
    } else {
        format!(
            "https://api.github.com/repos/{}/releases/tags/{}",
            repo_slug.replace(&format!("@{tag}"), ""),
            tag
        )
    };

    let client = reqwest::Client::new();
    let mut request = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "request");

    // @see: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
    // @see: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token
    if let Ok(auth_token) = env::var("GITHUB_TOKEN") {
        request = request.bearer_auth(auth_token);
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Failed to fetch the release info: {}", err);
            exit(1);
        }
    };
    let response = response.json::<serde_json::Value>().await?;
    let release: GitHubApiRelease = serde_json::from_value(response)?;

    let mut asset_map: HashMap<String, ReleaseInfo> = HashMap::new();
    let mut sha256_map: HashMap<String, String> = HashMap::new();

    for asset in release.assets.clone() {
        if asset.name.ends_with(".wasm.sha256") && !exclude.contains(&asset.name) {
            let base_name = asset.name.trim_end_matches(".sha256").to_string();
            sha256_map.insert(base_name, asset.browser_download_url.clone());
        }
    }

    for asset in release.assets {
        if asset.content_type == "application/wasm"
            || asset.name.ends_with(".wasm") && !exclude.contains(&asset.name)
        {
            let base_name = asset.name.clone();

            let release = asset_map
                .entry(base_name.clone())
                .or_insert_with(|| ReleaseInfo {
                    name: base_name.clone(),
                    tag: release.tag_name.clone(),
                    url: asset.browser_download_url.clone(),
                    sha256_url: None,
                    repo_url: github_repo_url.to_string(),
                    published_at: release.published_at.clone(),
                });

            if let Some(sha256_url) = sha256_map.get(&base_name) {
                release.sha256_url = Some(sha256_url.clone());
            }
        }
    }

    Ok(asset_map.into_values().collect())
}

async fn update_plugins(release_info: Vec<ReleaseInfo>) -> Result<()> {
    let plugin_file_path = &PLUGINS_TOML_FILE_PATH.clone();
    if !plugin_file_path.exists() {
        fs::write(plugin_file_path, "")?;
    }

    let plugin_content = fs::read_to_string(plugin_file_path)?;
    let mut toml_document = plugin_content.parse::<DocumentMut>()?;
    let plugins = match toml_document["plugin"].as_array_of_tables_mut() {
        Some(plugins) => plugins,
        None => &mut ArrayOfTables::default(),
    };

    let plugins_name = plugins
        .iter()
        .map(|plugin| plugin["name"].to_string().trim().to_string())
        .collect::<Vec<String>>();

    for release in release_info {
        let release_url = release.url.as_str();
        let release_sha256_url = release.sha256_url.unwrap_or_default();
        let release_sha256_url = release_sha256_url.as_str();

        let name = value(release.name.to_string());
        let tag = value(release.tag.to_string());
        let url = value(release.url.to_string());
        let repo_url = value(release.repo_url.to_string());
        let published_at = value(release.published_at.clone().unwrap_or("".to_string()));

        if plugins.get(0).is_none() {
            let sha256 = download_plugin(release_url, release_sha256_url, &release.name).await?;

            let mut table = Table::new();
            table["name"] = name;
            table["tag"] = tag;
            table["url"] = url;
            table["sha256"] = sha256;
            table["repo_url"] = repo_url;
            table["published_at"] = published_at;

            plugins.push(table);

            eprintln!("Successfully installed the plugin {}", release_url);

            continue;
        }

        let mut updated_plugins = vec![];

        if plugins_name.contains(&name.to_string()) {
            for plugin in plugins.iter_mut() {
                if plugin["name"].as_str() == name.as_str()
                    && plugin["tag"].as_str() != tag.as_str()
                {
                    let sha256 =
                        download_plugin(release_url, release_sha256_url, &release.name).await?;

                    plugin["tag"] = tag;
                    plugin["url"] = url;
                    plugin["sha256"] = sha256;
                    plugin["repo_url"] = repo_url;
                    plugin["published_at"] = published_at;

                    eprintln!("Successfully updated the plugin {}", release_url);

                    break;
                }
            }
        } else {
            let sha256 = download_plugin(release_url, release_sha256_url, &release.name).await?;

            let mut table = Table::new();
            table["name"] = name;
            table["tag"] = tag;
            table["url"] = url;
            table["sha256"] = sha256;
            table["repo_url"] = repo_url;
            table["published_at"] = published_at;

            updated_plugins.push(table);

            eprintln!("Successfully installed the plugin {}", release_url);
        }

        plugins.extend(updated_plugins);
    }

    toml_document["plugin"] = Item::ArrayOfTables(plugins.clone());

    fs::write(
        PLUGINS_TOML_FILE_PATH.clone(),
        toml_document.to_string().trim(),
    )?;

    Ok(())
}

async fn download_plugin(url: &str, sha256_url: &str, name: &str) -> Result<Item> {
    let wasm = get_file_content(url).await?;
    let content: &[u8] = &wasm;
    let plugins_folder = PLUGINS_FOLDER.clone();

    if !plugins_folder.exists() {
        fs::create_dir_all(&plugins_folder)?;
    }

    let file_path = plugins_folder.join(name);
    let mut file = File::create(file_path)?;
    file.write_all(content)?;

    let sha256 = if !sha256_url.is_empty() {
        if sha256_url.to_string().is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(content);
            let hash = hasher.finish();
            hash.iter().fold(String::new(), |mut acc, byte| {
                acc.push_str(&format!("{:02x}", byte));
                acc
            })
        } else {
            let sha256_content = get_file_content(sha256_url).await?;
            match String::from_utf8(sha256_content) {
                Ok(sha256) => sha256,
                Err(err) => {
                    eprintln!("Failed to convert the sha256 content to string: {}", err);
                    String::new()
                }
            }
        }
    } else {
        String::new()
    };

    Ok(value(sha256.trim()))
}

// https://github.com/moonrepo/proto/blob/1b1dabb440089dc899aca107bba71fa6888015ef/crates/warpgate/src/loader.rs#L217
// https://github.com/moonrepo/starbase/blob/9e4e07ce2ae0014a5855551fe7413ea586e3bf8b/crates/utils/src/net.rs#L16
async fn get_file_content(url: &str) -> Result<Vec<u8>> {
    let response = match reqwest::get(url.to_string()).await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Failed to send the request: {}", err);
            exit(1);
        }
    };

    if response.status() == StatusCode::NOT_FOUND {
        eprintln!("File {} not found", url);
        exit(1);
    }

    if !response.status().is_success() {
        eprintln!("Failed to download the file {}", url);
        exit(1);
    }

    Ok(response.bytes().await?.to_vec())
}

async fn update_command() -> Result<()> {
    let plugin_file_path = &PLUGINS_TOML_FILE_PATH.clone();

    if !plugin_file_path.exists() {
        fs::write(plugin_file_path, "")?;
    }

    let plugin_content = fs::read_to_string(plugin_file_path)?;
    let mut toml_document = plugin_content.parse::<DocumentMut>()?;
    let plugins = match toml_document["plugin"].as_array_of_tables_mut() {
        Some(plugins) => plugins,
        None => &mut ArrayOfTables::default(),
    };

    let repo_urls = plugins
        .iter()
        .map(|plugin| &plugin["repo_url"])
        .collect::<Vec<&Item>>();

    for repo_url in repo_urls {
        let repo_url = repo_url.as_str().unwrap_or_default();
        let releases_info = get_release_info(repo_url, vec![])
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to get the release info: {}", err);
                exit(1);
            });

        update_plugins(releases_info).await?;
    }

    eprintln!("The update has been completed!");

    Ok(())
}

pub async fn push_command(args: &PluginPushArgs) -> Result<()> {
    let path = args
        .path
        .clone()
        .unwrap_or(PLUGINS_FOLDER.clone().to_string_lossy().to_string());
    let metadata = fs::metadata(&path)?;
    let is_file = metadata.is_file();

    if is_file {
        let data = match fs::read(&path) {
            Ok(data) => data,
            Err(e) => {
                panic!(r#"The plugin file "{path}" error: {e}"#);
            }
        };

        let name = path
            .split(std::path::MAIN_SEPARATOR)
            .last()
            .unwrap()
            .to_string();

        let mut hasher = Sha256::new();
        hasher.update(&data);
        let sha256 = hasher.finish();
        let sha256 = sha256.iter().fold(String::new(), |mut acc, byte| {
            acc.push_str(&format!("{:02x}", byte));
            acc
        });

        let body = json!({
            "data": data,
            "name": name,
            "sha256": sha256
        })
        .to_string();

        match http_client("plugin-builder", Some(&body), Method::POST).await {
            Ok(_) => {
                eprintln!("Successfully plugin updated!!!!");
            }
            Err(err) => eprintln!("{}", err),
        };
    } else {
        let plugin_file_path = &PLUGINS_TOML_FILE_PATH.clone();
        if !plugin_file_path.exists() {
            fs::write(plugin_file_path, "")?;
        }

        let plugin_content = fs::read_to_string(plugin_file_path)?;
        let mut toml_document = plugin_content.parse::<DocumentMut>()?;
        let plugins = match toml_document["plugin"].as_array_of_tables_mut() {
            Some(plugins) => plugins,
            None => &mut ArrayOfTables::default(),
        };

        for plugin in plugins.iter() {
            let name = plugin["name"].as_str().unwrap_or_default();
            let sha256 = plugin["sha256"].as_str().unwrap_or_default();

            let file_path = PLUGINS_FOLDER.clone().join(name);
            let file_path = match file_path.to_str() {
                Some(file_path) => file_path,
                None => {
                    eprintln!(
                        "The file path {} is not found",
                        file_path.to_str().unwrap_or_default()
                    );
                    exit(1);
                }
            };

            let mut cache = Cache::new();
            let is_cached = match cache.get(file_path) {
                Some(cache_item) => cache_item.value == sha256,
                None => false,
            };

            if is_cached {
                eprintln!("Plugin cached: {file_path}");
                continue;
            }

            let data = match fs::read(file_path) {
                Ok(data) => data,
                Err(e) => {
                    panic!(r#"The plugin file "{path}" error: {e}"#);
                }
            };

            let body = json!({
                "data": data,
                "name": name,
                "sha256": sha256
            })
            .to_string();

            match http_client("plugin-builder", Some(&body), Method::POST).await {
                Ok(_) => {
                    eprintln!("Plugin updated: {}", file_path);
                    cache.set(CacheItem {
                        key: file_path.to_string(),
                        value: sha256.to_string(),
                    })?;
                }
                Err(err) => eprintln!("{}", err),
            };
        }
    };

    Ok(())
}
