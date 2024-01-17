use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use anyhow::Result;
use hyper::{
    body::Incoming,
    header::{
        HeaderValue, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, STRICT_TRANSPORT_SECURITY,
        X_CONTENT_TYPE_OPTIONS,
    },
    Method, Request, Response, StatusCode,
};
use rusqlite::named_params;
use serde::Deserialize;
use tracing::instrument;

use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        http_error::{not_found, HttpError},
    },
    sqlite::connect_db::connect_asset_db,
};

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
            let asset_name = segments.last().unwrap();

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
                |row| {
                    let data: Vec<u8> = row.get(0)?;
                    let name: String = row.get(1)?;
                    let mime_type: String = row.get(2)?;

                    Ok(Asset {
                        data,
                        name_hashed: name.to_string(),
                        mime_type,
                        name,
                    })
                },
            ) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Err(not_found());
                }
            };

            let data_owned = asset.data.to_owned();
            let body = Body::from(data_owned.to_owned());

            let mut res = Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap();
            let re = regex::Regex::new(r"-(\d+)\.[a-z0-9]{2,6}$").unwrap();
            let asset_name = segments[0..].join("/");
            let hash = re.captures(&asset_name);
            let has_hash = hash.is_some();

            let headers = res.headers_mut();

            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_bytes(asset.mime_type.as_bytes()).unwrap(),
            );

            let length = data_owned.len().to_string();
            headers.insert(
                CONTENT_LENGTH,
                HeaderValue::from_bytes(length.as_bytes()).unwrap(),
            );

            if has_hash || asset_name.contains("/cache/") {
                let etag = if has_hash {
                    hash.unwrap().get(1).unwrap().as_str().to_string()
                } else {
                    let data_bytes: &[u8] = &data_owned;
                    let mut hasher = DefaultHasher::new();
                    Hash::hash_slice(data_bytes, &mut hasher);
                    let etag = hasher.finish();

                    etag.to_string()
                };

                headers.insert(ETAG, HeaderValue::from_str(&etag).unwrap());
                headers.insert(
                    CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000000, immutable"),
                );
                headers.insert(
                    STRICT_TRANSPORT_SECURITY,
                    HeaderValue::from_static("max-age=31536000000"),
                );
                headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            } else {
                let data_bytes: &[u8] = &data_owned;
                let mut hasher = DefaultHasher::new();
                Hash::hash_slice(data_bytes, &mut hasher);
                let etag = hasher.finish();
                let etag_str = etag.to_string();

                headers.insert(ETAG, HeaderValue::from_str(&etag_str).unwrap());
                headers.insert(
                    CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=300, must-revalidate"),
                );
                headers.insert(
                    STRICT_TRANSPORT_SECURITY,
                    HeaderValue::from_static("max-age=300"),
                );
                headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            }

            Ok(res)
        }
        _ => Err(not_found()),
    }
}
