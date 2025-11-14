//! Namespace constants and validation
//!
//! Provides standard namespace URIs and validation utilities for XML, HTML, SVG, MathML, etc.

/// XML namespace URI
pub const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";

/// XMLNS namespace URI (for namespace declarations)
pub const XMLNS_NAMESPACE: &str = "http://www.w3.org/2000/xmlns/";

/// HTML namespace URI
pub const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

/// SVG namespace URI
pub const SVG_NAMESPACE: &str = "http://www.w3.org/2000/svg";

/// MathML namespace URI
pub const MATHML_NAMESPACE: &str = "http://www.w3.org/1998/Math/MathML";

/// XLink namespace URI
pub const XLINK_NAMESPACE: &str = "http://www.w3.org/1999/xlink";

/// Validate a namespace URI against known namespaces
pub fn is_valid_namespace_uri(uri: &str) -> bool {
    matches!(
        uri,
        XML_NAMESPACE
            | XMLNS_NAMESPACE
            | HTML_NAMESPACE
            | SVG_NAMESPACE
            | MATHML_NAMESPACE
            | XLINK_NAMESPACE
    ) || uri.starts_with("http://")
        || uri.starts_with("https://")
        || uri.starts_with("urn:")
}

/// Validate a qualified name according to XML Namespaces specification
///
/// # Arguments
/// * `qualified_name` - The qualified name to validate (e.g., "svg:rect")
///
/// # Returns
/// * `Ok((prefix, local_name))` - If valid, returns the prefix (if any) and local name
/// * `Err(message)` - If invalid, returns error message
pub fn validate_qualified_name(qualified_name: &str) -> Result<(Option<String>, String), String> {
    if qualified_name.is_empty() {
        return Err("Qualified name cannot be empty".to_string());
    }

    // Check for multiple colons (invalid)
    let colon_count = qualified_name.matches(':').count();
    if colon_count > 1 {
        return Err("Qualified name cannot contain multiple colons".to_string());
    }

    // Split by colon if present
    if colon_count == 1 {
        let parts: Vec<&str> = qualified_name.split(':').collect();
        let prefix = parts[0];
        let local_name = parts[1];

        // Validate prefix
        if prefix.is_empty() {
            return Err("Prefix cannot be empty".to_string());
        }
        if !is_valid_ncname(prefix) {
            return Err(format!("Invalid prefix: {}", prefix));
        }

        // Validate local name
        if local_name.is_empty() {
            return Err("Local name cannot be empty".to_string());
        }
        if !is_valid_ncname(local_name) {
            return Err(format!("Invalid local name: {}", local_name));
        }

        Ok((Some(prefix.to_string()), local_name.to_string()))
    } else {
        // No prefix, just validate local name
        if !is_valid_ncname(qualified_name) {
            return Err(format!("Invalid name: {}", qualified_name));
        }

        Ok((None, qualified_name.to_string()))
    }
}

/// Validate an NCName (non-colonized name) according to XML specification
///
/// NCNames are XML names without colons, used for local names and prefixes
fn is_valid_ncname(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let chars: Vec<char> = name.chars().collect();

    // First character must be letter or underscore
    let first = chars[0];
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Subsequent characters can be letters, digits, dots, hyphens, or underscores
    for c in &chars[1..] {
        if !c.is_alphanumeric() && *c != '.' && *c != '-' && *c != '_' {
            return false;
        }
    }

    true
}

