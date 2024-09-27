use std::{
    collections::HashMap,
    fmt::Write,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use futures_util::StreamExt;
use http_body_util::BodyStream;
use hyper::{body::Incoming, header::CONTENT_TYPE, http::HeaderName, Request, Response};
use multer::Multipart;
use query_runtime::{timers::TimerPoller, Runtime};
use rbase64::encode;
use regex::Regex;
use rquickjs::{async_with, Function, Module, Object, Promise, Value};
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::env::Env;
use crate::sqlite::connect_db::connect_cache_function_db;
use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        http_error::{internal_server_error, not_found, HttpError},
    },
    sqlite::connect_db::connect_function_db,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HandleResponse {
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub status: u16,
}

#[derive(Debug)]
struct CacheFunction {
    pub body: Vec<u8>,
    pub expires_at: usize,
    pub headers: String,
    pub status: u16,
}

#[instrument(err(Debug))]
pub async fn function(req: &mut Request<Incoming>) -> Result<Response<BoxBody>, HttpError> {
    let method = req.method().as_str();
    let cache_function_path = &req.uri().path().to_string();
    let mut path = req.uri().path().to_string();

    if Env::app() == "true" && !path.starts_with("/api") && !path.starts_with("/_/") {
        path.insert_str(0, "/pages");
    }

    path = path.replace("/_/function", "");

    if path.is_empty() {
        path = "/".to_string();
    }

    let method_lower_case = method.to_lowercase();

    if method == "GET" {
        if path == "/pages/" {
            path = "/pages".to_string();
        }

        let row_to_cache_function =
            |row: &rusqlite::Row| -> Result<CacheFunction, rusqlite::Error> {
                let body: String = row.get(0)?;
                let expires_at: usize = row.get(1)?;
                let headers: String = row.get(2)?;
                let status: u16 = row.get(3)?;

                Ok(CacheFunction {
                    body: body.as_bytes().to_vec(),
                    expires_at,
                    headers,
                    status,
                })
            };

        let cache_function: CacheFunction = match connect_cache_function_db()?.query_row(
            r#"
                SELECT
                    body,
                    expires_at,
                    headers,
                    status
                FROM
                    cache_function
                WHERE
                    path = :path
            "#,
            named_params! {
                ":path": cache_function_path,
            },
            row_to_cache_function,
        ) {
            Ok(v) => v,
            Err(e) => {
                tracing::info!("No Cached: {} - {:?}", cache_function_path, e);

                CacheFunction {
                    body: vec![],
                    expires_at: 0,
                    headers: "".to_string(),
                    status: 0,
                }
            }
        };

        if cache_function.expires_at > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|t| t.as_millis() as usize)
                .unwrap_or(0);

            if now < cache_function.expires_at {
                let mut response = match Response::builder()
                    .status(cache_function.status)
                    .body(Body::from(cache_function.body))
                {
                    Ok(r) => Ok(r),
                    Err(e) => Err(internal_server_error(e.to_string())),
                }?;

                let headers: HashMap<String, String> =
                    serde_json::from_str(&cache_function.headers)
                        .map_err(|e| internal_server_error(e.to_string()))?;

                for (key, value) in headers {
                    let key = key.to_uppercase();

                    response.headers_mut().insert(
                        HeaderName::from_bytes(key.as_bytes()).unwrap(),
                        value.parse().unwrap(),
                    );
                }

                response.headers_mut().insert(
                    HeaderName::from_bytes("Query-Cache-Hit".as_bytes()).unwrap(),
                    "true".parse().unwrap(),
                );

                return Ok(response);
            } else if cache_function.status != 0 && is_primary() {
                let cache_function_path_cloned = cache_function_path.clone();

                thread::spawn(move || -> Result<()> {
                    match connect_cache_function_db()?.execute(
                        r#"DELETE FROM cache_function WHERE path = ?"#,
                        [&cache_function_path_cloned],
                    ) {
                        Ok(_) => tracing::info!("Cache Deleted: {}", cache_function_path_cloned),
                        Err(e) => tracing::error!("Error: {:?}", e),
                    };

                    Ok(())
                });
            }
        }
    }

    let path = path_match(&path, method)?;

    let row_to_function = |row: &Row| -> Result<String, rusqlite::Error> {
        let function: Vec<u8> = row.get(0)?;
        let function = String::from_utf8(function).unwrap();

        Ok(function)
    };

    let function: String = match connect_function_db()?.query_row(
        r#"
            SELECT
                function
            FROM
                function
            WHERE
                path = :path
            AND
                method = :method
            AND
                active = :active
        "#,
        named_params! {
            ":active": 1,
            ":method": method,
            ":path": path,
        },
        row_to_function,
    ) {
        Ok(v) => Ok(v),
        Err(_) => Err(not_found()),
    }?;

    let mut headers: HashMap<String, String> = HashMap::new();

    for (key, value) in req.headers() {
        // NOTE: workaround to fix an error in the js-engine caused by sec-ch-ua
        headers.insert(
            key.as_str().to_lowercase(),
            value.to_str().unwrap().to_string().replace('"', "'"),
        );
    }

    let req_headers = req.headers().clone();
    let is_multipart = match req_headers.get(CONTENT_TYPE) {
        Some(ct) => match ct.to_str() {
            Ok(ct_str) => ct_str.starts_with("multipart/form-data"),
            Err(_) => false,
        },
        None => false,
    };

    let body_mount = req.body_mut();
    let body = if is_multipart {
        let boundary = req_headers
            .get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .and_then(|ct| multer::parse_boundary(ct).ok());
        let multipart = if let Some(boundary) = boundary {
            process_multipart(body_mount, boundary).await?
        } else {
            return Err(internal_server_error("Boundary not found".to_string()));
        };
        multipart.as_bytes().to_vec()
    } else {
        let bytes = Body::to_bytes(body_mount).await?;
        bytes.to_vec()
    };

    let host = req.headers().get("host").unwrap();
    let host = host.to_str().unwrap();
    let uri = req.uri().to_string();
    let scheme = if host.starts_with("localhost") || host.starts_with("0.0.0.0") {
        "http"
    } else {
        "https"
    };

    let ctx = match Runtime::new().await {
        Ok(r) => Ok(r.ctx),
        Err(e) => Err(internal_server_error(e.to_string())),
    }?;
    let module_name = format!("{}::{}", path, method_lower_case);
    let method_str = req.method().as_str();
    let url = format!("{}://{}{}", scheme, host, uri);

    let handle_response = format!(
        r#"
        import 'polyfill/blob';
        import 'polyfill/console';
        import 'polyfill/fetch';
        import 'polyfill/file';
        import 'polyfill/form-data';
        import 'polyfill/request';
        import 'polyfill/response';
        import 'polyfill/web-streams';

        import 'js/database';
        import 'js/handle-response';

        {function}
        "#,
    );

    let res = async_with!(ctx => |ctx| {
        let module = match Module::declare(ctx.clone(), module_name, handle_response) {
            Ok(m) => m,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            }
        };

        let _ = match module.eval() {
            Ok(m) => m,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

        let global_this: Object = match ctx.clone().globals().get("globalThis") {
            Ok(o) => o,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };
        let handle_response: Function = match global_this.get("___handleResponse") {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

        let promise: Promise = match handle_response.call((headers, method_str, url, body)) {
            Ok(o) => o,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

        loop {
            if !ctx.poll_timers() {
                break;
            }
        }

        let response: Object = match promise.into_future().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

        let body: Value = response.get("body").unwrap();
        let body = body.as_string().unwrap();
        let body = body.to_string().unwrap();
        let body = if body.is_empty() { None } else { Some(body) };

        let headers = response.get("headers").unwrap();
        let status = response.get("status").unwrap();

        HandleResponse {
            body,
            headers,
            status
        }
    })
    .await;

    let body = res.body.unwrap_or_default();
    let cloned_body = body.clone();

    let mut response = match Response::builder()
        .status(res.status as u16)
        .body(Body::from(body))
    {
        Ok(r) => Ok(r),
        Err(e) => Err(internal_server_error(e.to_string())),
    }?;

    if let Some(headers) = res.headers {
        for (key, value) in headers {
            let key = key.to_uppercase();

            response.headers_mut().insert(
                HeaderName::from_bytes(key.as_bytes()).unwrap(),
                value.parse().unwrap(),
            );
        }
    }

    let status = response.status().as_u16().to_string();

    if response.headers().contains_key("query-cache-control")
        && status.starts_with('2')
        && is_primary()
    {
        let query_cache_control = response.headers().get("query-cache-control");

        if query_cache_control.is_some() {
            let content: &str = query_cache_control.unwrap().to_str().unwrap();
            let re = Regex::new(r"max-age=(\d+)").unwrap();
            let max_age = re
                .captures(content)
                .and_then(|captures| captures.get(1))
                .map_or(0, |max_age_value| {
                    max_age_value.as_str().parse::<u32>().unwrap_or(0)
                });

            if max_age > 0 {
                let expires_at = now() + (max_age as usize);
                let headers = response
                    .headers()
                    .iter()
                    .fold(String::new(), |acc, (key, value)| {
                        format!(r#"{}"{}":"{}","#, acc, key, value.to_str().unwrap())
                    });
                let headers = headers.trim_end_matches(',');
                let headers = format!("{{{}}}", headers);

                let cache_function_path_cloned = cache_function_path.clone();

                thread::spawn(move || -> Result<()> {
                    match connect_cache_function_db()?.execute(
                        r#"
                            INSERT OR IGNORE INTO cache_function (path, body, expires_at, headers, status)
                            VALUES (?, ?, ?, ?, ?)
                            "#,
                        [&cache_function_path_cloned, &cloned_body, &expires_at.to_string(), &headers, &status],
                    ) {
                        Ok(_) => tracing::info!("Cache Created: {}", cache_function_path_cloned),
                        Err(e) => tracing::error!("Error: {:?}", e),
                    };

                    Ok(())
                });
            }
        }
    }

    Ok(response)
}

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t.as_millis() as usize)
        .unwrap_or(0)
}

