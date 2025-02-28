use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};
use minijinja::{context, Environment};
use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug)]
struct Page {
    title: String,
    description: String,
    file_path: PathBuf,
    output_path: PathBuf,
    position: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
struct Navigation<'a> {
    previous: Option<NavLink<'a>>,
    next: Option<NavLink<'a>>,
    current: Current<'a>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct NavLink<'a> {
    title: &'a str,
    url: &'a str,
}

#[derive(Debug, Clone, serde::Serialize)]
struct Current<'a> {
    title: &'a str,
    url: &'a str,
}

#[derive(Debug, Clone, serde::Serialize)]
struct TableOfContents {
    items: Vec<GroupOfItems>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct GroupOfItems {
    name: String,
    items: Vec<TocItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct TocItem {
    group: String,
    title: String,
    url: String,
    level: usize,
    children: Vec<TocItem>,
}

#[derive(Debug, serde::Serialize)]
struct SearchDocument {
    id: usize,
    title: String,
    content: String,
    url: String,
}

struct MarkdownToHtmlGenerator {
    input_dir: String,
    output_dir: String,
    enable_search: bool,
    template_path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Markdown to HTML Generator")
        .version("1.0")
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
                .short('t')
                .long("template")
                .value_name("TEMPLATE_FILE")
                .help("HTML template file to use for generating pages (defaults to INPUT_DIR/template.html)")
        )
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .help("Disable search functionality")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let input_dir = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();
    let enable_search = !matches.get_flag("no-search");

    let template_path = if let Some(template) = matches.get_one::<String>("template") {
        PathBuf::from(template)
    } else {
        PathBuf::from(input_dir).join("template.html")
    };

    if !template_path.exists() {
        return Err(format!("Template file not found: {:?}\nA template file is required either in the input directory named 'template.html' or specified with --template", template_path).into());
    }

    println!("Using template: {:?}", template_path);

    let generator = MarkdownToHtmlGenerator::new(
        input_dir.clone(),
        output_dir.clone(),
        enable_search,
        template_path,
    );
    generator.run()
}

impl MarkdownToHtmlGenerator {
    fn new(
        input_dir: String,
        output_dir: String,
        enable_search: bool,
        template_path: PathBuf,
    ) -> Self {
        Self {
            input_dir,
            output_dir,
            enable_search,
            template_path,
        }
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.output_dir)?;

        let markdown_files = self.find_markdown_files()?;
        println!("Found {} markdown files", markdown_files.len());

        let summary_path = Path::new(&self.input_dir).join("SUMMARY.md");
        if !summary_path.exists() {
            return Err(format!("SUMMARY.md not found in {}", self.input_dir).into());
        }

        let summary_content = fs::read_to_string(&summary_path)?;
        let toc = self.generate_toc_from_events(&summary_content);
        let pages = self.extract_pages(&summary_path)?;

        println!("Found {} pages defined in SUMMARY.md", pages.len());

        let mut search_data = Vec::new();
        let mut env = Environment::new();
        let template_contents =
            fs::read_to_string(&self.template_path).expect("Failed to read template file");
        env.add_template("page", &template_contents)
            .expect("Failed to add template");

        let templates_path = Path::new(&self.input_dir).join("templates");
        let mut templates = Vec::new();
        for entry in fs::read_dir(templates_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() {
                let name = entry_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| format!("Invalid template filename: {:?}", &entry_path))?
                    .to_string();
                let content = fs::read_to_string(&entry_path)?.to_string();
                templates.push((name + ".html", content));
            }
        }

        for (template_name, template_content) in &templates {
            eprintln!("Adding template: {}", template_name);
            env.add_template(template_name, template_content)?;
        }

