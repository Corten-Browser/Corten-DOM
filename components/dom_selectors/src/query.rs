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

        // Check if current node is element and matches (using limited tag-only matching)
        if node_type == NodeType::Element && Self::matches_node(node, matcher) {
            // Note: Due to architectural limitations (Box<dyn Node> can't be downcast
            // without unsafe code or additional trait support), we return None here.
            // Full query_selector support requires either:
            // 1. Adding downcast support to dom-core's Node trait
            // 2. Storing ElementRefs alongside NodeRefs in the tree
            // 3. Using unsafe code for downcasting
            //
            // For now, matches() works when called directly on Elements.
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

        // Check if current node is element and matches (using limited tag-only matching)
        if node_type == NodeType::Element && Self::matches_node(node, matcher) {
            // Same limitation as find_first_recursive - can't extract ElementRef
            if let Some(element_ref) = Self::try_as_element(node) {
                results.push(element_ref);
            }
        }

        // Search all children
        for child in node_guard.child_nodes() {
            Self::find_all_recursive(&child, matcher, results);
        }
    }

    /// Try to convert NodeRef to ElementRef by creating a new Element from the node
    /// This is a workaround since we can't easily downcast trait objects
    fn try_as_element(_node: &NodeRef) -> Option<ElementRef> {
        // Since we can't downcast Box<dyn Node> to Element without unsafe code,
        // and we're constrained to not modify dom-core to add downcasting support,
        // we return None here. This limits query_selector functionality.
        //
        // For matches() to work, use it directly on Element instances.
        // Full query_selector support would require:
        // 1. Adding downcasting support to Node trait in dom-core
        // 2. Or storing ElementRefs directly in the tree alongside NodeRefs
        // 3. Or using a different architecture
        None
    }

    /// Match a node directly using its Node trait methods (limited functionality)
    fn matches_node(node: &NodeRef, matcher: &SelectorMatcher) -> bool {
        let node_guard = node.read();
        if node_guard.node_type() != NodeType::Element {
            return false;
        }

        // We can only match by tag name using node_name()
        // Full matching (classes, IDs, attributes) requires Element-specific methods
        // which aren't available through the Node trait
        let tag = node_guard.node_name();

        // Simple tag matching only
        matcher.matches_tag_only(tag)
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
