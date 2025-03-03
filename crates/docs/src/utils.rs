use anyhow::{Context, Result};
use pulldown_cmark::{html, Event, Options, Parser};
use regex::Regex;
use serde_json::{Map, Number, Value};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};
use yaml_rust2::{Yaml, YamlLoader};

use crate::types::{Current, NavLink, Navigation, Page};

pub fn extract_pages(summary_path: &Path, input_dir: &str, output_dir: &str) -> Result<Vec<Page>> {
    let file = File::open(summary_path).context("Failed to open summary file")?;
    let reader = BufReader::new(file);
    let mut pages = Vec::new();
    let mut position = 0;

    let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)(?:\s+(.*))?")?;

    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(captures) = link_regex.captures(trimmed) {
            let title = captures[1].to_string();
            let relative_path = captures[2].to_string();
            let description = captures
                .get(3)
                .map_or(String::new(), |m| m.as_str().to_string());

            let input_path = Path::new(input_dir).join(&relative_path);
            let relative_html_path = relative_path.replace(".md", ".html");
            let output_path = Path::new(output_dir).join(&relative_html_path);
            let path = relative_path
                .trim_start_matches("./")
                .trim_end_matches(".md")
                .to_string();

            if let Some(parent_dir) = output_path.parent() {
                std::fs::create_dir_all(parent_dir).context("Failed to create output directory")?;
            }

            let mut content = String::new();
            File::open(&input_path)
                .context("Failed to open input file")?
                .read_to_string(&mut content)
                .context("Failed to read input file")?;

            pages.push(Page {
                title,
                description,
                content,
                output_path,
                path,
                position,
            });

            position += 1;
        }
    }

    Ok(pages)
}

pub fn generate_html(content: &str, url_output_path: &str) -> String {
    let mut html_content = String::new();
    let parser = Parser::new_ext(&content, Options::all());
    html::push_html(&mut html_content, parser);
    linkable_headings(&html_content, url_output_path)
}

pub fn linkable_headings(html: &str, url: &str) -> String {
    let heading_regex = Regex::new(r"<h([1-6])>(.*?)</h([1-6])>").unwrap();
    heading_regex
        .replace_all(html, |caps: &regex::Captures| {
            let level = &caps[1];
            let text = &caps[2];
            let slug = text
                .to_lowercase()
                .chars()
                .filter_map(|c| {
                    if c.is_alphanumeric() {
                        Some(c)
                    } else if c.is_whitespace() {
                        Some('-')
                    } else {
                        None
                    }
                })
                .collect::<String>();

            format!(
                "<h{level} id=\"{slug}\"><a href=\"{url}#{slug}\" class=\"header-anchor\">{text}</a></h{level}>",
            )
        })
        .to_string()
}

pub fn generate_navigation(pages: &[Page], page: &Page, output_dir: &str) -> Navigation {
    let url_output_path = get_relative_path(page.output_path.as_path(), output_dir);

    let prev_url = if page.position > 0 {
        let prev_page = &pages[page.position - 1];
        get_relative_path(prev_page.output_path.as_path(), output_dir)
    } else {
        String::new()
    };

    let next_url = if page.position < pages.len() - 1 {
        let next_page = &pages[page.position + 1];
        get_relative_path(next_page.output_path.as_path(), output_dir)
    } else {
        String::new()
    };

    Navigation {
        previous: if page.position > 0 {
            let prev_page = &pages[page.position - 1];
            Some(NavLink {
                title: prev_page.title.clone(),
                url: prev_url,
            })
        } else {
            None
        },
        next: if page.position < pages.len() - 1 {
            let next_page = &pages[page.position + 1];
            Some(NavLink {
                title: next_page.title.clone(),
                url: next_url,
            })
        } else {
            None
        },
        current: Current {
            title: page.title.clone(),
            url: url_output_path,
        },
    }
}

pub fn generate_plain_text(markdown: &str) -> String {
    let mut text = String::new();
    let parser = Parser::new_ext(markdown, Options::all());

    for event in parser {
        match event {
            Event::Text(t) => {
                text.push_str(&t);
                text.push(' ');
            }
            Event::Code(c) => {
                text.push_str(&c);
                text.push(' ');
            }
            Event::Html(h) => {
                let cleaned = h.trim_start_matches('<').trim_end_matches('>');
                text.push_str(cleaned);
                text.push(' ');
            }
            Event::SoftBreak | Event::HardBreak => {
                text.push(' ');
            }
            _ => {}
        }
    }

    text
}

pub fn get_relative_path(path: &Path, output_dir: &str) -> String {
    path.to_str()
        .unwrap_or_default()
        .to_string()
        .replace(output_dir, "")
        .trim_start_matches('/')
        .to_string()
}

pub fn extract_metadata(markdown: &str) -> Result<Option<Value>> {
    if !markdown.starts_with("---") {
        return Ok(None);
    }

    if let Some(end_idx) = markdown[3..].find("---") {
        let yaml_content = &markdown[3..end_idx + 3].trim();

        match YamlLoader::load_from_str(yaml_content) {
            Ok(docs) => {
                if docs.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(
                        yaml_to_json(&docs[0].clone()).context("Failed to convert YAML to JSON")?,
                    ))
                }
            }
            Err(e) => {
                eprintln!("Failed to parse metadata: {}", e);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

fn yaml_to_json(yaml: &Yaml) -> Result<Value> {
    match yaml {
        Yaml::Real(s) => {
            // Parse as f64 for numerical values
            if let Ok(f) = s.parse::<f64>() {
                Ok(Value::Number(
                    Number::from_f64(f).unwrap_or(Number::from(0)),
                ))
            } else {
                Ok(Value::String(s.clone()))
            }
        }
        Yaml::Integer(i) => Ok(Value::Number((*i).into())),
        Yaml::String(s) => Ok(Value::String(s.clone())),
        Yaml::Boolean(b) => Ok(Value::Bool(*b)),
        Yaml::Array(a) => {
            let mut json_array = Vec::new();
            for item in a {
                json_array.push(yaml_to_json(item).context("Failed to convert array item")?);
            }
            Ok(Value::Array(json_array))
        }
        Yaml::Hash(h) => {
            let mut json_map = Map::new();
            for (key, value) in h {
                let key_str = match key {
                    Yaml::String(s) => s.clone(),
                    _ => format!("{:?}", key),
                };
                json_map.insert(
                    key_str,
                    yaml_to_json(value).context("Failed to convert hash value")?,
                );
            }
            Ok(Value::Object(json_map))
        }
        Yaml::Null => Ok(Value::Null),
        Yaml::BadValue => anyhow::bail!("Bad YAML value encountered"),
        Yaml::Alias(_) => anyhow::bail!("YAML aliases are not supported"),
    }
}