        for (idx, page) in pages.iter().enumerate() {
            let mut content = String::new();
            let url_output_path = self.get_relative_path(&page.output_path);

            File::open(&page.file_path)?.read_to_string(&mut content)?;

            if self.enable_search {
                let plain_text = self.extract_plain_text(&content);
                search_data.push(SearchDocument {
                    id: idx,
                    title: page.title.clone(),
                    content: plain_text,
                    url: url_output_path.to_string(),
                });
            }

            let prev_url = if page.position > 0 {
                let prev_page = &pages[page.position - 1];
                self.get_relative_path(&prev_page.output_path)
            } else {
                String::new()
            };

            let next_url = if page.position < pages.len() - 1 {
                let next_page = &pages[page.position + 1];
                self.get_relative_path(&next_page.output_path)
            } else {
                String::new()
            };

            let navigation = Navigation {
                previous: if page.position > 0 {
                    let prev_page = &pages[page.position - 1];
                    Some(NavLink {
                        title: &prev_page.title,
                        url: &prev_url,
                    })
                } else {
                    None
                },
                next: if page.position < pages.len() - 1 {
                    let next_page = &pages[page.position + 1];
                    Some(NavLink {
                        title: &next_page.title,
                        url: &next_url,
                    })
                } else {
                    None
                },
                current: Current {
                    title: &page.title,
                    url: &url_output_path,
                },
            };

            let mut html_content = String::new();
            let parser = Parser::new_ext(&content, Options::all());
            html::push_html(&mut html_content, parser);
            html_content = self.linkable_headings(&html_content, &url_output_path);

            let tmpl = env.get_template("page")?;
            let final_html = tmpl.render(context! {
                title => &page.title,
                description => &page.description,
                navigation => &navigation,
                content => &html_content,
                toc => &toc,
                search_enabled => self.enable_search
            })?;

            let mut output_file = File::create(&page.output_path)?;
            output_file.write_all(final_html.as_bytes())?;
        }

        // Generate 404 page
        self.generate_404_page(&env, &toc, &self.input_dir)?;

        if self.enable_search {
            println!("Generating JSON search index...");
            self.create_search_json(&search_data)?;
        }

        println!(
            "Conversion complete! HTML files written to {}",
            self.output_dir
        );
        if self.enable_search {
            println!(
                "Search index generated at {}/search-index.json",
                self.output_dir
            );
        }

