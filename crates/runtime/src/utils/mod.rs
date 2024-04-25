use rquickjs::{
    cstr,
    module::{Declarations, Exports, ModuleDef},
    Ctx, Function, Result,
};

use crate::module::export_default;

pub mod bind_to_params;
pub mod class;
pub mod clone;
pub mod object;
pub mod query_to_json;
pub mod result;
pub mod string;

pub struct UtilModule;

impl ModuleDef for UtilModule {
    fn declare(declare: &Declarations<'_>) -> Result<()> {
        declare.declare(stringify!(TextDecoder))?;
        declare.declare(stringify!(TextEncoder))?;
        declare.declare_c_str(cstr!("default"))?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            let globals = ctx.globals();

            let encoder: Function = globals.get(stringify!(TextEncoder))?;
            let decoder: Function = globals.get(stringify!(TextDecoder))?;

            default.set(stringify!(TextEncoder), encoder)?;
            default.set(stringify!(TextDecoder), decoder)?;

            Ok(())
        })
    }
}
