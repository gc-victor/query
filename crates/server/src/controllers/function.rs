use std::{
    collections::HashMap,
    fmt::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::StreamExt;
use http_body_util::BodyStream;
use hyper::header::HOST;
use hyper::HeaderMap;
use hyper::{
    body::Incoming, header::CONTENT_TYPE, http::HeaderName, Request, Response, StatusCode,
};
use multer::Multipart;
use query_runtime::{poll_timers, Runtime};
use rbase64::encode;
use regex::Regex;
use rquickjs::{async_with, qjs, Function, Module, Object, Promise, Value};
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use tracing::instrument;

use crate::{
    controllers::{
        cache_manager::{cache, cache_response, CacheResponseType, CacheType},
        cache_response::CacheResponseValue,
        utils::{
            body::{Body, BoxBody},
            http_error::{internal_server_error, not_found, HttpError},
        },
    },
    env::Env,
    sqlite::connect_db::connect_function_db,
};

const HEADER_CACHE_CONTROL: &str = "query-cache-control";
const HEADER_CACHE_EXPIRES_AT: &str = "query-cache-expires-at";
const HEADER_CACHE_HIT: &str = "query-cache-hit";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HandleResponse {
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub status: u16,
}

#[instrument(err(Debug))]
pub async fn function(req: &mut Request<Incoming>) -> Result<Response<BoxBody>, HttpError> {
    let method = req.method().as_str().to_string();
    let mut path = req.uri().path().to_string();
    let path_and_query = match req.uri().path_and_query() {
        Some(path_and_query) => path_and_query.to_string(),
        None => "".to_string(),
    };
    let path_and_query = if path_and_query.is_empty() {
        path.to_string()
    } else {
        path_and_query
    };
    let function_cache_key = format!("{}{}", method, path_and_query);
    let function_response_cache_key = format!("res-{}{}", method, path_and_query);

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

        let function_response_cache = cache_response(CacheResponseType::Function);

        if let Some(cached_response) = function_response_cache.get(&function_response_cache_key) {
            if let Some(response) = check_cached_response(&cached_response) {
                return Ok(response);
            }
        }
    }

    let path = path_match(&path, &method)?;

    let row_to_function = |row: &Row| -> Result<String, rusqlite::Error> {
        let function: Vec<u8> = row.get(0)?;
        let function = String::from_utf8(function).unwrap();
        Ok(function)
    };

    let function_cache = cache(CacheType::Function);

    let function = if let Some(function) = function_cache.get(&function_cache_key) {
        function
    } else {
        static QUERY_SELECT_FUNCTION: &str = r#"
            SELECT
                function
            FROM
                function
            WHERE
                method = :method
            AND
                active = :active
            AND
                path = :path
        "#;
        let function: String = match connect_function_db()?
            .prepare_cached(QUERY_SELECT_FUNCTION)?
            .query_row(
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

        function_cache.insert(function_cache_key, function.clone());
        function
    };

    let mut headers: HashMap<String, String> = HashMap::new();

    let req_headers = req.headers().clone();
    for (key, value) in req_headers.iter() {
        // NOTE: workaround to fix an error in the js-engine caused by sec-ch-ua
        headers.insert(
            key.as_str().to_lowercase(),
            value.to_str().unwrap().to_string().replace('"', "'"),
        );
    }

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

    let host = req_headers
        .get(HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let uri = req.uri().to_string();
    let scheme = if host.starts_with("localhost") || host.starts_with("0.0.0.0") {
        "http"
    } else {
        "https"
    };

    let module_name = format!("{}::{}", path, method_lower_case);
    let method_str = method.as_str();
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
        import 'js/jsx-helpers';

        {function}
        "#,
    );

    let ctx = match Runtime::new().await {
        Ok(r) => Ok(r.ctx),
        Err(e) => Err(internal_server_error(e.to_string())),
    }?;
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

        let response: Object = match promise.into_future().await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

        let rt = unsafe { qjs::JS_GetRuntime(ctx.as_raw().as_ptr()) };
        let mut deadline = Instant::now();
        let mut executing_timers = Vec::new();

        while poll_timers(rt, &mut executing_timers, None, Some(&mut deadline)).map_err(|e| {
                tracing::error!("Error: {}", e);
                handle_fatal_error();
            }).unwrap_or(false) {
            ctx.execute_pending_job();
        }

        let body: Value = match response.get("body"){
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };
        let body = match body.as_string() {
            Some(v) => v,
            None => {
                tracing::error!("Error: body could not be converted to string");
                return handle_fatal_error();
            },
        };
        let body = match body.to_string() {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };
        let body = if body.is_empty() { None } else { Some(body) };

        let headers = match response.get("headers") {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };
        let status = match response.get("status") {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Error: {}", e);
                return handle_fatal_error();
            },
        };

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

    let mut headers_map = HeaderMap::new();
    if let Some(headers) = res.headers {
        let re = match Regex::new(r"max-age=(\d+)") {
            Ok(r) => r,
            Err(e) => return Err(internal_server_error(e.to_string())),
        };

        for (key, value) in headers {
            let key = key.to_uppercase();
            let header_name = HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| internal_server_error(e.to_string()))?;
            let header_value = value
                .parse::<hyper::header::HeaderValue>()
                .map_err(|e| internal_server_error(e.to_string()))?;

            if HEADER_CACHE_CONTROL.to_uppercase() == key {
                let max_age = {
                    let captures = match re.captures(&value) {
                        Some(c) => c,
                        None => continue,
                    };
                    match captures.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
                        Some(age) => age,
                        None => continue,
                    }
                };
                let expires_at = match (now() + max_age)
                    .to_string()
                    .parse::<hyper::header::HeaderValue>()
                {
                    Ok(v) => Ok(v),
                    Err(e) => Err(internal_server_error(e.to_string())),
                }?;

                headers_map.insert(HEADER_CACHE_EXPIRES_AT, expires_at);
            };

            headers_map.insert(&header_name, header_value.clone());
            response.headers_mut().insert(header_name, header_value);
        }
    }

    let status = response.status().as_u16().to_string();

    if method == "GET"
        && response.headers().contains_key(HEADER_CACHE_CONTROL)
        && status.starts_with('2')
    {
        let cache = cache_response(CacheResponseType::Function);

        cache.insert(
            function_response_cache_key,
            CacheResponseValue {
                body: cloned_body.into_bytes(),
                headers: headers_map,
            },
        );
    }

    Ok(response)
}

