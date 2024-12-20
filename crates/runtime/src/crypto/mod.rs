use llrt_utils::{class::get_class_name, result::ResultExt};
use once_cell::sync::Lazy;
use ring::rand::{SecureRandom, SystemRandom};
use rquickjs::{prelude::Func, Ctx, Exception, Object, Result};
use uuid::Uuid;
use uuid_simd::UuidExt;

use crate::utils::object::obj_to_array_buffer;

pub static SYSTEM_RANDOM: Lazy<SystemRandom> = Lazy::new(SystemRandom::new);

fn get_random_values<'js>(ctx: Ctx<'js>, obj: Object<'js>) -> Result<Object<'js>> {
    if let Some((array_buffer, source_length, source_offset)) = obj_to_array_buffer(&obj)? {
        let raw = array_buffer
            .as_raw()
            .ok_or("ArrayBuffer is detached")
            .or_throw(&ctx)?;

        if source_length > 0x10000 {
            return Err(Exception::throw_message(
                &ctx,
                "QuotaExceededError: The requested length exceeds 65,536 bytes",
            ));
        }

        let bytes = unsafe {
            std::slice::from_raw_parts_mut(raw.ptr.as_ptr().add(source_offset), source_length)
        };

        match get_class_name(&obj)?.unwrap().as_str() {
            "Int8Array" | "Uint8Array" | "Uint8ClampedArray" | "Int16Array" | "Uint16Array"
            | "Int32Array" | "Uint32Array" | "BigInt64Array" | "BigUint64Array" => {
                SYSTEM_RANDOM.fill(bytes).unwrap()
            }
            _ => return Err(Exception::throw_message(&ctx, "Unsupported TypedArray")),
        }
    }

    Ok(obj)
}

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    let crypto = Object::new(ctx.clone())?;

    crypto.set("randomUUID", Func::from(uuidv4))?;
    crypto.set("getRandomValues", Func::from(get_random_values))?;

    globals.set("crypto", crypto)?;

    Ok(())
}

fn uuidv4() -> String {
    Uuid::new_v4().format_hyphenated().to_string()
}
