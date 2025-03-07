use anyhow::{Context, Result};
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
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
    let parser = Parser::new_ext(content, Options::all());
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

pub fn generate_plain_text(markdown: &str) -> Result<String> {
    let markdown = markdown.trim_start();
    let markdown = if markdown.starts_with("---") {
        if let Some(end) = markdown[3..].find("---") {
            markdown[end + 6..].trim_start().to_string()
        } else {
            markdown.to_string()
        }
    } else {
        markdown.to_string()
    };

    let html_tag_re = Regex::new(r"<[^>]*>")?;
    let cleaned_markdown = html_tag_re.replace_all(&markdown, " ");
    let cleaned_markdown = cleaned_markdown.replace(r#"  "#, " ");

    let mut text = String::new();
    let parser = Parser::new_ext(&cleaned_markdown, Options::all());
    let mut in_code_block = false;

    for event in parser {
        match event {
            Event::Text(t) => {
                if !in_code_block {
                    text.push_str(&t);
                    text.push(' ');
                }
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
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            _ => {}
        }
    }

    Ok(text)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_extract_pages() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        std::fs::create_dir_all(&input_dir).unwrap();
        std::fs::create_dir_all(&output_dir).unwrap();

        let summary_content = "# Summary\n\n- [Test Page](./test.md) Test description";
        let summary_path = input_dir.join("SUMMARY.md");
        std::fs::write(&summary_path, summary_content).unwrap();

        std::fs::write(input_dir.join("test.md"), "# Test Content").unwrap();

        let pages = extract_pages(
            &summary_path,
            input_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        )
        .unwrap();

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "Test Page");
        assert_eq!(pages[0].description, "Test description");
        assert_eq!(pages[0].content, "# Test Content");
    }

    #[test]
    fn test_generate_html() {
        let content = "# Test\n\nParagraph";
        let html = generate_html(content, "test.html");
        assert!(html.contains("<h1 id=\"test\">"));
        assert!(html.contains("<p>Paragraph</p>"));
    }

    #[test]
    fn test_linkable_headings() {
        let html = "<h1>Test</h1>";
        let result = linkable_headings(html, "test.html");
        assert_eq!(
            result,
            "<h1 id=\"test\"><a href=\"test.html#test\" class=\"header-anchor\">Test</a></h1>"
        );
    }

    #[test]
    fn test_generate_navigation() {
        let pages = vec![
            Page {
                title: "Page 1".to_string(),
                description: String::new(),
                content: String::new(),
                output_path: Path::new("page1.html").to_path_buf(),
                path: "page1".to_string(),
                position: 0,
            },
            Page {
                title: "Page 2".to_string(),
                description: String::new(),
                content: String::new(),
                output_path: Path::new("page2.html").to_path_buf(),
                path: "page2".to_string(),
                position: 1,
            },
        ];

        let nav = generate_navigation(&pages, &pages[0], "");
        assert!(nav.previous.is_none());
        assert_eq!(nav.next.unwrap().title, "Page 2");
    }

    #[test]
    fn test_generate_plain_text() {
        let markdown = "# Heading\n\n```rust\nfn main() {}\n```\n\nText";
        let result = generate_plain_text(markdown).unwrap();
        assert!(result.contains("Heading"));
        assert!(result.contains("Text"));
        assert!(!result.contains("fn main()"));
    }

    #[test]
    fn test_plain_text_metadata_removal() {
        // Test with metadata
        let markdown_with_metadata = r#"---
title: Test Document
author: Test Author
---
# Actual Content
This is the main content."#;

        let result = generate_plain_text(markdown_with_metadata).unwrap();
        assert!(!result.contains("title:"));
        assert!(!result.contains("author:"));
        assert!(result.contains("Actual Content"));
        assert!(result.contains("This is the main content"));

        // Test without metadata
        let markdown_without_metadata = "# Direct Content\nNo metadata here";
        let result = generate_plain_text(markdown_without_metadata).unwrap();
        assert!(result.contains("Direct Content"));
        assert!(result.contains("No metadata here"));

        // Test with empty metadata
        let markdown_empty_metadata = r#"---
---
# Content After Empty Metadata"#;
        let result = generate_plain_text(markdown_empty_metadata).unwrap();
        assert!(result.contains("Content After Empty Metadata"));
        assert!(!result.contains("---"));

        // Test with malformed metadata (missing end delimiter)
        let markdown_malformed = r#"---
title: Test
# Content"#;
        let result = generate_plain_text(markdown_malformed).unwrap();
        assert!(result.contains("title: Test"));
        assert!(result.contains("Content"));

        // Test with multiple metadata sections (should only remove first)
        let markdown_multiple = r#"---
first: metadata
---
# Content
---
second: metadata
---"#;
        let result = generate_plain_text(markdown_multiple).unwrap();
        assert!(!result.contains("first: metadata"));
        assert!(result.contains("Content"));
        assert!(result.contains("second: metadata"));
    }

    #[test]
    fn test_plain_text_html_tag_removal() {
        // Test basic HTML tags
        let text_with_basic_tags = "<p>Simple paragraph</p><div>Another block</div>";
        let result = generate_plain_text(text_with_basic_tags).unwrap();
        assert_eq!(result.trim(), "Simple paragraph Another block");

        // Test nested HTML tags
        let text_with_nested_tags = "<div><p>Nested <span>content</span></p></div>";
        let result = generate_plain_text(text_with_nested_tags).unwrap();
        assert_eq!(result.trim(), "Nested content");

        // Test HTML with attributes
        let text_with_attributes =
            r#"<div class="test" id="main">Content with <a href="link">link</a></div>"#;
        let result = generate_plain_text(text_with_attributes).unwrap();
        assert_eq!(result.trim(), "Content with link");

        // Test self-closing tags
        let text_with_self_closing = "Text with<br/>break and<img src='test.jpg'/>image";
        let result = generate_plain_text(text_with_self_closing).unwrap();
        assert_eq!(result.trim(), "Text with break and image");

        // Test mixed markdown and HTML
        let mixed_content = "# Heading\n<div>HTML content</div>\n**Bold text**";
        let result = generate_plain_text(mixed_content).unwrap();
        assert!(result.contains("Heading"));
        assert!(result.contains("HTML content"));
        assert!(result.contains("Bold text"));
    }

    #[test]
    fn test_get_relative_path() {
        let path = Path::new("/test/dir/file.html");
        let output_dir = "/test";
        assert_eq!(get_relative_path(path, output_dir), "dir/file.html");
    }

    #[test]
    fn test_extract_metadata() {
        let markdown = "---\ntitle: Test\n---\nContent";
        let metadata = extract_metadata(markdown).unwrap().unwrap();
        assert_eq!(metadata["title"], "Test");
    }

    #[test]
    fn test_yaml_to_json() {
        let yaml = Yaml::String("test".to_string());
        let json = yaml_to_json(&yaml).unwrap();
        assert_eq!(json.as_str().unwrap(), "test");
    }

    #[test]
    fn test_end_to_end_document_processing() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        let summary_content = r#"# Summary

- [Introduction](./intro.md) Getting started
- [Advanced](./advanced/advanced.md) Advanced topics"#;

        let intro_content = r#"---
title: Introduction
author: Test Author
version: 1.0
---

# Introduction

This is the introduction page.

## Getting Started

Here's how to get started."#;

        let advanced_content = r#"---
title: Advanced
author: Test Author
version: 2.0
---

# Advanced Topics

This covers advanced topics.

## Complex Features

Advanced feature details here."#;

        let summary_path = input_dir.join("SUMMARY.md");
        fs::write(&summary_path, summary_content).unwrap();
        fs::write(input_dir.join("intro.md"), intro_content).unwrap();

        fs::create_dir_all(input_dir.join("advanced")).unwrap();
        fs::write(input_dir.join("advanced/advanced.md"), advanced_content).unwrap();

        let pages = extract_pages(
            &summary_path,
            input_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        )
        .unwrap();

        assert_eq!(pages.len(), 2);

        for page in &pages {
            let metadata = extract_metadata(&page.content).unwrap().unwrap();
            let html = generate_html(&page.content, &page.path);
            let plain_text = generate_plain_text(&page.content).unwrap();
            let nav = generate_navigation(&pages, page, output_dir.to_str().unwrap());

            assert!(metadata.get("title").is_some());
            assert!(metadata.get("author").is_some());
            assert!(metadata.get("version").is_some());

            assert!(!plain_text.contains("---"));

            assert!(html.contains("<h1"));
            assert!(html.contains("<h2"));
            assert!(html.contains("header-anchor"));

            assert_eq!(nav.current.title, page.title);
            if page.position == 0 {
                assert!(nav.previous.is_none());
                assert!(nav.next.is_some());
            } else {
                assert!(nav.previous.is_some());
                assert!(nav.next.is_none());
            }
        }
    }

    #[test]
    fn test_error_handling() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        let invalid_summary = r#"# Summary

