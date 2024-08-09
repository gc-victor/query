use std::env;

use anyhow::Result;
use serde::Deserialize;

use super::commands::PluginArgs;

#[derive(Debug, Deserialize)]
pub struct GitHubApiAsset {
    pub browser_download_url: String,
    pub content_type: String,
    pub name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct GitHubApiRelease {
    pub assets: Vec<GitHubApiAsset>,
    pub tag_name: String,
    pub url: String,
    pub prerelease: bool,
    pub published_at: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ReleaseInfo {
    pub name: String,
    pub tag_name: String,
    pub url: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Package {
    pub name: String,
    pub tag_name: String,
    pub url: String,
    pub sha256: Option<String>,
    pub published_at: Option<String>,
}

// @see: https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
pub async fn command_plugin(command: &PluginArgs) {
    let command = &command.command;

    match command {
        super::commands::PluginCommands::Install(args) => {
            // TODO:
            // - If there is not a args.github_repo_url, then we should use the .query/wasm.toml file to install the plugin
            // - Should be checked that the plugin is not already installed in the plugins directory
        }
        super::commands::PluginCommands::Update => todo!(),
        super::commands::PluginCommands::Push(args) => todo!(),
        super::commands::PluginCommands::Delete(args) => todo!(),
    }
}

// CREDIT: https://github.com/moonrepo/proto/blob/1b1dabb440089dc899aca107bba71fa6888015ef/crates/warpgate/src/loader.rs#L336
async fn get_release_info(github_repo_url: &str) -> Result<Vec<ReleaseInfo>> {
    let repo_slug = github_repo_url.split("/").collect::<Vec<&str>>();
    let repo_slug = repo_slug[repo_slug.len() - 2..].join("/");
    let tag = github_repo_url.split("@").nth(1).unwrap_or("latest");
    let url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        repo_slug, tag
    );

    let client = reqwest::Client::new();
    let mut request = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "request");

    // https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token
    if let Ok(auth_token) = env::var("GITHUB_TOKEN") {
        request = request.bearer_auth(auth_token);
    }

    let response = request.send().await?.json::<serde_json::Value>().await?;
    let release: GitHubApiRelease = serde_json::from_value(response)?;

    let mut release_info = vec![];

    for asset in release.assets {
        if asset.content_type == "application/wasm"
            || asset.name.ends_with(".wasm")
            || asset.name.ends_with(".wasm.sha256")
        {
            let release = ReleaseInfo {
                name: asset.name.clone(),
                tag_name: release.tag_name.clone(),
                url: asset.browser_download_url.clone(),
                published_at: release.published_at.clone(),
            };

            release_info.push(release);
        }
    }

    Ok(release_info)
}

// https://github.com/moonrepo/proto/blob/1b1dabb440089dc899aca107bba71fa6888015ef/crates/warpgate/src/loader.rs#L217
// https://github.com/moonrepo/starbase/blob/9e4e07ce2ae0014a5855551fe7413ea586e3bf8b/crates/utils/src/net.rs#L16
fn download_plugin() {}

/*
.query/wasm.toml

# WASM Packages

[[package]]
name = "app-v1.0.0-linux"
tag_name = "v1.0.0"
url = "https://example.com/downloads/app-v1.0.0-linux.tar.gz"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
published_at = "2023-08-15T14:30:00Z"

[[package]]
name = "app-v1.0.0-macos"
tag_name = "v1.0.0"
url = "https://example.com/downloads/app-v1.0.0-macos.dmg"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
published_at = "2023-08-15T14:30:00Z"

[[package]]
name = "app-v1.1.0-beta-linux"
tag_name = "v1.1.0-beta"
url = "https://example.com/downloads/app-v1.1.0-beta-linux.tar.gz"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
published_at = "2023-09-01T10:00:00Z"

[[package]]
name = "app-v1.1.0-beta-macos"
tag_name = "v1.1.0-beta"
url = "https://example.com/downloads/app-v1.1.0-beta-macos.dmg"
published_at = "2023-09-01T10:00:00Z"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

[[package]]
name = "app-v0.9.0-windows"
tag_name = "v0.9.0"
url = "https://example.com/downloads/app-v0.9.0-windows.exe"
published_at = null
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

*/
