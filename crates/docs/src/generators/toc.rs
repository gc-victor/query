use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::collections::HashMap;

use crate::types::{GroupOfItems, TableOfContents, TocItem};

pub fn generate_toc(content: &str) -> TableOfContents {
    let parser = Parser::new_ext(content, Options::all());

    let mut groups: HashMap<String, Vec<TocItem>> = HashMap::new();
    let mut context = TocContext {
        group: String::new(),
        title: String::new(),
        url: String::new(),
    };
    let mut levels: Vec<usize> = vec![];
    let mut is_heading = false;
    let mut group_order: Vec<String> = vec![];

    for event in parser {
        match event {
            Event::Text(text) => {
                context.title = text.to_string();
                if is_heading {
                    context.group = text.to_string();
                    if !group_order.contains(&context.group) {
                        group_order.push(context.group.clone());
                        groups.insert(context.group.clone(), Vec::new());
                    }
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
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                context.url = dest_url.to_string().replace(".md", ".html");
            }
            Event::End(TagEnd::Link) => {
                if !context.group.is_empty() {
                    let current_level = levels.last().copied().unwrap_or(1);

                    let item = TocItem {
                        group: context.group.clone(),
                        title: context.title.clone(),
                        url: context.url.clone(),
                        level: current_level,
                        children: Vec::new(),
                    };

                    let group_items = groups.entry(context.group.clone()).or_default();

                    if current_level == 1 {
                        group_items.push(item);
                    } else {
                        let target_level = current_level - 1;
                        let item_clone = item.clone();

                        let parent_found =
                            if let Some(parent) = find_parent(group_items, target_level) {
                                parent.children.push(item_clone);
                                true
                            } else {
                                false
                            };

                        if !parent_found {
                            group_items.push(item);
                        }
                    };
                }
            }
            Event::Start(Tag::Item) => {
                levels.push(levels.len() + 1);
            }
            Event::End(TagEnd::Item) => {
                if !levels.is_empty() {
                    levels.pop();
                }
            }
            _ => {}
        }
    }

    TableOfContents {
        items: group_order
            .into_iter()
            .filter_map(|group| {
                groups
                    .remove(&group)
                    .map(|items| GroupOfItems { name: group, items })
            })
            .collect(),
    }
}

struct TocContext {
    group: String,
    title: String,
    url: String,
}

fn find_parent<'a>(items: &'a mut [TocItem], target_level: usize) -> Option<&'a mut TocItem> {
    for item in items.iter_mut().rev() {
        if item.level == target_level {
            return Some(item);
        }

        if !item.children.is_empty() && item.level < target_level {
            if let Some(parent) = find_parent(&mut item.children, target_level) {
                return Some(parent);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_content() {
        let content = "";
        let toc = generate_toc(content);

        assert_eq!(
            toc.items.len(),
            0,
            "Empty content should generate empty TOC"
        );
    }

    #[test]
    fn test_basic_toc() {
        let content = "- [Item 1](path/to/item1.md)";
        let toc = generate_toc(content);

        assert_eq!(
            toc.items.len(),
            0,
            "No groups should be created without H2 headings"
        );
    }

    #[test]
    fn test_single_group() {
        let content = "\
## Group 1
- [Item 1](path/to/item1.md)
- [Item 2](path/to/item2.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should have one group");
        assert_eq!(toc.items[0].name, "Group 1", "Group name should match");
        assert_eq!(
            toc.items[0].items.len(),
            2,
            "Should have two items in the group"
        );

        assert_eq!(
            toc.items[0].items[0].title, "Item 1",
            "First item title should match"
        );
        assert_eq!(
            toc.items[0].items[0].url, "path/to/item1.html",
            "First item URL should match and have .md replaced with .html"
        );
        assert_eq!(
            toc.items[0].items[0].level, 1,
            "First item level should be 1"
        );

        assert_eq!(
            toc.items[0].items[1].title, "Item 2",
            "Second item title should match"
        );
        assert_eq!(
            toc.items[0].items[1].url, "path/to/item2.html",
            "Second item URL should match and have .md replaced with .html"
        );
    }

    #[test]
    fn test_multiple_groups() {
        let content = "\
## Group 1
- [Item 1](path/to/item1.md)
- [Item 2](path/to/item2.md)

## Group 2
- [Item 3](path/to/item3.md)
- [Item 4](path/to/item4.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 2, "Should have two groups");
        assert_eq!(
            toc.items[0].name, "Group 1",
            "First group name should match"
        );
        assert_eq!(
            toc.items[1].name, "Group 2",
            "Second group name should match"
        );

        assert_eq!(
            toc.items[0].items.len(),
            2,
            "First group should have two items"
        );
        assert_eq!(
            toc.items[1].items.len(),
            2,
            "Second group should have two items"
        );

        assert_eq!(
            toc.items[0].items[0].title, "Item 1",
            "First item of first group should match"
        );
        assert_eq!(
            toc.items[1].items[0].title, "Item 3",
            "First item of second group should match"
        );
    }

    #[test]
    fn test_nested_items() {
        let content = "\
## Group 1

- [Item 1](path/to/item1.md)
  - [Nested Item 1](path/to/nested1.md)
- [Item 2](path/to/item2.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should have one group");
        assert_eq!(
            toc.items[0].items.len(),
            2,
            "Should have two top-level items"
        );

        assert_eq!(
            toc.items[0].items[0].children.len(),
            1,
            "First item should have one child"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].title, "Nested Item 1",
            "Nested item title should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].url, "path/to/nested1.html",
            "Nested item URL should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].level, 2,
            "Nested item level should be 2"
        );
    }

    #[test]
    fn test_url_transformation() {
        let content = "## Group\n- [Item](file.md)";
        let toc = generate_toc(content);

        assert_eq!(
            toc.items[0].items[0].url, "file.html",
            "URL should have .md replaced with .html"
        );
    }

    #[test]
    fn test_multiple_nested_levels() {
        let content = "\
## Group 1

- [Item 1](path/to/item1.md)
  - [Nested Item 1](path/to/nested1.md)
    - [Double Nested Item](path/to/double_nested.md)
- [Item 2](path/to/item2.md)";

        let toc = generate_toc(content);

        assert_eq!(
            toc.items[0].items[0].children.len(),
            1,
            "First item should have one child"
        );

        assert_eq!(
            toc.items[0].items[0].title, "Item 1",
            "First level title should match"
        );
        assert_eq!(
            toc.items[0].items[0].url, "path/to/item1.html",
            "First level URL should match"
        );
        assert_eq!(toc.items[0].items[0].level, 1, "First level should be 1");

        assert_eq!(
            toc.items[0].items[0].children[0].title, "Nested Item 1",
            "Second level title should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].url, "path/to/nested1.html",
            "Second level URL should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].level, 2,
            "Second level should be 2"
        );

        assert_eq!(
            toc.items[0].items[0].children[0].children[0].title, "Double Nested Item",
            "Third level title should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].children[0].url, "path/to/double_nested.html",
            "Third level URL should match"
        );
        assert_eq!(
            toc.items[0].items[0].children[0].children[0].level, 3,
            "Third level should be 3"
        );

        assert_eq!(
            toc.items[0].items[1].title, "Item 2",
            "Item 2 title should match"
        );
        assert_eq!(
            toc.items[0].items[1].url, "path/to/item2.html",
            "Item 2 URL should match"
        );
        assert_eq!(toc.items[0].items[1].level, 1, "Item 2 level should be 1");
        assert_eq!(
            toc.items[0].items[1].children.len(),
            0,
            "Item 2 should have no children"
        );
    }

    #[test]
    fn test_no_links() {
        let content = "## Group 1\nSome content\n\n## Group 2\nMore content";
        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 2, "Should have two groups");
        assert_eq!(
            toc.items[0].items.len(),
            0,
            "First group should have no items"
        );
        assert_eq!(
            toc.items[1].items.len(),
            0,
            "Second group should have no items"
        );
    }

    #[test]
    fn test_links_before_any_headings() {
        let content = "\
- [Item Outside](outside.md)

## Group 1
- [Item Inside](inside.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should have one group");
        assert_eq!(toc.items[0].items.len(), 1, "Group should have one item");
        assert_eq!(
            toc.items[0].items[0].title, "Item Inside",
            "Item title inside group should match"
        );
    }

    #[test]
    fn test_malformed_links() {
        let content = "\
## Group
- [Broken Link](broken.md
- [Valid Link](valid.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should have one group");
    }

    #[test]
    fn test_complete_toc_functionality() {
        let content = r#"
# Document Title

Introduction paragraph

## First Section

Some content here

- [Basic Guide](guides/basic.md)
- [Advanced Topics](guides/advanced.md)
    - [Advanced Topic 1](guides/advanced/topic1.md)
    - [Advanced Topic 2](guides/advanced/topic2.md)
    - [Subtopic](guides/advanced/topic2/subtopic.md)

## API Reference

API documentation

- [Core API](api/core.md)
- [Extensions](api/extensions.md)

## Examples

Example code

- [Simple Example](examples/simple.md)
- [Complex Example](examples/complex.md)
    "#;

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 3, "Should have three groups");

        assert_eq!(
            toc.items[0].name, "First Section",
            "First group name should match"
        );
        assert_eq!(
            toc.items[1].name, "API Reference",
            "Second group name should match"
        );
        assert_eq!(
            toc.items[2].name, "Examples",
            "Third group name should match"
        );

        let first_group = &toc.items[0];
        assert_eq!(
            first_group.items.len(),
            2,
            "First group should have two items"
        );
        assert_eq!(
            first_group.items[0].title, "Basic Guide",
            "First item title should match"
        );
        assert_eq!(
            first_group.items[0].url, "guides/basic.html",
            "First item URL should match"
        );
        assert_eq!(
            first_group.items[0].level, 1,
            "First item level should be 1"
        );
        assert_eq!(
            first_group.items[0].children.len(),
            0,
            "First item should have no children"
        );

        assert_eq!(
            first_group.items[1].title, "Advanced Topics",
            "Second item title should match"
        );
        assert_eq!(
            first_group.items[1].children.len(),
            3,
            "Second item should have three children"
        );

        let advanced_topics_children = &first_group.items[1].children;
        assert_eq!(
            advanced_topics_children[0].title, "Advanced Topic 1",
            "First child title should match"
        );
        assert_eq!(
            advanced_topics_children[1].title, "Advanced Topic 2",
            "Second child title should match"
        );
    }

    #[test]
    fn test_find_parent_edge_cases() {
        let mut items = vec![TocItem {
            group: "Group".to_string(),
            title: "Level 1 Item".to_string(),
            url: "level1.html".to_string(),
            level: 1,
            children: vec![],
        }];

        let result = find_parent(&mut items, 2);
        assert!(
            result.is_none(),
            "Should not find a parent at level 2 when none exists"
        );

        items[0].children.push(TocItem {
            group: "Group".to_string(),
            title: "Level 2 Item".to_string(),
            url: "level2.html".to_string(),
            level: 2,
            children: vec![],
        });

        let result = find_parent(&mut items, 2);
        assert!(result.is_some(), "Should find a parent at level 2");
        assert_eq!(
            result.unwrap().title,
            "Level 2 Item",
            "Should find the correct parent"
        );

        items[0].children[0].children.push(TocItem {
            group: "Group".to_string(),
            title: "Level 3 Item".to_string(),
            url: "level3.html".to_string(),
            level: 3,
            children: vec![],
        });

        let result = find_parent(&mut items, 3);
        assert!(result.is_some(), "Should find a parent at level 3");
        assert_eq!(
            result.unwrap().title,
            "Level 3 Item",
            "Should find the correct parent"
        );
    }

    #[test]
    fn test_empty_url() {
        let content = "\
## Group 1
- [Item with no URL]()";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should have one group");
        assert_eq!(toc.items[0].items.len(), 1, "Should have one item");
        assert_eq!(toc.items[0].items[0].url, "", "URL should be empty");
    }

    #[test]
    fn test_link_without_group() {
        let content = "- [Orphan Link](orphan.md)";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 0, "Should have no groups with no headings");
    }

    #[test]
    fn test_malformed_markdown() {
        let content = "## Group\n- Broken bullet point\n[Text](url.md)\n- ]Malformed link](bad.md";

        let toc = generate_toc(content);

        assert_eq!(toc.items.len(), 1, "Should still parse the group");
        assert_eq!(toc.items[0].items.len(), 1, "Should have one valid link");
    }

    #[test]
    fn test_duplicate_group_names() {
        let content = "\
## Group 1
- [Item 1](item1.md)

## Group 1
- [Item 2](item2.md)";

        let toc = generate_toc(content);

        // The second group with the same name should update the existing group
        assert_eq!(toc.items.len(), 1, "Should have only one group");
        assert_eq!(toc.items[0].items.len(), 2, "Should have both items");
    }
}
