use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub description: String,
    pub content: String,
    pub output_path: PathBuf,
    pub path: String,
    pub position: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Navigation {
    pub previous: Option<NavLink>,
    pub next: Option<NavLink>,
    pub current: Current,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NavLink {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Current {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TableOfContents {
    pub items: Vec<GroupOfItems>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GroupOfItems {
    pub name: String,
    pub items: Vec<TocItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TocItem {
    pub group: String,
    pub title: String,
    pub url: String,
    pub level: usize,
    pub children: Vec<TocItem>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchDocument {
    pub title: String,
    pub content: String,
    pub url: String,
}

pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub generate_search: bool,
    pub generate_html: bool,
    pub template_path: Option<PathBuf>,
}

impl Config {
    pub fn new(
        input_dir: String,
        output_dir: String,
        generate_search: bool,
        generate_html: bool,
        template_path: Option<PathBuf>,
    ) -> Self {
        Self {
            input_dir,
            output_dir,
            generate_search,
            generate_html,
            template_path,
        }
    }
}
