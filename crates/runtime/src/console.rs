use rquickjs::{
    function::{Func, Rest},
    Ctx, Object, Result, Type, Value,
};

use std::io::{stderr, stdout, Write};

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    let console = Object::new(ctx.clone())?;

    console.set("log", Func::from(log_stdout))?;
    console.set("info", Func::from(log_stdout))?;
    console.set("trace", Func::from(log_stdout))?;
    console.set("debug", Func::from(log_stdout))?;
    console.set("error", Func::from(log_stderr))?;
    console.set("warn", Func::from(log_stderr))?;
    console.set("assert", Func::from(log_assert))?;

    globals.set("console", console)?;

    Ok(())
}

fn log_assert(expression: bool, args: Rest<Value<'_>>) -> Result<()> {
    if !expression {
        log_stderr(args)?;
    }

    Ok(())
}

fn log_stdout(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stdout(), args)
}

fn log_stderr(args: Rest<Value<'_>>) -> Result<()> {
    log_write(stderr(), args)
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
                result.push_str(&js_stringify(&value?)?);

                if i < array.len() - 1 {
                    result.push_str(", ");
                }
            }

            result.push(']');
        }

        Type::Symbol => {
            let description = value.as_symbol().unwrap().description()?;
            let description = description.to_string()?;

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

                result.push('\n');
            }

            if let Some(stack) = exception.stack() {
                result.push_str(&stack);
            }
        }

        // TODO: stringify these properly
        Type::Object => result.push_str("[Object]"),
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
    };

    Ok(result)
}

fn log_write<O>(mut output: O, args: Rest<Value<'_>>) -> Result<()>
where
    O: Write,
{
    let _ = output.write_all(js_format(args)?.as_bytes());

    let _ = output.write(b"\n");

    Ok(())
}

#[derive(rquickjs::class::Trace)]
#[rquickjs::class]
pub struct Console {}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl Console {
    #[qjs(constructor)]
    pub fn new() -> Self {
        // We ignore the parameters for now since we don't support stream
        Self {}
    }

    pub fn log(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }

    pub fn debug(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }
    pub fn info(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }
    pub fn trace(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }
    pub fn error(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }
    pub fn warn(&self, args: Rest<Value<'_>>) -> Result<()> {
        log_stdout(args)
    }
    pub fn assert(&self, expression: bool, args: Rest<Value<'_>>) -> Result<()> {
        log_assert(expression, args)
    }
}
