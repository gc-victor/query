use query_runtime::Runtime;

pub async fn runtime() -> Runtime {
    Runtime::new().await.expect("could not create the runtime")
}
