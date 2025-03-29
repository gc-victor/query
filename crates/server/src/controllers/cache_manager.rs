use std::{
    env,
    str::FromStr,
    sync::{
        atomic::{AtomicI64, Ordering},
        OnceLock,
    },
    time::Duration,
};

use query_runtime::sqlite::query_cache_invalidate;
use tokio::{task::JoinHandle, time};

use crate::sqlite::connect_db::connect_cache_invalidation_db;

use super::{
    cache::{Cache, CacheConfig},
    cache_response::{CacheResponse, CacheResponseConfig},
};

static LAST_KNOWN_INVALIDATION: AtomicI64 = AtomicI64::new(0);

#[cfg(test)]
const DEFAULT_CHECK_INTERVAL: u64 = 100; // 100ms
#[cfg(not(test))]
const DEFAULT_CHECK_INTERVAL: u64 = 5000; // 5s

const DEFAULT_ASSET_RESPONSE_CACHE_MAX_CAPACITY: u64 = 25 * 1024 * 1024; // 25 MB
const DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_IDLE: u64 = 86400; // 1 day
const DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_LIVE: u64 = 2592000; // 30 days

const DEFAULT_FUNCTION_RESPONSE_CACHE_MAX_CAPACITY: u64 = 25 * 1024 * 1024; // 25 MB
const DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_IDLE: u64 = 3600; // 1 hour
const DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_LIVE: u64 = 86400; // 1 day

const DEFAULT_PATH_CACHE_TIME_TO_IDLE: u64 = 3600; // 1 hour
const DEFAULT_PATH_CACHE_TIME_TO_LIVE: u64 = 86400; // 1 day

const DEFAULT_FUNCTION_CACHE_TIME_TO_IDLE: u64 = 600; // 10 min
const DEFAULT_FUNCTION_CACHE_TIME_TO_LIVE: u64 = 3600; // 1 hour

#[derive(Clone, Copy, Debug)]
pub enum CacheResponseType {
    Asset,
    Function,
}

#[derive(Clone, Copy, Debug)]
pub enum CacheType {
    Path,
    Function,
}

static RESPONSE_CACHE: [OnceLock<CacheResponse>; 2] = [OnceLock::new(), OnceLock::new()];
static CACHE: [OnceLock<Cache<String, String>>; 2] = [OnceLock::new(), OnceLock::new()];

fn env<T: FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn asset_response_cache_config() -> CacheResponseConfig {
    CacheResponseConfig {
        max_capacity: env(
            "QUERY_ASSET_CACHE_MAX_CAPACITY",
            DEFAULT_ASSET_RESPONSE_CACHE_MAX_CAPACITY,
        ),
        time_to_idle: Duration::from_secs(env(
            "QUERY_ASSET_CACHE_TIME_TO_IDLE",
            DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_IDLE,
        )),
        time_to_live: Duration::from_secs(env(
            "QUERY_ASSET_CACHE_TIME_TO_LIVE",
            DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_LIVE,
        )),
    }
}

fn function_response_cache_config() -> CacheResponseConfig {
    CacheResponseConfig {
        max_capacity: env(
            "QUERY_FUNCTION_CACHE_MAX_CAPACITY",
            DEFAULT_FUNCTION_RESPONSE_CACHE_MAX_CAPACITY,
        ),
        time_to_idle: Duration::from_secs(env(
            "QUERY_FUNCTION_CACHE_TIME_TO_IDLE",
            DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_IDLE,
        )),
        time_to_live: Duration::from_secs(env(
            "QUERY_FUNCTION_CACHE_TIME_TO_LIVE",
            DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_LIVE,
        )),
    }
}

pub fn cache_response(cache_type: CacheResponseType) -> &'static CacheResponse {
    match cache_type {
        CacheResponseType::Asset => {
            RESPONSE_CACHE[0].get_or_init(|| CacheResponse::new(asset_response_cache_config()))
        }
        CacheResponseType::Function => {
            RESPONSE_CACHE[1].get_or_init(|| CacheResponse::new(function_response_cache_config()))
        }
    }
}

fn path_cache_config() -> CacheConfig {
    CacheConfig {
        time_to_idle: Duration::from_secs(env(
            "QUERY_PATH_CACHE_TIME_TO_IDLE",
            DEFAULT_PATH_CACHE_TIME_TO_IDLE,
        )),
        time_to_live: Duration::from_secs(env(
            "QUERY_PATH_CACHE_TIME_TO_LIVE",
            DEFAULT_PATH_CACHE_TIME_TO_LIVE,
        )),
    }
}