        Ok(())
    }

    fn linkable_headings(&self, html: &str, url: &str) -> String {
        let heading_regex = Regex::new(r"<h([1-6])>(.*?)</h([1-6])>").unwrap();
        heading_regex.replace_all(html, |caps: &regex::Captures| {
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
        }).to_string()
    }

    // Generate a 404 error page using the same template as regular pages
    fn generate_404_page(
        &self,
        env: &Environment,
        toc: &TableOfContents,
        input_dir: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating 404 page...");

        // Get 404 content
        let html_404_path = Path::new(&input_dir).join("404.html");
        let html_content = if html_404_path.exists() {
            fs::read_to_string(html_404_path).expect("Failed to read template file")
        } else {
            r#"
                <div class="error-page">
                    <h1>404</h1>
                    <h2>Page Not Found</h2>
                    <p>The page you're looking for doesn't exist or has been moved.</p>
                </div>
                <style>
                    .error-page h1 {
                        font-size: 6rem;
                        line-height: 1;
                        margin: 0;
                    }
                    .error-page h2 {
                        font-size: 4rem;
                        line-height: 1;
                        margin: 0;
                    }
                    .error-page p {
                        font-size: 1.5rem;
                        line-height: 1.25;
                        margin: 0;
                    }
                </style>
            "#
            .to_string()
        };

        // Create a simplified navigation structure for the 404 page
        let navigation = Navigation {
            previous: None,
            next: None,
            current: Current {
                title: "Page Not Found",
                url: "404.html",
            },
        };

        // Render the 404 page using the main template
        let tmpl = env.get_template("page")?;
        let final_html = tmpl.render(context! {
            title => "404 - Page Not Found",
            description => "The requested page could not be found",
            navigation => &navigation,
            content => &html_content,
            toc => toc,
            search_enabled => self.enable_search
        })?;

        // Write the 404 page to the output directory
        let output_path = Path::new(&self.output_dir).join("404.html");
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(final_html.as_bytes())?;

        println!("404 page generated at {}/404.html", self.output_dir);

        Ok(())
    }

    fn find_markdown_files(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut markdown_files = Vec::new();

        for entry in WalkDir::new(&self.input_dir) {
            let entry = entry?;
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "md")
            {
                markdown_files.push(entry.path().to_path_buf());
            }
        }

        Ok(markdown_files)
    }

    fn generate_toc_from_events(&self, content: &str) -> TableOfContents {
        let parser = Parser::new_ext(content, Options::all());

        let mut groups: HashMap<String, Vec<TocItem>> =
            HashMap::from([(String::new(), Vec::new())]);
        let mut current_group_title = String::new();
        let mut current_title = String::new();
        let mut current_url = String::new();
        let mut level = 1;
        let mut is_heading = false;
        let mut group_order: Vec<String> = vec![];

        for event in parser {
            match event {
                Event::Text(text) => {
                    current_title = text.to_string();
                    if is_heading {
                        current_group_title = text.to_string();
                        group_order.push(current_group_title.clone());
                    }
                }
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H2,
                    ..
                }) => {
                    is_heading = true;
                }
                Event::End(TagEnd::Heading(_)) => {
                    is_heading = false;
                    current_group_title = current_title.clone();
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    current_url = dest_url.to_string().replace(".md", ".html");
                }
                Event::End(TagEnd::Link) => {
                    let item = TocItem {
                        group: current_group_title.clone(),
                        title: current_title.clone(),
                        url: current_url.clone(),
                        level,
                        children: Vec::new(),
                    };

                    let group_items = groups.entry(item.group.clone()).or_default();
                    if let Some(prev_item) = group_items.last_mut() {
                        if item.level > prev_item.level {
                            prev_item.children.push(item.clone());
                        } else {
                            group_items.push(item.clone());
                        }
                    } else {
                        group_items.push(item.clone());
                    }
                }
                Event::Start(Tag::Item) => {
                    level += 1;
                }
                Event::End(TagEnd::Item) => {
                    if level > 1 {
                        level -= 1;
                    }
                }
                _ => {}
            }
        }

        let grouped_items = group_order
            .into_iter()
            .filter_map(|group| {
                groups
                    .remove(&group)
                    .map(|items| GroupOfItems { name: group, items })
            })
            .collect::<Vec<_>>();

        TableOfContents {
            items: grouped_items,
        }
    }

    fn extract_pages(&self, summary_path: &Path) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        let file = File::open(summary_path)?;
        let reader = BufReader::new(file);
        let mut pages = Vec::new();
        let mut position = 0;

        // Regex to match Markdown links including any description text that follows
        // Format: [title](path.md) description
        let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)(?:\s+(.*))?")?;

        for line in reader.lines() {
            let line = line?;
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

                let input_path = Path::new(&self.input_dir).join(&relative_path);
                let relative_html_path = relative_path.replace(".md", ".html");
                let output_path = Path::new(&self.output_dir).join(&relative_html_path);

                if let Some(parent_dir) = output_path.parent() {
                    fs::create_dir_all(parent_dir)?;
                }

                pages.push(Page {
                    title,
                    description,
                    file_path: input_path,
                    output_path,
                    position,
                });

                position += 1;
            }
        }

        Ok(pages)
    }

    fn get_relative_path(&self, path: &Path) -> String {
        path.to_str()
            .unwrap_or_default()
            .to_string()
            .replace(&self.output_dir, "")
            .trim_start_matches("/")
            .to_string()
    }

    fn extract_plain_text(&self, markdown: &str) -> String {
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

    fn create_search_json(
        &self,
        search_data: &[SearchDocument],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(search_data)?;

        let output_path = Path::new(&self.output_dir).join("search-index.json");
        let mut output_file = File::create(output_path)?;
        output_file.write_all(json_data.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_new_generator() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            true,
            PathBuf::from("template.html"),
        );

        assert_eq!(generator.input_dir, "input");
        assert_eq!(generator.output_dir, "output");
        assert!(generator.enable_search);
        assert_eq!(generator.template_path, PathBuf::from("template.html"));
    }

    #[test]
    fn test_find_markdown_files() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input");
        fs::create_dir(&input_path).unwrap();

        // Create test markdown files
        fs::write(input_path.join("test1.md"), "# Test 1").unwrap();
        fs::write(input_path.join("test2.md"), "# Test 2").unwrap();
        fs::write(input_path.join("not_md.txt"), "Not markdown").unwrap();

        let generator = MarkdownToHtmlGenerator::new(
            input_path.to_str().unwrap().to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let files = generator.find_markdown_files().unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|p| p.extension().unwrap() == "md"));
    }

    #[test]
    fn test_extract_plain_text() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let markdown = "# Title\n\nSome `code` and **bold** text";
        let plain = generator.extract_plain_text(markdown);

        assert!(plain.contains("Title"));
        assert!(plain.contains("code"));
        assert!(plain.contains("bold"));
        assert!(!plain.contains('#'));
        assert!(!plain.contains('*'));
    }

    #[test]
    fn test_get_relative_path() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let path = Path::new("output/./test/file.html");
        let relative = generator.get_relative_path(path);

        assert_eq!(relative, "./test/file.html");
    }

    #[test]
    fn test_generate_toc_from_events() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let content = r#"
# Main Title

## Section 1
- [Page 1](page1.md)
- [Page 2](page2.md)

