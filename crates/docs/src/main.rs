mod generator;
mod generators;
mod types;
mod utils;

use std::{path::PathBuf, thread, time::Duration};

use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use indicatif::ProgressBar;
use types::Config;

use crate::generator::generator;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let matches = Command::new("Markdown to HTML Generator")
        .version(VERSION)
        .author("@qery/docs")
        .about("Converts markdown files to HTML with navigation and search")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("INPUT_DIR")
                .help("Directory containing markdown files")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_DIR")
                .help("Directory to output HTML files")
                .required(true)
        )
        .arg(
            Arg::new("template")
                .long("template")
                .value_name("TEMPLATE_FILE")
                .help("HTML template file to use for generating pages (defaults to INPUT_DIR/template.html)")
        )
        .arg(
            Arg::new("search")
                .long("search")
                .help("Generate search JSON file")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("html")
                .long("html")
                .help("Generate HTML files")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    
    let pb = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(5));
        pb.inc(1);
    }

    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();
    let generate_search = matches.get_flag("search");
    let generate_html = matches.get_flag("html");

    let template_path = match (generate_html, matches.get_one::<String>("template")) {
        (true, Some(template)) => {
            let path = PathBuf::from(template);
            if !path.exists() {
                anyhow::bail!("Template file not found: {:?}", path);
            }
            Some(path)
        }
        (true, None) => {
            let path = PathBuf::from(input_dir).join("template.html");
            if !path.exists() {
                anyhow::bail!("Template file not found: {:?}", path);
            }
            Some(path)
        }
        (false, _) => None,
    };

    generator(Config::new(
        input_dir.clone(),
        output_dir.clone(),
        generate_search,
        generate_html,
        template_path,
    ))
    .context("Failed to generate documentation")?;

    pb.finish_with_message("done");

    Ok(())
}
