//! CSS selector matching logic

use dom_core::ElementRef;
use dom_types::DomException;

/// Parsed selector matcher
pub struct SelectorMatcher {
    /// Parsed selector components
    components: Vec<SelectorComponent>,
}

/// A component of a CSS selector
#[derive(Debug, Clone, PartialEq)]
enum SelectorComponent {
    /// Tag name selector (e.g., "div")
    Tag(String),
    /// Class selector (e.g., ".button")
    Class(String),
    /// ID selector (e.g., "#main")
    Id(String),
    /// Universal selector ("*")
    Universal,
    /// Attribute exists (e.g., "[disabled]")
    AttributeExists(String),
    /// Attribute equals (e.g., "[type='text']")
    AttributeEquals(String, String),
    /// Descendant combinator (" ")
    Descendant,
    /// Child combinator (">")
    Child,
}

impl SelectorMatcher {
    /// Create a new selector matcher by parsing the selector string
    pub fn new(selector: &str) -> Result<Self, DomException> {
        let components = Self::parse_selector(selector)?;

        Ok(Self { components })
    }

    /// Check if an element matches this selector
    pub fn matches(&self, element: &ElementRef) -> Result<bool, DomException> {
        let elem = element.read();

        // Simple selector matching (no combinators)
        for component in &self.components {
            match component {
                SelectorComponent::Tag(tag) => {
                    if elem.tag_name().to_uppercase() != tag.to_uppercase() {
                        return Ok(false);
                    }
                }
                SelectorComponent::Class(class_name) => {
                    if !elem.class_list().iter().any(|c| c == class_name) {
                        return Ok(false);
                    }
                }
                SelectorComponent::Id(id) => {
                    if elem.id() != Some(id.as_str()) {
                        return Ok(false);
                    }
                }
                SelectorComponent::Universal => {
                    // Matches everything
                }
                SelectorComponent::AttributeExists(attr) => {
                    if !elem.has_attribute(attr) {
                        return Ok(false);
                    }
                }
                SelectorComponent::AttributeEquals(attr, value) => {
                    if elem.get_attribute(attr) != Some(value.as_str()) {
                        return Ok(false);
                    }
                }
                SelectorComponent::Descendant | SelectorComponent::Child => {
                    // Combinators are handled differently (require tree traversal)
                    // For now, we'll treat them as no-ops in simple matching
                }
            }
        }

        Ok(true)
    }

    /// Match only by tag name (limited matching for NodeRef)
    /// This is used when we can only access node_name() from the Node trait
    pub fn matches_tag_only(&self, tag_name: &str) -> bool {
        // Only check tag components, ignore everything else
        for component in &self.components {
            match component {
                SelectorComponent::Tag(tag) => {
                    if tag_name.to_uppercase() != tag.to_uppercase() {
                        return false;
                    }
                }
                SelectorComponent::Universal => {
                    // Matches everything
                }
                _ => {
                    // For other selectors (class, ID, attributes), we can't match
                    // without Element-specific methods, so we return false
                    return false;
                }
            }
        }
        true
    }

