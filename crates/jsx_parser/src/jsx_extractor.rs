/// A location of a JSX element in source code
#[derive(Debug, Clone, PartialEq)]
pub struct JSXLocation {
    start: usize,
    end: usize,
    pub content: String,
}

/// Main JSX extractor that parses JSX elements from source code
#[derive(Debug)]
pub struct JSXExtractor {
    input: String,
    pub locations: Vec<JSXLocation>,
    fragment_ranges: Vec<(usize, usize)>,
}

const INITIAL_DEPTH: usize = 1; // Initial depth for JSX element nesting
const INITIAL_POS_INCREMENT: usize = 1; // Initial position increment
const DOUBLE_QUOTE: char = '"'; // Character to denote double quote
const SINGLE_QUOTE: char = '\''; // Character to denote single quote
const BACKSLASH: char = '\\'; // Character to denote backslash
const OPEN_ANGLE_BRACKET: char = '<'; // Character to denote open angle bracket
const CLOSE_ANGLE_BRACKET: char = '>'; // Character to denote close angle bracket
const CLOSE_BRACKET_BYTE: u8 = b'>'; // Character to denote close bracket as byte
const UNDERSCORE: char = '_'; // Character to denote underscore
const HYPHEN: char = '-'; // Character to denote hypen
const DOLLAR_SIGN: char = '$'; // Character to denote dolar sign
const OPEN_CURLY_BRACE: char = '{'; // Character to denote opening curly brace
const SLASH: char = '/'; // Character to denote slash
const SLASH_BYTE: u8 = b'/'; // Character to denote slash as bytes
const EQUALS: char = '='; // Character to denote equals sign
const FRAGMENT_START: &str = "<>"; // Fragment start indicator
const FRAGMENT_END: &str = "</>"; // Fragment end indicator
const SELF_CLOSING_END: &str = "/>"; // Character sequence to denote self-closing element

impl JSXExtractor {
    /// Creates a new JSXExtractor instance
    pub fn new(input: String) -> Self {
        Self {
            input,
            locations: Vec::new(),
            fragment_ranges: Vec::new(),
        }
    }

    /// Main extraction method that handles JSX elements
    pub fn extract(&mut self) -> Result<Vec<String>, regex::Error> {
        self.extract_elements();
        self.locations.sort_by_key(|loc| loc.start);

        Ok(self
            .locations
            .iter()
            .map(|loc| loc.content.clone())
            .collect())
    }

    fn extract_elements(&mut self) {
        let mut pos = 0;
        while pos < self.input.len() {
            if let Some(start) = self.find_element_start(pos) {
                if let Some(end) = self.find_element_end(start) {
                    let content = &self.input[start..end];

                    self.fragment_ranges.push((start, end));
                    self.locations.push(JSXLocation {
                        start,
                        end,
                        content: content.to_string(),
                    });
                    pos = end;
                } else {
                    pos += INITIAL_POS_INCREMENT;
                }
            } else {
                break;
            }
        }
    }

