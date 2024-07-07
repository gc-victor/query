// CREDIT: https://github.com/yasojs/yaso/blob/e2b56fef23a46cb80f7ed188531d02be6cf1b6b5/src/console.rs
use std::{
    fmt,
    io::{stderr, stdout, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use rquickjs::{
    function::{Func, Rest},
    Ctx, Error, Object, Result, Type, Value,
};

#[derive(Debug)]
pub enum LogLevel {
    Debug = 20,
    Info = 30,
    Warn = 40,
    Error = 50,
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    let console = Object::new(ctx.clone())?;

    console.set("debug", Func::from(console_debug))?;
    console.set("error", Func::from(console_error))?;
    console.set("info", Func::from(console_info))?;
    console.set("log", Func::from(console_log))?;
    console.set("warn", Func::from(console_warn))?;

    globals.set("___print", console)?;

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

fn console_warn(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stderr(), args, LogLevel::Warn)
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
        LogLevel::Debug => 20,
        LogLevel::Info => 30,
        LogLevel::Warn => 40,
        LogLevel::Error => 50,
    };
    let msg = js_format(args)?;
    let msg = msg.replace('\n', "\\n");
    let msg = msg.replace('\r', "\\r");
    let msg = msg.replace('\t', "\\t");
    let msg = if msg.contains("\\\"") {
        msg
    } else {
        msg.replace('"', "\\\"")
    };
    let msg = msg.trim();
    let time = now();
    let log = format!(r#"{{"console":true,"level":{level},"msg":"{msg}","time":"{time}"}}"#);
    let buf = log.as_bytes();
    let _ = output.write_all(buf);
    let _ = output.write(b"\n");

    Ok(())
}

fn js_format(args: Rest<Value<'_>>) -> Result<String> {
    let mut result = String::new();

    for arg in args.iter() {
        result.push_str(js_stringify(arg)?.as_str());
        result.push(' ');
    }

    Ok(result)
}

pub fn js_stringify(value: &Value<'_>) -> Result<String> {
    let mut result = String::new();

    match value.type_of() {
        Type::String => {
            result = value
                .as_string()
                .ok_or(Error::new_from_js("value", "string"))?
                .to_string()?
        }
        Type::Bool => {
            result = value
                .as_bool()
                .ok_or(Error::new_from_js("value", "bool"))?
                .to_string()
        }
        Type::Int => {
            result = value
                .as_int()
                .ok_or(Error::new_from_js("value", "int"))?
                .to_string()
        }
        Type::BigInt => {
            result = value
                .as_big_int()
                .ok_or(Error::new_from_js("value", "bigint"))?
                .clone()
                .to_i64()?
                .to_string();
            result.push('n');
        }
        Type::Float => {
            let float = value
                .as_float()
                .ok_or(Error::new_from_js("value", "float"))?;

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
            let array = value
                .as_array()
                .ok_or(Error::new_from_js("value", "array"))?;

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
            let description = value
                .as_symbol()
                .ok_or(Error::new_from_js("value", "symbol"))?
                .description()?;

            let description = description.as_string().unwrap().to_string()?;

            result.push_str("Symbol(");

            if description != "undefined" {
                result.push_str(&description);
            }

            result.push(')');
        }
        Type::Exception => {
            let exception = value
                .as_exception()
                .ok_or(Error::new_from_js("value", "exception"))?;

            if let Some(message) = exception.message() {
                let name: String = exception.get("name")?;

                result.push_str(&name);
                result.push_str(": ");
                result.push_str(&message);
                result.push('\n');
            }

            if let Some(stack) = exception.stack() {
                result.push_str(&stack);
            }
        }
        Type::Object => {
            let obj = value
                .as_object()
                .ok_or(Error::new_from_js("value", "object"))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use rquickjs::function::Rest;
    use rquickjs::{Context, Ctx, Runtime};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Console {
        pub console: bool,
        pub level: u8,
        pub msg: String,
        pub time: String,
    }

    #[test]
    fn test_log_string() {
        test_with(|ctx| {
            let mut buffer = Vec::new();

            let value = ctx.eval(r#"["test"]"#).unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();

            assert_eq!("test", console.msg);
        });
    }

    #[test]
    fn test_log_int() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[1]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("1", console.msg);
        })
    }

    #[test]
    fn test_log_bool() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[true]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("true", console.msg);
        })
    }

    #[test]
    fn test_log_float() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[1.5]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("1.5", console.msg);
        })
    }

    #[test]
    fn test_log_bigint() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[BigInt('9007199254740991')]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("9007199254740991n", console.msg);
        })
    }

    #[test]
    fn test_log_array() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[[1,2,3]]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("[1,2,3]", console.msg);
        })
    }

    #[test]
    fn test_log_array_max_depth() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[[1,[2,[3]]]]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("[1,[2,[3]]]", console.msg);
        })
    }

    #[test]
    fn test_log_object() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[{'a':1}]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("{a:1}", console.msg);
        })
    }

    #[test]
    fn test_log_object_complex() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[{[['a','b']]:{'c': [{1: 'd'}]}}]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("{a,b:{c:[{1:'d'}]}}", console.msg);
        })
    }

    #[test]
    fn test_log_object_max_depth() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[{1:{2:{3:4}}}]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("{1:{2:{3:4}}}", console.msg);
        })
    }

    #[test]
    fn test_log_symbol() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[Symbol('a')]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("Symbol(a)", console.msg);
        })
    }

    #[test]
    fn test_log_function() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("const myfunc = () => {}; [myfunc]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("[Function: myfunc]", console.msg);
        })
    }

    #[test]
    fn test_log_promise() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[Promise.resolve(1)]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("[object Promise]", console.msg);
        })
    }

    #[test]
    fn test_log_proxy() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[new Proxy({ a: 1 }, {})]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("{a:1}", console.msg);
        })
    }

    #[test]
    fn test_log_undefined() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[undefined]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("undefined", console.msg);
        })
    }

    #[test]
    fn test_log_null() {
        test_with(|ctx| {
            let mut buffer = Vec::new();
            let value = ctx.eval("[null]").unwrap();
            log_write(&mut buffer, Rest(value), LogLevel::Info).unwrap();
            let output = &String::from_utf8(buffer).unwrap();
            let console: Console = serde_json::from_str(output).unwrap();
            assert_eq!("null", console.msg);
        })
    }

    fn test_with<F, R>(func: F) -> R
    where
        F: FnOnce(Ctx) -> R,
    {
        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();
        ctx.with(func)
    }
}
