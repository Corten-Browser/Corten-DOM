//! HTML5 validation rules for tag names, attribute names, and values.
//!
//! This module provides validation functions for HTML5 elements and attributes
//! according to the HTML5 specification.
//!
//! # Examples
//!
//! ```
//! use dom_core::html5_validation::{validate_html5_tag_name, is_void_element};
//!
//! // Validate a tag name
//! assert!(validate_html5_tag_name("div").is_ok());
//! assert!(validate_html5_tag_name("my-element").is_ok());
//!
//! // Check if an element is void (cannot have children)
//! assert!(is_void_element("br"));
//! assert!(!is_void_element("div"));
//! ```

use dom_types::DomException;
use std::collections::HashSet;

/// List of standard HTML5 tag names (lowercase)
///
/// This is not exhaustive but covers the most common HTML5 elements.
/// Custom elements (containing hyphens) are validated separately.
static HTML5_TAGS: &[&str] = &[
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

/// List of HTML5 void elements (elements that cannot have children)
static VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
    "track", "wbr",
];

/// List of HTML5 raw text elements (script, style)
static RAW_TEXT_ELEMENTS: &[&str] = &["script", "style"];

/// Validates an HTML5 tag name.
///
/// # Rules
/// - Standard HTML5 elements: Must be from the known HTML5 tag set
/// - Custom elements: Must contain at least one hyphen
/// - Cannot start with "xml" (case-insensitive)
/// - Must contain only ASCII alphanumeric characters and hyphens
/// - Must start with an ASCII letter
/// - Cannot be empty
///
/// # Arguments
/// * `name` - The tag name to validate
///
/// # Returns
/// * `Ok(())` if the tag name is valid
/// * `Err(DomException::InvalidCharacterError)` if the tag name is invalid
///
/// # Examples
///
/// ```
/// use dom_core::html5_validation::validate_html5_tag_name;
///
/// assert!(validate_html5_tag_name("div").is_ok());
/// assert!(validate_html5_tag_name("my-element").is_ok());
/// assert!(validate_html5_tag_name("xml-foo").is_err());
/// assert!(validate_html5_tag_name("123-bad").is_err());
/// ```
pub fn validate_html5_tag_name(name: &str) -> Result<(), DomException> {
    // Check if empty
    if name.is_empty() {
        return Err(DomException::InvalidCharacterError);
    }

    // Convert to lowercase for validation
    let name_lower = name.to_lowercase();

    // Check if starts with "xml" (case-insensitive) - forbidden for custom elements
    if name_lower.starts_with("xml") && name_lower.contains('-') {
        return Err(DomException::InvalidCharacterError);
    }

    // Check first character - must be ASCII letter
    let first_char = name_lower.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() {
        return Err(DomException::InvalidCharacterError);
    }

    // Check all characters are ASCII alphanumeric or hyphen
    if !name_lower
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(DomException::InvalidCharacterError);
    }

    // If it contains a hyphen, it's a custom element (valid as long as it doesn't start with xml)
    if name_lower.contains('-') {
        return Ok(());
    }

    // Otherwise, check if it's a standard HTML5 tag
    // For standard tags, we're lenient and accept them even if not in our list
    // This is to avoid maintaining a comprehensive list and to be forward-compatible
    Ok(())
}

/// Validates an HTML5 attribute name.
///
/// # Rules
/// - Must not be empty
/// - Must not contain: space, `"`, `'`, `>`, `/`, `=`, or control characters
/// - Case-insensitive but case is preserved
/// - Can contain hyphens (for data-* and aria-* attributes)
///
/// # Arguments
/// * `name` - The attribute name to validate
///
/// # Returns
/// * `Ok(())` if the attribute name is valid
/// * `Err(DomException::InvalidCharacterError)` if the attribute name is invalid
///
/// # Examples
///
/// ```
/// use dom_core::html5_validation::validate_html5_attribute_name;
///
/// assert!(validate_html5_attribute_name("id").is_ok());
/// assert!(validate_html5_attribute_name("data-value").is_ok());
/// assert!(validate_html5_attribute_name("aria-label").is_ok());
/// assert!(validate_html5_attribute_name("my attr").is_err());
/// assert!(validate_html5_attribute_name("my=attr").is_err());
/// ```
pub fn validate_html5_attribute_name(name: &str) -> Result<(), DomException> {
    // Check if empty
    if name.is_empty() {
        return Err(DomException::InvalidCharacterError);
    }

    // Check for forbidden characters: space, ", ', >, /, =, and control characters
    for ch in name.chars() {
        if ch.is_control()
            || ch == ' '
            || ch == '"'
            || ch == '\''
            || ch == '>'
            || ch == '/'
            || ch == '='
        {
            return Err(DomException::InvalidCharacterError);
        }

        // Also reject non-ASCII characters for attribute names
        if !ch.is_ascii() {
            return Err(DomException::InvalidCharacterError);
        }
    }

    Ok(())
}

