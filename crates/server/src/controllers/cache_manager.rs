use std::{
    env,
    str::FromStr,
    sync::{
        atomic::{AtomicI64, Ordering},
        LazyLock, OnceLock,
    },
    time::Duration,
};

use tracing::instrument;

use crate::sqlite::connect_db::connect_cache_invalidation_db;

use super::cache_response::{CacheResponse, CacheResponseConfig};

static LAST_KNOWN_INVALIDATION: LazyLock<AtomicI64> = LazyLock::new(|| AtomicI64::new(0));

const DEFAULT_ASSET_CACHE_MAX_CAPACITY: u64 = 25 * 1024 * 1024; // 25 MB
const DEFAULT_ASSET_CACHE_TIME_TO_IDLE: u64 = 86400; // 1 day
const DEFAULT_ASSET_CACHE_TIME_TO_LIVE: u64 = 2592000; // 30 days

const DEFAULT_FUNCTION_CACHE_MAX_CAPACITY: u64 = 25 * 1024 * 1024; // 25 MB
const DEFAULT_FUNCTION_CACHE_TIME_TO_IDLE: u64 = 3600; // 1 hour
const DEFAULT_FUNCTION_CACHE_TIME_TO_LIVE: u64 = 86400; // 1 day

#[derive(Clone, Copy, Debug)]
pub enum CacheType {
    Asset,
    Function,
}

static CACHES: [OnceLock<CacheResponse>; 2] = [OnceLock::new(), OnceLock::new()];

fn env<T: FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn asset_cache_config() -> CacheResponseConfig {
    CacheResponseConfig {
        max_capacity: env(
            "QUERY_ASSET_CACHE_MAX_CAPACITY",
            DEFAULT_ASSET_CACHE_MAX_CAPACITY,
        ),
        time_to_idle: Duration::from_secs(env(
            "QUERY_ASSET_CACHE_TIME_TO_IDLE",
            DEFAULT_ASSET_CACHE_TIME_TO_IDLE,
        )),
        time_to_live: Duration::from_secs(env(
            "QUERY_ASSET_CACHE_TIME_TO_LIVE",
            DEFAULT_ASSET_CACHE_TIME_TO_LIVE,
        )),
    }
}

