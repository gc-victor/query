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
        
        eprintln!("Processing page: {} - {}", page.title, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

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
        let plain_text = generate_plain_text(&page.content);

        search_data.push(SearchDocument {
            title: page.title.clone(),
            content: plain_text,
            url: url_output_path.to_string(),
        });
    }

    Ok(search_data)
}

pub fn generate_404_page(config: &Config, env: &Environment, toc: &TableOfContents) -> Result<()> {
    // println!("Generating 404 page...");

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

    // println!("404 page generated at {}/404.html", config.output_dir);

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

    // println!("Generated search index JSON file");
    Ok(())
}

pub fn default_html_generator(
    pages: &[Page],
    toc: &TableOfContents,
    config: &Config,
) -> Result<()> {
    let mut env = setup_jinja_env(config).context("Failed to setup Jinja environment")?;

    create_html_files(pages, toc, config, &mut env).context("Failed to create HTML files")?;
    generate_404_page(config, &env, toc).context("Failed to generate 404 page")?;

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
