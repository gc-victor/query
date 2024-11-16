use std::time::Duration;

use mini_moka::sync::{Cache as MokaCache, ConcurrentCacheExt};

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: u64,
    pub policy: String,
}

#[derive(Debug)]
pub struct CacheConfig {
    pub time_to_idle: Duration,
    pub time_to_live: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            time_to_idle: Duration::from_secs(300),  // 5 minutes
            time_to_live: Duration::from_secs(3600), // 1 hour
        }
    }
}

#[derive(Debug)]
pub struct Cache<
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
> {
    cache: MokaCache<K, V>,
    pub config: CacheConfig,
}

impl<K, V> Cache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(config: CacheConfig) -> Self {
        let cache = MokaCache::builder()
            .time_to_idle(config.time_to_idle)
            .time_to_live(config.time_to_live)
            .build();

        Self { cache, config }
    }

    pub fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key)
    }

    pub fn get_or_insert<F>(&self, key: K, fnc: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(cached) = self.cache.get(&key) {
            return cached;
        }

        let result = fnc();
        self.cache.insert(key, result.clone());
        result
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    pub fn remove(&self, key: &K) {
        self.cache.invalidate(key);
    }

    pub fn clear(&self) {
        if !self.is_empty() {
            self.cache.invalidate_all();
        }
    }

    pub fn len(&self) -> u64 {
        self.cache.entry_count()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.entry_count() == 0
    }

    pub fn sync(&self) {
        self.cache.sync();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            atomic::{AtomicI32, Ordering},
            Arc,
        },
        thread::sleep,
        time::Duration,
    };

    use mini_moka::sync::ConcurrentCacheExt;

    use super::*;

    fn make_config() -> CacheConfig {
        CacheConfig {
            time_to_idle: Duration::from_secs(5),
            time_to_live: Duration::from_secs(5),
        }
    }

    #[test]
    fn test_default_config() {
        let config = CacheConfig::default();
        assert_eq!(config.time_to_idle, Duration::from_secs(300));
        assert_eq!(config.time_to_live, Duration::from_secs(3600));
    }

    #[test]
    fn test_custom_config() {
        let config = CacheConfig {
            time_to_idle: Duration::from_secs(1),
            time_to_live: Duration::from_secs(2),
        };
        let cache = Cache::<String, Vec<u8>>::new(config);

        assert_eq!(cache.config.time_to_idle, Duration::from_secs(1));
        assert_eq!(cache.config.time_to_live, Duration::from_secs(2));
    }

    #[test]
    fn test_ttl_expiration() {
        let cache = Cache::new(CacheConfig {
            time_to_idle: Duration::from_millis(50),
            time_to_live: Duration::from_millis(100),
        });

        cache.insert("key1", vec![1]);
        assert!(cache.get(&"key1").is_some());

        sleep(Duration::from_millis(150));

        assert!(cache.get(&"key1").is_none());
    }

    #[test]
    fn test_time_to_idle_expiration() {
        let cache = Cache::new(CacheConfig {
            time_to_idle: Duration::from_millis(100),
            time_to_live: Duration::from_millis(500),
        });

        cache.insert("test_key", vec![1, 2, 3]);
        assert!(cache.get(&"test_key").is_some());

        // Access the entry to keep it alive
        sleep(Duration::from_millis(50));
        assert!(cache.get(&"test_key").is_some());

        // Wait for time_to_idle to expire without accessing
        sleep(Duration::from_millis(110));
        assert!(cache.get(&"test_key").is_none());
    }

    #[test]
    fn test_zero_ttl_config() {
        let cache = Cache::new(CacheConfig {
            time_to_idle: Duration::from_secs(0),
            time_to_live: Duration::from_secs(0),
        });

        cache.insert("test_key", vec![1]);
        cache.sync();
        assert!(!cache.contains_key(&"test_key"));
    }

    #[test]
    fn test_cache_get_or_insert() {
        let cache = Cache::new(make_config());
        let counter = Arc::new(AtomicI32::new(0));

        let expensive_query = {
            let counter = Arc::clone(&counter);
            move || {
                counter.fetch_add(1, Ordering::SeqCst);
                42
            }
        };

        // First call should execute the query
        let result1 = cache.get_or_insert("test_key".to_string(), expensive_query.clone());
        assert_eq!(result1, 42);

        // Second call within TTL should return cached result
        let result2 = cache.get_or_insert("test_key".to_string(), expensive_query.clone());
        assert_eq!(result2, 42);

        // Query should only have been executed once
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Wait for TTL to expire
        sleep(Duration::from_secs(6));

        // Query should be executed again after TTL expires
        let result3 = cache.get_or_insert("test_key".to_string(), expensive_query);
        assert_eq!(result3, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_cache_operations() {
        let cache = Cache::new(make_config());

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);

        cache.insert("key1".to_string(), vec![1]);
        cache.insert("key2".to_string(), vec![2]);
        cache.cache.sync();

        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());

        cache.remove(&"key1".to_string());
        cache.cache.sync();

        assert_eq!(cache.len(), 1);

        cache.clear();
        cache.cache.sync();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
}
