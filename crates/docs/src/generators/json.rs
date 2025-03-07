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

        let url_output_path = get_relative_path(&page.output_path, &config.output_dir);
        let html_content = generate_html(&page.content, &url_output_path);
        let metadata = extract_metadata(&page.content)?;
        let plain_text = generate_plain_text(&page.content)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::Value;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    use crate::types::{Config, GroupOfItems, Page, TableOfContents, TocItem};

    // Helper function to set up test environment
    fn setup_test_env() -> Result<(TempDir, Config)> {
        let temp_dir = TempDir::new()?;
        let output_dir = temp_dir.path().to_string_lossy().to_string();

        let config = Config::new("input".to_string(), output_dir, false, true, None);

        Ok((temp_dir, config))
    }

    // Helper to create mock pages for testing
    fn create_mock_pages() -> Vec<Page> {
        vec![
            Page {
                title: "First Page".to_string(),
                description: "First page description".to_string(),
                content: "# First Page\n\nThis is the first page content.".to_string(),
                output_path: PathBuf::from("output/first-page.html"),
                path: "first-page.md".to_string(),
                position: 0,
            },
            Page {
                title: "Second Page".to_string(),
                description: "Second page description".to_string(),
                content: "---\nkeywords: test, example\nauthor: Test Author\n---\n# Second Page\n\nThis is the second page content.".to_string(),
                output_path: PathBuf::from("output/second-page.html"),
                path: "second-page.md".to_string(),
                position: 1,
            },
        ]
    }

    // Helper to create a mock TOC
    fn create_mock_toc() -> TableOfContents {
        TableOfContents {
            items: vec![GroupOfItems {
                name: "Group 1".to_string(),
                items: vec![
                    TocItem {
                        group: "Group 1".to_string(),
                        title: "First Page".to_string(),
                        url: "first-page.html".to_string(),
                        level: 1,
                        children: Vec::new(),
                    },
                    TocItem {
                        group: "Group 1".to_string(),
                        title: "Second Page".to_string(),
                        url: "second-page.html".to_string(),
                        level: 1,
                        children: Vec::new(),
                    },
                ],
            }],
        }
    }

    // Helper to read JSON file and parse to Value
    fn read_json_file(path: &Path) -> Result<Value> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    #[test]
    fn test_create_json_files() -> Result<()> {
        let (temp_dir, config) = setup_test_env()?;
        let pages = create_mock_pages();

        create_json_files(&pages, &config)?;

        // Check first page
        let first_page_path = temp_dir.path().join("first-page.json");
        assert!(
            first_page_path.exists(),
            "First page JSON file was not created"
        );

        let first_page_json = read_json_file(&first_page_path)?;

        assert_eq!(first_page_json["title"], "First Page");
        assert_eq!(first_page_json["description"], "First page description");
        assert_eq!(
            first_page_json["path"],
            "output/first-page.html".to_string()
        );
        assert!(first_page_json["content"].as_str().unwrap().contains("<h1"));
        assert!(first_page_json["plain_text"]
            .as_str()
            .unwrap()
            .contains("First Page"));
        assert!(first_page_json["navigation"]["next"].is_object());
        assert!(first_page_json["navigation"]["previous"].is_null());

        // Check second page with metadata
        let second_page_path = temp_dir.path().join("second-page.json");
        assert!(
            second_page_path.exists(),
            "Second page JSON file was not created"
        );

        let second_page_json = read_json_file(&second_page_path)?;

        assert_eq!(second_page_json["title"], "Second Page");
        assert_eq!(second_page_json["description"], "Second page description");
        assert!(second_page_json["metadata"].is_object());
        assert_eq!(second_page_json["metadata"]["keywords"], "test, example");
        assert_eq!(second_page_json["metadata"]["author"], "Test Author");
        assert!(second_page_json["navigation"]["previous"].is_object());
        assert!(second_page_json["navigation"]["next"].is_null());

        Ok(())
    }

    #[test]
    fn test_create_toc_file() -> Result<()> {
        let (temp_dir, _) = setup_test_env()?;
        let output_dir = temp_dir.path().to_string_lossy().to_string();
        let toc = create_mock_toc();

        create_toc_file(&toc, &output_dir)?;

        let toc_file_path = temp_dir.path().join("toc.json");
        assert!(toc_file_path.exists(), "TOC file was not created");

        let toc_json = read_json_file(&toc_file_path)?;

        assert!(toc_json["items"].is_array());
        assert_eq!(toc_json["items"][0]["name"], "Group 1");
        assert_eq!(toc_json["items"][0]["items"][0]["title"], "First Page");
        assert_eq!(toc_json["items"][0]["items"][1]["title"], "Second Page");

        Ok(())
    }

    #[test]
    fn test_default_json_generator() -> Result<()> {
        let (temp_dir, config) = setup_test_env()?;
        let pages = create_mock_pages();
        let toc = create_mock_toc();

        default_json_generator(&pages, &toc, &config)?;

        // Verify both page files and TOC file were created
        let first_page_path = temp_dir.path().join("first-page.json");
        let second_page_path = temp_dir.path().join("second-page.json");
        let toc_file_path = temp_dir.path().join("toc.json");

        assert!(
            first_page_path.exists(),
            "First page JSON file was not created"
        );
        assert!(
            second_page_path.exists(),
            "Second page JSON file was not created"
        );
        assert!(toc_file_path.exists(), "TOC file was not created");

        Ok(())
    }
}
