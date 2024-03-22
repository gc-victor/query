use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path, str,
};

use anyhow::Result;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use tracing::error;
use tracing::info;
use walkdir::WalkDir;

use crate::{
    cache::{Cache, CacheItem},
    utils::{http_client, line_break},
};

use super::commands::AssetArgs;

#[derive(Debug, Deserialize)]
struct Asset {
    pub data: Vec<u8>,
    pub name: String,
    pub file_hash: String,
    pub mime_type: String,
}

pub async fn command_asset(command: &AssetArgs) -> Result<()> {
    let is_delete = command.delete;
    let path = command.path.clone().unwrap_or("public".to_string());
    let metadata = fs::metadata(&path)?;
    let is_file = metadata.is_file();

    if is_file {
        let Asset {
            data,
            name,
            file_hash,
            mime_type,
        } = asset_builder(&path)?;

        if is_delete {
            let body = json!({
                "active": true,
                "data": data,
                "name": name,
                "file_hash": file_hash,
                "mime_type": mime_type,
            })
            .to_string();

            match http_client("asset-builder", Some(&body), Method::DELETE).await {
                Ok(_) => {
                    line_break();
                    info!("Successfully asset deleted!!!!");
                    line_break();
                }
                Err(err) => error!("{}", err),
            };

            return Ok(());
        };

        let body = json!({
            "active": true,
            "data": data,
            "name": name,
            "file_hash": file_hash,
            "mime_type": mime_type,
        })
        .to_string();

        match http_client("asset-builder", Some(&body), Method::POST).await {
            Ok(_) => {
                line_break();
                info!("Successfully asset updated!!!!");
                line_break();
            }
            Err(err) => error!("{}", err),
        };
    } else {
        for entry in WalkDir::new(path) {
            let entry = entry?;

            if entry.file_type().is_file() {
                let file_path = entry.path().display().to_string();

                let Asset {
                    data,
                    name,
                    file_hash,
                    mime_type,
                } = asset_builder(&file_path)?;

                let body = json!({
                    "active": true,
                    "data": data,
                    "name": name,
                    "file_hash": file_hash,
                    "mime_type": mime_type,
                })
                .to_string();

                let mut cache = Cache::new();
                let value = file_hash;
                let is_cached = match cache.get(&file_path) {
                    Some(cache_item) => cache_item.value == value,
                    None => false,
                };

                if !is_cached {
                    match http_client("asset-builder", Some(&body), Method::POST).await {
                        Ok(_) => {
                            info!("Asset updated: {}", file_path);
                            cache.set(CacheItem {
                                key: file_path,
                                value,
                            })?;
                        }
                        Err(err) => error!("{}", err),
                    };
                } else {
                    info!("Asset cached: {file_path}");
                }
            }
        }
    };

    Ok(())
}

fn asset_builder(file_path: &str) -> Result<Asset> {
    let data = match fs::read(file_path) {
        Ok(data) => data,
        Err(e) => {
            panic!(r#"The asset file "{file_path}" error: {e}"#);
        }
    };

    let name = file_path
        .split(path::MAIN_SEPARATOR)
        .last()
        .unwrap()
        .to_string();
    let mime_type = mime_guess::from_path(name)
        .first_or_text_plain()
        .to_string();

    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&data, &mut hasher);
    let file_hash = hasher.finish().to_string();

    Ok(Asset {
        data,
        name: file_path.to_string(),
        file_hash,
        mime_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_exist() {
        // Test case 1: Existing asset file
        let file_name = "file.txt";

        let dir = "../../.tests/path/to/asset".to_string();
        let path = format!("{dir}/{file_name}");

        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, "Asset file content").unwrap();

        let expected_data = b"Asset file content".to_vec();
        let expected_name = "../../.tests/path/to/asset/file.txt".to_string();
        let expected_file_hash = "10334933136645715414".to_string();
        let expected_mime_type = "text/plain".to_string();

        let result = asset_builder(&path).unwrap();

        eprintln!("result: {:?}", result);

        assert_eq!(result.data, expected_data);
        assert_eq!(result.name, expected_name);
        assert_eq!(result.file_hash, expected_file_hash);
        assert_eq!(result.mime_type, expected_mime_type);
    }

    #[test]
    #[should_panic(
        expected = r#"The asset file "/path/to/non_existing_asset/file.txt" error: No such file or directory"#
    )]
    fn test_asset_non_exist() {
        let non_existing_file_path = "/path/to/non_existing_asset/file.txt";

        asset_builder(non_existing_file_path).unwrap();
    }
}
