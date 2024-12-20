mod fetcher;
mod headers;

use llrt_utils::class::CustomInspectExtension;
use rquickjs::{Class, Ctx, Result};

use crate::http::headers::Headers;

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    fetcher::init(&globals)?;

    Class::<Headers>::define_with_custom_inspect(&globals)?;

    Ok(())
}
