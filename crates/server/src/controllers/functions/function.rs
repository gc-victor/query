use std::collections::HashMap;

use anyhow::Result;
use hyper::{body::Incoming, header::CONTENT_TYPE, http::HeaderName, Method, Request, Response};
use regex::Regex;
use rquickjs::promise::Promise;
use rquickjs::{async_with, ArrayBuffer, Function, Object, Value};
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_json::json;
use tracing::instrument;

use crate::env::Env;
use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        http_error::{bad_request, internal_server_error, not_found, HttpError},
    },
    sqlite::connect_db::connect_function_db,
};

use super::runtime::runtime;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HandleResponse {
    pub body_used: bool,
    pub body: Option<ByteBuf>,
    pub headers: Option<HashMap<String, String>>,
    pub ok: bool,
    pub redirected: bool,
    pub status_text: String,
    pub status: u16,
    pub r#type: String,
    pub url: String,
}

#[instrument(err(Debug))]
pub async fn function(req: &mut Request<Incoming>) -> Result<Response<BoxBody>, HttpError> {
    let method = req.method().as_str();
    let mut path = req.uri().path().to_string();

    if Env::app() == "true" && !path.starts_with("/api") && !path.starts_with("/_/") {
        path.insert_str(0, "/pages");
    }

    path = path.replace("/_/function", "");

    if path.is_empty() {
        path = "/".to_string();
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

    let mut headers: Vec<String> = vec![];
    let mut boundary = String::new();

    for (key, value) in req.headers() {
        let key = key.as_str().to_lowercase();

        if key == "content-type" {
            let content_type_header = req.headers().get(CONTENT_TYPE).unwrap();
            let content_type = content_type_header
                .to_str()
                .map_err(|e| internal_server_error(e.to_string()))?;

            if content_type.contains("multipart/form-data") {
                boundary = content_type
                    .split(';')
                    .find_map(|part| part.trim().strip_prefix("boundary="))
                    .ok_or_else(|| bad_request("Can't find the boundary".to_string()))?
                    .to_string();
            }
        }

        headers.push(format!(
            r#""{key}": "{}""#,
            // NOTE: workaround to fix an error in the js-engine caused by sec-ch-ua
            value.to_str().unwrap().to_string().replace('"', "'"),
        ));
    }

    let body_multi_part = if !boundary.is_empty() {
        let bytes = Body::to_bytes(req.body_mut()).await?;
        let body_bytes = bytes.to_vec();
        let body = String::from_utf8_lossy(&body_bytes);

        formdata_to_json(&body, &boundary)?
    } else {
        "{}".to_string()
    };

    let body = if req.method() != Method::GET && boundary.is_empty() {
        let bytes = Body::to_bytes(req.body_mut()).await?;
        let body = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");

        format!("`{}`", body)
    } else {
        "''".to_string()
    };

    let host = req.headers().get("host").unwrap();
    let host = host.to_str().unwrap();
    let uri = req.uri().to_string();
    let scheme = if host.starts_with("localhost") || host.starts_with("0.0.0.0") {
        "http"
    } else {
        "https"
    };

    let handle_response = format!(
        r#"
        import {{ ___handleResponse as ___hr }} from 'js/handle-response';
        import 'js/sqlite';
        import 'polyfill/fetch';
        import 'polyfill/file';
        import 'polyfill/form-data';
        import 'polyfill/request';   
        import 'polyfill/response';   
        import {{
            ReadableStream,
            ReadableStreamBYOBReader,
            ReadableStreamDefaultReader,
            TransformStream,
            WritableStream,
            WritableStreamDefaultWriter,
        }} from 'polyfill/web-streams';

        globalThis.ReadableStream = ReadableStream;
        globalThis.ReadableStreamBYOBReader = ReadableStreamBYOBReader;
        globalThis.ReadableStreamDefaultReader = ReadableStreamDefaultReader;
        globalThis.TransformStream = TransformStream;
        globalThis.WritableStream = WritableStream;
        globalThis.WritableStreamDefaultWriter = WritableStreamDefaultWriter;

        {function}

        function ___handleRequestWrapper() {{
            const options = {{
                headers: {{ {headers} }},
                method: '{method}',
                url: '{url}',
            }};

            if (!/GET|HEAD/.test('{method}')) {{
                if (/multipart\/form-data/.test(options.headers['content-type'])) {{
                    const object = {body_multi_part};
                    const formData = new FormData();
            
                    for (const key in object) {{
                        let value = object[key];

                        try {{
                            const o = JSON.parse(value);
                            value = o && typeof o === "object" ? o : value;
                        }} catch {{}}

                        if (value.content && value.type && value.filename) {{
                            formData.append(key, new Blob([new Uint8Array(value.content).buffer], {{ type: value.type }}), value.filename);
                        }} else {{
                            formData.append(key, value);
                        }}
                    }}

                    options.body = formData;

                    // NOTE: it allows to the Request to create a new boundary
                    delete options.headers['content-type'];
                }} else {{
                    options.body = {body};
                }}
            }}

            const request = new Request('{url}', options);

            return ___handleRequest(request);
        }}

        export const ___handleResponse = ___hr.bind(null, ___handleRequestWrapper);
        "#,
        body = body,
        body_multi_part = body_multi_part,
        headers = headers.join(", "),
        method = req.method().as_str(),
        url = format!("{}://{}{}", scheme, host, uri),
    );

    let rt = runtime().await;
    let ctx = rt.ctx.clone();

    let res = async_with!(ctx => |ctx| {
        let m = ctx.compile("script", handle_response).unwrap();
        let func: Function = match m.get("___handleResponse") {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Error: {:?}", e);

                return HandleResponse {
                    body_used: false,
                    body: None,
                    headers: None,
                    ok: false,
                    redirected: false,
                    status: 500,
                    status_text: "Internal Server Error".to_string(),
                    r#type: "error".to_string(),
                    url: "".to_string(),
                };
            }
        };

        let promise: Promise<Object> = func.call(()).unwrap();
        let response = match promise.await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Error: {:?}", e.to_string());

                return HandleResponse {
                    body_used: false,
                    body: None,
                    headers: None,
                    ok: false,
                    redirected: false,
                    status: 500,
                    status_text: "Internal Server Error".to_string(),
                    r#type: "error".to_string(),
                    url: "".to_string(),
                };
            }
        };

        let body: Value = response.get("body").unwrap();
        let bytes = if body.is_object() {
            let body = body.as_object().unwrap();
            let buffer: ArrayBuffer = body.get("buffer").unwrap();

            buffer.as_bytes().unwrap().to_vec()
        } else {
            vec![]
        };

        let body_used = response.get("bodyUsed").unwrap();
        let headers = response.get("headers").unwrap();
        let ok = response.get("ok").unwrap();
        let redirected = response.get("redirected").unwrap();
        let status = response.get("status").unwrap();
        let status_text = response.get("statusText").unwrap();
        let r#type = response.get("type").unwrap();
        let url = response.get("url").unwrap();

        HandleResponse {
            body_used,
            body: Some(ByteBuf::from(bytes)),
            headers,
            ok,
            redirected,
            status,
            status_text,
            r#type,
            url,
        }
    })
    .await;

    let body = res
        .body
        .map(|b| String::from_utf8(b.to_vec()))
        .transpose()
        .map_err(|e| internal_server_error(e.to_string()))?
        .unwrap_or_default();

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

    Ok(response)
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