fn path_match(path: &str, method: &str) -> Result<String> {
    let path = if path.ends_with('/') && path != "/" {
        path.trim_end_matches('/')
    } else {
        path
    };
    let connect = connect_function_db()?;
    let mut stmt = connect.prepare(
        r#"
            SELECT
                path
            FROM
                function
            WHERE
                method = :method
            AND
                active = :active
            ORDER BY path DESC
        "#,
    )?;
    let rows = stmt.query_map(
        named_params! {
            ":active": 1,
            ":method": method,
        },
        |row| row.get(0),
    )?;
    let paths: Vec<String> = rows.map(|r| r.unwrap()).collect();

    let mut result = String::new();

    for item in paths {
        if item == path {
            result = item.to_string();
            break;
        } else {
            let item_parts: Vec<&str> = item.split('/').collect();
            let path_parts: Vec<&str> = path.split('/').collect();

            if item_parts.len() != path_parts.len() {
                continue;
            }

            let is_equivalent = item_parts.iter().enumerate().all(|(i, item)| {
                if &path_parts[i] == item {
                    true
                } else {
                    item.starts_with(':')
                }
            });

            if is_equivalent {
                result = item.to_string();
                break;
            }
        }
    }

    Ok(result)
}

fn is_primary() -> bool {
    let path = format!("{}/.primary", Env::dbs_path());

    if std::path::Path::new(&path).exists() {
        return false;
    }

    true
}

