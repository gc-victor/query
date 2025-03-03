use anyhow::{Context, Result};
use std::{fs, path::Path};

use crate::{
    generators::{
        html::{self},
        json::{self},
        toc::{self},
    },
    types::Config,
    utils::extract_pages,
};

pub fn generator(config: Config) -> Result<()> {
    fs::create_dir_all(&config.output_dir).context("Failed to create output directory")?;

    let summary_path = Path::new(&config.input_dir).join("SUMMARY.md");
    if !summary_path.exists() {
        return Err(anyhow::anyhow!(
            "SUMMARY.md not found in {}",
            config.input_dir
        ));
    }

    let summary_content = fs::read_to_string(&summary_path).context("Failed to read SUMMARY.md")?;
    let toc = toc::default_toc_generator(&summary_content);
    let pages = extract_pages(&summary_path, &config.input_dir, &config.output_dir)
        .context("Failed to extract pages")?;

    if config.generate_html {
        html::default_html_generator(&pages, &toc, &config).context("Failed to generate HTML")?;
    }

    if !config.generate_html {
        json::default_json_generator(&pages, &toc, &config).context("Failed to generate JSON")?;
    }

    Ok(())
}
