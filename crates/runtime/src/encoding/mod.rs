#![allow(clippy::inherent_to_string)]

pub mod encoder;

use rquickjs::{
    function::Opt,
    prelude::{Func, This},
    Class, Ctx, Result, TypedArray, Value,
};

use crate::utils::{object::get_bytes, result::ResultExt};

use self::encoder::{bytes_from_b64, bytes_to_b64_string, Encoder};

#[derive(rquickjs::class::Trace)]
#[rquickjs::class]
pub struct TextEncoder {}

#[rquickjs::methods]
impl TextEncoder {
    #[qjs(constructor)]
    pub fn new_enc() -> Self {
        Self {}
    }
    pub fn encode<'js>(&self, ctx: Ctx<'js>, string: String) -> Result<Value<'js>> {
        TypedArray::new(ctx, string.as_bytes()).map(|m| m.into_value())
    }
}

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct TextDecoder {
    #[qjs(skip_trace)]
    encoder: Encoder,
}

#[rquickjs::methods]
impl TextDecoder {
    #[qjs(constructor)]
    pub fn new_dec(ctx: Ctx<'_>, encoding: Opt<String>) -> Result<Self> {
        let mut encoding = encoding.0.unwrap_or(String::from("utf-8"));

        if encoding.is_empty() {
            encoding = String::from("utf-8");
        }

        let encoder = Encoder::from_str(&encoding).or_throw(&ctx)?;

        Ok(TextDecoder { encoder })
    }

    pub fn decode<'js>(&self, ctx: Ctx<'js>, buffer: Value<'js>) -> Result<String> {
        let bytes = get_bytes(&ctx, buffer)?;

        self.encoder.encode_to_string(&bytes).or_throw(&ctx)
    }
}

#[rquickjs::class]
#[derive(rquickjs::class::Trace, Debug)]
pub struct StringBuilder {
    #[qjs(skip_trace)]
    value: String,
}

#[rquickjs::methods(rename_all = "camelCase")]
impl StringBuilder {
    #[qjs(constructor)]
    fn new_string_builder(capacity: Opt<usize>) -> Self {
        Self {
            value: String::with_capacity(capacity.0.unwrap_or(256)),
        }
    }

    fn append<'js>(
        this: This<Class<'js, Self>>,
        _ctx: Ctx<'js>,
        value: Value<'js>,
    ) -> Result<Class<'js, Self>> {
        if value.is_string() {
            let string: String = value.get()?;
            this.borrow_mut().value.push_str(&string);
        } else if value.is_number() {
            let number: f64 = value.get()?;
            this.borrow_mut().value.push_str(&number.to_string());
        } else if value.is_bool() {
            let boolean: bool = value.get()?;
            this.0
                .borrow_mut()
                .value
                .push_str(if boolean { "true" } else { "false" });
        }
        Ok(this.0)
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_string(&mut self) -> String {
        self.value.clone()
    }
}

pub fn atob(ctx: Ctx<'_>, encoded_value: String) -> Result<String> {
    let vec = bytes_from_b64(encoded_value.as_bytes()).or_throw(&ctx)?;
    Ok(unsafe { String::from_utf8_unchecked(vec) })
}

pub fn btoa(value: String) -> String {
    bytes_to_b64_string(value.as_bytes())
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    globals.set("atob", Func::from(atob))?;
    globals.set("btoa", Func::from(btoa))?;

    Class::<TextEncoder>::define(&globals)?;
    Class::<TextDecoder>::define(&globals)?;
    Class::<StringBuilder>::define(&globals)?;

    Ok(())
}