fn function_cache_config() -> CacheConfig {
    CacheConfig {
        time_to_idle: Duration::from_secs(env(
            "QUERY_FUNCTION_CACHE_TIME_TO_IDLE",
            DEFAULT_FUNCTION_CACHE_TIME_TO_IDLE,
        )),
        time_to_live: Duration::from_secs(env(
            "QUERY_FUNCTION_CACHE_TIME_TO_LIVE",
            DEFAULT_FUNCTION_CACHE_TIME_TO_LIVE,
        )),
    }
}

pub fn cache(cache_type: CacheType) -> &'static Cache<String, String> {
    match cache_type {
        CacheType::Path => CACHE[0].get_or_init(|| Cache::new(path_cache_config())),
        CacheType::Function => CACHE[1].get_or_init(|| Cache::new(function_cache_config())),
    }
}

pub fn start_invalidation_task() -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = time::interval(interval_duration());

        tracing::info!(
            "Cache invalidation interval duration: {:?}",
            interval_duration()
        );

        loop {
            interval.tick().await;

            match check_database_invalidation() {
                Ok(true) => {
                    clear_response_cache(CacheResponseType::Asset);
                    clear_response_cache(CacheResponseType::Function);
                    clear_cache(CacheType::Path);
                    clear_cache(CacheType::Function);
                    query_cache_invalidate();
                    tracing::info!("Cache invalidated due to database update");
                }
                Err(e) => {
                    tracing::error!("Error checking cache invalidation: {}", e);
                }
                _ => {}
            }
        }
    })
}

fn interval_duration() -> Duration {
    #[cfg(test)]
    {
        Duration::from_millis(env("QUERY_CACHE_CHECK_INTERVAL", DEFAULT_CHECK_INTERVAL))
    }
    #[cfg(not(test))]
    {
        Duration::from_millis(env("QUERY_CACHE_CHECK_INTERVAL", DEFAULT_CHECK_INTERVAL))
    }
}

