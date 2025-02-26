use std::env;
use std::time::Duration;

use hyper::HeaderMap;
use mini_moka::sync::{Cache, ConcurrentCacheExt};
use tracing::instrument;

const QUERY_CACHE_FILE_MAX_CAPACITY: &str = "QUERY_CACHE_FILE_MAX_CAPACITY";

const DEFAULT_CACHE_MAX_CAPACITY: u64 = 25 * 1024 * 1024; // 25 MB
const DEFAULT_CACHE_TIME_TO_IDLE: u64 = 300; // 5 minutes
const DEFAULT_CACHE_TIME_TO_LIVE: u64 = 3600; // 1 hour
const DEFAULT_CACHE_FILE_MAX_CAPACITY: u64 = 1024 * 1024; // 1 MB default max file size

#[derive(Clone, Debug)]
pub struct CacheResponseValue {
    pub body: Vec<u8>,
    pub headers: HeaderMap,
}

#[derive(Debug)]
pub struct CacheResponse {
    cache: Cache<String, CacheResponseValue>,
    pub config: CacheResponseConfig,
}

impl PartialEq for CacheResponse {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

#[derive(Debug, PartialEq)]
pub struct CacheResponseConfig {
    pub max_capacity: u64,
    pub time_to_idle: Duration,
    pub time_to_live: Duration,
}

impl Default for CacheResponseConfig {
    fn default() -> Self {
        Self {
            max_capacity: DEFAULT_CACHE_MAX_CAPACITY,
            time_to_idle: Duration::from_secs(DEFAULT_CACHE_TIME_TO_IDLE),
            time_to_live: Duration::from_secs(DEFAULT_CACHE_TIME_TO_LIVE),
        }
    }
}

impl CacheResponse {
    #[instrument]
    pub fn new(config: CacheResponseConfig) -> Self {
        let cache = Cache::builder()
            .weigher(|_key, value: &CacheResponseValue| -> u32 { value.body.len() as u32 })
            .max_capacity(config.max_capacity)
            .time_to_idle(config.time_to_idle)
            .time_to_live(config.time_to_live)
            .build();

        Self { cache, config }
    }

    #[instrument(name = "cache_response_insert", skip(self, value))]
    pub fn insert(&self, key: String, value: CacheResponseValue) {
        let max_file_size = env::var(QUERY_CACHE_FILE_MAX_CAPACITY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_CACHE_FILE_MAX_CAPACITY);

        if value.body.len() as u64 <= max_file_size {
            self.cache.insert(key, value);
        }
    }

    #[instrument(name = "cache_response_get", skip(self))]
    pub fn get(&self, key: &String) -> Option<CacheResponseValue> {
        self.cache.get(key)
    }

    pub fn contains(&self, key: &String) -> bool {
        self.cache.contains_key(key)
    }

    #[instrument(name = "cache_response_clear", skip(self))]
    pub fn clear(&self) {
        self.cache.invalidate_all();
    }

    pub fn iter(
        &self,
    ) -> mini_moka::sync::Iter<'_, String, CacheResponseValue, std::hash::RandomState> {
        self.cache.iter()
    }

    pub fn len(&self) -> usize {
        self.cache.entry_count() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn sync(&self) {
        self.cache.sync();
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_cache_insertion() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "test_key".to_string();
        let value = CacheResponseValue {
            body: vec![1, 2, 3],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value.clone());
        cache.sync();

        assert!(cache.cache.contains_key(&key));
        assert_eq!(cache.len(), 1);
    }

    fn create_test_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert("x-custom", "test-value".parse().unwrap());
        headers
    }

    #[test]
    fn test_cache_retrieval() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "test_key".to_string();
        let headers = create_test_headers();
        let value = CacheResponseValue {
            body: vec![1, 2, 3],
            headers,
        };

        cache.insert(key.clone(), value.clone());
        let retrieved = cache.get(&key).unwrap();

