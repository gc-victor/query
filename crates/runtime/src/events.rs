use std::time::Duration;

use rquickjs::{
    class::{Trace, Tracer},
    prelude::{Opt, This},
    Class, Ctx, Exception, IntoJs, Result, Value,
};

use crate::CtxExtension;

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct AbortController<'js> {
    signal: Class<'js, AbortSignal<'js>>,
}

#[rquickjs::methods]
impl<'js> AbortController<'js> {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'js>) -> Result<Self> {
        let abort_controller = Self {
            signal: Class::instance(
                ctx,
                AbortSignal {
                    aborted: false,
                    reason: None,
                },
            )?,
        };
        Ok(abort_controller)
    }

    #[qjs(get)]
    pub fn signal(&self) -> Class<'js, AbortSignal<'js>> {
        self.signal.clone()
    }

    pub fn abort(
        ctx: Ctx<'js>,
        this: This<Class<'js, Self>>,
        reason: Opt<Value<'js>>,
    ) -> Result<Class<'js, Self>> {
        this.0.borrow_mut().signal.borrow_mut().set_aborted(true);
        if reason.0.is_some() {
            this.0.borrow_mut().signal.borrow_mut().set_reason(reason)?;
        } else {
            let abort_exception = Exception::from_value("AbortError".into_js(&ctx)?)?; // TODO: AbortError DOMException
            this.0
                .borrow_mut()
                .signal
                .borrow_mut()
                .set_reason(Opt(Some(abort_exception.into_value())))?;
        }

        Ok(this.0)
    }
}

//TODO implement static methods abort() and timeout(miliseconds)
#[rquickjs::class]
pub struct AbortSignal<'js> {
    aborted: bool,
    reason: Option<Value<'js>>,
}

impl<'js> Trace<'js> for AbortSignal<'js> {
    fn trace<'a>(&self, tracer: Tracer<'a, 'js>) {
        if let Some(reason) = &self.reason {
            tracer.mark(reason);
        }
    }
}

impl<'js> Default for AbortSignal<'js> {
    fn default() -> Self {
        Self::new()
    }
}

#[rquickjs::methods]
impl<'js> AbortSignal<'js> {
    #[qjs(constructor)]
    pub fn new() -> Self {
        Self {
            aborted: false,
            reason: None,
        }
    }

    #[qjs(get)]
    pub fn aborted(&self) -> bool {
        self.aborted
    }

    #[qjs(get)]
    pub fn reason(&self) -> Option<Value<'js>> {
        self.reason.clone()
    }

    #[qjs(skip)]
    pub fn set_reason(&mut self, reason: Opt<Value<'js>>) -> Result<Option<Value<'js>>> {
        if let Some(new_reason) = reason.0 {
            Ok(self.reason.replace(new_reason))
        } else {
            let old_reason = self.reason.take();
            Ok(old_reason)
        }
    }

    #[qjs(skip)]
    pub fn set_aborted(&mut self, is_aborted: bool) {
        self.aborted = is_aborted;
    }

    #[qjs(static)]
    pub fn abort(reason: Opt<Value<'js>>) -> AbortSignal {
        AbortSignal {
            aborted: true,
            reason: reason.0,
        }
    }

    #[qjs(static)]
    pub fn timeout(ctx: Ctx<'js>, milliseconds: u64) -> Result<Class<'js, Self>> {
        let timeout_exception = Exception::from_value("TimeoutError".into_js(&ctx)?)?; // TODO: Timeout DOMException
        let signal = AbortSignal {
            aborted: false,
            reason: None,
        };

        let signal_instance = Class::instance(ctx.clone(), signal)?;
        let signal_instance2 = signal_instance.clone();

        ctx.spawn_exit(async move {
            tokio::time::sleep(Duration::from_millis(milliseconds)).await;
            let mut borrow = signal_instance.borrow_mut();
            borrow.set_aborted(true);
            borrow.set_reason(Opt(Some(timeout_exception.into_value())))?;
            Ok(())
        })?;

        Ok(signal_instance2)
    }
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    Class::<AbortController>::define(&globals)?;
    Class::<AbortSignal>::define(&globals)?;

    Ok(())
}