/// Validate namespace and qualified name combination
///
/// Checks for invalid combinations like "xml" prefix without XML namespace
pub fn validate_namespace_and_qname(
    namespace_uri: Option<&str>,
    qualified_name: &str,
) -> Result<(), String> {
    let (prefix, _local_name) = validate_qualified_name(qualified_name)?;

    // Check namespace-prefix combinations
    if let Some(ref prefix_str) = prefix {
        match prefix_str.as_str() {
            "xml" => {
                // xml prefix must have XML namespace
                if namespace_uri != Some(XML_NAMESPACE) {
                    return Err(
                        "xml prefix must be used with XML namespace".to_string()
                    );
                }
            }
            "xmlns" => {
                // xmlns prefix is reserved and cannot be used
                return Err("xmlns prefix is reserved and cannot be used".to_string());
            }
            _ => {}
        }
    }

    // If namespace is XML namespace, prefix must be "xml"
    if namespace_uri == Some(XML_NAMESPACE) {
        if prefix.as_deref() != Some("xml") {
            return Err("XML namespace requires xml prefix".to_string());
        }
    }

    // If namespace is XMLNS namespace, this is invalid
    if namespace_uri == Some(XMLNS_NAMESPACE) {
        return Err("XMLNS namespace cannot be used for elements".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_constants() {
        assert_eq!(XML_NAMESPACE, "http://www.w3.org/XML/1998/namespace");
        assert_eq!(XMLNS_NAMESPACE, "http://www.w3.org/2000/xmlns/");
        assert_eq!(HTML_NAMESPACE, "http://www.w3.org/1999/xhtml");
        assert_eq!(SVG_NAMESPACE, "http://www.w3.org/2000/svg");
        assert_eq!(MATHML_NAMESPACE, "http://www.w3.org/1998/Math/MathML");
        assert_eq!(XLINK_NAMESPACE, "http://www.w3.org/1999/xlink");
    }

    #[test]
    fn test_is_valid_namespace_uri() {
        assert!(is_valid_namespace_uri(HTML_NAMESPACE));
        assert!(is_valid_namespace_uri(SVG_NAMESPACE));
        assert!(is_valid_namespace_uri(MATHML_NAMESPACE));
        assert!(is_valid_namespace_uri("http://example.com/ns"));
        assert!(is_valid_namespace_uri("https://example.com/ns"));
        assert!(is_valid_namespace_uri("urn:example:namespace"));

        assert!(!is_valid_namespace_uri("invalid"));
        assert!(!is_valid_namespace_uri("ftp://example.com"));
    }

    #[test]
    fn test_validate_qualified_name_simple() {
        let result = validate_qualified_name("div");
        assert!(result.is_ok());
        let (prefix, local_name) = result.unwrap();
        assert_eq!(prefix, None);
        assert_eq!(local_name, "div");
    }

    #[test]
    fn test_validate_qualified_name_with_prefix() {
        let result = validate_qualified_name("svg:rect");
        assert!(result.is_ok());
        let (prefix, local_name) = result.unwrap();
        assert_eq!(prefix, Some("svg".to_string()));
        assert_eq!(local_name, "rect");
    }

    #[test]
    fn test_validate_qualified_name_empty() {
        let result = validate_qualified_name("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_qualified_name_multiple_colons() {
        let result = validate_qualified_name("a:b:c");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_qualified_name_empty_prefix() {
        let result = validate_qualified_name(":local");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_qualified_name_empty_local() {
        let result = validate_qualified_name("prefix:");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_qualified_name_invalid_chars() {
        let result = validate_qualified_name("invalid name");
        assert!(result.is_err());

        let result = validate_qualified_name("123invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_ncname() {
        assert!(is_valid_ncname("div"));
        assert!(is_valid_ncname("my-element"));
        assert!(is_valid_ncname("my_element"));
        assert!(is_valid_ncname("element123"));
        assert!(is_valid_ncname("_element"));

        assert!(!is_valid_ncname(""));
        assert!(!is_valid_ncname("123element"));
        assert!(!is_valid_ncname("my:element"));
        assert!(!is_valid_ncname("my element"));
    }

    #[test]
    fn test_validate_namespace_and_qname_xml_prefix() {
        // xml prefix with XML namespace - valid
        let result = validate_namespace_and_qname(Some(XML_NAMESPACE), "xml:lang");
        assert!(result.is_ok());

        // xml prefix without XML namespace - invalid
        let result = validate_namespace_and_qname(Some(HTML_NAMESPACE), "xml:lang");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_namespace_and_qname_xmlns_prefix() {
        // xmlns prefix is always invalid
        let result = validate_namespace_and_qname(Some(HTML_NAMESPACE), "xmlns:foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_namespace_and_qname_xml_namespace() {
        // XML namespace requires xml prefix
        let result = validate_namespace_and_qname(Some(XML_NAMESPACE), "foo:bar");
        assert!(result.is_err());

        let result = validate_namespace_and_qname(Some(XML_NAMESPACE), "xml:lang");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_namespace_and_qname_xmlns_namespace() {
        // XMLNS namespace cannot be used for elements
        let result = validate_namespace_and_qname(Some(XMLNS_NAMESPACE), "foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_namespace_and_qname_svg() {
        // SVG namespace with svg prefix - valid
        let result = validate_namespace_and_qname(Some(SVG_NAMESPACE), "svg:rect");
        assert!(result.is_ok());

        // SVG namespace without prefix - also valid
        let result = validate_namespace_and_qname(Some(SVG_NAMESPACE), "rect");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_namespace_and_qname_no_namespace() {
        // No namespace with simple name - valid
        let result = validate_namespace_and_qname(None, "div");
        assert!(result.is_ok());

        // No namespace with prefixed name - also valid (HTML can have prefixes)
        let result = validate_namespace_and_qname(None, "custom:element");
        assert!(result.is_ok());
    }
}