        assert_eq!(retrieved.body, value.body);
        assert_eq!(
            retrieved.headers.get("content-type").unwrap(),
            "application/json"
        );
        assert_eq!(retrieved.headers.get("x-custom").unwrap(), "test-value");
    }

    #[test]
    fn test_max_file_size() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "file".to_string();

        // Create value larger than default max file size (1MB)
        let large_value = CacheResponseValue {
            body: vec![1; (DEFAULT_CACHE_FILE_MAX_CAPACITY + 1) as usize],
            headers: HeaderMap::new(),
        };

        // Create value within max file size
        let small_value = CacheResponseValue {
            body: vec![1; (DEFAULT_CACHE_FILE_MAX_CAPACITY - 1) as usize],
            headers: HeaderMap::new(),
        };

        // Large file should not be cached
        cache.insert(key.clone(), large_value);
        assert!(!cache.contains(&key));
        assert_eq!(cache.len(), 0);

        // Small file should be cached
        cache.insert(key.clone(), small_value);
        cache.sync();
        assert!(cache.contains(&key));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_contains() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "test_key".to_string();
        let value = CacheResponseValue {
            body: vec![1, 2, 3],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value.clone());

        assert!(cache.contains(&key));
    }
    #[test]
    fn test_cache_len() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "test_key".to_string();
        let value = CacheResponseValue {
            body: vec![1, 2, 3],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value.clone());
        cache.sync();

        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_is_empty() {
        let cache = CacheResponse::new(CacheResponseConfig::default());
        assert!(cache.is_empty());
    }

    #[test]
    fn test_clear_cache() {
        let cache = CacheResponse::new(CacheResponseConfig::default());

        for i in 0..3 {
            let key = format!("key_{}", i);
            let headers = HeaderMap::new();
            let value = CacheResponseValue {
                body: vec![i as u8],
                headers,
            };
            cache.insert(key, value);
        }
        cache.sync();
        assert_eq!(cache.len(), 3);

        cache.clear();
        cache.sync();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_time_to_live() {
        let config = CacheResponseConfig {
            max_capacity: 1024,
            time_to_idle: Duration::from_millis(10),
            time_to_live: Duration::from_millis(20),
        };
        let cache = CacheResponse::new(config);
        let key = "expiring_key".to_string();
        let headers = HeaderMap::new();
        let value = CacheResponseValue {
            body: vec![1],
            headers,
        };

        cache.insert(key.clone(), value);
        assert!(cache.get(&key).is_some());

        // Wait for time_to_idle to expire
        sleep(Duration::from_millis(21));
        assert!(cache.get(&key).is_none());
    }
    #[test]
    fn test_cache_time_to_idle_expiration() {
        let config = CacheResponseConfig {
            max_capacity: 1024,
            time_to_idle: Duration::from_millis(100),
            time_to_live: Duration::from_millis(500),
        };
        let cache = CacheResponse::new(config);
        let key = "idle_key".to_string();
        let value = CacheResponseValue {
            body: vec![1, 2, 3],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value);
        assert!(cache.get(&key).is_some());

        // Access the entry to keep it alive
        sleep(Duration::from_millis(50));
        assert!(cache.get(&key).is_some());

        // Wait for time_to_idle to expire without accessing
        sleep(Duration::from_millis(110));
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_cache_max_capacity() {
        let config = CacheResponseConfig {
            max_capacity: 3,
            ..Default::default()
        };
        let cache = CacheResponse::new(config);

        cache.insert(
            "key1".to_string(),
            CacheResponseValue {
                body: vec![1],
                headers: HeaderMap::new(),
            },
        );
        cache.insert(
            "key2".to_string(),
            CacheResponseValue {
                body: vec![2],
                headers: HeaderMap::new(),
            },
        );
        cache.insert(
            "key3".to_string(),
            CacheResponseValue {
                body: vec![3],
                headers: HeaderMap::new(),
            },
        );
        cache.sync();

        assert_eq!(cache.len(), 3);

        cache.insert(
            "key4".to_string(),
            CacheResponseValue {
                body: vec![4],
                headers: HeaderMap::new(),
            },
        );
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_cache_size_limit_bytes() {
        let config = CacheResponseConfig {
            max_capacity: 199,
            ..Default::default()
        };
        let cache = CacheResponse::new(config);

        cache.insert(
            "key1".to_string(),
            CacheResponseValue {
                body: vec![1; 100],
                headers: HeaderMap::new(),
            },
        );
        cache.sync();

        assert_eq!(cache.len(), 1);

        cache.insert(
            "key2".to_string(),
            CacheResponseValue {
                body: vec![2; 100],
                headers: HeaderMap::new(),
            },
        );
        cache.sync();

        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_default_config() {
        let config = CacheResponseConfig::default();
        assert_eq!(config.max_capacity, 25 * 1024 * 1024);
        assert_eq!(config.time_to_idle, Duration::from_secs(300));
        assert_eq!(config.time_to_live, Duration::from_secs(3600));
    }

    #[test]
    fn test_custom_config() {
        let config = CacheResponseConfig {
            max_capacity: 1024,
            time_to_idle: Duration::from_secs(1),
            time_to_live: Duration::from_secs(2),
        };
        let cache = CacheResponse::new(config);

        assert_eq!(cache.config.max_capacity, 1024);
        assert_eq!(cache.config.time_to_idle, Duration::from_secs(1));
        assert_eq!(cache.config.time_to_live, Duration::from_secs(2));
    }

    #[test]
    fn test_max_file_size_env_override() {
        unsafe {
            std::env::set_var("QUERY_CACHE_FILE_MAX_CAPACITY", "100");
        }
        let cache = CacheResponse::new(CacheResponseConfig::default());
        let key = "file".to_string();

        let large_value = CacheResponseValue {
            body: vec![1; 101],
            headers: HeaderMap::new(),
        };
        let small_value = CacheResponseValue {
            body: vec![1; 99],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), large_value);
        assert!(!cache.contains(&key));

        cache.insert(key.clone(), small_value);
        assert!(cache.contains(&key));

        unsafe {
            std::env::remove_var("QUERY_CACHE_FILE_MAX_CAPACITY");
        }
    }

    #[test]
    fn test_memory_pressure() {
        let config = CacheResponseConfig {
            max_capacity: 5,
            ..Default::default()
        };
        let cache = CacheResponse::new(config);

        for i in 1..=5 {
            cache.insert(
                format!("key_{}", i),
                CacheResponseValue {
                    body: vec![1; i],
                    headers: HeaderMap::new(),
                },
            );
        }

        cache.sync();

        assert!(cache.len() == 2);
        assert!(cache.contains(&"key_1".to_string()));
        assert!(cache.contains(&"key_2".to_string()));
    }

    #[test]
    fn test_zero_capacity_config() {
        let config = CacheResponseConfig {
            max_capacity: 0,
            ..Default::default()
        };
        let cache = CacheResponse::new(config);
        let key = "test_key".to_string();
        let value = CacheResponseValue {
            body: vec![1],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value);
        cache.sync();
        assert!(!cache.contains(&key));
    }

    #[test]
    fn test_zero_ttl_config() {
        let config = CacheResponseConfig {
            max_capacity: 1024,
            time_to_idle: Duration::from_secs(0),
            time_to_live: Duration::from_secs(0),
        };
        let cache = CacheResponse::new(config);
        let key = "test_key".to_string();
        let value = CacheResponseValue {
            body: vec![1],
            headers: HeaderMap::new(),
        };

        cache.insert(key.clone(), value);
        cache.sync();
        assert!(!cache.contains(&key));
    }
}
