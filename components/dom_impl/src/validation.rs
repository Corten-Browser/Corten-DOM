//! DOM Validation Module
//!
//! This module provides validation functions for DOM operations including:
//! - Namespace URI validation for `createElement_ns`, `createAttribute_ns`
//! - Qualified name validation for namespace-prefixed names
//! - HTML5-specific validation rules including reserved element names
//!
//! # Examples
//!
//! ```
//! use browser_dom_impl::validation::{
//!     validate_namespace, validate_qualified_name, validate_html5_element_name,
//!     HTML_NAMESPACE, SVG_NAMESPACE
//! };
//!
//! // Namespace validation
//! assert!(validate_namespace(Some(HTML_NAMESPACE)).is_ok());
//! assert!(validate_namespace(None).is_ok()); // No namespace is valid
//! assert!(validate_namespace(Some("")).is_err()); // Empty string is invalid
//!
//! // Qualified name validation
//! assert!(validate_qualified_name("div").is_ok());
//! assert!(validate_qualified_name("svg:rect").is_ok());
//! assert!(validate_qualified_name("123invalid").is_err());
//! ```

use dom_types::DomException;

// =============================================================================
// Namespace Constants
// =============================================================================

/// HTML namespace URI
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// SVG namespace URI
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// MathML namespace URI
pub const MATHML_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";

/// XLink namespace URI
pub const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

/// XML namespace URI
pub const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";

/// XMLNS (XML Namespaces) namespace URI
pub const XMLNS_NAMESPACE: &str = "http://www.w3.org/2000/xmlns/";

// =============================================================================
// Namespace Validation
// =============================================================================

/// Validate a namespace URI.
///
/// Per the DOM specification:
/// - `None` (no namespace) is valid
/// - A non-empty string is valid
/// - An empty string `""` is invalid and returns a `NamespaceError`
///
/// # Arguments
///
/// * `namespace` - Optional namespace URI to validate
///
/// # Returns
///
/// * `Ok(())` if the namespace is valid
/// * `Err(DomException::NamespaceError)` if the namespace is an empty string
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::validate_namespace;
/// use dom_types::DomException;
///
/// assert!(validate_namespace(None).is_ok());
/// assert!(validate_namespace(Some("http://example.com")).is_ok());
/// assert_eq!(validate_namespace(Some("")), Err(DomException::NamespaceError));
/// ```
pub fn validate_namespace(namespace: Option<&str>) -> Result<(), DomException> {
    match namespace {
        None => Ok(()),
        Some("") => Err(DomException::NamespaceError),
        Some(_) => Ok(()),
    }
}

/// Validate namespace and prefix consistency.
///
/// This function checks that the namespace and prefix are consistent per the
/// DOM specification rules:
///
/// 1. If prefix is "xml", namespace must be XML_NAMESPACE
/// 2. If prefix is "xmlns", namespace must be XMLNS_NAMESPACE
/// 3. If namespace is XMLNS_NAMESPACE, prefix must be "xmlns" or null
/// 4. If prefix is non-null, namespace must not be null
///
/// # Arguments
///
/// * `namespace` - The namespace URI
/// * `prefix` - The namespace prefix
///
/// # Returns
///
/// * `Ok(())` if the combination is valid
/// * `Err(DomException::NamespaceError)` if the combination is invalid
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::{validate_namespace_prefix, XML_NAMESPACE, XMLNS_NAMESPACE};
/// use dom_types::DomException;
///
/// // Valid combinations
/// assert!(validate_namespace_prefix(Some(XML_NAMESPACE), Some("xml")).is_ok());
/// assert!(validate_namespace_prefix(Some(XMLNS_NAMESPACE), Some("xmlns")).is_ok());
/// assert!(validate_namespace_prefix(Some("http://example.com"), Some("ex")).is_ok());
///
/// // Invalid combinations
/// assert!(validate_namespace_prefix(Some("http://wrong.com"), Some("xml")).is_err());
/// assert!(validate_namespace_prefix(None, Some("prefix")).is_err()); // prefix without namespace
/// ```
pub fn validate_namespace_prefix(
    namespace: Option<&str>,
    prefix: Option<&str>,
) -> Result<(), DomException> {
    match (prefix, namespace) {
        // Rule 4: If prefix is non-null, namespace must not be null
        (Some(_), None) => Err(DomException::NamespaceError),

        // Rule 1: If prefix is "xml", namespace must be XML_NAMESPACE
        (Some("xml"), Some(ns)) if ns != XML_NAMESPACE => Err(DomException::NamespaceError),

        // Rule 2: If prefix is "xmlns", namespace must be XMLNS_NAMESPACE
        (Some("xmlns"), Some(ns)) if ns != XMLNS_NAMESPACE => Err(DomException::NamespaceError),

        // Rule 3: If namespace is XMLNS_NAMESPACE, prefix must be "xmlns" or null
        (Some(p), Some(ns)) if ns == XMLNS_NAMESPACE && p != "xmlns" => {
            Err(DomException::NamespaceError)
        }

        // All other combinations are valid
        _ => Ok(()),
    }
}

