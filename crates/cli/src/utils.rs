use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Method, Url,
};
use serde_json::{self, Value};
use tabled::{builder::Builder, settings::Style};

use crate::config::CONFIG;

pub fn read_file_content(file_path: &str) -> Result<Vec<u8>> {
    let file = File::open(file_path)?;

    let file = &file;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    Ok(content.as_bytes().to_vec())
}

pub fn line_break() {
    eprintln!("\n");
}

pub fn json_to_table(value: &Value) -> Result<String> {
    let mut builder = Builder::default();

    if value.as_array().is_none() {
        return Ok(String::new());
    }

    let array = value.as_array().unwrap();

    if array.first().is_none() {
        return Ok(String::new());
    }
    let first_object = array.first().unwrap();
    let keys = first_object.as_object().unwrap().keys();

    let keys = keys.map(|key| {
        let key = key.to_string();
        key.to_uppercase()
    });

    builder.set_header(keys);

    for object in array {
        let values = object.as_object().unwrap().values();
        let values = values.map(|value| value.to_string());
        builder.push_record(values);
    }

    let mut table = builder.build();

    table
        .with(Style::markdown().vertical(' ').remove_left().remove_right())
        .to_string();

    Ok(table.to_string())
}

pub async fn http_client(path: &str, body: Option<&String>, method: Method) -> Result<Value> {
    let config_url = CONFIG.server.url.clone();
    let config_url = if !config_url.ends_with('/') {
        format!("{}/", config_url)
    } else {
        config_url.clone()
    };
    let url = &format!("{}{}", config_url, path);
    let url = Url::parse(url)?;

    let token: &str = CONFIG.cli.token.as_str();

    let mut headers = HeaderMap::new();

    if !token.is_empty() {
        headers.insert(
            HeaderName::from_lowercase(b"authorization")?,
            HeaderValue::from_str(format!("Bearer {}", token).as_str())?,
        );
    }

    let body = Body::from(match body {
        Some(body) => body.as_str().to_string(),
        None => String::new(),
    });

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()?;

    let response = client
        .request(method, url)
        .headers(headers)
        .body(body)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        if body.is_empty() {
            return Err(anyhow!("{}", status));
        } else {
            return Err(anyhow!("{}", body));
        }
    }

    if body.is_empty() {
        return Ok(Value::Null);
    }

    let value: Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(_) => Value::String(body),
    };

    Ok(value)
}
