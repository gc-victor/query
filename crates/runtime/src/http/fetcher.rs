// CREDIT: https://github.com/yollotltam/llrt/blob/23b93ced64a9ca507b2cb0400bf7d7131b066bf6/src/http/fetch.rs
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{Request, Uri};
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioTimer},
};
use llrt_utils::result::ResultExt;
use once_cell::sync::Lazy;
use rquickjs::{
    function::Opt,
    prelude::{Async, Func},
    Array, Ctx, Error, Exception, FromIteratorJs, FromJs, JsLifetime, Object, Result, Value,
};
use rustls::{crypto::ring, ClientConfig, RootCertStore};
use tracing::warn;
use webpki_roots::TLS_SERVER_ROOTS;

use core::str;
use std::{env, time::Duration};

use crate::{environment, http::headers::Headers, utils::object::get_bytes, VERSION};

pub const DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT_SECONDS: u64 = 15;

pub static TLS_CONFIG: Lazy<ClientConfig> = Lazy::new(|| {
    let mut root_certificates = RootCertStore::empty();

    for cert in TLS_SERVER_ROOTS.iter().cloned() {
        root_certificates.roots.push(cert)
    }

    ClientConfig::builder_with_provider(ring::default_provider().into())
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_root_certificates(root_certificates)
        .with_no_client_auth()
});

#[allow(dead_code)]
#[rquickjs::class]
#[derive(rquickjs::class::Trace, JsLifetime, Clone, Debug)]
pub struct ___FetcherResponse<'js> {
    #[qjs(skip_trace)]
    body: Option<Array<'js>>,
    #[qjs(skip_trace)]
    headers: Option<Object<'js>>,
    method: String,
    status: u16,
    url: String,
}

#[rquickjs::methods(rename_all = "camelCase")]
impl<'js> ___FetcherResponse<'js> {
    #[qjs(constructor)]
    pub fn new(
        body: Option<Array<'js>>,
        headers: Option<Object<'js>>,
        method: String,
        status: u16,
        url: String,
    ) -> Self {
        Self {
            body,
            headers,
            method,
            status,
            url,
        }
    }

    #[qjs(get)]
    pub fn body(&self) -> Option<Array<'js>> {
        self.body.clone()
    }

    #[qjs(get)]
    pub fn headers(&self) -> Option<Object<'js>> {
        self.headers.clone()
    }

    #[qjs(get)]
    pub fn method(&self) -> String {
        self.method.clone()
    }

    #[qjs(get)]
    pub fn status(&self) -> u16 {
        self.status
    }

    #[qjs(get)]
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

pub(crate) fn init(globals: &Object) -> Result<()> {
    let pool_idle_timeout: u64 = env::var(environment::QUERY_RUNTIME_NET_POOL_IDLE_TIMEOUT)
        .map(|timeout| {
            timeout
                .parse()
                .unwrap_or(DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT_SECONDS)
        })
        .unwrap_or(DEFAULT_CONNECTION_POOL_IDLE_TIMEOUT_SECONDS);
    if pool_idle_timeout > 300 {
        warn!(
            r#""{}" is exceeds 300s (5min), risking errors due to possible server connection closures."#,
            environment::QUERY_RUNTIME_NET_POOL_IDLE_TIMEOUT
        )
    }

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(TLS_CONFIG.clone())
        .https_or_http()
        .enable_http1()
        .build();

    let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new())
        .pool_idle_timeout(Duration::from_secs(pool_idle_timeout))
        .pool_timer(TokioTimer::new())
        .build(https);

    globals.set(
        "___fetcher",
        Func::from(Async(move |ctx, resource, options| {
            let client = client.clone();

            let options = get_fetch_options(&ctx, resource, options);

            async move {
                let options = options?;

                let uri: Uri = options.url.parse().or_throw(&ctx)?;
                let method_string = options.method.to_string();
                let method = options.method;

                let mut req = Request::builder()
                    .method(method)
                    .uri(uri)
                    .header("user-agent", format!("Query {}", VERSION));

                if let Some(headers) = options.headers {
                    for (key, value) in headers.iter() {
                        req = req.header(key, value)
                    }
                }

                let req = req.body(options.body).or_throw(&ctx)?;
                let (parts, body) = client.request(req).await.or_throw(&ctx)?.into_parts();

                let body = body.collect().await.or_throw(&ctx)?;
                let body = body.to_bytes().to_vec();

                // TODO: bytes_to_typed_array
                let body = Array::from_iter_js(&ctx, body)?;

                let status = parts.status.as_u16();
                let method = method_string;

                let headers = Object::new(ctx.clone())?;
                for (key, value) in parts.headers {
                    let key = match key {
                        Some(k) => k.to_string(),
                        None => continue,
                    };
                    let value = match value.to_str() {
                        Ok(v) => v.to_string(),
                        Err(e) => {
                            return Err(Exception::throw_type(
                                &ctx,
                                &format!("Invalid header value: {}", e),
                            ))
                        }
                    };

                    headers.set(key, value)?;
                }

                let response = Object::new(ctx.clone())?;
                response.set("body", Some(body))?;
                response.set("headers", Some(headers))?;
                response.set("method", method)?;
                response.set("status", status)?;
                response.set("url", options.url)?;

                Ok::<Object, Error>(response)
            }
        })),
    )?;

    Ok(())
}