fn formdata_to_json(formdata: &str, boundary: &str) -> Result<String> {
    let boundary = format!("--{}", boundary);
    let mut map = HashMap::new();
    let parts: Vec<&str> = formdata.split(&boundary).collect();

    for part in parts {
        if !part.contains("name=") {
            continue;
        }

        let key_re = Regex::new(r#"; name="([^"]*)""#).unwrap();
        let captures: regex::Captures<'_> = key_re.captures(part).unwrap();
        let key = captures.get(1).unwrap().as_str().to_string();
        let re = Regex::new(r#"name="[^"]*"\r\n([\s\S]*)$"#).unwrap();
        let captures: regex::Captures<'_> = re.captures(part).unwrap();
        let value = captures.get(1).unwrap().as_str();
        let value = value.strip_prefix("\r\n").unwrap_or(value);
        let mut value = value.strip_suffix("\r\n").unwrap_or(value).to_string();

        if value.starts_with("Content-Type:") {
            let filename_re = Regex::new(r#"; filename="([^"]*)""#).unwrap();
            let content_type_re = Regex::new(r#"Content-Type: ([\w/]+)"#).unwrap();
            let content_re = Regex::new(r#"\r\n\r\n([\s\S]*)$"#).unwrap();

            let filename = filename_re
                .captures(part)
                .and_then(|caps| caps.get(1))
                .map_or("", |m| m.as_str());
            let content_type = content_type_re
                .captures(&value)
                .and_then(|caps| caps.get(1))
                .map_or("", |m| m.as_str());
            // NOTE: Workaround to receive binary data as it fails when isn't a valid UTF-8 string
            // It expects to receive a string, so we have to convert the binary data to a stringify-array and set it back to the formData.
            // Example:
            // ```javascript
            // const arrayBuffer = await file.arrayBuffer();
            // const uint8Array = new Uint8Array(arrayBuffer);
            // formData.set(fieldName, new Blob([JSON.stringify(Array.from(uint8Array))], { type: file.type }), file.name);
            // ```
            let content = content_re
                .captures(&value)
                .and_then(|caps| caps.get(1))
                .map_or("", |m| m.as_str());

            if content_type.starts_with("text") {
                let content = content.as_bytes();

                value = format!(
                    r#"{{"content": {content:?}, "type": "{content_type}", "filename": "{filename}"}}"#
                );
            } else {
                value = format!(
                    r#"{{"content": {content}, "type": "{content_type}", "filename": "{filename}"}}"#
                );
            }
        }

        map.insert(key, value);
    }

    Ok(json!(map).to_string())
}