// =============================================================================
// Qualified Name Validation
// =============================================================================

/// Check if a character is a valid XML NameStartChar.
///
/// Per XML 1.0 spec, NameStartChar is:
/// - ":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] |
///   [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] |
///   [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] |
///   [#x10000-#xEFFFF]
///
/// Note: For qualified name validation, we exclude ':' from NameStartChar
/// since it's used as a separator.
fn is_name_start_char(c: char) -> bool {
    matches!(c,
        'A'..='Z' | 'a'..='z' | '_' |
        '\u{C0}'..='\u{D6}' | '\u{D8}'..='\u{F6}' | '\u{F8}'..='\u{2FF}' |
        '\u{370}'..='\u{37D}' | '\u{37F}'..='\u{1FFF}' |
        '\u{200C}'..='\u{200D}' | '\u{2070}'..='\u{218F}' |
        '\u{2C00}'..='\u{2FEF}' | '\u{3001}'..='\u{D7FF}' |
        '\u{F900}'..='\u{FDCF}' | '\u{FDF0}'..='\u{FFFD}' |
        '\u{10000}'..='\u{EFFFF}'
    )
}

/// Check if a character is a valid XML NameChar.
///
/// NameChar is NameStartChar plus:
/// - "-" | "." | [0-9] | #xB7 | [#x0300-#x036F] | [#x203F-#x2040]
fn is_name_char(c: char) -> bool {
    is_name_start_char(c)
        || matches!(c,
            '-' | '.' | '0'..='9' | '\u{B7}' |
            '\u{0300}'..='\u{036F}' | '\u{203F}'..='\u{2040}'
        )
}

/// Validate that a string is a valid XML Name.
///
/// An XML Name must:
/// - Start with a NameStartChar (letter, underscore, or certain Unicode chars)
/// - Contain only NameChars
///
/// # Arguments
///
/// * `name` - The name to validate
///
/// # Returns
///
/// * `Ok(())` if the name is valid
/// * `Err(DomException::InvalidCharacterError)` if invalid
fn validate_name(name: &str) -> Result<(), DomException> {
    let mut chars = name.chars();

    // Empty name is invalid
    let first = chars.next().ok_or(DomException::InvalidCharacterError)?;

    // First character must be a NameStartChar
    if !is_name_start_char(first) {
        return Err(DomException::InvalidCharacterError);
    }

    // Rest must be NameChars
    for c in chars {
        if !is_name_char(c) {
            return Err(DomException::InvalidCharacterError);
        }
    }

    Ok(())
}

