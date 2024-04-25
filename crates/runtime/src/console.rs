// CREDIT: https://github.com/yasojs/yaso/blob/e2b56fef23a46cb80f7ed188531d02be6cf1b6b5/src/console.rs
use std::{
    fmt,
    io::{stderr, stdout, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use rquickjs::{
    function::{Func, Rest},
    Ctx, Object, Result, Type, Value,
};

const NEWLINE: char = '\n';

#[derive(Debug)]
pub enum LogLevel {
    Trace = 10,
    Debug = 20,
    Info = 30,
    Warn = 40,
    Error = 50,
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    let console = Object::new(ctx.clone())?;

    console.set("assert", Func::from(console_assert))?;
    console.set("debug", Func::from(console_debug))?;
    console.set("error", Func::from(console_error))?;
    console.set("info", Func::from(console_info))?;
    console.set("log", Func::from(console_log))?;
    console.set("trace", Func::from(console_trace))?;
    console.set("warn", Func::from(console_warn))?;

    globals.set("console", console)?;

    Ok(())
}

fn console_assert(expression: bool, args: Rest<Value<'_>>) -> Result<()> {
    if !expression {
        log_write(stderr(), args, LogLevel::Info)?;
    }

    Ok(())
}

fn console_debug(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stdout(), args, LogLevel::Debug)
}

fn console_error(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stderr(), args, LogLevel::Error)
}

fn console_info(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stdout(), args, LogLevel::Info)
}

fn console_log(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stdout(), args, LogLevel::Info)
}

fn console_trace(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stdout(), args, LogLevel::Trace)
}

fn console_warn(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stderr(), args, LogLevel::Warn)
}

fn js_format(args: Rest<Value<'_>>) -> Result<String> {
    let mut result = String::new();

    for arg in args.iter() {
        result.push_str(js_stringify(arg)?.as_str());
        result.push(' ');
    }

    Ok(result)
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn log_write<O>(mut output: O, args: Rest<Value<'_>>, level: LogLevel) -> Result<()>
where
    O: Write,
{
    let level = match level {
        LogLevel::Trace => 10,
        LogLevel::Debug => 20,
        LogLevel::Info => 30,
        LogLevel::Warn => 40,
        LogLevel::Error => 50,
    };
    let msg = js_format(args)?;
    let msg = msg
        .replace(NEWLINE, "\\n")
        .replace('"', "\\\"")
        .trim()
        .to_string();
    let time = now();
    let log = format!(r#"{{"console":true,"level":{level},"msg":"{msg}","time":"{time}"}}"#);
    let buf = log.as_bytes();
    let _ = output.write_all(buf);
    let _ = output.write(b"\n");

    Ok(())
}

pub fn js_stringify(value: &Value<'_>) -> Result<String> {
    let mut result = String::new();

    match value.type_of() {
        Type::String => result = value.as_string().unwrap().to_string()?,
        Type::Bool => result = value.as_bool().unwrap().to_string(),
        Type::Int => result = value.as_int().unwrap().to_string(),
        Type::BigInt => {
            result = value.as_big_int().unwrap().clone().to_i64()?.to_string();
            result.push('n');
        }
        Type::Float => {
            let float = value.as_float().unwrap();

            if float.is_infinite() {
                if float.is_sign_negative() {
                    result.push('-');
                }

                result.push_str("Infinity");
            } else {
                result = float.to_string();
            }
        }
        Type::Array => {
            let array = value.as_array().unwrap();

            result.push('[');

            for (i, value) in array.clone().into_iter().enumerate() {
                let value = value?;

                if value.is_string() {
                    result.push('\'');
                }

                result.push_str(&js_stringify(&value)?);

                if value.is_string() {
                    result.push('\'');
                }

                if i < array.len() - 1 {
                    result.push(',');
                }
            }

            result.push(']');
        }
        Type::Symbol => {
            let description = value.as_symbol().unwrap().description()?;
            let description = description.as_string().unwrap().to_string()?;

            result.push_str("Symbol(");

            if description != "undefined" {
                result.push_str(&description);
            }

            result.push(')');
        }
        Type::Exception => {
            let exception = value.as_exception().unwrap();

            if let Some(message) = exception.message() {
                let name: String = exception.get("name")?;

                result.push_str(&name);
                result.push_str(": ");
                result.push_str(&message);
                result.push(NEWLINE);
            }

            if let Some(stack) = exception.stack() {
                result.push_str(&stack);
            }
        }
        Type::Object => {
            let obj = value.as_object().unwrap();
            let keys: Vec<String> = obj
                .keys::<String>()
                .map(|k| k.unwrap().to_string())
                .collect();

            result.push('{');

            for (i, key) in keys.iter().enumerate() {
                let value: Value = obj.get(key).unwrap();
                result.push_str(key);
                result.push(':');

                if value.is_string() {
                    result.push('\'');
                }

                result.push_str(&js_stringify(&value).unwrap());

                if value.is_string() {
                    result.push('\'');
                }

                if i < keys.len() - 1 {
                    result.push(',');
                }
            }

            result.push('}');
        }
        Type::Module => result.push_str("[Module]"),
        Type::Constructor | Type::Function => {
            result.push_str("[Function");

            let name: String = value.as_function().unwrap().get("name")?;

            if !name.is_empty() {
                result.push_str(": ");
                result.push_str(&name);
                result.push(']');
            } else {
                result.push_str(" (anonymous)]");
            }
        }
        Type::Uninitialized | Type::Undefined => result.push_str("undefined"),
        Type::Null => result.push_str("null"),
        Type::Unknown => result.push_str("{unknown}"),
        Type::Promise => result.push_str("[object Promise]"),
    };

    Ok(result)
}

fn now() -> std::string::String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let in_seconds = since_the_epoch.as_secs();
    let (year, month, day) = {
        let days = in_seconds / 86400;
        let years = (days - days / 146097 * 3 / 4 + 1) * 400 / 146097;
        let days_of_year = days - (years * 365 + years / 4 - years / 100 + years / 400);
        let months = (days_of_year * 12 + 6) / 367;
        let day = days_of_year - (months * 367 / 12);
        let month = months + 1;
        let year = years + 1970;
        (year as u32, month as u32, day as u32 + 1)
    };

    let hours = (in_seconds / 3600) % 24;
    let minutes = (in_seconds / 60) % 60;
    let seconds = in_seconds % 60;
    let millis = since_the_epoch.subsec_nanos();

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        year, month, day, hours, minutes, seconds, millis
    )
}
