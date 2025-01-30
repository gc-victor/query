use std::collections::HashMap;

use crate::{
    jsx_extractor::JSXExtractor,
    jsx_parser::{JSXAttribute, JSXAttributeValue, JSXNode, Parser},
};

#[derive(Debug)]
pub enum JSXErrorKind {
    InvalidAttribute(String),
    InvalidComponent(String),
    InvalidElement(String),
    UnsupportedSyntax(String),
    ExtractionError(String),
    ParsingError(String),
}

#[derive(Debug)]
pub enum JSXError {
    ExtractionError(String),
    ParsingError(String),
    TransformError(String),
}

impl JSXError {
    #[inline]
    pub fn with_kind(kind: JSXErrorKind) -> Self {
        match kind {
            JSXErrorKind::InvalidAttribute(msg) => {
                JSXError::TransformError(format!("Invalid attribute: {}", msg))
            }
            JSXErrorKind::InvalidComponent(msg) => {
                JSXError::TransformError(format!("Invalid component: {}", msg))
            }
            JSXErrorKind::InvalidElement(msg) => {
                JSXError::TransformError(format!("Invalid element: {}", msg))
            }
            JSXErrorKind::UnsupportedSyntax(msg) => {
                JSXError::TransformError(format!("Unsupported syntax: {}", msg))
            }
            JSXErrorKind::ExtractionError(msg) => JSXError::ExtractionError(msg),
            JSXErrorKind::ParsingError(msg) => JSXError::ParsingError(msg),
        }
    }
}

impl std::fmt::Display for JSXError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSXError::ExtractionError(msg) => write!(f, "JSX extraction error: {}", msg),
            JSXError::ParsingError(msg) => write!(f, "JSX parsing error: {}", msg),
            JSXError::TransformError(msg) => write!(f, "JSX transform error: {}", msg),
        }
    }
}

const OPENING_BRACKET: &str = "<";
const COMMA: &str = ",";
const UNDERSCORE: char = '_';
const DOLLAR_SIGN: char = '$';
const EMPTY_STRING: &str = "";

static SELF_CLOSING_TAGS: [&str; 23] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "circle", "ellipse", "image", "line", "path", "polygon", "polyline", "rect",
    "use",
];

#[inline]
fn is_component(tag: &str) -> bool {
    tag.chars()
        .next()
        .map(|c| c.is_uppercase() || c == UNDERSCORE || c == DOLLAR_SIGN)
        .unwrap_or(false)
}

#[inline]
fn is_self_closing(tag: &str) -> bool {
    SELF_CLOSING_TAGS.contains(&tag.to_lowercase().as_str())
}

pub fn jsx_precompile(source: &str) -> Result<String, JSXError> {
    let (mut result, store, key_counter) = extract_string_html(source)?;

    let locations = extract_jsx_locations(&result)?;
    let templates = transform_jsx_elements(locations)?;
    result = replace_jsx_placeholders(result, templates);
    result = restore_string_placeholders(result, store, key_counter);

    // Remove empty interpolation artifacts
    Ok(result.replace("${}", EMPTY_STRING))
}

