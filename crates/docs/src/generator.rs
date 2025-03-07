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
    let toc = toc::generate_toc(&summary_content);
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_env() -> Result<(TempDir, Config)> {
        let temp_dir = TempDir::new()?;
        let input_dir = temp_dir.path().join("input");
        let output_dir = temp_dir.path().join("output");

        fs::create_dir(&input_dir)?;

        let config = Config {
            input_dir: input_dir.to_string_lossy().to_string(),
            output_dir: output_dir.to_string_lossy().to_string(),
            generate_html: true,
            generate_search: true,
            template_path: None,
        };

        Ok((temp_dir, config))
    }

    fn create_test_summary(input_dir: &Path) -> Result<()> {
        let summary_content = r#"
# Summary

- [Chapter 1](./chapter1.md)
  - [Section 1.1](./section1_1.md)
- [Chapter 2](./chapter2.md)
"#;
        fs::write(input_dir.join("SUMMARY.md"), summary_content)?;
        fs::write(input_dir.join("chapter1.md"), "# Chapter 1\nContent")?;
        fs::write(input_dir.join("section1_1.md"), "# Section 1.1\nContent")?;
        fs::write(input_dir.join("chapter2.md"), "# Chapter 2\nContent")?;
        Ok(())
    }

    #[test]
    fn test_generator_creates_output_dir() -> Result<()> {
        let (temp_dir, config) = setup_test_env()?;
        let template_path = temp_dir.path().join("template.html");
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;
        let config = Config {
            template_path: Some(template_path),
            ..config
        };
        create_test_summary(temp_dir.path().join("input").as_path())?;

        generator(config)?;

        assert!(temp_dir.path().join("output").exists());
        Ok(())
    }

    #[test]
    fn test_generator_missing_summary() -> Result<()> {
        let (_temp_dir, config) = setup_test_env()?;

        let result = generator(config);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("SUMMARY.md not found"));
        Ok(())
    }

    #[test]
    fn test_generator_html_output() -> Result<()> {
        let (temp_dir, mut config) = setup_test_env()?;
        config.generate_html = true;
        let template_path = temp_dir.path().join("template.html");
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;
        config.template_path = Some(template_path);
        create_test_summary(temp_dir.path().join("input").as_path())?;

        generator(config)?;

        assert!(temp_dir
            .path()
            .join("output")
            .join("chapter1.html")
            .exists());
        assert!(temp_dir
            .path()
            .join("output")
            .join("section1_1.html")
            .exists());
        assert!(temp_dir
            .path()
            .join("output")
            .join("chapter2.html")
            .exists());
        Ok(())
    }

    #[test]
    fn test_generator_json_output() -> Result<()> {
        let (temp_dir, mut config) = setup_test_env()?;
        config.generate_html = false;
        create_test_summary(temp_dir.path().join("input").as_path())?;

        generator(config)?;

        assert!(temp_dir.path().join("output").join("toc.json").exists());
        assert!(temp_dir
            .path()
            .join("output")
            .join("chapter1.json")
            .exists());
        assert!(temp_dir
            .path()
            .join("output")
            .join("section1_1.json")
            .exists());
        assert!(temp_dir
            .path()
            .join("output")
            .join("chapter2.json")
            .exists());
        Ok(())
    }

    #[test]
    fn test_generator_content_validity() -> Result<()> {
        let (temp_dir, config) = setup_test_env()?;
        let template_path = temp_dir.path().join("template.html");
        fs::write(&template_path, "{{ title }}\n{{ content }}")?;
        let config = Config {
            template_path: Some(template_path),
            ..config
        };
        create_test_summary(temp_dir.path().join("input").as_path())?;

        generator(config)?;

        let html_content =
            fs::read_to_string(temp_dir.path().join("output").join("chapter1.html"))?;

        assert!(html_content.contains("Chapter 1"));
        assert!(html_content.contains("Content"));
        Ok(())
    }

    #[test]
    fn test_generator_with_invalid_input_dir() -> Result<()> {
        let (_temp_dir, mut config) = setup_test_env()?;
        config.input_dir = "nonexistent_directory".to_string();

        let result = generator(config);

        assert!(result.is_err());
        Ok(())
    }
}
