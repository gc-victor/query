mod blob;
mod fetcher;
mod file;
mod headers;
mod url;
mod url_search_params;

use rquickjs::{Class, Ctx, Result};

use crate::{http::headers::Headers, utils::class::CustomInspectExtension};

use self::{blob::Blob, url::URL, url_search_params::URLSearchParams};

pub fn init(ctx: &Ctx) -> Result<()> {
    let globals = ctx.globals();

    fetcher::init(&globals)?;

    Class::<Blob>::define(&globals)?;
    Class::<Headers>::define_with_custom_inspect(&globals)?;
    Class::<URLSearchParams>::define(&globals)?;
    Class::<URL>::define(&globals)?;

    Ok(())
}
