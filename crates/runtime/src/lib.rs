use std::{
    cmp::min, collections::HashSet, env, fmt, future::Future, mem::MaybeUninit,
    result::Result as StdResult, time::Instant,
};

pub use llrt_modules::timers::poll_timers;
use llrt_modules::{
    abort,
    buffer::{self, BufferModule},
    crypto::{self, CryptoModule, SYSTEM_RANDOM},
    events, exceptions, timers, url,
};
use llrt_utils::class::get_class_name;
use once_cell::sync::Lazy;
use ring::rand::{SecureRandom, SystemRandom};
use rquickjs::{
    atom::PredefinedAtom,
    function::{Constructor, Opt},
    loader::{BuiltinLoader, BuiltinResolver, ModuleLoader, Resolver},
    prelude::Func,
    AsyncContext, AsyncRuntime, CatchResultExt, CaughtError, Ctx, Error, Function, IntoJs, Object,
    Result, String as JsString, Value,
};

use tokio::sync::oneshot::{self, Receiver};

mod console;
mod email;
mod encoding;
mod environment;
mod http;
mod json;
mod module;
mod number;
mod plugin;
mod process;
pub mod sqlite;
mod test_utils;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub static mut STARTED: MaybeUninit<Instant> = MaybeUninit::uninit();

use crate::{
    json::{parse::json_parse, stringify::json_stringify_replacer_space},
    module::ModuleModule,
    number::number_to_string,
    url::UrlModule,
    utils::{clone::structured_clone, object::get_bytes},
};

#[derive(Debug, Default)]
pub struct ModuleResolver {
    modules: HashSet<String>,
}

impl ModuleResolver {
    #[must_use]
    pub fn with_module<P: Into<String>>(mut self, path: P) -> Self {
        self.modules.insert(path.into());
        self
    }
}

impl Resolver for ModuleResolver {
    fn resolve(&mut self, _: &Ctx<'_>, base: &str, name: &str) -> Result<String> {
        let name = name.trim_start_matches("node:");
        if self.modules.contains(name) {
            eprintln!("base: {}, name: {}", base, name);
            Ok(name.into())
        } else {
            eprintln!("Error - base: {}, name: {}", base, name);
            Err(Error::new_resolving(base, name))
        }
    }
}

pub struct Runtime {
    pub runtime: AsyncRuntime,
    pub ctx: AsyncContext,
}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime {{ runtime: <hidden>, ctx: <hidden> }}")
    }
}

// JS modules
const DATABASE_SCRIPT_MODULE: &str = include_str!("js/database.js");
const EMAIL_SCRIPT_MODULE: &str = include_str!("js/email.js");
const HANDLE_RESPONSE_SCRIPT_MODULE: &str = include_str!("js/handle-response.js");
const JSX_HELPERS_SCRIPT_MODULE: &str = include_str!("js/jsx-helpers.js");
const PLUGIN_SCRIPT_MODULE: &str = include_str!("js/plugin.js");
const TEST_SCRIPT_MODULE: &str = include_str!("js/test.js");
// Polyfill modules
const BLOB_SCRIPT_MODULE: &str = include_str!("js/polyfills/blob.js");
const CONSOLE_SCRIPT_MODULE: &str = include_str!("js/polyfills/console.js");
const FETCH_SCRIPT_MODULE: &str = include_str!("js/polyfills/fetch.js");
const FILE_SCRIPT_MODULE: &str = include_str!("js/polyfills/file.js");
const FORM_DATA_SCRIPT_MODULE: &str = include_str!("js/polyfills/form-data.js");
const REQUEST_SCRIPT_MODULE: &str = include_str!("js/polyfills/request.js");
const RESPONSE_SCRIPT_MODULE: &str = include_str!("js/polyfills/response.js");
const WEB_STREAMS_SCRIPT_MODULE: &str = include_str!("js/polyfills/web-streams.js");

impl Runtime {
    pub const ENV_TASK_ROOT: &'static str = "TASK_ROOT";

