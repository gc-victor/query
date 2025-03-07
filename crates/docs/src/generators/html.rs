use anyhow::{Context, Result};
use minijinja::{context, Environment};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    types::{Config, Page, SearchDocument, TableOfContents},
    utils::{generate_html, generate_navigation, generate_plain_text, get_relative_path},
};

pub fn create_html_files(
    pages: &[Page],
    toc: &TableOfContents,
    config: &Config,
    env: &mut Environment,
) -> Result<()> {
    for page in pages.iter() {
        let url_output_path = get_relative_path(&page.output_path, &config.output_dir);

        let navigation = generate_navigation(pages, page, &config.output_dir);
        let html_content = generate_html(&page.content, &url_output_path);

        let tmpl = env
            .get_template("page")
            .context("Failed to get page template")?;
        let final_html = tmpl
            .render(context! {
                title => &page.title,
                description => &page.description,
                navigation => &navigation,
                content => &html_content,
                toc => &toc,
                search_enabled => config.generate_search
            })
            .context("Failed to render page template")?;

        let mut output_file =
            File::create(&page.output_path).context("Failed to create output file")?;
        output_file
            .write_all(final_html.as_bytes())
            .context("Failed to write HTML content")?;
    }

    Ok(())
}

pub fn generate_search_documents(pages: &[Page], config: &Config) -> Result<Vec<SearchDocument>> {
    let mut search_data = Vec::new();

    if !config.generate_search {
        return Ok(search_data);
    }

    for page in pages.iter() {
        let url_output_path = get_relative_path(&page.output_path, &config.output_dir);
        let plain_text = generate_plain_text(&page.content)?;

        search_data.push(SearchDocument {
            title: page.title.clone(),
            content: plain_text,
            url: url_output_path.to_string(),
        });
    }

    Ok(search_data)
}

