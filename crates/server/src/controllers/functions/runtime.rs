use std::time::Duration;

use rustyscript::{Runtime, RuntimeOptions};
use tracing::error;

use super::ext::{
    init_argon2::init_argon2, init_handle_response::init_handle_response,
    init_process::init_process, init_sqlite::init_sqlite,
};

pub fn with_runtime<T, F: FnMut(&mut Runtime) -> T>(mut callback: F) -> T {
    let mut runtime = match Runtime::new(RuntimeOptions {
        extensions: vec![
            init_argon2::init_ops_and_esm(),
            init_handle_response::init_ops_and_esm(),
            init_process::init_ops_and_esm(),
            init_sqlite::init_ops_and_esm(),
        ],
        timeout: Duration::from_millis(5000),
        ..Default::default()
    }) {
        Ok(runtime) => runtime,
        Err(err) => {
            error!("Failed to create runtime: {:?}", err);
            std::process::exit(0);
        }
    };

    callback(&mut runtime)
}