fn extract_string_html(source: &str) -> Result<(String, HashMap<String, String>, usize), JSXError> {
    let mut store: HashMap<String, String> = HashMap::new();
    let mut key_counter = 0;
    let mut result = source.to_string();

    let regex = regex::Regex::new(r#"StringHTML\((.*?)\)"#)
        .map_err(|e| JSXError::ExtractionError(e.to_string()))?;

    for caps in regex.captures_iter(source) {
        let str_key = format!("__str_{}", key_counter);
        let value = caps
            .get(1)
            .ok_or_else(|| JSXError::ExtractionError("Failed to capture value".to_string()))?
            .as_str()
            .to_string();

        store.insert(str_key.clone(), value.clone());
        result = result.replace(&format!("StringHTML({})", value), &str_key);
        key_counter += 1;
    }

    Ok((result, store, key_counter))
}

fn extract_jsx_locations(source: &str) -> Result<Vec<String>, JSXError> {
    let mut extractor = JSXExtractor::new(source.to_owned());
    extractor
        .extract()
        .map_err(|e| JSXError::with_kind(JSXErrorKind::ExtractionError(e.to_string())))
}

fn transform_jsx_elements(locations: Vec<String>) -> Result<HashMap<String, String>, JSXError> {
    let mut templates: HashMap<String, String> = HashMap::new();

    for location in locations {
        let content = location.trim().to_string();
        let mut parser = Parser::new(&content);

        match parser.parse() {
            Ok(ast) => {
                let template = transform_to_template(&ast)?;
                templates.insert(content, template);
            }
            Err(e) => {
                return Err(JSXError::with_kind(JSXErrorKind::ParsingError(format!(
                    "{}. {}",
                    e, content
                ))));
            }
        }
    }

    Ok(templates)
}

#[inline]
fn replace_jsx_placeholders(mut result: String, templates: HashMap<String, String>) -> String {
    for (key, template) in templates {
        let template = template.split_whitespace().collect::<Vec<&str>>().join(" ");
        let transformed = if template.contains(".map(")
            || template.contains(".filter(")
            || template.contains(".reduce(")
        {
            format!("`${{__jsxTemplate(`{}`)}}`", template)
        } else {
            format!("`{}`", template)
        };

        result = result
            .replace(&format!("`{key}`"), &transformed)
            .replace(&key, &transformed);
    }
    result
}

#[inline]
fn restore_string_placeholders(
    mut result: String,
    store: HashMap<String, String>,
    key_counter: usize,
) -> String {
    for i in 0..key_counter {
        let str_key = format!("__str_{}", i);
        if let Some(value) = store.get(&str_key) {
            result = result.replace(&str_key, value);
        }
    }
    result
}

fn transform_to_template(ast: &JSXNode) -> Result<String, JSXError> {
    match ast {
        JSXNode::Element {
            tag,
            attributes,
            children,
        } => {
            if is_component(tag) {
                transform_component(tag, attributes, children)
            } else {
                transform_element(tag, attributes, children)
            }
        }
        JSXNode::Expression(expr) => Ok(format!(r#"${{{}}}"#, expr)),
        JSXNode::Fragment { children } => transform_fragment(children),
        JSXNode::Text(text) => Ok(text.to_string()),
    }
}

fn transform_component(
    tag: &str,
    attributes: &[JSXAttribute],
    children: &[JSXNode],
) -> Result<String, JSXError> {
    let attr_parts = transform_component_attributes(attributes)?;
    let children_parts = transform_component_children(children)?;

    if children_parts.is_empty() {
        Ok(format!(r#"${{__jsxComponent({}, {})}}"#, tag, attr_parts))
    } else {
        let children_str = children_parts.join("");
        Ok(format!(
            r#"${{__jsxComponent({}, {}, `{}`)}}"#,
            tag,
            attr_parts,
            children_str.trim()
        ))
    }
}

fn transform_component_attributes(attributes: &[JSXAttribute]) -> Result<String, JSXError> {
    let mut attr_parts = Vec::new();
    for attr in attributes.iter() {
        match &attr.value {
            Some(JSXAttributeValue::Expression(expr)) => {
                attr_parts.push(format!(r#"{{"{}":{}}}"#, &attr.name, expr));
            }
            Some(JSXAttributeValue::DoubleQuote(value)) => {
                attr_parts.push(format!(r#"{{"{}":"{}"}}"#, &attr.name, value));
            }
            Some(JSXAttributeValue::SingleQuote(value)) => {
                attr_parts.push(format!(r#"{{"{}":'{}'}}"#, &attr.name, value));
            }
            None => {
                if attr.name.starts_with("...") {
                    attr_parts.push(format!("{{{}}}", attr.name));
                } else {
                    attr_parts.push(format!(r#"{{"{}":true}}"#, &attr.name));
                }
            }
        }
    }
    Ok(format!("[{}]", attr_parts.join(COMMA)))
}

#[inline]
fn transform_component_children(children: &[JSXNode]) -> Result<Vec<String>, JSXError> {
    let mut children_parts = Vec::new();
    for child in children {
        children_parts.push(transform_to_template(child)?);
    }
    Ok(children_parts)
}

fn transform_element(
    tag: &str,
    attributes: &[JSXAttribute],
    children: &[JSXNode],
) -> Result<String, JSXError> {
    let attrs = transform_element_attributes(attributes)?;

    let attrs_str = if !attrs.is_empty() {
        attrs
            .iter()
            .map(|attr| {
                if attr.starts_with("${__jsxSpread") {
                    attr.to_string()
                } else {
                    format!(" {}", attr)
                }
            })
            .collect::<String>()
    } else {
        String::new()
    };

    if is_self_closing(tag) {
        return Ok(format!("<{}{}/>", tag, attrs_str));
    }

    let children_str = transform_jsx_children(children)?;

    Ok(format!("<{}{}>{}</{}>", tag, attrs_str, children_str, tag))
}

fn transform_element_attributes(attributes: &[JSXAttribute]) -> Result<Vec<String>, JSXError> {
    let mut attr_parts = Vec::new();
    for attr in attributes {
        let name = normalize_html_attr_name(&attr.name);

        match &attr.value {
            Some(JSXAttributeValue::Expression(expr)) => {
                attr_parts.push(format!(r#"{}="${{{}}}""#, name, expr));
            }
            Some(JSXAttributeValue::DoubleQuote(value)) => {
                attr_parts.push(format!(r#"{}="{}""#, name, value));
            }
            Some(JSXAttributeValue::SingleQuote(value)) => {
                attr_parts.push(format!("{}='{}'", name, value));
            }
            None => {
                if attr.name.starts_with("...") {
                    attr_parts.push(format!(
                        "${{__jsxSpread({})}}",
                        attr.name.replace("...", "")
                    ));
                } else {
                    attr_parts.push(attr.name.to_string());
                }
            }
        }
    }
    Ok(attr_parts)
}

#[inline]
fn transform_fragment(children: &[JSXNode]) -> Result<String, JSXError> {
    transform_jsx_children(children)
}

fn transform_jsx_children(children: &[JSXNode]) -> Result<String, JSXError> {
    let mut children_parts = Vec::new();

    for child in children {
        match child {
            JSXNode::Text(text) => {
                children_parts.push(text.to_string());
            }
            JSXNode::Expression(expr) => {
                if expr.contains(OPENING_BRACKET) {
                    let nested = jsx_precompile(expr)?;
                    children_parts.push(format!("${{{}}}", nested.trim()));
                } else {
                    children_parts.push(format!("${{{}}}", expr));
                }
            }
            _ => {
                let child_content = transform_to_template(child)?;
                children_parts.push(child_content);
            }
        }
    }

    Ok(children_parts.join("").trim().to_string())
}

// @see: https://github.com/denoland/deno_ast/blob/3aba071b59d71802398c2fbcd2d01c99a51553cf/src/transpiling/jsx_precompile.rs#L89
#[inline]
fn normalize_html_attr_name(name: &str) -> String {
    match name {
        // JSX specific
        "htmlFor" => "for".to_string(),
        "className" => "class".to_string(),
        "dangerouslySetInnerHTML" => name.to_string(),

        "panose1" => "panose-1".to_string(),
        "xlinkActuate" => "xlink:actuate".to_string(),
        "xlinkArcrole" => "xlink:arcrole".to_string(),

        // xlink:href was removed from SVG and isn't needed
        "xlinkHref" => "href".to_string(),
        "xlink:href" => "href".to_string(),

        "xlinkRole" => "xlink:role".to_string(),
        "xlinkShow" => "xlink:show".to_string(),
        "xlinkTitle" => "xlink:title".to_string(),
        "xlinkType" => "xlink:type".to_string(),
        "xmlBase" => "xml:base".to_string(),
        "xmlLang" => "xml:lang".to_string(),
        "xmlSpace" => "xml:space".to_string(),

        // Attributes that are kebab-cased
        "accentHeight"
        | "acceptCharset"
        | "alignmentBaseline"
        | "arabicForm"
        | "baselineShift"
        | "capHeight"
        | "clipPath"
        | "clipRule"
        | "colorInterpolation"
        | "colorInterpolationFilters"
        | "colorProfile"
        | "colorRendering"
        | "contentScriptType"
        | "contentStyleType"
        | "dominantBaseline"
        | "enableBackground"
        | "fillOpacity"
        | "fillRule"
        | "floodColor"
        | "floodOpacity"
        | "fontFamily"
        | "fontSize"
        | "fontSizeAdjust"
        | "fontStretch"
        | "fontStyle"
        | "fontVariant"
        | "fontWeight"
        | "glyphName"
        | "glyphOrientationHorizontal"
        | "glyphOrientationVertical"
        | "horizAdvX"
        | "horizOriginX"
        | "horizOriginY"
        | "httpEquiv"
        | "imageRendering"
        | "letterSpacing"
        | "lightingColor"
        | "markerEnd"
        | "markerMid"
        | "markerStart"
        | "overlinePosition"
        | "overlineThickness"
        | "paintOrder"
        | "pointerEvents"
        | "renderingIntent"
        | "shapeRendering"
        | "stopColor"
        | "stopOpacity"
        | "strikethroughPosition"
        | "strikethroughThickness"
        | "strokeDasharray"
        | "strokeDashoffset"
        | "strokeLinecap"
        | "strokeLinejoin"
        | "strokeMiterlimit"
        | "strokeOpacity"
        | "strokeWidth"
        | "textAnchor"
        | "textDecoration"
        | "textRendering"
        | "transformOrigin"
        | "underlinePosition"
        | "underlineThickness"
        | "unicodeBidi"
        | "unicodeRange"
        | "unitsPerEm"
        | "vAlphabetic"
        | "vectorEffect"
        | "vertAdvY"
        | "vertOriginX"
        | "vertOriginY"
        | "vHanging"
        | "vMathematical"
        | "wordSpacing"
        | "writingMode"
        | "xHeight" => name
            .chars()
            .map(|ch| match ch {
                'A'..='Z' => format!("-{}", ch.to_lowercase()),
                _ => ch.to_string(),
            })
            .collect(),

        // Attributes that are camelCased and should be kept as is.
        "allowReorder"
        | "attributeName"
        | "attributeType"
        | "baseFrequency"
        | "baseProfile"
        | "calcMode"
        | "clipPathUnits"
        | "diffuseConstant"
        | "edgeMode"
        | "filterUnits"
        | "glyphRef"
        | "gradientTransform"
        | "gradientUnits"
        | "kernelMatrix"
        | "kernelUnitLength"
        | "keyPoints"
        | "keySplines"
        | "keyTimes"
        | "lengthAdjust"
        | "limitingConeAngle"
        | "markerHeight"
        | "markerUnits"
        | "markerWidth"
        | "maskContentUnits"
        | "maskUnits"
        | "numOctaves"
        | "pathLength"
        | "patternContentUnits"
        | "patternTransform"
        | "patternUnits"
        | "pointsAtX"
        | "pointsAtY"
        | "pointsAtZ"
        | "preserveAlpha"
        | "preserveAspectRatio"
        | "primitiveUnits"
        | "referrerPolicy"
        | "refX"
        | "refY"
        | "repeatCount"
        | "repeatDur"
        | "requiredExtensions"
        | "requiredFeatures"
        | "specularConstant"
        | "specularExponent"
        | "spreadMethod"
        | "startOffset"
        | "stdDeviation"
        | "stitchTiles"
        | "surfaceScale"
        | "systemLanguage"
        | "tableValues"
        | "targetX"
        | "targetY"
        | "textLength"
        | "viewBox"
        | "xChannelSelector"
        | "yChannelSelector"
        | "zoomAndPan" => name.to_string(),

        _ => {
            // Devs expect attributes in the HTML document to be lowercased.
            name.to_lowercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_jsx_element() {
        let source = "const el = <div>Hello</div>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(result, "const el = `<div>Hello</div>`;");
    }

    #[test]
    fn test_p_tag_with_strong_child() {
        let source = "const el = <p>Normal text <strong>Bold text</strong> some text</p>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<p>Normal text <strong>Bold text</strong> some text</p>`;"
        );
    }

    #[test]
    fn test_jsx_with_attributes() {
        let source = "const el = <div className=\"container\" id=\"main\">Content</div>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<div class=\"container\" id=\"main\">Content</div>`;"
        );
    }

    #[test]
    fn test_boolean_attribute_component() {
        let source = r#"const el = <CustomComponent disabled />;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxComponent(CustomComponent, [{\"disabled\":true}])}`;"
        );
    }

    #[test]
    fn test_boolean_attribute_element() {
        let source = r#"const el = <input type="checkbox" disabled />;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(result, r#"const el = `<input type="checkbox" disabled/>`;"#);
    }

    #[test]
    fn test_normalize_html_attr_name() {
        let values = HashMap::from([
            ("accentHeight", "accent-height"),
            ("acceptCharset", "accept-charset"),
            ("alignmentBaseline", "alignment-baseline"),
            ("allowReorder", "allowReorder"),
            ("arabicForm", "arabic-form"),
            ("attributeName", "attributeName"),
            ("attributeType", "attributeType"),
            ("baseFrequency", "baseFrequency"),
            ("baselineShift", "baseline-shift"),
            ("baseProfile", "baseProfile"),
            ("calcMode", "calcMode"),
            ("capHeight", "cap-height"),
            ("className", "class"),
            ("clipPath", "clip-path"),
            ("clipPathUnits", "clipPathUnits"),
            ("clipRule", "clip-rule"),
            ("colorInterpolation", "color-interpolation"),
            ("colorInterpolationFilters", "color-interpolation-filters"),
            ("colorProfile", "color-profile"),
            ("colorRendering", "color-rendering"),
            ("contentScriptType", "content-script-type"),
            ("contentStyleType", "content-style-type"),
            ("diffuseConstant", "diffuseConstant"),
            ("dominantBaseline", "dominant-baseline"),
            ("edgeMode", "edgeMode"),
            ("enableBackground", "enable-background"),
            ("fillOpacity", "fill-opacity"),
            ("fillRule", "fill-rule"),
            ("filterUnits", "filterUnits"),
            ("floodColor", "flood-color"),
            ("floodOpacity", "flood-opacity"),
            ("fontFamily", "font-family"),
            ("fontSize", "font-size"),
            ("fontSizeAdjust", "font-size-adjust"),
            ("fontStretch", "font-stretch"),
            ("fontStyle", "font-style"),
            ("fontVariant", "font-variant"),
            ("fontWeight", "font-weight"),
            ("glyphName", "glyph-name"),
            ("glyphOrientationHorizontal", "glyph-orientation-horizontal"),
            ("glyphOrientationVertical", "glyph-orientation-vertical"),
            ("glyphRef", "glyphRef"),
            ("gradientTransform", "gradientTransform"),
            ("gradientUnits", "gradientUnits"),
            ("horizAdvX", "horiz-adv-x"),
            ("horizOriginX", "horiz-origin-x"),
            ("horizOriginY", "horiz-origin-y"),
            ("htmlFor", "for"),
            ("httpEquiv", "http-equiv"),
            ("imageRendering", "image-rendering"),
            ("kernelMatrix", "kernelMatrix"),
            ("kernelUnitLength", "kernelUnitLength"),
            ("keyPoints", "keyPoints"),
            ("keySplines", "keySplines"),
            ("keyTimes", "keyTimes"),
            ("lengthAdjust", "lengthAdjust"),
            ("letterSpacing", "letter-spacing"),
            ("lightingColor", "lighting-color"),
            ("limitingConeAngle", "limitingConeAngle"),
            ("markerEnd", "marker-end"),
            ("markerHeight", "markerHeight"),
            ("markerMid", "marker-mid"),
            ("markerStart", "marker-start"),
            ("markerUnits", "markerUnits"),
            ("markerWidth", "markerWidth"),
            ("maskContentUnits", "maskContentUnits"),
            ("maskUnits", "maskUnits"),
            ("numOctaves", "numOctaves"),
            ("overlinePosition", "overline-position"),
            ("overlineThickness", "overline-thickness"),
            ("paintOrder", "paint-order"),
            ("panose1", "panose-1"),
            ("pathLength", "pathLength"),
            ("patternContentUnits", "patternContentUnits"),
            ("patternTransform", "patternTransform"),
            ("patternUnits", "patternUnits"),
            ("pointsAtX", "pointsAtX"),
            ("pointsAtY", "pointsAtY"),
            ("pointsAtZ", "pointsAtZ"),
            ("pointerEvents", "pointer-events"),
            ("preserveAlpha", "preserveAlpha"),
            ("preserveAspectRatio", "preserveAspectRatio"),
            ("primitiveUnits", "primitiveUnits"),
            ("referrerPolicy", "referrerPolicy"),
            ("refX", "refX"),
            ("refY", "refY"),
            ("renderingIntent", "rendering-intent"),
            ("repeatCount", "repeatCount"),
            ("repeatDur", "repeatDur"),
            ("requiredExtensions", "requiredExtensions"),
            ("requiredFeatures", "requiredFeatures"),
            ("shapeRendering", "shape-rendering"),
            ("specularConstant", "specularConstant"),
            ("specularExponent", "specularExponent"),
            ("spreadMethod", "spreadMethod"),
            ("startOffset", "startOffset"),
            ("stdDeviation", "stdDeviation"),
            ("stitchTiles", "stitchTiles"),
            ("stopColor", "stop-color"),
            ("stopOpacity", "stop-opacity"),
            ("strikethroughPosition", "strikethrough-position"),
            ("strikethroughThickness", "strikethrough-thickness"),
            ("strokeDasharray", "stroke-dasharray"),
            ("strokeDashoffset", "stroke-dashoffset"),
            ("strokeLinecap", "stroke-linecap"),
            ("strokeLinejoin", "stroke-linejoin"),
            ("strokeMiterlimit", "stroke-miterlimit"),
            ("strokeOpacity", "stroke-opacity"),
            ("strokeWidth", "stroke-width"),
            ("surfaceScale", "surfaceScale"),
            ("systemLanguage", "systemLanguage"),
            ("tableValues", "tableValues"),
            ("targetX", "targetX"),
            ("targetY", "targetY"),
            ("textAnchor", "text-anchor"),
            ("textDecoration", "text-decoration"),
            ("textLength", "textLength"),
            ("textRendering", "text-rendering"),
            ("transformOrigin", "transform-origin"),
            ("underlinePosition", "underline-position"),
            ("underlineThickness", "underline-thickness"),
            ("unicodeBidi", "unicode-bidi"),
            ("unicodeRange", "unicode-range"),
            ("unitsPerEm", "units-per-em"),
            ("vAlphabetic", "v-alphabetic"),
            ("viewBox", "viewBox"),
            ("vectorEffect", "vector-effect"),
            ("vertAdvY", "vert-adv-y"),
            ("vertOriginX", "vert-origin-x"),
            ("vertOriginY", "vert-origin-y"),
            ("vHanging", "v-hanging"),
            ("vMathematical", "v-mathematical"),
            ("wordSpacing", "word-spacing"),
            ("writingMode", "writing-mode"),
            ("xChannelSelector", "xChannelSelector"),
            ("xHeight", "x-height"),
            ("xlinkActuate", "xlink:actuate"),
            ("xlinkArcrole", "xlink:arcrole"),
            ("xlinkHref", "href"),
            ("xlink:href", "href"),
            ("xlinkRole", "xlink:role"),
            ("xlinkShow", "xlink:show"),
            ("xlinkTitle", "xlink:title"),
            ("xlinkType", "xlink:type"),
            ("xmlBase", "xml:base"),
            ("xmlLang", "xml:lang"),
            ("xmlSpace", "xml:space"),
            ("yChannelSelector", "yChannelSelector"),
            ("zoomAndPan", "zoomAndPan"),
        ]);

        for (input, expected) in values {
            assert_eq!(normalize_html_attr_name(input), expected);
        }
    }

    #[test]
    fn test_jsx_with_dynamic_attributes() {
        let source = "const el = <div className={dynamicClass} {...spread} moto>Content</div>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<div class=\"${dynamicClass}\"${__jsxSpread(spread)} moto>Content</div>`;"
        );
    }

    #[test]
    fn test_attribute_value_single_quotes() {
        let source = r#"const el = <div class='single-quote'></div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(result, "const el = `<div class='single-quote'></div>`;");
    }

    #[test]
    fn test_label_html_for() {
        let source = r#"const el = <label htmlFor={data}>My Label<input id="myInput" /></label>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<label for=\"${data}\">My Label<input id=\"myInput\"/></label>`;"
        );
    }

    #[test]
    fn test_self_close_element() {
        let source =
            r#"const el = <label htmlFor="myInput">My Label<input id="myInput" /></label>;"#;
        let result = jsx_precompile(source).unwrap();
        let expected =
            "const el = `<label for=\"myInput\">My Label<input id=\"myInput\"/></label>`;";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_web_components() {
        let source = r#"const el = <my-web-component attr="val" data-custom="5"><slot>Default</slot><h1 slot="header">Title</h1></my-web-component>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<my-web-component attr=\"val\" data-custom=\"5\"><slot>Default</slot><h1 slot=\"header\">Title</h1></my-web-component>`;"
        );
    }

    #[test]
    fn test_self_close_component() {
        let source = "const el = <Component/>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(result, "const el = `${__jsxComponent(Component, [])}`;");

        let source_with_space = "const el = <Component />;";
        let result_with_space = jsx_precompile(source_with_space).unwrap();
        assert_eq!(
            result_with_space,
            "const el = `${__jsxComponent(Component, [])}`;"
        );
    }

    #[test]
    fn test_underscore_components() {
        let source = "const el = <_CustomComponent prop=\"value\">child</_CustomComponent>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxComponent(_CustomComponent, [{\"prop\":\"value\"}], `child`)}`;"
        );
    }

    #[test]
    fn test_dollar_sign_components() {
        let source = "const el = <$Component {...props}>content</$Component>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxComponent($Component, [{...props}], `content`)}`;"
        );
    }

    #[test]
    fn test_nested_special_components() {
        let source = "const el = <_Parent><$Child>nested</$Child></_Parent>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxComponent(_Parent, [], `${__jsxComponent($Child, [], `nested`)}`)}`;"
        );
    }

    #[test]
    fn test_component_with_multiple_attributes() {
        let source = r#"const el = <Component className="test-class" htmlFor="input-id" onClick={handleClick}>Content</Component>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxComponent(Component, [{\"className\":\"test-class\"},{\"htmlFor\":\"input-id\"},{\"onClick\":handleClick}], `Content`)}`;"
        );
    }

    #[test]
    fn test_html_doctype() {
        let source =
            r#"const el = `<!DOCTYPE html>${StringHTML(`<html lang="en">${html}</html>`)}`;"#;
        let result = jsx_precompile(source).unwrap();
        let expected = r#"const el = `<!DOCTYPE html>${`<html lang="en">${html}</html>`}`;"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_components_and_element() {
        let source = r#"const el = <ParentComponent attr="val"  moto><ChildComponent {...spread}><div>Inner content</div></ChildComponent></ParentComponent>;"#;
        let result = jsx_precompile(source).unwrap();
        let expected = "const el = `${__jsxComponent(ParentComponent, [{\"attr\":\"val\"},{\"moto\":true}], `${__jsxComponent(ChildComponent, [{...spread}], `<div>Inner content</div>`)}`)}`;";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fragment_with_nested_components() {
        let source = r#"const el = <><A>Head</A><A class="n"><B><div>Inner</div></B></A></>;"#;
        let result = jsx_precompile(source).unwrap();
        let expected = r#"const el = `${__jsxComponent(A, [], `Head`)}${__jsxComponent(A, [{"class":"n"}], `${__jsxComponent(B, [], `<div>Inner</div>`)}`)}`;"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fragment_with_child_text() {
        let source = r#"const el = <><div>First Element</div><span>Second Element</span></>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<div>First Element</div><span>Second Element</span>`;"
        );
    }

    #[test]
    fn test_fragment_with_child_expresion() {
        let source = r#"const el = <>
            <label>After Image</label>

            <input
                type="text"
            /><span>After Input</span>

            {description ? (
                <span>{description}</span>
            ) : (
                ""
            )}
        </>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            r#"const el = `<label>After Image</label> <input type="text"/><span>After Input</span> ${description ? ( `<span>${description}</span>` ) : ( "" )}`;"#
        );
    }

    #[test]
    fn test_seo_component() {
        let source = r#"const el = <A><title>Query - An All-In-One Solution For Your Side-Projects And Startups</title><link rel="stylesheet" href={r}/></A>;"#;
        let result = jsx_precompile(source).unwrap();
        let expected = "const el = `${__jsxComponent(A, [], `<title>Query - An All-In-One Solution For Your Side-Projects And Startups</title><link rel=\"stylesheet\" href=\"${r}\"/>`)}`;";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_elements() {
        let source = r#"const el = <div><span class={classes}><span className={classes1}>Nested 1</span><span className={classes2}>Nested 2</span></span></div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `<div><span class=\"${classes}\"><span class=\"${classes1}\">Nested 1</span><span class=\"${classes2}\">Nested 2</span></span></div>`;"
        );
    }

    #[test]
    fn test_dynamic_content() {
        let source = "const el = <div>{dynamicContent}</div>;";
        let result = jsx_precompile(source).unwrap();
        assert_eq!(result, "const el = `<div>${dynamicContent}</div>`;");
    }

    #[test]
    fn test_array_transformations() {
        let source = r#"const el = <div>{items.map(item => <li>{item}</li>)}</div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxTemplate(`<div>${items.map(item => `<li>${item}</li>`)}</div>`)}`;"
        );
    }

    #[test]
    fn test_loop_component_spread() {
        let source = r#"const el = <div>{posts.map((post) => <Component {...post} />)}</div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxTemplate(`<div>${posts.map((post) => `${__jsxComponent(Component, [{...post}])}`)}</div>`)}`;"
        );
    }

    #[test]
    fn test_filter_transformation() {
        let source = r#"const el = <div>{items.filter(item => item.active).map(item => <li>{item.name}</li>)}</div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxTemplate(`<div>${items.filter(item => item.active).map(item => `<li>${item.name}</li>`)}</div>`)}`;"
        );
    }

    #[test]
    fn test_reduce_transformation() {
        let source = r#"const el = <div>{items.reduce((acc, item) => acc + item, 0)}</div>;"#;
        let result = jsx_precompile(source).unwrap();
        assert_eq!(
            result,
            "const el = `${__jsxTemplate(`<div>${items.reduce((acc, item) => acc + item, 0)}</div>`)}`;"
        );
    }

    #[test]
    fn test_complex_jsx() {
        let source = r#"const TodoList = ({items, onToggle}) => (
<div className={`todo-list ${items.length ? 'has-items' : ''}`}>
<header className="todo-header">
<h1>{items.length} Tasks Remaining</h1>
<input type="text" {...inputProps} placeholder="Add new task" />
</header>
<ul className="todo-items">
{items.map((item, index) => (
<li key={item.id} className={item.completed ? 'completed' : ''}>
<input
type="checkbox"
checked={item.completed}
onChange={() => onToggle(index)}
/>
<span className="todo-text">{item.text}</span>
<button onClick={() => onDelete(item.id)}>Delete</button>
</li>
))}
</ul>
</div>)"#
            .trim();

        let result = jsx_precompile(source).unwrap();
        let expected = "const TodoList = ({items, onToggle}) => (\n`${__jsxTemplate(`<div class=\"${`todo-list ${items.length ? 'has-items' : ''}`}\"><header class=\"todo-header\"><h1>${items.length} Tasks Remaining</h1> <input type=\"text\"${__jsxSpread(inputProps)} placeholder=\"Add new task\"/></header> <ul class=\"todo-items\">${items.map((item, index) => ( `<li key=\"${item.id}\" class=\"${item.completed ? 'completed' : ''}\"><input type=\"checkbox\" checked=\"${item.completed}\" onchange=\"${() => onToggle(index)}\"/> <span class=\"todo-text\">${item.text}</span> <button onclick=\"${() => onDelete(item.id)}\">Delete</button></li>` ))}</ul></div>`)}`)";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_jsx_with_conditions() {
        let input = r#"const el = <div className={`container ${theme}`}>
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
            </div>
        ;"#;
        let result = jsx_precompile(input).unwrap();
        let expected = "const el = `${__jsxTemplate(`<div class=\"${`container ${theme}`}\"><header class=\"${styles.header}\"><h1>${title || \"Default Title\"}</h1> <nav>${menuItems.map((item, index) => ( `<a key=\"${index}\" href=\"${item.href}\" class=\"${`${styles.link} ${currentPath === item.href ? styles.active : ''}`}\">${item.icon && `${__jsxComponent(Icon, [{\"name\":item.icon}])}`} <span>${item.label}</span> ${item.badge && ( `${__jsxComponent(Badge, [{\"count\":item.badge},{\"type\":item.badgeType}])}` )}</a>` ))}</nav> ${user ? ( `<div class=\"${styles.userMenu}\"><img src=\"${user.avatar}\" alt=\"User avatar\"/> <span>${user.name}</span> <button onclick=\"${handleLogout}\">Logout</button></div>` ) : ( `<button class=\"${styles.loginButton}\" onclick=\"${handleLogin}\">Login</button>` )}</header> <main class=\"${styles.main}\">${loading ? ( `<div class=\"${styles.loader}\">${__jsxComponent(Spinner, [{\"size\":\"large\"},{\"color\":theme === 'dark' ? 'white' : 'black'}])}</div>` ) : error ? ( `${__jsxComponent(ErrorMessage, [{\"message\":error},{\"onRetry\":handleRetry}])}` ) : ( `${children}` )}</main> <footer class=\"${styles.footer}\"><p>&copy; ${currentYear} My Application</p></footer></div>`)}`\n        ;";
        assert_eq!(result, expected);
    }
}
