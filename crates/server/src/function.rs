use std::collections::HashMap;
use std::env;

use anyhow::Result;
use hyper::{http::HeaderName, Body, Method, Request, Response};
use rusqlite::named_params;
use rustyscript::{json_args, Module};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use tokio::task;
use tracing::instrument;

use crate::runtime::with_runtime;
use crate::sqlite::connect_db::connect_function_db;
use crate::utils::get_body::get_body;
use crate::utils::http_error::not_found;
use crate::{
    utils::http_error::HttpError,
    utils::http_error::{bad_request, internal_server_error},
};

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
pub async fn function(req: &mut Request<Body>) -> Result<Response<Body>, HttpError> {
    let method = req.method().as_str();
    let path = req.uri().path().replace("/function", "");
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path
    };

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

    let body = if req.method() == Method::GET {
        "".to_string()
    } else {
        get_body(req).await?
    };

    let mut headers: Vec<String> = vec![];
    for (key, value) in req.headers() {
        headers.push(format!(
            r#""{key}": "{}""#,
            // NOTE: workaround to fix an error in the js-engine caused by sec-ch-ua
            value.to_str().unwrap().to_string().replace('"', "'"),
        ));
    }

    let host = req.headers().get("host").unwrap();
    let uri = req.uri().to_string();
    let handle_request = format!(
        "
        {function}
        globalThis.___handleRequestWrapper = () => {{
            const request = new Request('{url}', {{
                headers: {{ {headers} }},
                method: '{method}',
                url: '{url}',
            }});
            if (!/GET|HEAD/.test('{method}')) {{
                request.body = '{body}';
            }}

            return ___handleRequest(request);
        }};
        ",
        body = body,
        headers = headers.join(", "),
        method = req.method().as_str(),
        url = format!("https://{}{}", host.to_str().unwrap(), uri),
    );

    let res: HandleResponse = match task::spawn_blocking(move || {
        match with_runtime(move |runtime| {
            let module = Module::new("function.js", &handle_request);

            let module_handle = match runtime.load_module(&module) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            runtime.call_function(&module_handle, "___handleResponse", json_args!())
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
    let path = if path.ends_with("/") && path != "/" {
        path.trim_end_matches("/")
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
