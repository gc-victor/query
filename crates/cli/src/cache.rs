use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use crate::config::CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub key: String,
    pub value: String,
}

type CacheList = Vec<CacheItem>;

#[derive(Debug)]
pub struct Cache {
    cache_path: String,
    #[allow(dead_code)] // Needed to keep TempDir alive
    temp_dir: Option<TempDir>,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        let cache_path = if cfg!(test) {
            let temp_dir = tempfile::tempdir().unwrap();
            let path = temp_dir
                .path()
                .join("query_cli_test_cache.json")
                .to_string_lossy()
                .into_owned();
            (path, Some(temp_dir))
        } else {
            (CONFIG.cli.cache_file_path.clone(), None)
        };

        let (path, temp_dir) = cache_path;
        if !Path::new(&path).exists() {
            std::fs::write(&path, "").unwrap();
        }

        Self {
            cache_path: path,
            temp_dir,
        }
    }

    pub fn get(&self, key: &str) -> Option<CacheItem> {
        let content = std::fs::read_to_string(&self.cache_path).unwrap();

        if content.is_empty() {
            return None;
        }

        serde_json::from_str::<CacheList>(&content)
            .unwrap()
            .into_iter()
            .find(|item| item.key == key)
    }

    pub fn set(&mut self, item: CacheItem) -> Result<()> {
        let content = std::fs::read_to_string(&self.cache_path)?;

        let cache = if content.is_empty() {
            vec![]
        } else {
            serde_json::from_str::<CacheList>(&content)?
        };

        let key = &item.key;
        let mut cache: CacheList = cache.into_iter().filter(|item| &item.key != key).collect();

        cache.push(item);

        let cache = serde_json::to_string(&cache)?;

        std::fs::write(&self.cache_path, cache)?;

        Ok(())
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        let content = std::fs::read_to_string(&self.cache_path)?;
        let cache = if content.is_empty() {
            vec![]
        } else {
            serde_json::from_str::<CacheList>(&content)?
        };
        let cache: CacheList = cache.into_iter().filter(|item| item.key != key).collect();

        let cache = serde_json::to_string(&cache)?;

        std::fs::write(&self.cache_path, cache)?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        std::fs::write(&self.cache_path, "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_file_creation() {
        let cache = Cache::new();
        assert!(Path::new(&cache.cache_path).exists());
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = Cache::new();
        let item = CacheItem {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };
        cache.set(item.clone()).unwrap();

        let retrieved = cache.get(&item.key).unwrap();
        assert_eq!(retrieved.key, item.key);
        assert_eq!(retrieved.value, item.value);
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = Cache::new();
        let item = CacheItem {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };
        cache.set(item.clone()).unwrap();
        cache.remove(&item.key).unwrap();

        assert!(cache.get(&item.key).is_none());
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = Cache::new();
        let item = CacheItem {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };
        cache.set(item.clone()).unwrap();
        cache.clear().unwrap();

        assert!(cache.get(&item.key).is_none());
    }

    #[test]
    fn test_cache_multiple_items() {
        let mut cache = Cache::new();
        let items = [CacheItem {
                key: "key1".to_string(),
                value: "value1".to_string(),
            },
            CacheItem {
                key: "key2".to_string(),
                value: "value2".to_string(),
            },
            CacheItem {
                key: "key3".to_string(),
                value: "value3".to_string(),
            }];

        for item in items.iter() {
            cache.set(item.clone()).unwrap();
        }

        for item in items.iter() {
            let retrieved = cache.get(&item.key).unwrap();
            assert_eq!(retrieved.key, item.key);
            assert_eq!(retrieved.value, item.value);
        }
    }

    #[test]
    fn test_cache_update_existing_key() {
        let mut cache = Cache::new();
        let item1 = CacheItem {
            key: "test_key".to_string(),
            value: "value1".to_string(),
        };
        let item2 = CacheItem {
            key: "test_key".to_string(),
            value: "value2".to_string(),
        };

        cache.set(item1).unwrap();
        cache.set(item2.clone()).unwrap();

        let retrieved = cache.get(&item2.key).unwrap();
        assert_eq!(retrieved.value, item2.value);
    }
}