fn check_cached_response(cached_response: &CacheResponseValue) -> Option<Response<BoxBody>> {
    if !cached_response
        .headers
        .contains_key(HEADER_CACHE_EXPIRES_AT)
    {
        return None;
    }

    let expires_at = cached_response
        .headers
        .get(HEADER_CACHE_EXPIRES_AT)?
        .to_str()
        .ok()?
        .parse::<u64>()
        .ok()?;

    if now() >= expires_at {
        return None;
    }

    let mut response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(cached_response.body.clone()))
        .ok()?;

    let headers = response.headers_mut();
    for (key, value) in cached_response.headers.iter() {
        if key != HEADER_CACHE_EXPIRES_AT {
            headers.insert(key, value.clone());
        }
    }

    headers.insert(
        HeaderName::from_static(HEADER_CACHE_HIT),
        "true".parse().unwrap(),
    );

    Some(response)
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t.as_millis() as u64)
        .unwrap_or(0)
}

fn path_match(path: &str, method: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = if path.ends_with('/') && path != "/" {
        path.trim_end_matches('/')
    } else {
        path
    };

    let path_cache = cache(CacheType::Path);

    if let Some(cached_path) = path_cache.get(&format!("{method}:{path}")) {
        return Ok(cached_path);
    }

    static QUERY_PATH: &str = r#"
        SELECT
            path
        FROM
            function
        WHERE
            method = :method
        AND
            active = :active
        ORDER BY path DESC
    "#;
    let connect = connect_function_db()?;
    let mut stmt = connect.prepare_cached(QUERY_PATH)?;
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

    if !result.is_empty() {
        path_cache.insert(format!("{method}:{path}"), result.clone());
    }

    Ok(result)
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
