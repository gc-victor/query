use anyhow::{Context, Result};
use serde_json::json;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    types::{Config, Page, TableOfContents},
    utils::{
        extract_metadata, generate_html, generate_navigation, generate_plain_text,
        get_relative_path,
    },
};

pub fn create_json_files(pages: &[Page], config: &Config) -> Result<()> {
    let output_dir = Path::new(&config.output_dir);

    for page in pages.iter() {
        let navigation = generate_navigation(pages, page, &config.output_dir);

        let plain_text = generate_plain_text(&page.content);

        let url_output_path = get_relative_path(&page.output_path, &config.output_dir);
        let html_content = generate_html(&page.content, &url_output_path);
        let metadata = extract_metadata(&page.content)?;

        let page_data = json!({
            "metadata": metadata,
            "title": page.title,
            "description": page.description,
            "path": url_output_path,
            "plain_text": plain_text,
            "content": html_content,
            "markdown": page.content,
            "navigation": navigation
        });

        let output_path = Path::new(&page.path);
        let json_path = output_dir.join(output_path).with_extension("json");

        if let Some(parent) = json_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json_data = serde_json::to_string_pretty(&page_data)?;
        let mut output_file = File::create(json_path)?;
        output_file.write_all(json_data.as_bytes())?;
    }

    Ok(())
}

pub fn create_toc_file(toc: &TableOfContents, output_dir: &str) -> Result<()> {
    let output_dir = Path::new(output_dir);

    let toc_path = output_dir.join("toc.json");
    let toc_data = serde_json::to_string_pretty(&toc).context("failed to serialize TOC")?;

    let mut output_file = File::create(&toc_path).context("failed to create TOC file")?;
    output_file
        .write_all(toc_data.as_bytes())
        .context("failed to write TOC data")?;

    Ok(())
}

pub fn default_json_generator(
    pages: &[Page],
    toc: &TableOfContents,
    config: &Config,
) -> Result<()> {
    create_json_files(pages, config).context("failed to create json files")?;
    create_toc_file(toc, &config.output_dir).context("failed to create TOC file")?;

    Ok(())
}
