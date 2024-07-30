use query_runtime::Runtime as QueryRuntime;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::env::Env;

lazy_static::lazy_static! {
    pub static ref RUNTIME: Mutex<Runtime> = Mutex::new(Runtime::new());
}

#[derive(Default)]
pub struct Runtime {
    value: Option<QueryRuntime>,
    max_malloc_size: i64,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            value: None,
            max_malloc_size: Env::runtime_max_malloc_size(),
        }
    }

    pub async fn get(&mut self) -> &Option<QueryRuntime> {
        let malloc_size = match &self.value {
            Some(value) => value.runtime.memory_usage().await.malloc_size,
            None => self.max_malloc_size,
        };

        info!("Malloc size {}", malloc_size);

        if self.value.is_none() || malloc_size > self.max_malloc_size {
            if let Some(value) = self.value.take() {
                match value.idle() {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Failed to idle runtime: {:?}", e);
                    }
                };
            }

            let runtime = match QueryRuntime::new().await {
                Ok(runtime) => runtime,
                Err(e) => {
                    error!("Failed to create runtime: {:?}", e);
                    return &self.value;
                }
            };

            self.value = Some(runtime);
        }

        &self.value
    }
}
