use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use anyhow::Result;
use hyper::{
    body::Incoming,
    header::{
        HeaderName, HeaderValue, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS
    },
    Method, Request, Response, StatusCode,
};
use rusqlite::{named_params, Row};
use serde::Deserialize;
use tracing::instrument;

use crate::{
    controllers::{
        cache_manager::{self, CacheType},
        utils::{
            body::{Body, BoxBody},
            http_error::{internal_server_error, not_found, HttpError},
        },
    },
    sqlite::connect_db::connect_asset_db,
};

use super::cache_response::CacheResponseValue;

const HEADER_CACHE_HIT: &str = "query-cache-hit";

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Asset {
    pub data: Vec<u8>,
    pub name: String,
    pub name_hashed: String,
    pub mime_type: String,
}

#[instrument(err(Debug), skip(req))]
pub async fn asset(
    req: &mut Request<Incoming>,
    segments: &[&str],
) -> Result<Response<BoxBody>, HttpError> {
    match req.method() {
        &Method::GET => {
            let asset_name = segments[1..].join("/");
            let cache = cache_manager::cache(CacheType::Asset);

            if let Some(cache) = cache.get(&asset_name) {
                tracing::info!("Cache hit for asset: {}", asset_name);

                let body = Body::from(cache.body.clone());
                let mut res = Response::builder()
                    .status(StatusCode::OK)
                    .body(body)
                    .unwrap();

                let headers = res.headers_mut();
                let cache_headers = &cache.headers;
                for (key, value) in cache_headers.iter() {
                    headers.insert(key, value.clone());
                }

                headers.insert(
                    HeaderName::from_static(HEADER_CACHE_HIT),
                    "true".parse().unwrap(),
                );

                return Ok(res);
            }

            let row_to_asset = |row: &Row| -> Result<Asset, rusqlite::Error> {
                let data: Vec<u8> = row.get(0)?;
                let name: String = row.get(1)?;
                let mime_type: String = row.get(2)?;

                Ok(Asset {
                    data,
                    name_hashed: name.to_string(),
                    mime_type,
                    name,
                })
            };

            let asset: Asset = match connect_asset_db()?.query_row(
                r#"
                    SELECT
                        data,
                        name,
                        mime_type
                    FROM
                        asset
                    WHERE
                        name_hashed = :name
                    OR
                        name = :name
                    AND
                        active = 1;
                "#,
                named_params! {
                    ":name": asset_name,
                },
                row_to_asset,
            ) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err(not_found());
                }
            };

            let data_owned = asset.data.to_owned();
            let re = regex::Regex::new(r"-(\d+)\.[a-z0-9]{2,6}$").unwrap();
            let hash = re.captures(&asset_name);
            let has_hash = hash.is_some();

            let (etag, cache_control, strict_transport_security) =
                if has_hash || asset_name.contains("/cache/") {
                    let etag = if has_hash {
                        hash.unwrap().get(1).unwrap().as_str().to_string()
                    } else {
                        let data_bytes: &[u8] = &data_owned;
                        let mut hasher = DefaultHasher::new();
                        Hash::hash_slice(data_bytes, &mut hasher);
                        hasher.finish().to_string()
                    };

                    (
                        etag,
                        "public, max-age=31536000000, immutable".to_string(),
                        "max-age=31536000000".to_string(),
                    )
                } else {
                    let data_bytes: &[u8] = &data_owned;
                    let mut hasher = DefaultHasher::new();
                    Hash::hash_slice(data_bytes, &mut hasher);
                    (
                        hasher.finish().to_string(),
                        "public, max-age=300, must-revalidate".to_string(),
                        "max-age=300".to_string(),
                    )
                };

            let body = Body::from(data_owned.clone());
            let mut res = Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .map_err(|e| internal_server_error(e.to_string()))?;

            let headers = res.headers_mut();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(&asset.mime_type).unwrap(),
            );
            headers.insert(
                CONTENT_LENGTH,
                HeaderValue::from_str(&asset.data.len().to_string()).unwrap(),
            );
            headers.insert(ETAG, HeaderValue::from_str(&etag).unwrap());
            headers.insert(
                CACHE_CONTROL,
                HeaderValue::from_str(&cache_control).unwrap(),
            );
            headers.insert(
                STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_str(&strict_transport_security).unwrap(),
            );
            headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));

            cache.insert(
                asset_name,
                CacheResponseValue {
                    body: data_owned.clone(),
                    headers: headers.clone(),
                },
            );

            Ok(res)
        }
        _ => Err(not_found()),
    }
}
