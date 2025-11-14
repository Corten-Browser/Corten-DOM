//! CSS selector matching logic

use dom_core::{ElementRef, Node, NodeRef};
use dom_types::{DomException, NodeType};

/// Parsed selector matcher
pub struct SelectorMatcher {
    /// Parsed selector segments with combinators
    segments: Vec<SelectorSegment>,
}

/// A segment of a selector (sequence of components without combinators)
#[derive(Debug, Clone)]
struct SelectorSegment {
    /// Components in this segment
    components: Vec<SelectorComponent>,
    /// Combinator that follows this segment (None for last segment)
    combinator: Option<Combinator>,
}

/// CSS combinators
#[derive(Debug, Clone, PartialEq)]
enum Combinator {
    /// Descendant combinator (" ")
    Descendant,
    /// Child combinator (">")
    Child,
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
}

impl SelectorMatcher {
    /// Create a new selector matcher by parsing the selector string
    pub fn new(selector: &str) -> Result<Self, DomException> {
        let segments = Self::parse_selector(selector)?;

        Ok(Self { segments })
    }

    /// Check if an element matches this selector (with tree context for combinators)
    pub fn matches(&self, element: &ElementRef) -> Result<bool, DomException> {
        // If selector has no combinators, use simple matching
        if self.segments.len() == 1 && self.segments[0].combinator.is_none() {
            return Ok(Self::matches_segment(element, &self.segments[0]));
        }

        // For selectors with combinators, we need tree context
        // Start from the rightmost segment and match right-to-left
        self.matches_with_segments(element, &self.segments)
    }

    /// Match an element against segments (handles combinators)
    fn matches_with_segments(
        &self,
        element: &ElementRef,
        segments: &[SelectorSegment],
    ) -> Result<bool, DomException> {
        if segments.is_empty() {
            return Ok(true);
        }

        // Get the last segment (rightmost)
        let last_idx = segments.len() - 1;
        let last_segment = &segments[last_idx];

        // Element must match the last segment
        if !Self::matches_segment(element, last_segment) {
            return Ok(false);
        }

        // If this is the only segment, we're done
        if segments.len() == 1 {
            return Ok(true);
        }

        // Get the combinator before the last segment
        let penultimate_segment = &segments[last_idx - 1];
        let combinator = penultimate_segment
            .combinator
            .as_ref()
            .ok_or_else(|| DomException::syntax_error("Missing combinator"))?;

        // Get remaining segments (everything except the last)
        let remaining_segments = &segments[..last_idx];

        // Check combinator relationship using parent pointers from element
        match combinator {
            Combinator::Child => {
                // Immediate parent must match remaining segments
                if let Some(parent) = element.read().parent_node() {
                    if Self::node_matches_segments(&parent, remaining_segments, self)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Combinator::Descendant => {
                // Any ancestor must match remaining segments
                let mut current = element.read().parent_node();
                while let Some(ancestor) = current {
                    if Self::node_matches_segments(&ancestor, remaining_segments, self)? {
                        return Ok(true);
                    }
                    current = ancestor.read().parent_node();
                }
                Ok(false)
            }
        }
    }

    /// Check if a NodeRef matches segments (for use with parent pointers)
    fn node_matches_segments(
        node: &NodeRef,
        segments: &[SelectorSegment],
        matcher: &SelectorMatcher,
    ) -> Result<bool, DomException> {
        // Check if this is an element node
        if node.read().node_type() != NodeType::Element {
            return Ok(false);
        }

        // Downcast to Element to check matching
        let node_guard = node.read();
        if let Some(element) = node_guard.as_any().downcast_ref::<dom_core::Element>() {
            // For simple case (no more combinators), just check if element matches last segment
            if segments.len() == 1 && segments[0].combinator.is_none() {
                return Ok(Self::matches_segment_raw(element, &segments[0]));
            }

            // For combinators, need to check recursively
            // Create an ElementRef to call matches_with_segments
            let elem_clone = element.clone();
            drop(node_guard);
            let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem_clone));
            matcher.matches_with_segments(&elem_ref, segments)
        } else {
            Ok(false)
        }
    }

