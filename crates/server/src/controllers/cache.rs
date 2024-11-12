use std::time::Duration;

use moka::future::Cache;

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: u64,
    pub policy: String,
}

pub struct ContentCache<K, V> {
    cache: Cache<K, V>,
}

#[derive(Debug)]
pub struct CacheConfig {
    pub max_capacity: u64,
    pub time_to_idle: Duration,
    pub time_to_live: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 1024 * 1024,               // 1MB
            time_to_idle: Duration::from_secs(300),  // 5 minutes
            time_to_live: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl<K, V> ContentCache<K, V>
where
    K: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(config: CacheConfig) -> Self {
        eprintln!("Creating cache with config: {:?}", config);

        let cache = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_idle(config.time_to_idle)
            .time_to_live(config.time_to_live)
            .build();

        Self { cache }
    }

    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    pub async fn remove(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    pub fn clear(&self) {
        self.cache.invalidate_all();
    }

    pub fn len(&self) -> u64 {
        self.cache.entry_count()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.entry_count() == 0
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entry_count: self.cache.entry_count(),
            policy: format!("{:?}", self.cache.policy()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread, time::Duration};

    // Helper function to create CacheConfig with default TTL values
    fn make_config(max_capacity: u64) -> CacheConfig {
        CacheConfig {
            max_capacity,
            time_to_idle: Duration::from_secs(5),
            time_to_live: Duration::from_secs(5),
        }
    }

    #[tokio::test]
    async fn test_capacity_eviction() {
        let cache = ContentCache::new(make_config(2));

        cache.insert("key1", vec![1]).await;
        cache.insert("key2", vec![2]).await;
        cache.insert("key3", vec![3]).await;

        assert!(
            cache.get(&"key1").await.is_some(),
            "key3 should still exist"
        );
        assert!(
            cache.get(&"key2").await.is_some(),
            "key2 should still exist"
        );

        assert_eq!(cache.len(), 2, "cache should contain exactly 2 items");
    }

    #[tokio::test]
    async fn test_ttl_expiration() {
        let cache = ContentCache::new(CacheConfig {
            max_capacity: 100,
            time_to_idle: Duration::from_millis(50),
            time_to_live: Duration::from_millis(100),
        });

        cache.insert("key1", vec![1]).await;
        assert!(cache.get(&"key1").await.is_some());

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(cache.get(&"key1").await.is_none());
    }

    #[tokio::test]
    async fn test_update_existing() {
        let cache = ContentCache::new(make_config(100));

        cache.insert("key1", vec![1]).await;
        cache.insert("key1", vec![2]).await;

        assert_eq!(cache.get(&"key1").await.unwrap(), vec![2]);
        assert_eq!(cache.len(), 1);
    }

    #[tokio::test]
    async fn test_edge_cases() {
        let cache = ContentCache::new(make_config(1));

        cache.insert("key1", vec![0; 11]).await;
        cache.insert("key1", vec![]).await;
        assert!(cache.get(&"nonexistent").await.is_none());
    }

    #[tokio::test]
    async fn test_concurrent_modifications() {
        let cache = Arc::new(ContentCache::new(make_config(100)));
        let cache_clone = Arc::clone(&cache);

        let handle = tokio::spawn(async move {
            for i in 0..50 {
                let key = format!("key{}", i);
                cache_clone.insert(key, vec![i as u8]).await;
            }
        });

        for i in 50..100 {
            let key = format!("key{}", i);
            cache.insert(key, vec![i as u8]).await;
        }

        handle.await.unwrap();
        assert_eq!(cache.len(), 100);
    }

    #[tokio::test]
    async fn test_size_limits() {
        let cache = ContentCache::new(make_config(100));

        cache.insert("small", vec![0; 512]).await;
        assert!(cache.get(&"small").await.is_some());

        cache.insert("large", vec![0; 2048]).await;
        assert!(cache.get(&"large").await.is_none());
    }

    #[tokio::test]
    async fn test_ttl_with_size() {
        let cache = ContentCache::new(CacheConfig {
            max_capacity: 100,
            time_to_idle: Duration::from_millis(100),
            time_to_live: Duration::from_millis(3600),
        });

        cache.insert("key1", "value1".to_string()).await;
        assert!(cache.get(&"key1").await.is_some());

        thread::sleep(Duration::from_millis(150));
        assert!(cache.get(&"key1").await.is_none());
    }

    #[tokio::test]
    async fn test_multiple_sizes() {
        let cache = Cache::builder()
            .weigher(|_, value: &Vec<u8>| -> u32 {
                eprintln!(
                    "Weighing value: {:?}",
                    value.len().try_into().unwrap_or(u32::MAX)
                );

                value.len().try_into().unwrap_or(u32::MAX)
            })
            .max_capacity(106)
            .build();

        cache.insert("tiny", vec![0u8; 5]).await;
        cache.insert("small", vec![0u8; 100]).await;
        cache.insert("medium", vec![0u8; 1024]).await;
        cache.insert("medium", vec![0u8; 1024 * 1024]).await;
        cache.insert("huge", vec![0u8; 2 * 1024 * 1024]).await;

        eprintln!("Cache policy: {:?}", cache.policy());

        eprintln!("Tiny: {:?}", cache.contains_key(&"tiny"));
        eprintln!("Small: {:?}", cache.contains_key(&"small"));
        eprintln!("Medium: {:?}", cache.contains_key(&"medium"));
        eprintln!("Huge: {:?}", cache.contains_key(&"huge"));

        assert_eq!(cache.entry_count(), 3);
    }

    #[tokio::test]
    async fn test_string_cache() {
        let cache: ContentCache<&str, String> = ContentCache::new(make_config(100));

        cache.insert("key1", "small value".to_string()).await;
        cache.insert("key2", "medium sized value".to_string()).await;

        assert_eq!(cache.get(&"key1").await.unwrap(), "small value");
        assert_eq!(cache.get(&"key2").await.unwrap(), "medium sized value");
    }

    #[tokio::test]
    async fn test_vec_cache() {
        let cache: ContentCache<String, Vec<u8>> = ContentCache::new(make_config(100));

        cache.insert("key1".to_string(), vec![1, 2, 3]).await;
        cache.insert("key2".to_string(), vec![4, 5, 6, 7]).await;

        assert_eq!(cache.get(&"key1".to_string()).await.unwrap(), vec![1, 2, 3]);
        assert_eq!(
            cache.get(&"key2".to_string()).await.unwrap(),
            vec![4, 5, 6, 7]
        );
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let cache: ContentCache<String, Vec<u8>> = ContentCache::new(make_config(100));

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);

        cache.insert("key1".to_string(), vec![1]).await;
        cache.insert("key2".to_string(), vec![2]).await;

        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());

        cache.remove(&"key1".to_string()).await;
        assert_eq!(cache.len(), 1);

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent() {
        let cache: ContentCache<String, Vec<u8>> = ContentCache::new(make_config(100));
        cache.insert("key1".to_string(), vec![1]).await;

        cache.remove(&"nonexistent".to_string()).await;
        assert_eq!(cache.len(), 1);
    }

    #[tokio::test]
    async fn test_clear_empty_cache() {
        let cache: ContentCache<String, Vec<u8>> = ContentCache::new(make_config(100));
        assert!(cache.is_empty());

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[tokio::test]
    async fn test_remove_and_len() {
        let cache: ContentCache<&str, String> = ContentCache::new(make_config(100));

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);

        cache.insert("key1", "value1".to_string()).await;
        cache.insert("key2", "value2".to_string()).await;
        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());

        cache.remove(&"key1").await;
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&"key1").await.is_none());
        assert!(cache.get(&"key2").await.is_some());
    }

    #[tokio::test]
    async fn test_clear() {
        let cache: ContentCache<&str, Vec<u8>> = ContentCache::new(make_config(100));

        cache.insert("key1", vec![1, 2, 3]).await;
        cache.insert("key2", vec![4, 5, 6]).await;
        cache.insert("key3", vec![7, 8, 9]).await;

        assert_eq!(cache.len(), 3);
        assert!(!cache.is_empty());

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert!(cache.get(&"key1").await.is_none());
        assert!(cache.get(&"key2").await.is_none());
        assert!(cache.get(&"key3").await.is_none());

        let stats = cache.stats();
        assert_eq!(stats.entry_count, 0);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache: ContentCache<&str, Vec<u8>> = ContentCache::new(make_config(100));

        cache.insert("key1", vec![1, 2, 3]).await;
        cache.insert("key2", vec![4, 5, 6]).await;
        cache.insert("key3", vec![7, 8, 9]).await;

        let stats = cache.stats();
        assert_eq!(stats.entry_count, 3);
        assert_eq!(
            stats.policy,
            "Policy { max_capacity: Some(100), time_to_live: Some(5s), time_to_idle: Some(5s) }"
                .to_string()
        );
    }
}
