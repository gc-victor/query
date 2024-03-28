use std::env;
use std::{collections::HashMap, vec};

use anyhow::Result;
use hyper::{body::Incoming, header::CONTENT_TYPE, http::HeaderName, Method, Request, Response};
use regex::Regex;
use rusqlite::named_params;
use rustyscript::{json_args, Module, ModuleHandle, Runtime};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_json::json;
use tokio::task;
use tracing::instrument;

use crate::env::Env;
use crate::{
    controllers::utils::{
        body::{Body, BoxBody},
        http_error::{bad_request, internal_server_error, not_found, HttpError},
    },
    sqlite::connect_db::connect_function_db,
};

use super::runtime::with_runtime;

// For functions returning nothing
pub type Undefined = serde_json::Value;

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

#[instrument(err(Debug), skip(req))]
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
        |row| {
            let function: Vec<u8> = row.get(0)?;
            let function = String::from_utf8(function).unwrap();

            Ok(function)
        },
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

    let handle_request = format!(
        r#"
        {function}
        globalThis.___handleRequestWrapper = () => {{
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
                            formData.append(key, new Blob([new Uint8Array(value.content)], {{ type: value.type }}), value.filename);
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
        }};
        "#,
        body = body,
        body_multi_part = body_multi_part,
        headers = headers.join(", "),
        method = req.method().as_str(),
        url = format!("{}://{}{}", scheme, host, uri),
    );

    let res: HandleResponse = match task::spawn_blocking(move || {
        match with_runtime(move |runtime| {
            let module = Module::new("function.js", &handle_request);

            let module_handle = match runtime.load_module(&module) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            match runtime.call_function::<Undefined>(
                &ModuleHandle::default(),
                "init_global_this_backup",
                Runtime::EMPTY_ARGS,
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            let res = runtime.call_function(&module_handle, "___handleResponse", json_args!());

            match runtime.call_function::<Undefined>(
                &ModuleHandle::default(),
                "reset_global_this_backup",
                Runtime::EMPTY_ARGS,
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            res
        }) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    })
    .await
    {
        Ok(v) => match v {
            Ok(v) => Ok(v),
            Err(e) => Err(bad_request(remove_file_path(e.to_string())?)),
        },
        Err(e) => Err(bad_request(remove_file_path(e.to_string())?)),
    }?;

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

fn remove_file_path(e: String) -> Result<String> {
    Ok(e.replace(&env::current_dir()?.to_string_lossy().to_string(), ""))
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
