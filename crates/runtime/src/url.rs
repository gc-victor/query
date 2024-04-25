use rquickjs::{
    function::Constructor,
    module::{Declarations, Exports, ModuleDef},
    Ctx, Result,
};

use crate::module::export_default;
pub struct UrlModule;

impl ModuleDef for UrlModule {
    fn declare(declare: &Declarations<'_>) -> Result<()> {
        declare.declare(stringify!(URL))?;
        declare.declare(stringify!(URLSearchParams))?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        let globals = ctx.globals();
        let url: Constructor = globals.get(stringify!(URL))?;
        let url_search_params: Constructor = globals.get(stringify!(URLSearchParams))?;

        export_default(ctx, exports, |default| {
            default.set(stringify!(URL), url)?;
            default.set(stringify!(URLSearchParams), url_search_params)?;
            Ok(())
        })?;

        Ok(())
    }
}
