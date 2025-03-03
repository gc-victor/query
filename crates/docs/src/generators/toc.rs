use std::collections::HashMap;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::types::{GroupOfItems, TableOfContents, TocItem};

pub fn generate_toc(content: &str) -> TableOfContents {
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

pub fn default_toc_generator(content: &str) -> TableOfContents {
    generate_toc(content)
}