fn check_database_invalidation() -> Result<bool, anyhow::Error> {
    let conn = connect_cache_invalidation_db()?;

    static QUERY: &str = "SELECT version FROM cache_invalidation;";
    let latest_invalidation: Option<i64> = conn
        .prepare_cached(QUERY)?
        .query_row([], |row| row.get(0))?;

    let last_known = LAST_KNOWN_INVALIDATION.load(Ordering::Acquire);

    if let Some(latest) = latest_invalidation {
        if latest > last_known {
            LAST_KNOWN_INVALIDATION.store(latest, Ordering::Release);
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn clear_response_cache(cache_type: CacheResponseType) {
    match cache_type {
        CacheResponseType::Asset => {
            let cache =
                RESPONSE_CACHE[0].get_or_init(|| CacheResponse::new(asset_response_cache_config()));
            cache.clear();
            tracing::info!("Response asset cache invalidated due to database update");
        }
        CacheResponseType::Function => {
            let cache = RESPONSE_CACHE[1]
                .get_or_init(|| CacheResponse::new(function_response_cache_config()));
            cache.clear();
            tracing::info!("Response function cache invalidated due to database update");
        }
    }
}

pub fn clear_cache(cache_type: CacheType) {
    match cache_type {
        CacheType::Path => {
            let cache = CACHE[0].get_or_init(|| Cache::new(path_cache_config()));
            cache.clear();
            tracing::info!("Path cache invalidated due to database update");
        }
        CacheType::Function => {
            let cache = CACHE[1].get_or_init(|| Cache::new(function_cache_config()));
            cache.clear();
            tracing::info!("Function cache invalidated due to database update");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, time::Duration};

    use hyper::HeaderMap;
    use tokio::sync::Semaphore;

    use crate::controllers::cache_response::CacheResponseValue;
    use crate::sqlite::connect_db::connect_cache_invalidation_db;
    use crate::sqlite::create_cache_invalidation_db::create_cache_invalidation_db;

    use super::*;

    static PERMIT: Semaphore = Semaphore::const_new(1);

    #[tokio::test]
    async fn test_asset_response_cache() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache_response(CacheResponseType::Asset);

        // Test cache operations
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "text/plain".parse().unwrap());
        cache.insert(
            "asset_key1".to_string(),
            CacheResponseValue {
                body: b"test data".to_vec(),
                headers: headers.clone(),
            },
        );
        let retrieved = cache.get(&"asset_key1".to_string()).unwrap();
        assert_eq!(retrieved.body, b"test data");
        assert_eq!(retrieved.headers, headers);

        // Test cache size limit
        cache.insert(
            "asset_key1".to_string(),
            CacheResponseValue {
                body: b"test1".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        cache.insert(
            "asset_key2".to_string(),
            CacheResponseValue {
                body: b"test2".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        cache.insert(
            "asset_key3".to_string(),
            CacheResponseValue {
                body: b"test3".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        cache.sync();

        assert_eq!(cache.len(), 3);

        // Test cache clear
        clear_response_cache(CacheResponseType::Asset);
        cache.sync();

        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());

        drop(permit);
    }

    #[tokio::test]
    async fn test_function_response_cache() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache_response(CacheResponseType::Function);

        // Test cache operations
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        cache.insert(
            "function1".to_string(),
            CacheResponseValue {
                body: b"{\"data\": \"test\"}".to_vec(),
                headers: headers.clone(),
            },
        );
        let retrieved = cache.get(&"function1".to_string()).unwrap();
        assert_eq!(retrieved.body, b"{\"data\": \"test\"}");
        assert_eq!(retrieved.headers, headers);

        // Test cache size limit
        cache.insert(
            "function2".to_string(),
            CacheResponseValue {
                body: b"{\"data\": \"test2\"}".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        cache.sync();
        assert!(cache.len() >= 2);

        // Test cache clear
        clear_response_cache(CacheResponseType::Function);
        cache.sync();
        assert!(cache.is_empty());

        drop(permit);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache1 = cache(CacheType::Path);

        // Test insertion and retrieval
        cache1.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache1.get(&"key1".to_string()), Some("value1".to_string()));

        // Test updating existing key
        cache1.insert("key1".to_string(), "value2".to_string());
        assert_eq!(cache1.get(&"key1".to_string()), Some("value2".to_string()));

        // Test concurrent operations
        let mut handles = vec![];
        for i in 0..10 {
            handles.push(tokio::spawn(async move {
                let cache2 = cache(CacheType::Path);
                cache2.insert(format!("concurrent_key{}", i), format!("value{}", i));
                cache2.get(&format!("concurrent_key{}", i))
            }));
        }

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.await.unwrap();
            assert_eq!(result, Some(format!("value{}", i)));
        }

        drop(permit);
    }

    #[tokio::test]
    async fn test_invalidation_task() {
        let permit = PERMIT.acquire().await.unwrap();

        const TEST_DATABASE_PATH: &str = "../../.tests/test_cache_invalidation";
        unsafe {
            std::env::set_var("QUERY_SERVER_DBS_PATH", TEST_DATABASE_PATH);
        }
        fs::create_dir_all(TEST_DATABASE_PATH).unwrap();

        struct Cleanup;
        impl Drop for Cleanup {
            fn drop(&mut self) {
                fs::remove_dir_all(TEST_DATABASE_PATH).unwrap();
            }
        }
        let _cleanup = Cleanup;

        // Create fresh database
        create_cache_invalidation_db();
        let conn = connect_cache_invalidation_db().unwrap();

        // Start invalidation task
        let handle = start_invalidation_task();

        // Insert initial version
        conn.execute("INSERT INTO cache_invalidation DEFAULT VALUES;", [])
            .unwrap();

        // Wait for a check interval
        time::sleep(interval_duration() * 2).await;

        // Insert new version
        conn.execute("INSERT INTO cache_invalidation DEFAULT VALUES;", [])
            .unwrap();

        // Wait for another check interval
        time::sleep(interval_duration() * 2).await;

        // Cleanup
        handle.abort();

        drop(permit);
    }

    #[test]
    fn test_interval_duration_default() {
        unsafe {
            env::remove_var("QUERY_CACHE_CHECK_INTERVAL");
        }
        let duration = interval_duration();
        // In test mode, duration should be in milliseconds
        assert_eq!(duration, Duration::from_millis(DEFAULT_CHECK_INTERVAL));
    }

    #[test]
    fn test_interval_duration_custom() {
        unsafe {
            env::set_var("QUERY_CACHE_CHECK_INTERVAL", "120");
        }
        let duration = interval_duration();
        // In test mode, duration should be in milliseconds
        assert_eq!(duration, Duration::from_millis(120));
        unsafe {
            env::remove_var("QUERY_CACHE_CHECK_INTERVAL");
        }
    }

    #[tokio::test]
    async fn test_asset_response_cache_config() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache_response(CacheResponseType::Asset);
        assert_eq!(
            cache.config.max_capacity,
            DEFAULT_ASSET_RESPONSE_CACHE_MAX_CAPACITY
        );
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_ASSET_RESPONSE_CACHE_TIME_TO_LIVE)
        );

        drop(permit);
    }

    #[tokio::test]
    async fn test_function_response_cache_config() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache_response(CacheResponseType::Function);
        assert_eq!(
            cache.config.max_capacity,
            DEFAULT_FUNCTION_RESPONSE_CACHE_MAX_CAPACITY
        );
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_FUNCTION_RESPONSE_CACHE_TIME_TO_LIVE)
        );

        drop(permit);
    }

    #[tokio::test]
    async fn test_path_cache_config() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache(CacheType::Path);
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_PATH_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_PATH_CACHE_TIME_TO_LIVE)
        );

        drop(permit);
    }

    #[tokio::test]
    async fn test_function_cache_config() {
        let permit = PERMIT.acquire().await.unwrap();

        let cache = cache(CacheType::Function);
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_FUNCTION_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_FUNCTION_CACHE_TIME_TO_LIVE)
        );

        drop(permit);
    }

    #[test]
    fn test_cache_types() {
        let asset_cache = cache_response(CacheResponseType::Asset);
        let function_cache = cache_response(CacheResponseType::Function);

        // Verify we get different cache instances
        assert_ne!(asset_cache, function_cache);

        // Test Asset cache
        asset_cache.insert(
            "asset1".to_string(),
            CacheResponseValue {
                body: b"asset data".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        asset_cache.sync();

        assert!(!asset_cache.is_empty());

        // Test Function cache
        function_cache.insert(
            "func1".to_string(),
            CacheResponseValue {
                body: b"function data".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        function_cache.sync();

        assert!(!function_cache.is_empty());

        // Clear specific cache type
        clear_response_cache(CacheResponseType::Asset);
        asset_cache.sync();

        assert_eq!(asset_cache.len(), 0);

        clear_response_cache(CacheResponseType::Function);
        function_cache.sync();
        assert_eq!(function_cache.len(), 0);
    }

    #[test]
    fn test_env_override_function_response_cache_config() {
        // Test that environment variables can override defaults
        unsafe {
            env::set_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY", "5000000"); // 5MB
            env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE", "1800"); // 30 minutes
            env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE", "43200"); // 12 hours
        }

        let config = function_response_cache_config();
        assert_eq!(config.max_capacity, 5000000);
        assert_eq!(config.time_to_idle, Duration::from_secs(1800));
        assert_eq!(config.time_to_live, Duration::from_secs(43200));

        // Clean up environment variables
        unsafe {
            env::remove_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY");
            env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE");
            env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE");
        }
    }

    #[test]
    fn test_env_override_asset_response_cache_config() {
        // Test that environment variables can override defaults
        unsafe {
            env::set_var("QUERY_ASSET_CACHE_MAX_CAPACITY", "5000000"); // 5MB
            env::set_var("QUERY_ASSET_CACHE_TIME_TO_IDLE", "1800"); // 30 minutes
            env::set_var("QUERY_ASSET_CACHE_TIME_TO_LIVE", "43200"); // 12 hours
        }

        let config = asset_response_cache_config();
        assert_eq!(config.max_capacity, 5000000);
        assert_eq!(config.time_to_idle, Duration::from_secs(1800));
        assert_eq!(config.time_to_live, Duration::from_secs(43200));

        // Clean up environment variables
        unsafe {
            env::remove_var("QUERY_ASSET_CACHE_MAX_CAPACITY");
            env::remove_var("QUERY_ASSET_CACHE_TIME_TO_IDLE");
            env::remove_var("QUERY_ASSET_CACHE_TIME_TO_LIVE");
        }
    }

    #[test]
    fn test_env_override_path_cache_config() {
        // Set environment variables before initializing cache
        unsafe {
            env::set_var("QUERY_PATH_CACHE_MAX_CAPACITY", "1000");
            env::set_var("QUERY_PATH_CACHE_TIME_TO_IDLE", "300");
            env::set_var("QUERY_PATH_CACHE_TIME_TO_LIVE", "600");
        }

        // Get a fresh config directly (not through cache())
        let config = path_cache_config();

        assert_eq!(config.time_to_idle.as_secs(), 300);
        assert_eq!(config.time_to_live.as_secs(), 600);

        // Clean up environment variables
        unsafe {
            env::remove_var("QUERY_PATH_CACHE_MAX_CAPACITY");
            env::remove_var("QUERY_PATH_CACHE_TIME_TO_IDLE");
            env::remove_var("QUERY_PATH_CACHE_TIME_TO_LIVE");
        }
    }

    #[test]
    fn test_env_override_function_cache_config() {
        // Set environment variables before initializing cache
        unsafe {
            env::set_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY", "1000");
            env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE", "300");
            env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE", "600");
        }

        // Get a fresh config directly (not through cache())
        let config = function_cache_config();

        assert_eq!(config.time_to_idle.as_secs(), 300);
        assert_eq!(config.time_to_live.as_secs(), 600);

        // Clean up environment variables
        unsafe {
            env::remove_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY");
            env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE");
            env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE");
        }
    }
}