    /// Match an element (raw, not wrapped in Arc) against a segment
    fn matches_segment_raw(element: &dom_core::Element, segment: &SelectorSegment) -> bool {
        for component in &segment.components {
            match component {
                SelectorComponent::Tag(tag) => {
                    if element.tag_name().to_uppercase() != tag.to_uppercase() {
                        return false;
                    }
                }
                SelectorComponent::Class(class) => {
                    if let Some(class_attr) = element.get_attribute("class") {
                        let classes: Vec<&str> = class_attr.split_whitespace().collect();
                        if !classes.contains(&class.as_str()) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                SelectorComponent::Id(id) => {
                    if element.get_attribute("id").as_deref() != Some(id) {
                        return false;
                    }
                }
                SelectorComponent::Universal => {
                    // Universal selector matches everything
                }
                SelectorComponent::AttributeExists(name) => {
                    if element.get_attribute(name).is_none() {
                        return false;
                    }
                }
                SelectorComponent::AttributeEquals(name, value) => {
                    if element.get_attribute(name).as_deref() != Some(value) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Convert NodeRef to ElementRef if it's an element
    fn node_to_element(node: &NodeRef) -> Option<ElementRef> {
        let node_guard = node.read();
        if node_guard.node_type() != NodeType::Element {
            return None;
        }

        if let Some(element) = node_guard.as_any().downcast_ref::<dom_core::Element>() {
            let element_clone = element.clone();
            drop(node_guard);
            Some(std::sync::Arc::new(parking_lot::RwLock::new(
                element_clone,
            )))
        } else {
            None
        }
    }

    /// Check if an element matches a single segment (no combinators)
    fn matches_segment(element: &ElementRef, segment: &SelectorSegment) -> bool {
        let elem = element.read();

        for component in &segment.components {
            match component {
                SelectorComponent::Tag(tag) => {
                    if elem.tag_name().to_uppercase() != tag.to_uppercase() {
                        return false;
                    }
                }
                SelectorComponent::Class(class) => {
                    if let Some(class_attr) = elem.get_attribute("class") {
                        let classes: Vec<&str> = class_attr.split_whitespace().collect();
                        if !classes.contains(&class.as_str()) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                SelectorComponent::Id(id) => {
                    if elem.get_attribute("id").as_deref() != Some(id) {
                        return false;
                    }
                }
                SelectorComponent::Universal => {
                    // Universal selector matches everything
                }
                SelectorComponent::AttributeExists(name) => {
                    if elem.get_attribute(name).is_none() {
                        return false;
                    }
                }
                SelectorComponent::AttributeEquals(name, value) => {
                    if elem.get_attribute(name).as_deref() != Some(value) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Match tag only (for backwards compatibility)
    pub fn matches_tag_only(&self, tag: &str) -> bool {
        // Only check if first segment has a tag component
        if let Some(first_segment) = self.segments.first() {
            for component in &first_segment.components {
                if let SelectorComponent::Tag(selector_tag) = component {
                    return selector_tag.to_uppercase() == tag.to_uppercase();
                }
            }
        }
        false
    }

    /// Parse a selector string into segments
    fn parse_selector(selector: &str) -> Result<Vec<SelectorSegment>, DomException> {
        let selector = selector.trim();

        if selector.is_empty() {
            return Err(DomException::syntax_error("Empty selector"));
        }

        // Check for obviously invalid selectors
        if selector.starts_with("###") || selector.contains("...") {
            return Err(DomException::syntax_error("Invalid selector syntax"));
        }

        let mut segments = Vec::new();
        let mut current_components = Vec::new();
        let mut current = String::new();
        let mut chars = selector.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                // Class selector
                '.' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut current_components)?;
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

                    current_components.push(SelectorComponent::Class(class_name));
                }

                // ID selector
                '#' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut current_components)?;
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

                    current_components.push(SelectorComponent::Id(id));
                }

                // Attribute selector
                '[' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut current_components)?;
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

                    Self::parse_attribute(&attr_selector, &mut current_components)?;
                }

                // Combinator: child (>)
                '>' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut current_components)?;
                        current.clear();
                    }
                    if !current_components.is_empty() {
                        segments.push(SelectorSegment {
                            components: current_components.clone(),
                            combinator: Some(Combinator::Child),
                        });
                        current_components.clear();
                    }
                }

                // Whitespace (descendant combinator)
                ' ' | '\t' | '\n' => {
                    if !current.is_empty() {
                        Self::parse_component(&current, &mut current_components)?;
                        current.clear();
                    }
                    // Only add descendant combinator if we have components
                    if !current_components.is_empty() {
                        // Check if next non-whitespace is '>' (child combinator)
                        let mut is_child_combinator = false;
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch == '>' {
                                is_child_combinator = true;
                                break;
                            } else if !next_ch.is_whitespace() {
                                break;
                            }
                            chars.next();
                        }

                        if !is_child_combinator {
                            segments.push(SelectorSegment {
                                components: current_components.clone(),
                                combinator: Some(Combinator::Descendant),
                            });
                            current_components.clear();
                        }
                    }
                }

                // Universal selector
                '*' => {
                    if current.is_empty() {
                        current_components.push(SelectorComponent::Universal);
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
            Self::parse_component(&current, &mut current_components)?;
        }

        // Add final segment
        if !current_components.is_empty() {
            segments.push(SelectorSegment {
                components: current_components,
                combinator: None,
            });
        }

        if segments.is_empty() {
            return Err(DomException::syntax_error("No valid selector components"));
        }

        Ok(segments)
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
        assert_eq!(matcher.segments.len(), 1);
        assert_eq!(matcher.segments[0].components.len(), 1);
        assert!(matches!(
            matcher.segments[0].components[0],
            SelectorComponent::Tag(_)
        ));
    }

    #[test]
    fn test_parse_class_selector() {
        let matcher = SelectorMatcher::new(".button").unwrap();
        assert_eq!(matcher.segments.len(), 1);
        assert!(matches!(
            matcher.segments[0].components[0],
            SelectorComponent::Class(_)
        ));
    }

    #[test]
    fn test_parse_id_selector() {
        let matcher = SelectorMatcher::new("#main").unwrap();
        assert_eq!(matcher.segments.len(), 1);
        assert!(matches!(
            matcher.segments[0].components[0],
            SelectorComponent::Id(_)
        ));
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

    #[test]
    fn test_parse_descendant_combinator() {
        let matcher = SelectorMatcher::new("div li").unwrap();
        assert_eq!(matcher.segments.len(), 2);
        assert_eq!(
            matcher.segments[0].combinator,
            Some(Combinator::Descendant)
        );
    }

    #[test]
    fn test_parse_child_combinator() {
        let matcher = SelectorMatcher::new("div > ul").unwrap();
        assert_eq!(matcher.segments.len(), 2);
        assert_eq!(matcher.segments[0].combinator, Some(Combinator::Child));
    }
}