/// Validate a qualified name (prefix:localName).
///
/// A qualified name must satisfy:
/// - Not be empty
/// - Start with a letter or underscore (NameStartChar)
/// - Contain only valid Name characters (NameChar)
/// - Have at most one colon (':')
/// - If a colon is present, both prefix and localName must be non-empty
/// - Both prefix and localName (if present) must be valid Names
///
/// # Arguments
///
/// * `name` - The qualified name to validate
///
/// # Returns
///
/// * `Ok(())` if the qualified name is valid
/// * `Err(DomException::InvalidCharacterError)` if the name contains invalid characters
/// * `Err(DomException::NamespaceError)` if the colon usage is invalid
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::validate_qualified_name;
/// use dom_types::DomException;
///
/// // Valid qualified names
/// assert!(validate_qualified_name("div").is_ok());
/// assert!(validate_qualified_name("svg:rect").is_ok());
/// assert!(validate_qualified_name("_private").is_ok());
/// assert!(validate_qualified_name("my-element").is_ok());
///
/// // Invalid qualified names
/// assert!(validate_qualified_name("").is_err()); // Empty
/// assert!(validate_qualified_name("123").is_err()); // Starts with number
/// assert!(validate_qualified_name(":local").is_err()); // Empty prefix
/// assert!(validate_qualified_name("prefix:").is_err()); // Empty local name
/// assert!(validate_qualified_name("a:b:c").is_err()); // Multiple colons
/// ```
pub fn validate_qualified_name(name: &str) -> Result<(), DomException> {
    // Empty name is invalid
    if name.is_empty() {
        return Err(DomException::InvalidCharacterError);
    }

    // Count colons
    let colon_count = name.chars().filter(|&c| c == ':').count();

    if colon_count == 0 {
        // No prefix, just validate as a name
        validate_name(name)
    } else if colon_count == 1 {
        // Has prefix:localName structure
        let colon_pos = name.find(':').unwrap();

        // Check for empty prefix or localName
        if colon_pos == 0 || colon_pos == name.len() - 1 {
            return Err(DomException::NamespaceError);
        }

        let prefix = &name[..colon_pos];
        let local_name = &name[colon_pos + 1..];

        // Both must be valid names
        validate_name(prefix)?;
        validate_name(local_name)?;

        Ok(())
    } else {
        // More than one colon is invalid
        Err(DomException::NamespaceError)
    }
}

/// Parse a qualified name into (prefix, localName).
///
/// This function splits a qualified name at the colon separator and returns
/// the prefix (if any) and local name.
///
/// # Arguments
///
/// * `name` - The qualified name to parse
///
/// # Returns
///
/// * `Ok((None, localName))` if there's no prefix
/// * `Ok((Some(prefix), localName))` if there's a prefix
/// * `Err(DomException)` if the name is invalid
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::parse_qualified_name;
///
/// // No prefix
/// let (prefix, local) = parse_qualified_name("div").unwrap();
/// assert_eq!(prefix, None);
/// assert_eq!(local, "div");
///
/// // With prefix
/// let (prefix, local) = parse_qualified_name("svg:rect").unwrap();
/// assert_eq!(prefix, Some("svg"));
/// assert_eq!(local, "rect");
/// ```
pub fn parse_qualified_name(name: &str) -> Result<(Option<&str>, &str), DomException> {
    // First validate the qualified name
    validate_qualified_name(name)?;

    // Then parse it
    match name.find(':') {
        None => Ok((None, name)),
        Some(pos) => {
            let prefix = &name[..pos];
            let local_name = &name[pos + 1..];
            Ok((Some(prefix), local_name))
        }
    }
}

// =============================================================================
// HTML5 Validation
// =============================================================================

/// Reserved HTML element names that cannot be used with `createElement`.
///
/// These are MathML and SVG elements that have special handling in HTML5
/// and cannot be created via the generic `createElement` method.
pub const RESERVED_ELEMENT_NAMES: &[&str] = &[
    "annotation-xml",
    "color-profile",
    "font-face",
    "font-face-src",
    "font-face-uri",
    "font-face-format",
    "font-face-name",
    "missing-glyph",
];

