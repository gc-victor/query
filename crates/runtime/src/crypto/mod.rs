use rquickjs::{prelude::Func, Ctx, Object, Result};

use uuid::Uuid;
use uuid_simd::UuidExt;

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    let crypto = Object::new(ctx.clone())?;

    crypto.set("randomUUID", Func::from(uuidv4))?;

    globals.set("crypto", crypto)?;

    Ok(())
}

fn uuidv4() -> String {
    Uuid::new_v4().format_hyphenated().to_string()
}