    pub async fn new() -> StdResult<Self, Box<dyn std::error::Error + Send + Sync>> {
        llrt_modules::time::init();

        if let Some(rng) = Lazy::<SystemRandom>::get(&SYSTEM_RANDOM) {
            let mut buf = [0u8; 8];
            rng.fill(&mut buf)
                .expect("Failed to initialize SystemRandom");
        }

        const DEFAULT_GC_THRESHOLD_MB: usize = 20;

        let gc_threshold_mb: usize = env::var(environment::QUERY_RUNTIME_GC_THRESHOLD_MB)
            .map(|threshold| threshold.parse().unwrap_or(DEFAULT_GC_THRESHOLD_MB))
            .unwrap_or(DEFAULT_GC_THRESHOLD_MB);

        let runtime = AsyncRuntime::new()?;
        runtime.set_max_stack_size(512 * 1024).await;
        runtime
            .set_gc_threshold(gc_threshold_mb * 1024 * 1024)
            .await;

        let resolver = (
            BuiltinResolver::default()
                .with_module("js/database")
                .with_module("js/handle-response")
                .with_module("js/jsx-helpers")
                .with_module("polyfill/blob")
                .with_module("polyfill/console")
                .with_module("polyfill/fetch")
                .with_module("polyfill/file")
                .with_module("polyfill/form-data")
                .with_module("polyfill/request")
                .with_module("polyfill/response")
                .with_module("polyfill/web-streams")
                .with_module("query:email")
                .with_module("query:database")
                .with_module("query:plugin")
                .with_module("query:test"),
            ModuleResolver::default()
                .with_module("buffer")
                .with_module("crypto")
                .with_module("timers"),
        );
        let loader = (
            BuiltinLoader::default()
                .with_module("js/database", DATABASE_SCRIPT_MODULE)
                .with_module("js/handle-response", HANDLE_RESPONSE_SCRIPT_MODULE)
                .with_module("js/jsx-helpers", JSX_HELPERS_SCRIPT_MODULE)
                .with_module("polyfill/blob", BLOB_SCRIPT_MODULE)
                .with_module("polyfill/console", CONSOLE_SCRIPT_MODULE)
                .with_module("polyfill/fetch", FETCH_SCRIPT_MODULE)
                .with_module("polyfill/file", FILE_SCRIPT_MODULE)
                .with_module("polyfill/form-data", FORM_DATA_SCRIPT_MODULE)
                .with_module("polyfill/request", REQUEST_SCRIPT_MODULE)
                .with_module("polyfill/response", RESPONSE_SCRIPT_MODULE)
                .with_module("polyfill/web-streams", WEB_STREAMS_SCRIPT_MODULE)
                .with_module("query:database", DATABASE_SCRIPT_MODULE)
                .with_module("query:email", EMAIL_SCRIPT_MODULE)
                .with_module("query:plugin", PLUGIN_SCRIPT_MODULE)
                .with_module("query:test", TEST_SCRIPT_MODULE),
            ModuleLoader::default()
                .with_module("crypto", CryptoModule)
                .with_module("buffer", BufferModule)
                .with_module("module", ModuleModule)
                .with_module("url", UrlModule),
        );
        runtime.set_loader(resolver, loader).await;

        let ctx = AsyncContext::full(&runtime).await?;
        ctx.with(|ctx| {
            (|| {
                buffer::init(&ctx)?;
                crypto::init(&ctx)?;
                events::init(&ctx)?;
                exceptions::init(&ctx)?;
                abort::init(&ctx)?;
                url::init(&ctx)?;
                console::init(&ctx)?;
                email::init(&ctx)?;
                encoding::init(&ctx)?;
                http::init(&ctx)?;
                plugin::init(&ctx)?;
                process::init(&ctx)?;
                timers::init(&ctx)?;
                sqlite::init(&ctx)?;

                init(&ctx)?;

                ctx.execute_pending_job();

                Ok(())
            })()
            .catch(&ctx)
            .unwrap_or_else(|err| print_error(err));

            Ok::<_, Error>(())
        })
        .await?;

        Ok(Runtime { runtime, ctx })
    }
}

fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    let number: Function = globals.get(PredefinedAtom::Number)?;
    let number_proto: Object = number.get(PredefinedAtom::Prototype)?;
    number_proto.set(PredefinedAtom::ToString, Func::from(number_to_string))?;

    globals.set("global", ctx.globals())?;
    globals.set("globalThis", ctx.globals())?;
    globals.set("self", ctx.globals())?;
    globals.set("print", Func::from(print))?;
    globals.set(
        "structuredClone",
        Func::from(|ctx, value, options| structured_clone(&ctx, value, options)),
    )?;

    let json_module: Object = globals.get(PredefinedAtom::JSON)?;
    json_module.set("parse", Func::from(json_parse_string))?;
    json_module.set(
        "stringify",
        Func::from(|ctx, value, replacer, space| {
            struct StringifyArgs<'js>(Ctx<'js>, Value<'js>, Opt<Value<'js>>, Opt<Value<'js>>);
            let StringifyArgs(ctx, value, replacer, space) =
                StringifyArgs(ctx, value, replacer, space);

            let mut space_value = None;
            let mut replacer_value = None;

            if let Some(replacer) = replacer.0 {
                if let Some(space) = space.0 {
                    if let Some(space) = space.as_string() {
                        let mut space = space.clone().to_string()?;
                        space.truncate(20);
                        space_value = Some(space);
                    }
                    if let Some(number) = space.as_int() {
                        if number > 0 {
                            space_value = Some(" ".repeat(min(10, number as usize)));
                        }
                    }
                }
                replacer_value = Some(replacer);
            }

            json_stringify_replacer_space(&ctx, value, replacer_value, space_value)
                .map(|v| v.into_js(&ctx))?
        }),
    )?;

    Ok(())
}

fn print(value: String, stdout: Opt<bool>) {
    if stdout.0.unwrap_or_default() {
        println!("{value}");
    } else {
        eprintln!("{value}")
    }
}

fn json_parse_string<'js>(ctx: Ctx<'js>, value: Value<'js>) -> Result<Value<'js>> {
    let bytes = get_bytes(&ctx, value)?;
    json_parse(&ctx, bytes)
}

pub trait ErrorExtensions<'js> {
    fn into_value(self, ctx: &Ctx<'js>) -> Result<Value<'js>>;
}

impl<'js> ErrorExtensions<'js> for Error {
    fn into_value(self, ctx: &Ctx<'js>) -> Result<Value<'js>> {
        Err::<(), _>(self).catch(ctx).unwrap_err().into_value(ctx)
    }
}

impl<'js> ErrorExtensions<'js> for CaughtError<'js> {
    fn into_value(self, ctx: &Ctx<'js>) -> Result<Value<'js>> {
        Ok(match self {
            CaughtError::Error(err) => {
                JsString::from_str(ctx.clone(), &err.to_string())?.into_value()
            }
            CaughtError::Exception(ex) => ex.into_value(),
            CaughtError::Value(val) => val,
        })
    }
}

pub trait CtxExtension<'js> {
    fn spawn_exit<F, R>(&self, future: F) -> Result<Receiver<R>>
    where
        F: Future<Output = Result<R>> + 'js,
        R: 'js;
}

impl<'js> CtxExtension<'js> for Ctx<'js> {
    fn spawn_exit<F, R>(&self, future: F) -> Result<Receiver<R>>
    where
        F: Future<Output = Result<R>> + 'js,
        R: 'js,
    {
        let ctx = self.clone();

        let type_error_ctor: Constructor = ctx.globals().get(PredefinedAtom::TypeError)?;
        let type_error: Object = type_error_ctor.construct(())?;
        let stack: Option<String> = type_error.get(PredefinedAtom::Stack).ok();

        let (join_channel_tx, join_channel_rx) = oneshot::channel();

        self.spawn(async move {
            match future.await.catch(&ctx) {
                Ok(res) => {
                    //result here dosn't matter if receiver has dropped
                    let _ = join_channel_tx.send(res);
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);

                    if let CaughtError::Exception(err) = err {
                        if err.stack().is_none() {
                            if let Some(stack) = stack {
                                err.set(PredefinedAtom::Stack, stack).unwrap();
                            }
                        }
                        print_error(CaughtError::Exception(err));
                    } else {
                        print_error(err);
                    }
                }
            }
        });
        Ok(join_channel_rx)
    }
}

fn print_error(err: CaughtError<'_>) {
    let error_msg = match err {
        CaughtError::Error(err) => format!("Error: {:?}", &err),
        CaughtError::Exception(ex) => {
            let error_name = get_class_name(&ex)
                .unwrap_or(None)
                .unwrap_or(String::from("Error"));
            let mut str = String::with_capacity(100);
            str.push_str(&error_name);
            str.push_str(": ");
            str.push_str(&ex.message().unwrap_or_default());
            str.push('\n');
            if let Some(stack) = ex.stack() {
                str.push_str(&stack);
            }
            str
        }
        CaughtError::Value(value) => {
            format!("Error: {:?}", value)
        }
    };

    tracing::error!("{}", error_msg);
}