/// Validate an element name for HTML5.
///
/// HTML5 element names must:
/// - Not be empty
/// - Start with a letter (a-z, A-Z)
/// - Contain only valid characters (letters, digits, hyphens)
/// - Not be a reserved element name
///
/// # Arguments
///
/// * `name` - The element name to validate
///
/// # Returns
///
/// * `Ok(())` if the name is valid for HTML5
/// * `Err(DomException::InvalidCharacterError)` if the name contains invalid characters
/// * `Err(DomException::NotSupportedError)` if the name is reserved
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::validate_html5_element_name;
/// use dom_types::DomException;
///
/// // Valid HTML5 element names
/// assert!(validate_html5_element_name("div").is_ok());
/// assert!(validate_html5_element_name("my-element").is_ok());
/// assert!(validate_html5_element_name("custom-element-123").is_ok());
///
/// // Invalid names
/// assert!(validate_html5_element_name("").is_err()); // Empty
/// assert!(validate_html5_element_name("123abc").is_err()); // Starts with number
/// assert!(validate_html5_element_name("font-face").is_err()); // Reserved
/// ```
pub fn validate_html5_element_name(name: &str) -> Result<(), DomException> {
    // Empty name is invalid
    if name.is_empty() {
        return Err(DomException::InvalidCharacterError);
    }

    // Check if reserved
    let name_lower = name.to_ascii_lowercase();
    if RESERVED_ELEMENT_NAMES.contains(&name_lower.as_str()) {
        return Err(DomException::NotSupportedError);
    }

    // First character must be a letter
    let mut chars = name.chars();
    let first = chars.next().unwrap(); // Safe: we checked non-empty

    if !first.is_ascii_alphabetic() {
        return Err(DomException::InvalidCharacterError);
    }

    // Rest must be letters, digits, or hyphens
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '-' {
            return Err(DomException::InvalidCharacterError);
        }
    }

    Ok(())
}

/// Validate an attribute name for HTML5.
///
/// HTML5 attribute names must:
/// - Not be empty
/// - Not contain any of the following characters: space, ", ', >, /, =
/// - Not start with reserved characters
///
/// # Arguments
///
/// * `name` - The attribute name to validate
///
/// # Returns
///
/// * `Ok(())` if the attribute name is valid
/// * `Err(DomException::InvalidCharacterError)` if invalid
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::validate_html5_attribute_name;
/// use dom_types::DomException;
///
/// // Valid attribute names
/// assert!(validate_html5_attribute_name("class").is_ok());
/// assert!(validate_html5_attribute_name("data-value").is_ok());
/// assert!(validate_html5_attribute_name("aria-label").is_ok());
///
/// // Invalid attribute names
/// assert!(validate_html5_attribute_name("").is_err()); // Empty
/// assert!(validate_html5_attribute_name("my attr").is_err()); // Contains space
/// assert!(validate_html5_attribute_name("attr=value").is_err()); // Contains =
/// ```
pub fn validate_html5_attribute_name(name: &str) -> Result<(), DomException> {
    // Empty name is invalid
    if name.is_empty() {
        return Err(DomException::InvalidCharacterError);
    }

    // Check for forbidden characters per HTML5 spec
    const FORBIDDEN_CHARS: &[char] = &[' ', '"', '\'', '>', '/', '='];

    for c in name.chars() {
        if FORBIDDEN_CHARS.contains(&c) {
            return Err(DomException::InvalidCharacterError);
        }
        // Also reject control characters
        if c.is_control() {
            return Err(DomException::InvalidCharacterError);
        }
    }

    Ok(())
}

