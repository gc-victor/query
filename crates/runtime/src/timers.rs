use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use once_cell::sync::Lazy;
use rquickjs::{prelude::Func, qjs, Ctx, Function, Persistent, Result, Value};

static TIMER_ID: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct RuntimeTimerState {
    timers: Vec<TimeoutRef>,
    rt: *mut qjs::JSRuntime,
}
impl RuntimeTimerState {
    fn new(rt: *mut qjs::JSRuntime) -> Self {
        Self {
            timers: Vec::new(),
            rt,
        }
    }
}

unsafe impl Send for RuntimeTimerState {}

pub(crate) static RUNTIME_TIMERS: Lazy<Mutex<Vec<RuntimeTimerState>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub struct TimeoutRef {
    callback: Option<Persistent<Function<'static>>>,
    pub expires: usize,
    id: usize,
    repeating: bool,
}

unsafe impl Send for TimeoutRef {}

fn set_immediate(cb: Function) -> Result<()> {
    cb.defer::<()>(())?;
    Ok(())
}

fn get_current_time_millis() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as usize
}

pub fn set_timeout_interval<'js>(
    ctx: &Ctx<'js>,
    cb: Function<'js>,
    delay: usize,
    repeating: bool,
) -> Result<usize> {
    let expires = get_current_time_millis() + delay;
    let id = TIMER_ID.fetch_add(1, Ordering::Relaxed);

    let callback = Persistent::<Function>::save(ctx, cb);

    let rt = unsafe { qjs::JS_GetRuntime(ctx.as_raw().as_ptr()) };
    let mut rt_timers = RUNTIME_TIMERS.lock().unwrap();

    let timeout_ref = TimeoutRef {
        expires,
        callback: Some(callback),
        id,
        repeating,
    };

    if let Some(entry) = rt_timers.iter_mut().find(|state| state.rt == rt) {
        entry.timers.push(timeout_ref);
    } else {
        let mut entry = RuntimeTimerState::new(rt);
        entry.timers.push(timeout_ref);
        rt_timers.push(entry);
    }

    Ok(id)
}

fn clear_timeout_interval(ctx: &Ctx<'_>, id: usize) -> Result<()> {
    let rt = unsafe { qjs::JS_GetRuntime(ctx.as_raw().as_ptr()) };
    let mut rt_timers = RUNTIME_TIMERS.lock().unwrap();

    if let Some(entry) = rt_timers.iter_mut().find(|t| t.rt == rt) {
        if let Some(timeout) = entry.timers.iter_mut().find(|t| t.id == id) {
            if let Some(timeout) = timeout.callback.take() {
                timeout.restore(ctx)?; //prevent memory leaks
            }
            timeout.expires = 0;
            timeout.repeating = false;
        }
    }

    Ok(())
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    globals.set(
        "setTimeout",
        Func::from(move |ctx, cb, delay| set_timeout_interval(&ctx, cb, delay, false)),
    )?;

    globals.set(
        "setInterval",
        Func::from(move |ctx, cb, delay| set_timeout_interval(&ctx, cb, delay, true)),
    )?;

    globals.set(
        "clearTimeout",
        Func::from(move |ctx: Ctx, id: Value| {
            if let Some(id) = id.as_number() {
                clear_timeout_interval(&ctx, id as _)
            } else {
                Ok(())
            }
        }),
    )?;

    globals.set(
        "clearInterval",
        Func::from(move |ctx: Ctx, id: Value| {
            if let Some(id) = id.as_number() {
                clear_timeout_interval(&ctx, id as _)
            } else {
                Ok(())
            }
        }),
    )?;

    globals.set("setImmediate", Func::from(set_immediate))?;

    Ok(())
}