## Section 2
- [Page 3](page3.md)
- [Page 4](page4.md)
"#;

        let toc = generator.generate_toc_from_events(content);

        assert_eq!(toc.items.len(), 2);
        assert_eq!(toc.items[0].name, "Section 1");
        assert_eq!(toc.items[0].items.len(), 2);
        assert_eq!(toc.items[1].name, "Section 2");
        assert_eq!(toc.items[1].items.len(), 2);
    }

    #[test]
    fn test_create_search_json() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("output");
        fs::create_dir(&output_path).unwrap();

        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            output_path.to_str().unwrap().to_string(),
            true,
            PathBuf::from("template.html"),
        );

        let search_data = vec![SearchDocument {
            id: 0,
            title: "Test Page".to_string(),
            content: "Test content".to_string(),
            url: "test.html".to_string(),
        }];

        generator.create_search_json(&search_data).unwrap();

        let json_path = output_path.join("search-index.json");
        assert!(json_path.exists());

        let contents = fs::read_to_string(json_path).unwrap();
        assert!(contents.contains("Test Page"));
        assert!(contents.contains("Test content"));
    }

    #[test]
    fn test_extract_pages() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input");
        fs::create_dir(&input_path).unwrap();

        let summary_content = r#"
# Summary

- [Page 1](page1.md)
- [Page 2](page2.md)
"#;
        let summary_path = input_path.join("SUMMARY.md");
        fs::write(&summary_path, summary_content).unwrap();

        let generator = MarkdownToHtmlGenerator::new(
            input_path.to_str().unwrap().to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let pages = generator.extract_pages(&summary_path).unwrap();

        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].title, "Page 1");
        assert_eq!(pages[1].title, "Page 2");
        assert_eq!(pages[0].position, 0);
        assert_eq!(pages[1].position, 1);
    }

    #[test]
    fn test_navigation_generation() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input");
        fs::create_dir(&input_path).unwrap();

        // Test first page navigation
        let first_nav = Navigation {
            previous: None,
            next: Some(NavLink {
                title: "Second Page",
                url: "second.html",
            }),
            current: Current {
                title: "First Page",
                url: "first.html",
            },
        };

        assert!(first_nav.previous.is_none());
        assert!(first_nav.next.is_some());
        assert_eq!(first_nav.current.title, "First Page");
    }

    #[test]
    fn test_complex_toc_generation() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let content = r#"
# Main Title

## Section 1
- [Page 1](page1.md)
  - [Nested Page 1.1](nested1.md)
  - [Nested Page 1.2](nested2.md)

## Section 2
- [Page 2](page2.md)