/// Check if a custom element name is valid per HTML5 spec.
///
/// Custom element names must:
/// - Start with a lowercase ASCII letter
/// - Contain a hyphen
/// - Not contain uppercase letters
/// - Not start with restricted prefixes
/// - Not be a reserved name
///
/// # Arguments
///
/// * `name` - The custom element name to validate
///
/// # Returns
///
/// * `true` if the name is a valid custom element name
/// * `false` otherwise
///
/// # Examples
///
/// ```
/// use browser_dom_impl::validation::is_valid_custom_element_name;
///
/// assert!(is_valid_custom_element_name("my-element"));
/// assert!(is_valid_custom_element_name("custom-component"));
///
/// assert!(!is_valid_custom_element_name("div")); // No hyphen
/// assert!(!is_valid_custom_element_name("My-Element")); // Uppercase
/// assert!(!is_valid_custom_element_name("-element")); // Doesn't start with letter
/// ```
pub fn is_valid_custom_element_name(name: &str) -> bool {
    // Must be non-empty
    if name.is_empty() {
        return false;
    }

    // Must start with lowercase letter
    let mut chars = name.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    // Must contain a hyphen
    if !name.contains('-') {
        return false;
    }

    // Check all characters are valid (lowercase, digits, hyphen, period, underscore, specific Unicode)
    // For simplicity, we allow lowercase ASCII, digits, and hyphens
    for c in name.chars() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-' && c != '.' && c != '_' {
            return false;
        }
    }

    // Check not reserved
    if RESERVED_ELEMENT_NAMES.contains(&name) {
        return false;
    }

    // Check doesn't start with restricted prefixes
    const RESTRICTED_PREFIXES: &[&str] = &["xml", "xmlns"];
    for prefix in RESTRICTED_PREFIXES {
        if name.starts_with(prefix) {
            return false;
        }
    }

    true
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Namespace Validation Tests
    // -------------------------------------------------------------------------

    mod namespace_tests {
        use super::*;

        #[test]
        fn test_validate_namespace_none() {
            assert!(validate_namespace(None).is_ok());
        }

        #[test]
        fn test_validate_namespace_empty_string() {
            assert_eq!(validate_namespace(Some("")), Err(DomException::NamespaceError));
        }

        #[test]
        fn test_validate_namespace_valid_uri() {
            assert!(validate_namespace(Some(HTML_NAMESPACE)).is_ok());
            assert!(validate_namespace(Some(SVG_NAMESPACE)).is_ok());
            assert!(validate_namespace(Some(MATHML_NAMESPACE)).is_ok());
            assert!(validate_namespace(Some(XLINK_NAMESPACE)).is_ok());
            assert!(validate_namespace(Some(XML_NAMESPACE)).is_ok());
            assert!(validate_namespace(Some(XMLNS_NAMESPACE)).is_ok());
        }

        #[test]
        fn test_validate_namespace_custom_uri() {
            assert!(validate_namespace(Some("http://example.com/ns")).is_ok());
            assert!(validate_namespace(Some("urn:example:namespace")).is_ok());
        }

        #[test]
        fn test_validate_namespace_prefix_valid() {
            assert!(validate_namespace_prefix(Some(XML_NAMESPACE), Some("xml")).is_ok());
            assert!(validate_namespace_prefix(Some(XMLNS_NAMESPACE), Some("xmlns")).is_ok());
            assert!(validate_namespace_prefix(Some("http://example.com"), Some("ex")).is_ok());
            assert!(validate_namespace_prefix(None, None).is_ok());
            assert!(validate_namespace_prefix(Some(XMLNS_NAMESPACE), None).is_ok());
        }

        #[test]
        fn test_validate_namespace_prefix_invalid() {
            // Prefix without namespace
            assert_eq!(
                validate_namespace_prefix(None, Some("prefix")),
                Err(DomException::NamespaceError)
            );

            // Wrong namespace for "xml" prefix
            assert_eq!(
                validate_namespace_prefix(Some("http://wrong.com"), Some("xml")),
                Err(DomException::NamespaceError)
            );

            // Wrong namespace for "xmlns" prefix
            assert_eq!(
                validate_namespace_prefix(Some("http://wrong.com"), Some("xmlns")),
                Err(DomException::NamespaceError)
            );

            // XMLNS namespace with wrong prefix
            assert_eq!(
                validate_namespace_prefix(Some(XMLNS_NAMESPACE), Some("wrong")),
                Err(DomException::NamespaceError)
            );
        }
    }

    // -------------------------------------------------------------------------
    // Qualified Name Validation Tests
    // -------------------------------------------------------------------------

    mod qualified_name_tests {
        use super::*;

        #[test]
        fn test_validate_simple_names() {
            assert!(validate_qualified_name("div").is_ok());
            assert!(validate_qualified_name("span").is_ok());
            assert!(validate_qualified_name("_private").is_ok());
            assert!(validate_qualified_name("element123").is_ok());
            assert!(validate_qualified_name("my-element").is_ok());
            assert!(validate_qualified_name("my.element").is_ok());
        }

        #[test]
        fn test_validate_prefixed_names() {
            assert!(validate_qualified_name("svg:rect").is_ok());
            assert!(validate_qualified_name("xlink:href").is_ok());
            assert!(validate_qualified_name("xml:lang").is_ok());
            assert!(validate_qualified_name("ns:element-name").is_ok());
        }

        #[test]
        fn test_validate_empty_name() {
            assert_eq!(
                validate_qualified_name(""),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_invalid_start_char() {
            assert_eq!(
                validate_qualified_name("123abc"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_qualified_name("-element"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_qualified_name(".element"),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_empty_prefix_or_local() {
            // Empty prefix
            assert_eq!(
                validate_qualified_name(":local"),
                Err(DomException::NamespaceError)
            );
            // Empty local name
            assert_eq!(
                validate_qualified_name("prefix:"),
                Err(DomException::NamespaceError)
            );
        }

        #[test]
        fn test_validate_multiple_colons() {
            assert_eq!(
                validate_qualified_name("a:b:c"),
                Err(DomException::NamespaceError)
            );
            assert_eq!(
                validate_qualified_name("a::b"),
                Err(DomException::NamespaceError)
            );
        }

        #[test]
        fn test_validate_unicode_names() {
            // Unicode letters should be valid
            assert!(validate_qualified_name("élément").is_ok());
            assert!(validate_qualified_name("要素").is_ok());
        }

        #[test]
        fn test_parse_qualified_name_simple() {
            let (prefix, local) = parse_qualified_name("div").unwrap();
            assert_eq!(prefix, None);
            assert_eq!(local, "div");
        }

        #[test]
        fn test_parse_qualified_name_with_prefix() {
            let (prefix, local) = parse_qualified_name("svg:rect").unwrap();
            assert_eq!(prefix, Some("svg"));
            assert_eq!(local, "rect");
        }

        #[test]
        fn test_parse_qualified_name_invalid() {
            assert!(parse_qualified_name("").is_err());
            assert!(parse_qualified_name(":local").is_err());
            assert!(parse_qualified_name("prefix:").is_err());
        }
    }

    // -------------------------------------------------------------------------
    // HTML5 Validation Tests
    // -------------------------------------------------------------------------

    mod html5_tests {
        use super::*;

        #[test]
        fn test_validate_html5_element_name_valid() {
            assert!(validate_html5_element_name("div").is_ok());
            assert!(validate_html5_element_name("span").is_ok());
            assert!(validate_html5_element_name("my-element").is_ok());
            assert!(validate_html5_element_name("custom-123").is_ok());
            assert!(validate_html5_element_name("DIV").is_ok()); // HTML is case-insensitive
        }

        #[test]
        fn test_validate_html5_element_name_empty() {
            assert_eq!(
                validate_html5_element_name(""),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_html5_element_name_starts_with_number() {
            assert_eq!(
                validate_html5_element_name("123abc"),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_html5_element_name_reserved() {
            assert_eq!(
                validate_html5_element_name("annotation-xml"),
                Err(DomException::NotSupportedError)
            );
            assert_eq!(
                validate_html5_element_name("font-face"),
                Err(DomException::NotSupportedError)
            );
            assert_eq!(
                validate_html5_element_name("FONT-FACE"),
                Err(DomException::NotSupportedError)
            );
            assert_eq!(
                validate_html5_element_name("color-profile"),
                Err(DomException::NotSupportedError)
            );
        }

        #[test]
        fn test_validate_html5_element_name_invalid_chars() {
            assert_eq!(
                validate_html5_element_name("my element"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_element_name("my_element"),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_html5_attribute_name_valid() {
            assert!(validate_html5_attribute_name("class").is_ok());
            assert!(validate_html5_attribute_name("id").is_ok());
            assert!(validate_html5_attribute_name("data-value").is_ok());
            assert!(validate_html5_attribute_name("aria-label").is_ok());
            assert!(validate_html5_attribute_name("onclick").is_ok());
        }

        #[test]
        fn test_validate_html5_attribute_name_empty() {
            assert_eq!(
                validate_html5_attribute_name(""),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_validate_html5_attribute_name_forbidden_chars() {
            assert_eq!(
                validate_html5_attribute_name("my attr"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_attribute_name("attr=value"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_attribute_name("attr>"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_attribute_name("attr/"),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_attribute_name("attr\""),
                Err(DomException::InvalidCharacterError)
            );
            assert_eq!(
                validate_html5_attribute_name("attr'"),
                Err(DomException::InvalidCharacterError)
            );
        }

        #[test]
        fn test_is_valid_custom_element_name() {
            // Valid custom element names
            assert!(is_valid_custom_element_name("my-element"));
            assert!(is_valid_custom_element_name("custom-component"));
            assert!(is_valid_custom_element_name("x-button"));
            assert!(is_valid_custom_element_name("app-header"));
            assert!(is_valid_custom_element_name("element-123"));
            assert!(is_valid_custom_element_name("my-element.v1"));
            assert!(is_valid_custom_element_name("my_element-v1"));

            // Invalid: no hyphen
            assert!(!is_valid_custom_element_name("div"));
            assert!(!is_valid_custom_element_name("element"));

            // Invalid: starts with uppercase
            assert!(!is_valid_custom_element_name("My-element"));

            // Invalid: doesn't start with letter
            assert!(!is_valid_custom_element_name("-element"));
            assert!(!is_valid_custom_element_name("123-element"));

            // Invalid: contains uppercase
            assert!(!is_valid_custom_element_name("my-Element"));

            // Invalid: empty
            assert!(!is_valid_custom_element_name(""));

            // Invalid: reserved names
            assert!(!is_valid_custom_element_name("annotation-xml"));
            assert!(!is_valid_custom_element_name("font-face"));

            // Invalid: starts with restricted prefix
            assert!(!is_valid_custom_element_name("xml-element"));
            assert!(!is_valid_custom_element_name("xmlns-element"));
        }
    }

    // -------------------------------------------------------------------------
    // Internal Function Tests
    // -------------------------------------------------------------------------

    mod internal_tests {
        use super::*;

        #[test]
        fn test_is_name_start_char() {
            // Valid start chars
            assert!(is_name_start_char('a'));
            assert!(is_name_start_char('Z'));
            assert!(is_name_start_char('_'));
            assert!(is_name_start_char('é')); // Unicode letter

            // Invalid start chars
            assert!(!is_name_start_char('0'));
            assert!(!is_name_start_char('-'));
            assert!(!is_name_start_char('.'));
            assert!(!is_name_start_char(':'));
        }

        #[test]
        fn test_is_name_char() {
            // Valid name chars (includes start chars)
            assert!(is_name_char('a'));
            assert!(is_name_char('Z'));
            assert!(is_name_char('_'));

            // Additional valid name chars
            assert!(is_name_char('0'));
            assert!(is_name_char('9'));
            assert!(is_name_char('-'));
            assert!(is_name_char('.'));

            // Invalid name chars
            assert!(!is_name_char(':'));
            assert!(!is_name_char(' '));
            assert!(!is_name_char('!'));
        }

        #[test]
        fn test_validate_name() {
            assert!(validate_name("element").is_ok());
            assert!(validate_name("_private").is_ok());
            assert!(validate_name("element123").is_ok());
            assert!(validate_name("my-element").is_ok());

            assert!(validate_name("").is_err());
            assert!(validate_name("123").is_err());
            assert!(validate_name("-element").is_err());
        }
    }
}
