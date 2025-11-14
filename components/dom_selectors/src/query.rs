//! Selectable trait and query methods

use crate::matcher::SelectorMatcher;
use dom_collections::NodeList;
use dom_core::{Element, ElementRef, Node, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// Trait for elements and documents that support CSS selector queries
pub trait Selectable {
    /// Find the first element matching the selector
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;

    /// Find all elements matching the selector
    fn query_selector_all(&self, selector: &str) -> Result<NodeList, DomException>;

    /// Check if this element matches the selector
    fn matches(&self, selector: &str) -> Result<bool, DomException>;

    /// Find the closest ancestor (including self) matching the selector
    fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
}

/// Helper struct for executing selector queries
pub struct SelectorQuery;

impl SelectorQuery {
    /// Find first matching element in tree rooted at node
    pub fn find_first(root: &NodeRef, selector: &str) -> Result<Option<ElementRef>, DomException> {
        // Parse selector
        let matcher = SelectorMatcher::new(selector)?;

        // Depth-first search
        Self::find_first_recursive(root, &matcher)
    }

    /// Find all matching elements in tree rooted at node
    pub fn find_all(root: &NodeRef, selector: &str) -> Result<Vec<ElementRef>, DomException> {
        // Parse selector
        let matcher = SelectorMatcher::new(selector)?;

        // Depth-first search collecting all matches
        let mut results = Vec::new();
        Self::find_all_recursive(root, &matcher, &mut results);

        Ok(results)
    }

    /// Recursive helper for find_first
    fn find_first_recursive(
        node: &NodeRef,
        matcher: &SelectorMatcher,
    ) -> Result<Option<ElementRef>, DomException> {
        let node_guard = node.read();
        let node_type = node_guard.node_type();

        // Check if current node is element and matches (using full element matching)
        if node_type == NodeType::Element && Self::matches_node(node, matcher) {
            // Downcast to Element using as_any() for full selector support
            drop(node_guard);
            return Ok(Self::try_as_element(node));
        }

        // Search children
        for child in node_guard.child_nodes() {
            if let Ok(Some(found)) = Self::find_first_recursive(&child, matcher) {
                return Ok(Some(found));
            }
        }

        Ok(None)
    }

    /// Recursive helper for find_all
    fn find_all_recursive(
        node: &NodeRef,
        matcher: &SelectorMatcher,
        results: &mut Vec<ElementRef>,
    ) {
        let node_guard = node.read();
        let node_type = node_guard.node_type();

        // Check if current node is element and matches (using full element matching)
        if node_type == NodeType::Element && Self::matches_node(node, matcher) {
            // Downcast to Element using as_any() for full selector support
            if let Some(element_ref) = Self::try_as_element(node) {
                results.push(element_ref);
            }
        }

        // Search all children
        for child in node_guard.child_nodes() {
            Self::find_all_recursive(&child, matcher, results);
        }
    }

    /// Try to convert NodeRef to ElementRef by downcasting using as_any()
    fn try_as_element(node: &NodeRef) -> Option<ElementRef> {
        let node_guard = node.read();

        // Use as_any() to downcast to Element
        if let Some(element) = node_guard.as_any().downcast_ref::<Element>() {
            // Clone the element to create an ElementRef
            let element_clone = element.clone();
            drop(node_guard);
            return Some(Arc::new(RwLock::new(element_clone)));
        }

        None
    }

    /// Match a node using full element matching (classes, IDs, attributes)
    fn matches_node(node: &NodeRef, matcher: &SelectorMatcher) -> bool {
        let node_guard = node.read();
        if node_guard.node_type() != NodeType::Element {
            return false;
        }

        // Use as_any() to downcast to Element for full matching
        if let Some(element) = node_guard.as_any().downcast_ref::<Element>() {
            let element_clone = element.clone();
            drop(node_guard);

            // Create an ElementRef for matching
            let element_ref = Arc::new(RwLock::new(element_clone));

            // Use full matcher (supports classes, IDs, attributes)
            matcher.matches(&element_ref).unwrap_or(false)
        } else {
            // Fallback to simple tag matching if downcast fails
            let tag = node_guard.node_name();
            matcher.matches_tag_only(tag)
        }
    }
}

// Implement Selectable for Element
impl Selectable for Element {
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException> {
        let matcher = SelectorMatcher::new(selector)?;

        // Search descendants
        for child in self.child_nodes() {
            if let Ok(Some(found)) = SelectorQuery::find_first_recursive(&child, &matcher) {
                return Ok(Some(found));
            }
        }

        Ok(None)
    }

    fn query_selector_all(&self, selector: &str) -> Result<NodeList, DomException> {
        let matcher = SelectorMatcher::new(selector)?;
        let mut results = Vec::new();

        // Search all descendants
        for child in self.child_nodes() {
            SelectorQuery::find_all_recursive(&child, &matcher, &mut results);
        }

        // Convert ElementRef vec to NodeRef vec
        let node_refs: Vec<NodeRef> = results
            .into_iter()
            .map(|elem_ref| {
                Arc::new(RwLock::new(
                    Box::new(elem_ref.read().clone()) as Box<dyn Node>
                ))
            })
            .collect();

        Ok(NodeList::new_static(node_refs))
    }

    fn matches(&self, selector: &str) -> Result<bool, DomException> {
        let matcher = SelectorMatcher::new(selector)?;
        let elem_ref = Arc::new(RwLock::new(self.clone()));
        matcher.matches(&elem_ref)
    }

    fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException> {
        let matcher = SelectorMatcher::new(selector)?;
        let elem_ref = Arc::new(RwLock::new(self.clone()));

        // Check self first
        if matcher.matches(&elem_ref)? {
            return Ok(Some(elem_ref));
        }

        // Walk up parent chain
        let mut current = self.parent_node();
        while let Some(parent) = current {
            let parent_guard = parent.read();
            if parent_guard.node_type() == NodeType::Element {
                if let Some(parent_elem) = SelectorQuery::try_as_element(&parent) {
                    if matcher.matches(&parent_elem)? {
                        return Ok(Some(parent_elem));
                    }
                }
            }
            current = parent_guard.parent_node();
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Document;

    #[test]
    fn test_selector_query_basic() {
        let mut doc = Document::new();
        let elem = doc.create_element("div").unwrap();

        // Basic test - should not panic
        let result = elem.read().query_selector("span");
        assert!(result.is_ok());
    }

    #[test]
    fn test_matches_basic() {
        let elem = Element::new("button");
        let result = elem.matches("button");
        assert!(result.is_ok());
    }
}