### Subsection 2.1
- [Nested Page 2.1](nested3.md)
"#;

        let toc = generator.generate_toc_from_events(content);
        assert_eq!(toc.items.len(), 2);

        // Test nested structure
        let section1 = &toc.items[0];
        assert_eq!(section1.name, "Section 1");
        assert!(!section1.items.is_empty());
        assert!(!section1.items[0].children.is_empty());

        // Test different heading levels
        let section2 = &toc.items[1];
        assert_eq!(section2.name, "Section 2");
        assert!(!section2.items.is_empty());
    }

    #[test]
    fn test_error_handling() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input");
        fs::create_dir(&input_path).unwrap();

        // Test non-existent SUMMARY.md
        let generator = MarkdownToHtmlGenerator::new(
            input_path.to_str().unwrap().to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        assert!(generator.run().is_err());

        // Test malformed SUMMARY.md
        fs::write(input_path.join("SUMMARY.md"), "Invalid [markdown(syntax").unwrap();

        assert!(generator
            .extract_pages(&input_path.join("SUMMARY.md"))
            .is_ok());
        // Should not panic on malformed markdown
    }

    #[test]
    fn test_content_processing() {
        let generator = MarkdownToHtmlGenerator::new(
            "input".to_string(),
            "output".to_string(),
            false,
            PathBuf::from("template.html"),
        );

        let complex_markdown = r#"
# Title with special chars: &<>"'

```rust
fn main() {
    println!("Hello");
}
```

| Table | Header |
|-------|--------|
| Cell  | Data   |

<div>Mixed HTML content</div>

*Emphasis* and **strong**
"#;

        let plain_text = generator.extract_plain_text(complex_markdown);
        assert!(plain_text.contains("Title with special chars"));
        assert!(plain_text.contains("Hello"));
        assert!(plain_text.contains("Table"));
        assert!(plain_text.contains("Mixed HTML content"));
        assert!(plain_text.contains("Emphasis"));
        assert!(plain_text.contains("strong"));
    }

    #[test]
    fn test_toc_current_navigation() {
        // Create minimal TOC with just what we need to test
        let toc = TableOfContents {
            items: vec![GroupOfItems {
                name: "Section".to_string(),
                items: vec![
                    TocItem {
                        group: "Section".to_string(),
                        title: "Page One".to_string(),
                        url: "./page1.html".to_string(),
                        level: 1,
                        children: vec![],
                    },
                    TocItem {
                        group: "Section".to_string(),
                        title: "Page Two".to_string(),
                        url: "./page2.html".to_string(),
                        level: 1,
                        children: vec![],
                    },
                ],
            }],
        };

        // Simplified template that only tests what we need
        let template_str = r#"
            {% for group in toc.items %}
                {% for item in group.items %}
                    <li class="toc-item{% if item.url == navigation.current.url %} active{% endif %}">
                        <a href="{{ item.url }}"
                           {% if item.url == navigation.current.url %}aria-current="page"{% endif %}>
                            {{ item.title }}
                        </a>
                    </li>
                {% endfor %}
            {% endfor %}
        "#;

        let mut env = Environment::new();
        env.add_template("toc_test", template_str).unwrap();

        // Test with page1 as current
        let nav1 = Navigation {
            previous: None,
            next: None,
            current: Current {
                title: "Page One",
                url: "./page1.html",
            },
        };

        let rendered1 = env
            .get_template("toc_test")
            .unwrap()
            .render(context! {
                toc => &toc,
                navigation => &nav1,
            })
            .unwrap();

        // Core assertions for page1
        assert!(rendered1.contains("toc-item active"));
        assert!(rendered1.contains("href=\"./page1.html\""));
        assert!(rendered1.contains("aria-current=\"page\""));
        assert!(!rendered1.contains("<a href=\"./page2.html\" aria-current=\"page\">"));

        // Test with page2 as current
        let nav2 = Navigation {
            previous: None,
            next: None,
            current: Current {
                title: "Page Two",
                url: "./page2.html",
            },
        };

        let rendered2 = env
            .get_template("toc_test")
            .unwrap()
            .render(context! {
                toc => &toc,
                navigation => &nav2,
            })
            .unwrap();

        // Core assertions for page2
        assert!(rendered2.contains("href=\"./page2.html\""));
        assert!(rendered2.contains("aria-current=\"page\""));
        assert!(!rendered2.contains("<a href=\"./page1.html\" aria-current=\"page\">"));
    }

    #[test]
    fn test_generate_404_page() {
        // Create temporary directories
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input");
        let output_path = temp_dir.path().join("output");
        fs::create_dir_all(&input_path).unwrap();
        fs::create_dir_all(&output_path).unwrap();

        // Create a simple template file
        let template_content = r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>{{ title }}</title>
                <meta name="description" content="{{ description }}">
            </head>
            <body>
                <main>{{ content }}</main>
            </body>
            </html>
            "#;
        let template_path = input_path.join("template.html");
        fs::write(&template_path, template_content).unwrap();

        // Create generator
        let generator = MarkdownToHtmlGenerator::new(
            input_path.to_str().unwrap().to_string(),
            output_path.to_str().unwrap().to_string(),
            false,
            template_path,
        );

        // Create minimal environment and TOC
        let mut env = Environment::new();
        env.add_template("page", template_content).unwrap();

        let toc = TableOfContents { items: vec![] };

        // Test default 404 page generation
        generator
            .generate_404_page(&env, &toc, &input_path.to_str().unwrap().to_string())
            .unwrap();

        // Verify 404.html was created
        let output_404_path = output_path.join("404.html");
        assert!(output_404_path.exists(), "404.html was not created");

        // Check content contains expected elements
        let content = fs::read_to_string(output_404_path).unwrap();
        assert!(
            content.contains("<title>404 - Page Not Found</title>"),
            "Title not found in 404 page"
        );
        assert!(
            content.contains("Page Not Found"),
            "Page Not Found text not found"
        );

        // Test custom 404 content
        let custom_404_content = "<div>Custom 404 content</div>";
        fs::write(input_path.join("404.html"), custom_404_content).unwrap();

        // Regenerate 404 page with custom content
        generator
            .generate_404_page(&env, &toc, &input_path.to_str().unwrap().to_string())
            .unwrap();

        // Verify custom content is used
        let content = fs::read_to_string(output_path.join("404.html")).unwrap();
        assert!(
            content.contains(custom_404_content),
            "Custom 404 content not found"
        );
    }
}