#[derive(Debug)]
struct FetchOptions {
    method: hyper::Method,
    url: String,
    headers: Option<Headers>,
    body: Full<Bytes>,
}

fn get_fetch_options<'js>(
    ctx: &Ctx<'js>,
    resource: Value<'js>,
    options: Opt<Value<'js>>,
) -> Result<FetchOptions> {
    let mut body = None;
    let mut headers = None;
    let mut method = None;
    let mut url = None;

    let mut options_opts = None;
    let mut resource_opts = None;

    if let Some(obj) = resource.as_object() {
        let obj = obj.clone();
        let ctor: Object = obj.get("constructor").unwrap();
        let name: String = ctor.get("name").unwrap();

        if name == "Request" {
            resource_opts = Some(obj);
        } else if name == "URL" {
            let href: String = obj.get("href")?;
            url = Some(href);
        } else {
            resource_opts = Some(obj);
        }
    } else {
        let url_string: String = resource.get()?;
        url = Some(url_string);
    }

    if let Some(options) = options.0 {
        options_opts = options.into_object();
    }

    if resource_opts.is_some() || options_opts.is_some() {
        if let Some(method_opt) = get_option::<String>("method", &options_opts, &resource_opts)? {
            method = Some(match method_opt.as_str() {
                "CONNECT" => Ok(hyper::Method::CONNECT),
                "DELETE" => Ok(hyper::Method::DELETE),
                "GET" => Ok(hyper::Method::GET),
                "HEAD" => Ok(hyper::Method::HEAD),
                "PATCH" => Ok(hyper::Method::PATCH),
                "POST" => Ok(hyper::Method::POST),
                "PUT" => Ok(hyper::Method::PUT),
                _ => Err(Exception::throw_type(
                    ctx,
                    &format!("Invalid HTTP method: {}", method_opt),
                )),
            }?);
        }

        if let Some(body_opt) = get_option::<Value>("body", &options_opts, &resource_opts)? {
            let bytes = get_bytes(ctx, body_opt)?;
            body = Some(Full::from(bytes));
        }

        if let Some(url_opt) = get_option::<String>("url", &options_opts, &resource_opts)? {
            url = Some(url_opt);
        }

        if let Some(headers_op) = get_option::<Value>("headers", &options_opts, &resource_opts)? {
            headers = Some(Headers::from_value(ctx, headers_op)?);
        }
    }

    let url = match url {
        Some(url) => url,
        None => return Err(Exception::throw_reference(ctx, "Missing required url")),
    };

    Ok(FetchOptions {
        body: body.unwrap_or_default(),
        headers,
        method: method.unwrap_or_default(),
        url,
    })
}

fn get_option<'js, V: FromJs<'js> + Sized>(
    key: &str,
    options: &Option<Object<'js>>,
    resource: &Option<Object<'js>>,
) -> Result<Option<V>> {
    if let Some(opt) = options {
        if let Some(value) = opt.get::<_, Option<V>>(key)? {
            return Ok(Some(value));
        }
    }

    if let Some(opt) = resource {
        if let Some(value) = opt.get::<_, Option<V>>(key)? {
            return Ok(Some(value));
        }
    }

    Ok(None)
}