    fn find_element_start(&self, from: usize) -> Option<usize> {
        let input = &self.input[from..];
        let mut chars = input.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            if c == OPEN_ANGLE_BRACKET {
                if let Some(&(_, next)) = chars.peek() {
                    // Handle fragments (empty element name)
                    if next == CLOSE_ANGLE_BRACKET {
                        return Some(from + i);
                    }

                    // Handle regular elements - first character must be a letter or underscore or a dollar sign
                    if !next.is_ascii_alphabetic() && next != UNDERSCORE && next != DOLLAR_SIGN {
                        continue;
                    }

                    chars.next(); // consume the first valid character

                    // Look ahead to ensure the element name is valid
                    while let Some(&(_, c)) = chars.peek() {
                        if c.is_whitespace()
                            || c == CLOSE_ANGLE_BRACKET
                            || c == OPEN_CURLY_BRACE
                            || (c == SLASH
                                && chars
                                    .next()
                                    .and_then(|_| chars.peek())
                                    .is_some_and(|&(_, c)| c == CLOSE_ANGLE_BRACKET))
                        {
                            return Some(from + i);
                        }

                        if !c.is_alphanumeric() && c != UNDERSCORE && c != DOLLAR_SIGN && c != HYPHEN {
                            break;
                        }
                        chars.next();
                    }
                }
            }
        }
        None
    }

    fn find_element_end(&self, start: usize) -> Option<usize> {
        let mut depth = INITIAL_DEPTH;
        let mut pos = start + INITIAL_POS_INCREMENT;
        let mut in_string = false;
        let mut string_char = None;
        let mut escaped = false;
        let is_fragment = self.input[start..].starts_with(FRAGMENT_START);

        while pos < self.input.len() {
            let c = self.input[pos..].chars().next()?;

            // Handle string context
            if in_string {
                if !escaped && Some(c) == string_char {
                    in_string = false;
                }
                escaped = c == BACKSLASH && !escaped;
                pos += c.len_utf8();
                continue;
            }

            // Start string context only if character is after an equals sign in an attribute
            if (c == DOUBLE_QUOTE || c == SINGLE_QUOTE)
                && self.is_in_attribute_context(&self.input[..pos])
            {
                in_string = true;
                string_char = Some(c);
                pos += c.len_utf8();
                continue;
            }

            match c {
                OPEN_ANGLE_BRACKET => {
                    let next_pos = pos + 1;
                    if next_pos < self.input.len() {
                        if self.input.as_bytes()[next_pos] == SLASH_BYTE {
                            // Found a closing tag
                            if self.input[pos..].starts_with(FRAGMENT_END) {
                                depth -= 1;
                                if depth == 0 {
                                    return Some(pos + FRAGMENT_END.len());
                                }
                                pos += FRAGMENT_END.len() - 1;
                                continue;
                            } else {
                                depth -= 1;
                                if depth == 0 {
                                    return self.find_closing_bracket(next_pos);
                                }
                            }
                        } else if self.is_valid_jsx_start(&self.input[pos..])
                            || self.input[pos..].starts_with(FRAGMENT_START)
                        {
                            depth += 1;
                        }
                    }
                }
                SLASH => {
                    if pos > 0 && self.input.as_bytes()[pos - 1] == OPEN_ANGLE_BRACKET as u8 {
                        // Handle self-closing tags
                        if is_fragment && self.input[pos..].starts_with(SELF_CLOSING_END) {
                            depth -= 1;
                            if depth == 0 {
                                return Some(pos + 2);
                            }
                            pos += 2;
                            continue;
                        }
                    } else if pos + 1 < self.input.len()
                        && self.input.as_bytes()[pos + 1] == CLOSE_BRACKET_BYTE
                    {
                        // Handle self-closing tags with />
                        depth -= 1;
                        if depth == 0 {
                            return Some(pos + 2);
                        }
                        pos += 1;
                    }
                }
                _ => {}
            }
            pos += c.len_utf8();
        }
        None
    }

    #[inline]
    fn is_in_attribute_context(&self, input: &str) -> bool {
        let chars = input.chars().rev();
        let mut found_equals = false;

        for c in chars {
            match c {
                EQUALS => {
                    found_equals = true;
                    break;
                }
                CLOSE_ANGLE_BRACKET => return false,
                c if !c.is_whitespace() && c != DOUBLE_QUOTE && c != SINGLE_QUOTE => continue,
                _ => continue,
            }
        }
        found_equals
    }

    #[inline]
    fn find_closing_bracket(&self, mut pos: usize) -> Option<usize> {
        while pos < self.input.len() {
            if self.input.as_bytes()[pos] == CLOSE_BRACKET_BYTE {
                return Some(pos + INITIAL_POS_INCREMENT);
            }
            pos += INITIAL_POS_INCREMENT;
        }
        None
    }

    #[inline]
    fn is_valid_jsx_start(&self, input: &str) -> bool {
        input
            .chars()
            .nth(1)
            .map(|c| c.is_ascii_alphabetic() || c == UNDERSCORE || c == DOLLAR_SIGN)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsx_doctype_handling() {
        let input = r#"<!DOCTYPE html><html lang="en">Test</html>"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<html lang=\"en\">Test</html>".to_string()]);
    }

    #[test]
    fn test_jsx_extractor_extract_simple_jsx() {
        let input = r#"
            function App() {
                return <div>Hello World</div>;
            }
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<div>Hello World</div>".to_string()]);
    }

    #[test]
    fn test_jsx_extractor_copy_button_and_code() {
        let input = r#"
                <script>
                    const copyClipboardButton = document.getElementById("js-copyClipboardButton");
                    const code = copyClipboardButton.querySelector("code");
                    const originalInnerHTML = copyClipboardButton.innerHTML;

                    let timeout;

                    copyClipboardButton.addEventListener("click", () => {
                        clearTimeout(timeout);
                        navigator.clipboard.writeText(code.textContent)
                            .then(() => {
                                copyClipboardButton.innerHTML = "<span class=\"text-sm font-mono mr-2\">Copied!</span>";
                                timeout = setTimeout(() => {
                                    copyClipboardButton.innerHTML = originalInnerHTML;
                                }, 1000);
                            })
                            .catch(err => {
                                console.error("Failed to copy: ", err);
                            });
                    });
                </script>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec!["<script>\n                    const copyClipboardButton = document.getElementById(\"js-copyClipboardButton\");\n                    const code = copyClipboardButton.querySelector(\"code\");\n                    const originalInnerHTML = copyClipboardButton.innerHTML;\n\n                    let timeout;\n\n                    copyClipboardButton.addEventListener(\"click\", () => {\n                        clearTimeout(timeout);\n                        navigator.clipboard.writeText(code.textContent)\n                            .then(() => {\n                                copyClipboardButton.innerHTML = \"<span class=\\\"text-sm font-mono mr-2\\\">Copied!</span>\";\n                                timeout = setTimeout(() => {\n                                    copyClipboardButton.innerHTML = originalInnerHTML;\n                                }, 1000);\n                            })\n                            .catch(err => {\n                                console.error(\"Failed to copy: \", err);\n                            });\n                    });\n                </script>".to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_with_apostrophe() {
        let input = "<p>We don't share it with third parties</p>";
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec!["<p>We don't share it with third parties</p>".to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_with_web_component() {
        let input = r#"
            function App() {
                return <web-component><span>Hello World</span></web-component>;
            }
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec!["<web-component><span>Hello World</span></web-component>".to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_components_with_underscore_and_dollar() {
        let input = r#"
            const element = (
                <div>
                    <_CustomComponent>
                        <span>Inside underscore component</span>
                    </_CustomComponent>
                    <$DollarComponent prop={value}>
                        <p>Inside dollar component</p>
                    </$DollarComponent>
                    <_NestedComponent>
                        <$InnerComponent>
                            <div>Nested special components</div>
                        </$InnerComponent>
                    </_NestedComponent>
                </div>
            );
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div>
                    <_CustomComponent>
                        <span>Inside underscore component</span>
                    </_CustomComponent>
                    <$DollarComponent prop={value}>
                        <p>Inside dollar component</p>
                    </$DollarComponent>
                    <_NestedComponent>
                        <$InnerComponent>
                            <div>Nested special components</div>
                        </$InnerComponent>
                    </_NestedComponent>
                </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_extract_jsx_with_props() {
        let input = r#"
                const element = <Button className="primary" onClick={() => {}}>
                    Click me
                </Button>;
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<Button className="primary" onClick={() => {}}>
                    Click me
                </Button>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_with_spread_operator_and_template_literals() {
        let input =
            r#"<button{...o}className={`w-full ${p[c]}${n?` ${n}`:""}`}type={r}>{e}</button>"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![
                r#"<button{...o}className={`w-full ${p[c]}${n?` ${n}`:""}`}type={r}>{e}</button>"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_extract_self_closing_tag() {
        let input = r#"const img = <img src="test.jpg" alt="Test" />;"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<img src="test.jpg" alt="Test" />"#.to_string()]
        );
    }

    #[test]
    fn test_self_closing_component_without_space() {
        let input = r#"const el = <Component/>;"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<Component/>".to_string()]);
    }

    #[test]
    fn test_self_closing_component_with_slash_in_name() {
        let input = r#"const el = <Comp/onent />;"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, Vec::<String>::new());
    }

    #[test]
    fn test_self_closing_component_with_space() {
        let input = r#"const el = <Component />;"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<Component />".to_string()]);
    }

    #[test]
    fn test_jsx_extractor_extract_fragment() {
        let input = r#"
            let element = <>
                <div>First</div>
                <div>Second</div>
            </>;
            function Fragment() {
                return <>
                    <div>First</div>
                    <div>Second</div>
                </>;
            }
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec!["<>\n                <div>First</div>\n                <div>Second</div>\n            </>".to_string()
                ,
                "<>\n                    <div>First</div>\n                    <div>Second</div>\n                </>".to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_extract_complex_jsx() {
        let input = r#"
            function ComplexComponent() {
                return (
                    <div className={`container ${active ? 'active' : ''}`}>
                        <header>
                            {loading ? (
                                <Spinner />
                            ) : (
                                <h1>{title}</h1>
                            )}
                        </header>
                        <nav>
                            {items.map(item => (
                                <a key={item.id} href={`/item/${item.id}`}>
                                    {item.name}
                                </a>
                            ))}
                        </nav>
                    </div>
                );
            }
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec!["<div className={`container ${active ? 'active' : ''}`}>\n                        <header>\n                            {loading ? (\n                                <Spinner />\n                            ) : (\n                                <h1>{title}</h1>\n                            )}\n                        </header>\n                        <nav>\n                            {items.map(item => (\n                                <a key={item.id} href={`/item/${item.id}`}>\n                                    {item.name}\n                                </a>\n                            ))}\n                        </nav>\n                    </div>".to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_nested_expressions() {
        let input = r#"
            <div prop={{
                nested: {
                    object: true
                }
            }}>
                <span>{(() => {
                    const x = { y: 1 };
                    return x.y;
                })()}</span>
            </div>
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div prop={{
                nested: {
                    object: true
                }
            }}>
                <span>{(() => {
                    const x = { y: 1 };
                    return x.y;
                })()}</span>
            </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_invalid_jsx() {
        let input = r#"
            const x = < 5;
            const y = <invalid;
            const valid = <div>This is valid</div>;
        "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<div>This is valid</div>".to_string()]);
    }

    #[test]
    fn test_jsx_extractor_string_escaping() {
        let input = r#"
                const element = <div title="Quote \"inside\" string" data-value='Single\'s quote'>
                    <span alt={"Mixed \"quotes' and `ticks`"}>Text</span>
                </div>;
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![
                r#"<div title="Quote \"inside\" string" data-value='Single\'s quote'>
                    <span alt={"Mixed \"quotes' and `ticks`"}>Text</span>
                </div>"#
                    .to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_mixed_fragments_and_elements() {
        let input = r#"
                <>
                    <div>First</div>
                    <>
                        <span>Nested</span>
                        <p>Fragment</p>
                    </>
                    <div>Last</div>
                </>
                <div>Outside fragment</div>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![
                r#"<>
                    <div>First</div>
                    <>
                        <span>Nested</span>
                        <p>Fragment</p>
                    </>
                    <div>Last</div>
                </>"#
                    .to_string(),
                r#"<div>Outside fragment</div>"#.to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_nested_fragments_() {
        let input = r#"
                <>
                    <>
                        <>
                            <f>F</f>
                        </>
                    </>
                </>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![
                "<>\n                    <>\n                        <>\n                            <f>F</f>\n                        </>\n                    </>\n                </>".to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_attribute_edge_cases() {
        let input = r#"
                <div>
                    <input disabled />
                    <button className={true ? 'active' : ''} />
                    <div data={`template${expr}`} />
                    <span {...spread} />
                    <custom-element some-attr="" />
                </div>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div>
                    <input disabled />
                    <button className={true ? 'active' : ''} />
                    <div data={`template${expr}`} />
                    <span {...spread} />
                    <custom-element some-attr="" />
                </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_comments_inside_jsx() {
        let input = r#"
                <div>
                    {/* JSX comment */}
                    <span>
                        // This is not a comment
                        /* Also not a comment */
                    </span>
                    {/*
                        Multiline
                        JSX comment
                    */}
                </div>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div>
                    {/* JSX comment */}
                    <span>
                        // This is not a comment
                        /* Also not a comment */
                    </span>
                    {/*
                        Multiline
                        JSX comment
                    */}
                </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_complex_expressions() {
        let input = r#"
                <div>
                    {(() => {
                        const obj = { key: "value" };
                        return <span>{`${obj.key}`}</span>;
                    })()}
                    {items?.map?.(item => (
                        <div key={item?.id ?? "default"}>
                            {item?.name || "Unnamed"}
                        </div>
                    ))}
                </div>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div>
                    {(() => {
                        const obj = { key: "value" };
                        return <span>{`${obj.key}`}</span>;
                    })()}
                    {items?.map?.(item => (
                        <div key={item?.id ?? "default"}>
                            {item?.name || "Unnamed"}
                        </div>
                    ))}
                </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_error_recovery() {
        let input = r#"
                    <div>Valid</div>
                    <Incomplete>
                    <div>Still valid</div>
                    < 123
                    <div>Another valid</div>
                "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![
                r#"<div>Valid</div>"#.to_string(),
                r#"<div>Still valid</div>"#.to_string(),
                r#"<div>Another valid</div>"#.to_string()
            ]
        );
    }

    #[test]
    fn test_jsx_extractor_special_characters() {
        let input = r#"
                <div data-special="©®™">
                    <span>Em—dash</span>
                    <p>{"Unicode: \u{1F604}"}</p>
                </div>
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div data-special="©®™">
                    <span>Em—dash</span>
                    <p>{"Unicode: \u{1F604}"}</p>
                </div>"#
                .to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_complex_layout() {
        let input = r#"const element = (<div className={`container ${theme}`}>
                <header className={styles.header}>
                    <h1>{title || "Default Title"}</h1>
                    <nav>
                        {menuItems.map((item, index) => (
                            <a
                                key={index}
                                href={item.href}
                                className={`${styles.link} ${currentPath === item.href ? styles.active : ''}`}
                            >
                                {item.icon && <Icon name={item.icon} />}
                                <span>{item.label}</span>
                                {item.badge && (
                                    <Badge count={item.badge} type={item.badgeType} />
                                )}
                            </a>
                        ))}
                    </nav>
                    {user ? (
                        <div className={styles.userMenu}>
                            <img src={user.avatar} alt="User avatar" />
                            <span>{user.name}</span>
                            <button onClick={handleLogout}>Logout</button>
                        </div>
                    ) : (
                        <button className={styles.loginButton} onClick={handleLogin}>
                            Login
                        </button>
                    )}
                </header>
                <main className={styles.main}>
                    {loading ? (
                        <div className={styles.loader}>
                            <Spinner size="large" color={theme === 'dark' ? 'white' : 'black'} />
                        </div>
                    ) : error ? (
                        <ErrorMessage message={error} onRetry={handleRetry} />
                    ) : (
                        <>{children}</>
                    )}
                </main>
                <footer className={styles.footer}>
                    <p>&copy; {currentYear} My Application</p>
                </footer>
            </div>)
        ;"#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(
            locations,
            vec![r#"<div className={`container ${theme}`}>
                <header className={styles.header}>
                    <h1>{title || "Default Title"}</h1>
                    <nav>
                        {menuItems.map((item, index) => (
                            <a
                                key={index}
                                href={item.href}
                                className={`${styles.link} ${currentPath === item.href ? styles.active : ''}`}
                            >
                                {item.icon && <Icon name={item.icon} />}
                                <span>{item.label}</span>
                                {item.badge && (
                                    <Badge count={item.badge} type={item.badgeType} />
                                )}
                            </a>
                        ))}
                    </nav>
                    {user ? (
                        <div className={styles.userMenu}>
                            <img src={user.avatar} alt="User avatar" />
                            <span>{user.name}</span>
                            <button onClick={handleLogout}>Logout</button>
                        </div>
                    ) : (
                        <button className={styles.loginButton} onClick={handleLogin}>
                            Login
                        </button>
                    )}
                </header>
                <main className={styles.main}>
                    {loading ? (
                        <div className={styles.loader}>
                            <Spinner size="large" color={theme === 'dark' ? 'white' : 'black'} />
                        </div>
                    ) : error ? (
                        <ErrorMessage message={error} onRetry={handleRetry} />
                    ) : (
                        <>{children}</>
                    )}
                </main>
                <footer className={styles.footer}>
                    <p>&copy; {currentYear} My Application</p>
                </footer>
            </div>"#.to_string()]
        );
    }

    #[test]
    fn test_jsx_extractor_invalid_jsx_with_broken_syntax() {
        let input = r#"
                const code = <n.length;r++)n[r]&&(n[r].__=e,t=Ne(n[r],t,o));return t></n>;
                const valid = <div>Valid element</div>;
            "#;
        let mut extractor = JSXExtractor::new(String::from(input));
        let locations = extractor
            .extract()
            .expect("Failed to extract JSX locations");

        assert_eq!(locations, vec!["<div>Valid element</div>".to_string()]);
    }
}