    /// Parse a selector string into components
    fn parse_selector(selector: &str) -> Result<Vec<SelectorComponent>, DomException> {
        let selector = selector.trim();

        if selector.is_empty() {
            return Err(DomException::syntax_error("Empty selector"));
        }

        // Check for obviously invalid selectors
        if selector.starts_with("###") || selector.contains("...") {
            return Err(DomException::syntax_error("Invalid selector syntax"));
        }

        let mut components = Vec::new();
        let mut current = String::new();
        let mut chars = selector.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                // Class selector
                '.' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut components)?;
                        current.clear();
                    }

                    // Read class name
                    let mut class_name = String::new();
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                            class_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if class_name.is_empty() {
                        return Err(DomException::syntax_error("Empty class name"));
                    }

                    components.push(SelectorComponent::Class(class_name));
                }

                // ID selector
                '#' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut components)?;
                        current.clear();
                    }

                    // Read ID
                    let mut id = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                            id.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    if id.is_empty() {
                        return Err(DomException::syntax_error("Empty ID selector"));
                    }

                    components.push(SelectorComponent::Id(id));
                }

                // Attribute selector
                '[' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut components)?;
                        current.clear();
                    }

                    // Parse attribute selector
                    let mut attr_selector = String::new();
                    let mut depth = 1;

                    for ch in chars.by_ref() {
                        if ch == '[' {
                            depth += 1;
                        } else if ch == ']' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        attr_selector.push(ch);
                    }

                    Self::parse_attribute(&attr_selector, &mut components)?;
                }

                // Combinator: child (>)
                '>' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut components)?;
                        current.clear();
                    }
                    components.push(SelectorComponent::Child);
                }

                // Whitespace (descendant combinator)
                ' ' | '\t' | '\n' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut components)?;
                        current.clear();
                        components.push(SelectorComponent::Descendant);
                    }
                }

                // Universal selector
                '*' => {
                    if current.is_empty() {
                        components.push(SelectorComponent::Universal);
                    } else {
                        current.push(ch);
                    }
                }

                // Regular character (part of tag name or similar)
                _ => {
                    current.push(ch);
                }
            }
        }

        // Process any remaining component
        if !current.is_empty() {
            Self::parse_component(&current, &mut components)?;
        }

        if components.is_empty() {
            return Err(DomException::syntax_error("No valid selector components"));
        }

        Ok(components)
    }

    /// Parse a simple component (tag name, etc.)
    fn parse_component(
        component: &str,
        components: &mut Vec<SelectorComponent>,
    ) -> Result<(), DomException> {
        let trimmed = component.trim();

        if trimmed.is_empty() {
            return Ok(());
        }

        if trimmed == "*" {
            components.push(SelectorComponent::Universal);
        } else {
            // Assume it's a tag name
            components.push(SelectorComponent::Tag(trimmed.to_string()));
        }

        Ok(())
    }

    /// Parse an attribute selector
    fn parse_attribute(
        attr: &str,
        components: &mut Vec<SelectorComponent>,
    ) -> Result<(), DomException> {
        let attr = attr.trim();

        if attr.is_empty() {
            return Err(DomException::syntax_error("Empty attribute selector"));
        }

        // Check for attribute=value pattern
        if let Some(eq_pos) = attr.find('=') {
            let name = attr[..eq_pos].trim();
            let mut value = attr[eq_pos + 1..].trim();

            // Remove quotes if present
            if (value.starts_with('\'') && value.ends_with('\''))
                || (value.starts_with('"') && value.ends_with('"'))
            {
                value = &value[1..value.len() - 1];
            }

            components.push(SelectorComponent::AttributeEquals(
                name.to_string(),
                value.to_string(),
            ));
        } else {
            // Just attribute existence
            components.push(SelectorComponent::AttributeExists(attr.to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Element;
    use parking_lot::RwLock;
    use std::sync::Arc;

    #[test]
    fn test_parse_tag_selector() {
        let matcher = SelectorMatcher::new("div").unwrap();
        assert_eq!(matcher.components.len(), 1);
        assert!(matches!(matcher.components[0], SelectorComponent::Tag(_)));
    }

    #[test]
    fn test_parse_class_selector() {
        let matcher = SelectorMatcher::new(".button").unwrap();
        assert_eq!(matcher.components.len(), 1);
        assert!(matches!(matcher.components[0], SelectorComponent::Class(_)));
    }

    #[test]
    fn test_parse_id_selector() {
        let matcher = SelectorMatcher::new("#main").unwrap();
        assert_eq!(matcher.components.len(), 1);
        assert!(matches!(matcher.components[0], SelectorComponent::Id(_)));
    }

    #[test]
    fn test_parse_invalid_selector() {
        let result = SelectorMatcher::new("###invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_match_tag() {
        let matcher = SelectorMatcher::new("div").unwrap();
        let elem = Element::new("div");
        let elem_ref = Arc::new(RwLock::new(elem));

        assert!(matcher.matches(&elem_ref).unwrap());
    }

    #[test]
    fn test_match_class() {
        let matcher = SelectorMatcher::new(".button").unwrap();
        let mut elem = Element::new("div");
        elem.set_attribute("class", "button").unwrap();
        let elem_ref = Arc::new(RwLock::new(elem));

        assert!(matcher.matches(&elem_ref).unwrap());
    }

    #[test]
    fn test_match_id() {
        let matcher = SelectorMatcher::new("#main").unwrap();
        let mut elem = Element::new("div");
        elem.set_attribute("id", "main").unwrap();
        let elem_ref = Arc::new(RwLock::new(elem));

        assert!(matcher.matches(&elem_ref).unwrap());
    }

    #[test]
    fn test_no_match() {
        let matcher = SelectorMatcher::new("button").unwrap();
        let elem = Element::new("div");
        let elem_ref = Arc::new(RwLock::new(elem));

        assert!(!matcher.matches(&elem_ref).unwrap());
    }
}