fn handle_fatal_error() -> HandleResponse {
    HandleResponse {
        body: None,
        headers: None,
        status: 500,
    }
}

// NOTE: This function is a workaround to fix an issue when converting the body of a multipart to formData in the Request object.
async fn process_multipart(body: &mut Incoming, boundary: String) -> anyhow::Result<String> {
    let body_stream = BodyStream::new(body)
        .filter_map(|result| async move { result.map(|frame| frame.into_data().ok()).transpose() });

    let mut multipart = Multipart::new(body_stream, boundary.clone());

    let mut reconstructed = String::new();
    writeln!(reconstructed, "--{}", boundary)?;

    while let Some(mut field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().map(|s| s.to_string());
        let content_type = field.content_type().map(|s| s.to_string());
        let is_file = file_name.is_some();

        write!(
            reconstructed,
            "Content-Disposition: form-data; name=\"{}\"",
            name
        )?;
        if let Some(filename) = &file_name {
            write!(reconstructed, "; filename=\"{}\"", filename)?;
        }
        writeln!(reconstructed)?;

        if let Some(ct) = &content_type {
            writeln!(reconstructed, "Content-Type: {}", ct)?;
        }
        writeln!(reconstructed)?;

        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk?;
            if is_file {
                let chunk = chunk.iter().copied().collect::<Vec<u8>>();
                data.extend_from_slice(&chunk);
            } else {
                data.extend_from_slice(&chunk);
            }
        }

        if is_file {
            reconstructed.push_str(&encode(&data));
        } else {
            reconstructed.push_str(&String::from_utf8_lossy(&data));
        }

        writeln!(reconstructed)?;
        writeln!(reconstructed, "--{}", boundary)?;
    }

    write!(reconstructed, "--")?;

    Ok(reconstructed)
}
