use std::collections::HashMap;

use anyhow::{anyhow, Result};
use hyper::{Body, Request};
use url::form_urlencoded;

pub fn get_query_string(req: &Request<Body>, param: &str) -> Result<String> {
    let queries = match req.uri().query() {
        Some(v) => Ok(v),
        None => Err(anyhow!("Missing query string")),
    }?;
    let params: HashMap<String, String> = form_urlencoded::parse(queries.as_bytes())
        .into_owned()
        .collect();

    match params.get(param) {
        Some(v) => Ok(v.to_string()),
        None => Err(anyhow!("Missing query string: {}", param)),
    }
}

// TODO: test it