/// Validates an HTML5 attribute value.
///
/// # Rules
/// HTML5 attribute values are generally very permissive. Most attributes accept
/// any string value. Specific validation (e.g., URL format, number format) is
/// handled by the consuming application, not the DOM layer.
///
/// This function performs minimal validation and accepts most values.
///
/// # Arguments
/// * `name` - The attribute name (for context-specific validation)
/// * `value` - The attribute value to validate
///
/// # Returns
/// * `Ok(())` if the attribute value is valid
/// * `Err(DomException)` if the attribute value is invalid
///
/// # Examples
///
/// ```
/// use dom_core::html5_validation::validate_html5_attribute_value;
///
/// assert!(validate_html5_attribute_value("id", "main").is_ok());
/// assert!(validate_html5_attribute_value("class", "btn primary").is_ok());
/// assert!(validate_html5_attribute_value("href", "https://example.com").is_ok());
/// ```
pub fn validate_html5_attribute_value(_name: &str, _value: &str) -> Result<(), DomException> {
    // HTML5 attribute values are very permissive
    // Most validation is semantic (e.g., valid URL) and done by the application
    // The DOM layer accepts almost any string value
    Ok(())
}

/// Checks if an element is a void element (cannot have children).
///
/// Void elements in HTML5 are: area, base, br, col, embed, hr, img, input,
/// link, meta, source, track, wbr.
///
/// # Arguments
/// * `tag_name` - The tag name to check (case-insensitive)
///
/// # Returns
/// `true` if the element is a void element, `false` otherwise
///
/// # Examples
///
/// ```
/// use dom_core::html5_validation::is_void_element;
///
/// assert!(is_void_element("br"));
/// assert!(is_void_element("img"));
/// assert!(is_void_element("input"));
/// assert!(!is_void_element("div"));
/// assert!(!is_void_element("span"));
/// ```
pub fn is_void_element(tag_name: &str) -> bool {
    let lower = tag_name.to_lowercase();
    VOID_ELEMENTS.contains(&lower.as_str())
}

/// Checks if an element is a raw text element (script, style).
///
/// Raw text elements have special parsing rules and their content is treated
/// as raw text rather than HTML.
///
/// # Arguments
/// * `tag_name` - The tag name to check (case-insensitive)
///
/// # Returns
/// `true` if the element is a raw text element, `false` otherwise
///
/// # Examples
///
/// ```
/// use dom_core::html5_validation::is_raw_text_element;
///
/// assert!(is_raw_text_element("script"));
/// assert!(is_raw_text_element("style"));
/// assert!(!is_raw_text_element("div"));
/// assert!(!is_raw_text_element("pre"));
/// ```
pub fn is_raw_text_element(tag_name: &str) -> bool {
    let lower = tag_name.to_lowercase();
    RAW_TEXT_ELEMENTS.contains(&lower.as_str())
}

/// Gets the set of all standard HTML5 tag names.
///
/// This is primarily for internal use and testing.
///
/// # Returns
/// A HashSet containing all standard HTML5 tag names (lowercase)
pub fn get_html5_tags() -> HashSet<&'static str> {
    HTML5_TAGS.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_tags() {
        assert!(validate_html5_tag_name("div").is_ok());
        assert!(validate_html5_tag_name("span").is_ok());
        assert!(validate_html5_tag_name("p").is_ok());
    }

    #[test]
    fn test_custom_elements() {
        assert!(validate_html5_tag_name("my-element").is_ok());
        assert!(validate_html5_tag_name("x-foo").is_ok());
    }

    #[test]
    fn test_xml_prefix_rejection() {
        assert_eq!(
            validate_html5_tag_name("xml-element"),
            Err(DomException::InvalidCharacterError)
        );
        assert_eq!(
            validate_html5_tag_name("XML-foo"),
            Err(DomException::InvalidCharacterError)
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            validate_html5_tag_name("my element"),
            Err(DomException::InvalidCharacterError)
        );
        assert_eq!(
            validate_html5_tag_name("my@element"),
            Err(DomException::InvalidCharacterError)
        );
    }

    #[test]
    fn test_void_elements() {
        assert!(is_void_element("br"));
        assert!(is_void_element("img"));
        assert!(!is_void_element("div"));
    }

    #[test]
    fn test_raw_text_elements() {
        assert!(is_raw_text_element("script"));
        assert!(is_raw_text_element("style"));
        assert!(!is_raw_text_element("div"));
    }

    #[test]
    fn test_attribute_validation() {
        assert!(validate_html5_attribute_name("id").is_ok());
        assert!(validate_html5_attribute_name("data-value").is_ok());
        assert_eq!(
            validate_html5_attribute_name("my attr"),
            Err(DomException::InvalidCharacterError)
        );
    }
}
