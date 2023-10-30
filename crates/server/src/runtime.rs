use std::cell::{OnceCell, RefCell};

use rustyscript::{Runtime, RuntimeOptions};

use crate::ext::{init_handle_response::init_handle_response, init_sqlite::init_sqlite};

thread_local! {
    static RUNTIME_CELL: OnceCell<RefCell<Runtime>> = OnceCell::new();
}

pub fn with_runtime<T, F: FnMut(&mut Runtime) -> T>(mut callback: F) -> T {
    RUNTIME_CELL.with(|once_lock| {
        let rt_mut = once_lock.get_or_init(|| {
            RefCell::new(
                Runtime::new(RuntimeOptions {
                    extensions: vec![
                        init_handle_response::init_ops_and_esm(),
                        init_sqlite::init_ops_and_esm(),
                    ],
                    ..Default::default()
                })
                .expect("could not create the runtime"),
            )
        });
        let mut runtime = rt_mut.borrow_mut();
        callback(&mut runtime)
    })
}