pub fn create_404_file(config: &Config, env: &Environment, toc: &TableOfContents) -> Result<()> {
    let html_404_path = Path::new(&config.input_dir).join("404.html");
    let html_content = if html_404_path.exists() {
        fs::read_to_string(html_404_path).context("Failed to read 404 template file")?
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

    let navigation = crate::types::Navigation {
        previous: None,
        next: None,
        current: crate::types::Current {
            title: "Page Not Found".to_string(),
            url: "404.html".to_string(),
        },
    };

    let tmpl = env
        .get_template("page")
        .context("Failed to get page template for 404")?;
    let final_html = tmpl
        .render(context! {
            title => "404 - Page Not Found",
            description => "The requested page could not be found",
            navigation => &navigation,
            content => &html_content,
            toc => toc,
            search_enabled => config.generate_search
        })
        .context("Failed to render 404 template")?;

    let output_path = Path::new(&config.output_dir).join("404.html");
    let mut output_file = File::create(&output_path).context("Failed to create 404.html")?;
    output_file
        .write_all(final_html.as_bytes())
        .context("Failed to write 404 content")?;

    Ok(())
}

pub fn create_search_json(search_data: &[SearchDocument], output_dir: &str) -> Result<()> {
    let json_data =
        serde_json::to_string_pretty(search_data).context("Failed to serialize search data")?;

    let output_path = Path::new(output_dir).join("search-index.json");
    let mut output_file =
        File::create(output_path).context("Failed to create search index file")?;
    output_file
        .write_all(json_data.as_bytes())
        .context("Failed to write search index")?;

    Ok(())
}

pub fn default_html_generator(
    pages: &[Page],
    toc: &TableOfContents,
    config: &Config,
) -> Result<()> {
    let mut env = setup_jinja_env(config).context("Failed to setup Jinja environment")?;

    create_html_files(pages, toc, config, &mut env).context("Failed to create HTML files")?;
    create_404_file(config, &env, toc).context("Failed to create the 404 HTML file")?;

    if config.generate_search {
        let search_data = generate_search_documents(pages, config)
            .context("Failed to generate search documents")?;

        if !search_data.is_empty() {
            create_search_json(&search_data, &config.output_dir)
                .context("Failed to create search index")?;
        }
    }

    Ok(())
}

pub fn setup_jinja_env(config: &Config) -> Result<Environment> {
    let mut env = Environment::new();

    if let Some(template_path) = &config.template_path {
        let template_contents =
            fs::read_to_string(template_path).context("Failed to read template file")?;
        env.add_template_owned("page".to_string(), template_contents)
            .context("Failed to add page template")?;

        let templates_path = Path::new(&config.input_dir).join("templates");
        if templates_path.exists() {
            for entry in
                fs::read_dir(templates_path).context("Failed to read templates directory")?
            {
                let entry = entry.context("Failed to read template entry")?;
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let name =
                        entry_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .ok_or_else(|| {
                                anyhow::anyhow!("Invalid template filename: {:?}", &entry_path)
                            })?;

                    let content =
                        fs::read_to_string(&entry_path).context("Failed to read template file")?;
                    let template_name = format!("{}.html", name);
                    env.add_template_owned(template_name, content)
                        .context("Failed to add template")?;
                }
            }
        }
    }

    Ok(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_config() -> Config {
        Config {
            input_dir: "test_input".to_string(),
            output_dir: "test_output".to_string(),
            template_path: Some(PathBuf::from("test_template.html")),
            generate_search: true,
            generate_html: true,
        }
    }

    fn create_test_page(title: &str, path: &str) -> Page {
        Page {
            title: title.to_string(),
            description: format!("Description for {}", title),
            content: format!("<h1>{}</h1><p>Content for {}</p>", title, title),
            output_path: PathBuf::from(path),
            path: format!("source/{}.md", title.to_lowercase()),
            position: 0,
        }
    }

    fn create_test_toc() -> TableOfContents {
        TableOfContents { items: vec![] }
    }

    #[test]
    fn test_create_html_files() -> Result<()> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir)?;

        let config = Config {
            input_dir: "input".to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: None,
            generate_search: true,
            generate_html: true,
        };

        let page1_path = output_dir.join("page1.html").to_str().unwrap().to_string();
        let page2_path = output_dir.join("page2.html").to_str().unwrap().to_string();

        let pages = vec![
            create_test_page("Page 1", &page1_path),
            create_test_page("Page 2", &page2_path),
        ];

        let toc = create_test_toc();

        let mut env = Environment::new();
        env.add_template_owned("page".to_string(), "{{ title }}\n{{ content }}".to_string())?;

        create_html_files(&pages, &toc, &config, &mut env)?;

        // Verify files were created
        assert!(Path::new(&page1_path).exists());
        assert!(Path::new(&page2_path).exists());

        // Verify content
        let content1 = fs::read_to_string(page1_path)?;
        let content2 = fs::read_to_string(page2_path)?;

        assert!(content1.contains("Page 1"));
        assert!(content2.contains("Page 2"));

        Ok(())
    }

    #[test]
    fn test_create_html_files_with_navigation() -> Result<()> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir)?;

        let config = Config {
            input_dir: "input".to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: None,
            generate_search: true,
            generate_html: true,
        };

        let mut pages = vec![];
        // Create pages with specific positions to ensure correct navigation
        let page1 = Page {
            title: "Page 1".to_string(),
            description: "Description for Page 1".to_string(),
            content: "<h1>Page 1</h1><p>Content for Page 1</p>".to_string(),
            output_path: PathBuf::from(output_dir.join("page1.html").to_str().unwrap()),
            path: "source/page1.md".to_string(),
            position: 0,
        };

        let page2 = Page {
            title: "Page 2".to_string(),
            description: "Description for Page 2".to_string(),
            content: "<h1>Page 2</h1><p>Content for Page 2</p>".to_string(),
            output_path: PathBuf::from(output_dir.join("page2.html").to_str().unwrap()),
            path: "source/page2.md".to_string(),
            position: 1,
        };

        let page3 = Page {
            title: "Page 3".to_string(),
            description: "Description for Page 3".to_string(),
            content: "<h1>Page 3</h1><p>Content for Page 3</p>".to_string(),
            output_path: PathBuf::from(output_dir.join("page3.html").to_str().unwrap()),
            path: "source/page3.md".to_string(),
            position: 2,
        };

        pages.push(page1);
        pages.push(page2);
        pages.push(page3);

        let toc = create_test_toc();

        let mut env = Environment::new();
        env.add_template_owned(
            "page".to_string(),
            r#"{{ title }}
{{ content }}
{% if navigation.previous %}<a class="prev" href="{{ navigation.previous.url }}">Previous: {{ navigation.previous.title }}</a>{% endif %}
{% if navigation.next %}<a class="next" href="{{ navigation.next.url }}">Next: {{ navigation.next.title }}</a>{% endif %}"#
            .to_string()
        )?;

        create_html_files(&pages, &toc, &config, &mut env)?;

        // Verify files were created
        let page1_path = output_dir.join("page1.html").to_str().unwrap().to_string();
        let page2_path = output_dir.join("page2.html").to_str().unwrap().to_string();
        let page3_path = output_dir.join("page3.html").to_str().unwrap().to_string();

        assert!(Path::new(&page1_path).exists());
        assert!(Path::new(&page2_path).exists());
        assert!(Path::new(&page3_path).exists());

        // Verify content and navigation
        let content1 = fs::read_to_string(&page1_path)?;
        let content2 = fs::read_to_string(&page2_path)?;
        let content3 = fs::read_to_string(&page3_path)?;

        // First page should only have Next
        assert!(!content1.contains("Previous:"));
        assert!(content1.contains("Next: Page 2"));

        // Middle page should have both Previous and Next
        assert!(content2.contains("Previous: Page 1"));
        assert!(content2.contains("Next: Page 3"));

        // Last page should only have Previous
        assert!(content3.contains("Previous: Page 2"));
        assert!(!content3.contains("Next:"));

        Ok(())
    }

    #[test]
    fn test_generate_search_documents() -> Result<()> {
        let config = create_test_config();
        let pages = vec![
            create_test_page("Page 1", "output/page1.html"),
            create_test_page("Page 2", "output/page2.html"),
        ];

        // Test with search enabled
        let search_docs = generate_search_documents(&pages, &config)?;
        assert_eq!(search_docs.len(), 2);
        assert_eq!(search_docs[0].title, "Page 1");
        assert_eq!(search_docs[1].title, "Page 2");
        assert!(search_docs[0].content.contains("Content for Page 1"));

        // Test with search disabled
        let config_no_search = Config {
            input_dir: config.input_dir.clone(),
            output_dir: config.output_dir.clone(),
            template_path: config.template_path.clone(),
            generate_search: false,
            generate_html: config.generate_html,
        };
        let empty_docs = generate_search_documents(&pages, &config_no_search)?;
        assert!(empty_docs.is_empty());

        // Test with empty pages
        let empty_pages: Vec<Page> = vec![];
        let no_docs = generate_search_documents(&empty_pages, &config)?;
        assert!(no_docs.is_empty());

        Ok(())
    }

    #[test]
    fn test_generate_404_page() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: None,
            generate_search: false,
            generate_html: true,
        };

        let mut env = Environment::new();
        env.add_template_owned("page".to_string(), "{{ title }}\n{{ content }}".to_string())?;

        let toc = create_test_toc();

        // Test default 404 template (no custom template exists)
        create_404_file(&config, &env, &toc)?;

        let output_path = output_dir.join("404.html");
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path)?;
        assert!(content.contains("404 - Page Not Found"));
        assert!(content.contains("Page Not Found"));

        // Test custom 404 template
        let custom_404 = "<h1>Custom 404 Page</h1>";
        fs::write(input_dir.join("404.html"), custom_404)?;

        create_404_file(&config, &env, &toc)?;

        let content = fs::read_to_string(&output_path)?;
        assert!(content.contains("Custom 404 Page"));

        Ok(())
    }

    #[test]
    fn test_generate_404_page_with_search() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: None,
            generate_search: true,
            generate_html: true,
        };

        let mut env = Environment::new();
        env.add_template_owned(
            "page".to_string(),
            "{{ title }}\n{{ content }}\n{% if search_enabled %}Search{% endif %}".to_string(),
        )?;

        let toc = create_test_toc();

        create_404_file(&config, &env, &toc)?;

        let output_path = output_dir.join("404.html");
        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path)?;
        assert!(content.contains("404 - Page Not Found"));
        assert!(content.contains("Page Not Found"));
        assert!(content.contains("Search"));

        Ok(())
    }

    #[test]
    fn test_create_search_json() -> Result<()> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path();

        let search_docs = vec![
            SearchDocument {
                title: "Page 1".to_string(),
                content: "Content 1".to_string(),
                url: "page1.html".to_string(),
            },
            SearchDocument {
                title: "Page 2".to_string(),
                content: "Content 2".to_string(),
                url: "page2.html".to_string(),
            },
        ];

        create_search_json(&search_docs, output_dir.to_str().unwrap())?;

        let json_path = output_dir.join("search-index.json");
        assert!(json_path.exists());

        let json_content = fs::read_to_string(json_path)?;
        let parsed_docs: Vec<serde_json::Value> = serde_json::from_str(&json_content)?;

        assert_eq!(parsed_docs.len(), 2);
        assert_eq!(parsed_docs[0]["title"].as_str().unwrap(), "Page 1");
        assert_eq!(parsed_docs[1]["url"].as_str().unwrap(), "page2.html");

        // Test with empty search docs
        let empty_docs: Vec<SearchDocument> = vec![];
        create_search_json(&empty_docs, output_dir.to_str().unwrap())?;

        let json_content = fs::read_to_string(output_dir.join("search-index.json"))?;
        let parsed_empty: Vec<serde_json::Value> = serde_json::from_str(&json_content)?;
        assert!(parsed_empty.is_empty());

        Ok(())
    }

    #[test]
    fn test_default_html_generator() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        let template_path = input_dir.join("template.html");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        // Create template file
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: Some(PathBuf::from(&template_path)),
            generate_search: true,
            generate_html: true,
        };

        let page_path = output_dir.join("page.html").to_str().unwrap().to_string();
        let pages = vec![create_test_page("Test Page", &page_path)];
        let toc = create_test_toc();

        default_html_generator(&pages, &toc, &config)?;

        // Verify page was created
        assert!(Path::new(&page_path).exists());

        // Verify 404 was created
        assert!(output_dir.join("404.html").exists());

        // Verify search index was created
        assert!(output_dir.join("search-index.json").exists());

        Ok(())
    }

    #[test]
    fn test_default_html_generator_no_search() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");
        let template_path = input_dir.join("template.html");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&output_dir)?;

        fs::write(&template_path, "{{ title }}\n{{ content }}")?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: output_dir.to_str().unwrap().to_string(),
            template_path: Some(PathBuf::from(&template_path)),
            generate_search: false,
            generate_html: true,
        };

        let page_path = output_dir.join("page.html").to_str().unwrap().to_string();
        let pages = vec![create_test_page("Test Page", &page_path)];
        let toc = create_test_toc();

        default_html_generator(&pages, &toc, &config)?;

        // Verify page was created
        assert!(Path::new(&page_path).exists());

        // Verify 404 was created
        assert!(output_dir.join("404.html").exists());

        // Verify search index was NOT created
        assert!(!output_dir.join("search-index.json").exists());

        Ok(())
    }

    #[test]
    fn test_setup_jinja_env() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let templates_dir = input_dir.join("templates");
        let template_path = input_dir.join("template.html");

        fs::create_dir_all(&input_dir)?;
        fs::create_dir_all(&templates_dir)?;

        // Create main template
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;

        // Create additional templates
        fs::write(
            templates_dir.join("header.html"),
            "<header>{{ title }}</header>",
        )?;
        fs::write(templates_dir.join("footer.html"), "<footer>Footer</footer>")?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: Some(template_path),
            generate_search: true,
            generate_html: true,
        };

        let env = setup_jinja_env(&config)?;

        // Test main template
        let main_template = env.get_template("page")?;
        let rendered = main_template.render(context! { title => "Test", content => "Content" })?;
        assert_eq!(rendered, "Test\nContent");

        // Test additional templates
        let header_template = env.get_template("header.html")?;
        let rendered = header_template.render(context! { title => "Test Header" })?;
        assert_eq!(rendered, "<header>Test Header</header>");

        let footer_template = env.get_template("footer.html")?;
        let rendered = footer_template.render(context! {})?;
        assert_eq!(rendered, "<footer>Footer</footer>");

        // Test with non-existent template path
        let config_no_template = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: Some(PathBuf::from("non_existent_template.html")),
            generate_search: true,
            generate_html: true,
        };

        assert!(setup_jinja_env(&config_no_template).is_err());

        Ok(())
    }

    #[test]
    fn test_setup_jinja_env_no_template() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: None,
            generate_search: true,
            generate_html: true,
        };

        let env = setup_jinja_env(&config)?;

        // Verify environment is created but has no templates
        assert!(env.get_template("page").is_err());
        assert!(env.get_template("header.html").is_err());
        assert!(env.get_template("footer.html").is_err());

        Ok(())
    }

    #[test]
    fn test_setup_jinja_env_non_existent_template() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");

        // Test with non-existent template path
        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: Some(PathBuf::from("non_existent_template.html")),
            generate_search: true,
            generate_html: true,
        };

        assert!(setup_jinja_env(&config).is_err());

        Ok(())
    }

    #[test]
    fn test_setup_jinja_env_no_template_path() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        fs::create_dir_all(&input_dir)?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: None, // Explicitly set to None
            generate_search: true,
            generate_html: true,
        };

        let env = setup_jinja_env(&config)?;

        // Assert that the environment is created successfully.
        assert!(env.get_template("page").is_err());
        assert!(env.get_template("header.html").is_err());
        Ok(())
    }

    #[test]
    fn test_setup_jinja_env_templates_dir_error() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_dir = temp_dir.path().join("input");
        let template_path = input_dir.join("template.html");

        fs::create_dir_all(&input_dir)?;
        // Don't create the 'templates' directory

        // Create main template
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;

        let config = Config {
            input_dir: input_dir.to_str().unwrap().to_string(),
            output_dir: "output".to_string(),
            template_path: Some(template_path), // Provide a valid template
            generate_search: true,
            generate_html: true,
        };

        let env = setup_jinja_env(&config)?; // Expect success since the initial template loading should succeed.

        // Test the existence of the 'page' template which should be available:
        assert!(env.get_template("page").is_ok());
        // Test the non-existence of other templates:
        assert!(env.get_template("header.html").is_err());

        Ok(())
    }
}