- [Missing](./missing.md)
- [Invalid Markdown](./invalid.md)"#;

        let invalid_markdown = r#"---
title: Invalid
---
# Heading

[Invalid link](Invalid"#;

        let summary_path = input_dir.join("SUMMARY.md");
        fs::write(&summary_path, invalid_summary).unwrap();
        fs::write(input_dir.join("invalid.md"), invalid_markdown).unwrap();

        let result = extract_pages(
            &summary_path,
            input_dir.to_str().unwrap(),
            output_dir.to_str().unwrap(),
        );

        assert!(result.is_err() || result.unwrap().is_empty());

        let html = generate_html(invalid_markdown, "test.html");
        assert!(html.contains("<h1"));

        let plain = generate_plain_text(invalid_markdown).unwrap();
        assert!(plain.contains("Heading"));
        assert!(!plain.contains("---"));
    }

    #[test]
    fn test_plain_text_with_code_elements() {
        let markdown = "Regular text with `inline code` elements";
        let result = generate_plain_text(markdown).unwrap();
        assert!(result.contains("inline code"));
        assert!(result.contains("Regular text"));
    }

    #[test]
    fn test_plain_text_with_breaks() {
        let markdown = "Line with soft break\nand hard break\n\nNext paragraph";
        let result = generate_plain_text(markdown).unwrap();
        assert!(result.contains("Line with soft break"));
        assert!(result.contains("and hard break"));
        assert!(result.contains("Next paragraph"));
        assert!(result.matches(' ').count() >= 3);
    }

    #[test]
    fn test_extract_metadata_error_handling() {
        let invalid_yaml = r#"---
title: "Test
unclosed_quote
---
Content"#;

        let result = extract_metadata(invalid_yaml);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_extract_metadata_no_end_marker() {
        let missing_end = r#"---
title: Test
Content"#;

        let result = extract_metadata(missing_end);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_extract_metadata_empty_yaml() {
        let empty_yaml = r#"---
---
Content"#;

        let result = extract_metadata(empty_yaml);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_none() || value.unwrap().as_object().unwrap().is_empty());
    }

    #[test]
    fn test_yaml_to_json_numeric_types() {
        let real_unparseable = Yaml::Real("not.a.number".to_string());
        let result = yaml_to_json(&real_unparseable).unwrap();
        assert_eq!(result, Value::String("not.a.number".to_string()));

        let integer = Yaml::Integer(42);
        let result = yaml_to_json(&integer).unwrap();
        assert_eq!(result, Value::Number(42.into()));
    }

    #[test]
    fn test_yaml_to_json_boolean() {
        let boolean_true = Yaml::Boolean(true);
        let result = yaml_to_json(&boolean_true).unwrap();
        assert_eq!(result, Value::Bool(true));

        let boolean_false = Yaml::Boolean(false);
        let result = yaml_to_json(&boolean_false).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_yaml_to_json_array() {
        let array = Yaml::Array(vec![
            Yaml::Integer(1),
            Yaml::String("test".to_string()),
            Yaml::Boolean(true),
        ]);

        let result = yaml_to_json(&array).unwrap();
        let array_value = result.as_array().unwrap();
        assert_eq!(array_value.len(), 3);
        assert_eq!(array_value[0], Value::Number(1.into()));
        assert_eq!(array_value[1], Value::String("test".to_string()));
        assert_eq!(array_value[2], Value::Bool(true));
    }

    #[test]
    fn test_yaml_to_json_null() {
        let null = Yaml::Null;
        let result = yaml_to_json(&null).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_yaml_to_json_error_cases() {
        let bad_value = Yaml::BadValue;
        let result = yaml_to_json(&bad_value);
        assert!(result.is_err());
    }
}
