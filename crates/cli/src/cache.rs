use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheItem {
    pub key: String,
    pub value: String,
}

type CacheList = Vec<CacheItem>;

pub struct Cache {}

impl Cache {
    pub fn new() -> Self {
        if !Path::new(&CONFIG.cli.cache_file_path).exists() {
            std::fs::write(&CONFIG.cli.cache_file_path, "").unwrap();
        }

        Self {}
    }

    pub fn get(&self, key: &str) -> Option<CacheItem> {
        let content = std::fs::read_to_string(&CONFIG.cli.cache_file_path).unwrap();

        if content.is_empty() {
            return None;
        }

        serde_json::from_str::<CacheList>(&content)
            .unwrap()
            .into_iter()
            .find(|item| item.key == key)
    }

    pub fn set(&mut self, item: CacheItem) -> Result<()> {
        let content = std::fs::read_to_string(&CONFIG.cli.cache_file_path)?;

        let cache = if content.is_empty() {
            vec![]
        } else {
            serde_json::from_str::<CacheList>(&content)?
        };

        let key = &item.key;
        let mut cache: CacheList = cache.into_iter().filter(|item| &item.key != key).collect();

        cache.push(item);

        let cache = serde_json::to_string(&cache)?;

        std::fs::write(&CONFIG.cli.cache_file_path, cache)?;

        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        let content = std::fs::read_to_string(&CONFIG.cli.cache_file_path)?;
        let cache = if content.is_empty() {
            vec![]
        } else {
            serde_json::from_str::<CacheList>(&content)?
        };
        let cache: CacheList = cache.into_iter().filter(|item| item.key == key).collect();

        let cache = serde_json::to_string(&cache)?;

        std::fs::write(&CONFIG.cli.cache_file_path, cache)?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        std::fs::write(&CONFIG.cli.cache_file_path, "")?;

        Ok(())
    }
}
