pub mod encoder;
pub mod text_decoder;
pub mod text_encoder;

use llrt_utils::result::ResultExt;
use rquickjs::{prelude::Func, Class, Ctx, Result};

use self::encoder::{bytes_from_b64, bytes_to_b64_string};
use self::text_decoder::TextDecoder;
use self::text_encoder::TextEncoder;

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

    Ok(())
}