fn function_cache_config() -> CacheResponseConfig {
    CacheResponseConfig {
        max_capacity: env(
            "QUERY_FUNCTION_CACHE_MAX_CAPACITY",
            DEFAULT_FUNCTION_CACHE_MAX_CAPACITY,
        ),
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

pub fn cache(cache_type: CacheType) -> &'static CacheResponse {
    if let Err(e) = check_invalidations(cache_type) {
        tracing::error!("Error checking cache invalidations: {}", e);
    }

    match cache_type {
        CacheType::Asset => CACHES[0].get_or_init(|| CacheResponse::new(asset_cache_config())),
        CacheType::Function => {
            CACHES[1].get_or_init(|| CacheResponse::new(function_cache_config()))
        }
    }
}

#[instrument(err(Debug))]
fn check_invalidations(cache_type: CacheType) -> Result<bool, anyhow::Error> {
    let conn = connect_cache_invalidation_db()?;
    let last_known = LAST_KNOWN_INVALIDATION.load(Ordering::Acquire);

    let latest_invalidation: Option<i64> =
        conn.query_row("SELECT version FROM cache_invalidation;", [], |row| {
            row.get(0)
        })?;

    tracing::debug!("Last known invalidation: {}", last_known);
    tracing::debug!("Latest invalidation: {:?}", latest_invalidation);

    if let Some(latest) = latest_invalidation {
        eprintln!("latest > last_known: {:?}", latest > last_known);

        if latest > last_known {
            LAST_KNOWN_INVALIDATION.store(latest, Ordering::Release);
            clear_cache(cache_type);
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn clear_cache(cache_type: CacheType) {
    match cache_type {
        CacheType::Asset => {
            if let Some(cache) = CACHES[0].get() {
                cache.clear();
            }
        }
        CacheType::Function => {
            if let Some(cache) = CACHES[1].get() {
                cache.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, time::Duration};

    use hyper::HeaderMap;

    use crate::controllers::cache_response::CacheResponseValue;
    use crate::sqlite::connect_db::connect_cache_invalidation_db;
    use crate::sqlite::create_cache_invalidation_db::create_cache_invalidation_db;

    use super::*;

    const TEST_DB_PATH: &str = "../../.tests/cache_invalidation_test";

    struct Cleanup;
    impl Drop for Cleanup {
        fn drop(&mut self) {
            fs::remove_dir_all(TEST_DB_PATH).unwrap();
        }
    }

    #[test]
    fn test_asset_cache() {
        let cache = cache(CacheType::Asset);

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
        clear_cache(CacheType::Asset);
        cache.sync();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_function_cache() {
        let cache = cache(CacheType::Function);

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
        clear_cache(CacheType::Function);
        cache.sync();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_invalidation() {
        env::set_var("QUERY_SERVER_DBS_PATH", TEST_DB_PATH);
        fs::create_dir_all(TEST_DB_PATH).unwrap();

        let _cleanup = Cleanup;

        create_cache_invalidation_db();
        let conn = connect_cache_invalidation_db().unwrap();

        conn.execute("INSERT INTO cache_invalidation DEFAULT VALUES;", [])
            .unwrap();

        let cache_type = CacheType::Function;
        assert!(check_invalidations(cache_type).unwrap());
        assert!(!check_invalidations(cache_type).unwrap());
    }

    #[test]
    fn test_asset_cache_config() {
        let cache = cache(CacheType::Asset);
        assert_eq!(cache.config.max_capacity, DEFAULT_ASSET_CACHE_MAX_CAPACITY);
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_ASSET_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_ASSET_CACHE_TIME_TO_LIVE)
        );
    }

    #[test]
    fn test_function_cache_config() {
        let cache = cache(CacheType::Function);
        assert_eq!(
            cache.config.max_capacity,
            DEFAULT_FUNCTION_CACHE_MAX_CAPACITY
        );
        assert_eq!(
            cache.config.time_to_idle,
            Duration::from_secs(DEFAULT_FUNCTION_CACHE_TIME_TO_IDLE)
        );
        assert_eq!(
            cache.config.time_to_live,
            Duration::from_secs(DEFAULT_FUNCTION_CACHE_TIME_TO_LIVE)
        );
    }

    #[test]
    fn test_cache_types() {
        let asset_cache = cache(CacheType::Asset);
        let function_cache = cache(CacheType::Function);

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

        assert!(asset_cache.len() > 0);

        // Test Function cache
        function_cache.insert(
            "func1".to_string(),
            CacheResponseValue {
                body: b"function data".to_vec(),
                headers: HeaderMap::new(),
            },
        );
        function_cache.sync();

        assert!(function_cache.len() > 0);

        // Clear specific cache type
        clear_cache(CacheType::Asset);
        asset_cache.sync();

        assert_eq!(asset_cache.len(), 0);

        clear_cache(CacheType::Function);
        function_cache.sync();
        assert_eq!(function_cache.len(), 0);
    }

    #[test]
    fn test_env_override_function_cache_config() {
        // Test that environment variables can override defaults
        env::set_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY", "5000000"); // 5MB
        env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE", "1800"); // 30 minutes
        env::set_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE", "43200"); // 12 hours

        let config = function_cache_config();
        assert_eq!(config.max_capacity, 5000000);
        assert_eq!(config.time_to_idle, Duration::from_secs(1800));
        assert_eq!(config.time_to_live, Duration::from_secs(43200));

        // Clean up environment variables
        env::remove_var("QUERY_FUNCTION_CACHE_MAX_CAPACITY");
        env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_IDLE");
        env::remove_var("QUERY_FUNCTION_CACHE_TIME_TO_LIVE");
    }
}
