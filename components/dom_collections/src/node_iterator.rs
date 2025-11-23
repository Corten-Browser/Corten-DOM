//! NodeIterator implementation for tree traversal
//!
//! Provides sequential traversal of nodes in document tree order according to DOM Level 4.
//!
//! # Example
//!
//! ```rust
//! use dom_collections::node_iterator::{NodeIterator, SHOW_ALL, SHOW_ELEMENT};
//! use dom_core::{Document, Node};
//! use parking_lot::RwLock;
//! use std::sync::Arc;
//!
//! let mut doc = Document::new();
//! let root = doc.create_element("div").unwrap();
//! let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
//!
//! // Iterate over all nodes
//! let mut iter = NodeIterator::new(root_node.clone(), SHOW_ALL, None);
//! while let Some(node) = iter.next_node() {
//!     println!("Node: {}", node.read().node_name());
//! }
//!
//! // Iterate over elements only
//! let mut iter = NodeIterator::new(root_node.clone(), SHOW_ELEMENT, None);
//! while let Some(element) = iter.next_node() {
//!     println!("Element: {}", element.read().node_name());
//! }
//! ```

use dom_core::NodeRef;
use dom_types::NodeType;
use std::sync::Arc;

/// Node filter callback for NodeIterator
///
/// A custom filter that can accept, reject, or skip nodes during traversal.
pub type NodeFilter = Option<Arc<dyn Fn(&NodeRef) -> FilterResult + Send + Sync>>;

/// Result of a node filter
///
/// Determines how the NodeIterator should handle a node during traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterResult {
    /// Accept the node (include in traversal)
    Accept = 1,
    /// Reject the node and its descendants (exclude from traversal)
    Reject = 2,
    /// Skip the node but check its descendants
    Skip = 3,
}

/// Constants for what_to_show bitmask

/// Show all node types
pub const SHOW_ALL: u32 = 0xFFFFFFFF;

/// Show only element nodes
pub const SHOW_ELEMENT: u32 = 0x1;

/// Show only text nodes
pub const SHOW_TEXT: u32 = 0x4;

/// Show only comment nodes
pub const SHOW_COMMENT: u32 = 0x80;

/// Show only document nodes
pub const SHOW_DOCUMENT: u32 = 0x100;

/// NodeIterator provides sequential traversal of DOM nodes
///
/// Traverses nodes in depth-first pre-order (document order).
/// Maintains a reference node pointer and supports both forward and backward traversal.
///
/// # DOM Level 4 Specification
///
/// NodeIterator provides sequential access to nodes in a tree:
/// - Maintains a reference node pointer
/// - `next_node()` moves forward in tree order
/// - `previous_node()` moves backward in tree order
/// - `what_to_show` is a bitmask filter for node types
/// - Optional NodeFilter callback for custom filtering
pub struct NodeIterator {
    /// Root node of the iteration
    root: NodeRef,
    /// Current reference node in the iteration
    reference_node: NodeRef,
    /// Bitmask filter for node types
    what_to_show: u32,
    /// Optional custom filter callback
    filter: NodeFilter,
    /// Tracks whether pointer is before reference node (for previous_node behavior)
    pointer_before_reference_node: bool,
}

impl NodeIterator {
    /// Creates a new NodeIterator
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start iteration from
    /// * `what_to_show` - Bitmask of node types to include (use SHOW_* constants)
    /// * `filter` - Optional custom filter callback
    ///
    /// # Example
    ///
    /// ```rust
    /// use dom_collections::node_iterator::{NodeIterator, SHOW_ELEMENT};
    /// # use dom_core::{Document, Node};
    /// # use parking_lot::RwLock;
    /// # use std::sync::Arc;
    /// # let mut doc = Document::new();
    /// # let root = doc.create_element("div").unwrap();
    /// # let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
    ///
    /// let mut iter = NodeIterator::new(root_node, SHOW_ELEMENT, None);
    /// ```
    pub fn new(root: NodeRef, what_to_show: u32, filter: NodeFilter) -> Self {
        let reference_node = root.clone();
        NodeIterator {
            root,
            reference_node,
            what_to_show,
            filter,
            pointer_before_reference_node: true,
        }
    }

    /// Returns the next node in document order
    ///
    /// Traverses the tree in depth-first pre-order, respecting the what_to_show
    /// bitmask and optional filter.
    ///
    /// Returns `None` when iteration reaches the end of the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dom_collections::node_iterator::{NodeIterator, SHOW_ALL};
    /// # use dom_core::{Document, Node};
    /// # use parking_lot::RwLock;
    /// # use std::sync::Arc;
    /// # let mut doc = Document::new();
    /// # let root = doc.create_element("div").unwrap();
    /// # let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
    /// let mut iter = NodeIterator::new(root_node, SHOW_ALL, None);
    ///
    /// while let Some(node) = iter.next_node() {
    ///     println!("Node: {}", node.read().node_name());
    /// }
    /// ```
    pub fn next_node(&mut self) -> Option<NodeRef> {
        let mut node = if self.pointer_before_reference_node {
            // Starting position - check reference node first
            self.reference_node.clone()
        } else {
            // Get next node in tree order after reference_node
            // If there are no more nodes, this returns None and we're done
            self.next_in_tree_order(&self.reference_node)?
        };

        loop {
            // Check filter
            match self.accept_node(&node) {
                FilterResult::Accept => {
                    self.reference_node = node.clone();
                    self.pointer_before_reference_node = false;
                    return Some(node);
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    // Find next sibling or ancestor's sibling
                    node = self.skip_subtree(&node)?;
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    node = self.next_in_tree_order(&node)?;
                }
            }
        }
    }

    /// Returns the previous node in reverse document order
    ///
    /// Traverses the tree backwards from the current position.
    /// Returns `None` when iteration reaches the beginning.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dom_collections::node_iterator::{NodeIterator, SHOW_ALL};
    /// # use dom_core::{Document, Node};
    /// # use parking_lot::RwLock;
    /// # use std::sync::Arc;
    /// # let mut doc = Document::new();
    /// # let root = doc.create_element("div").unwrap();
    /// # let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
    /// let mut iter = NodeIterator::new(root_node, SHOW_ALL, None);
    ///
    /// // Move to end first
    /// while iter.next_node().is_some() {}
    ///
    /// // Now traverse backwards
    /// while let Some(node) = iter.previous_node() {
    ///     println!("Node: {}", node.read().node_name());
    /// }
    /// ```
    pub fn previous_node(&mut self) -> Option<NodeRef> {
        // Symmetric to next_node():
        // - When pointer_before_reference_node is false: check reference_node first
        // - When pointer_before_reference_node is true: get the previous node
        let mut node = if !self.pointer_before_reference_node {
            // Pointer is after reference_node - check reference_node first
            self.reference_node.clone()
        } else {
            // Pointer is before reference_node - get previous node in tree order
            self.previous_in_tree_order(&self.reference_node)?
        };

        loop {
            // Check filter
            match self.accept_node(&node) {
                FilterResult::Accept => {
                    self.reference_node = node.clone();
                    self.pointer_before_reference_node = true;
                    return Some(node);
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    // Find previous sibling or parent
                    node = self.skip_subtree_backwards(&node)?;
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    node = self.previous_in_tree_order(&node)?;
                }
            }
        }
    }

    /// Detaches the iterator (deprecated, no-op)
    ///
    /// This method is deprecated in DOM Level 4 but included for compatibility.
    /// It has no effect in modern implementations.
    pub fn detach(&mut self) {
        // No-op as per modern DOM specification
        // Included for API compatibility
    }

    /// Returns the root node of this iterator
    pub fn root(&self) -> NodeRef {
        self.root.clone()
    }

    /// Returns the current reference node
    pub fn reference_node(&self) -> NodeRef {
        self.reference_node.clone()
    }

    /// Returns the what_to_show bitmask
    pub fn what_to_show(&self) -> u32 {
        self.what_to_show
    }

    // Internal helper methods

    /// Checks if a node should be accepted based on what_to_show and filter
    fn accept_node(&self, node: &NodeRef) -> FilterResult {
        // First check what_to_show bitmask
        let node_type = node.read().node_type();
        let type_bit = match node_type {
            NodeType::Element => SHOW_ELEMENT,
            NodeType::Text => SHOW_TEXT,
            NodeType::Comment => SHOW_COMMENT,
            NodeType::Document => SHOW_DOCUMENT,
            NodeType::CDataSection => 0x8,
            NodeType::ProcessingInstruction => 0x40,
            NodeType::DocumentType => 0x200,
            NodeType::DocumentFragment => 0x400,
            NodeType::Attribute => 0x2,
        };

        if self.what_to_show & type_bit == 0 {
            return FilterResult::Skip;
        }

        // Then apply custom filter if present
        if let Some(ref filter) = self.filter {
            filter(node)
        } else {
            FilterResult::Accept
        }
    }

    /// Returns the next node in tree order (depth-first pre-order)
    ///
    /// This implementation traverses from root to find the target node,
    /// then returns the next node in pre-order sequence. This is more robust
    /// than relying on parent_node() which may not be set correctly in all cases.
    fn next_in_tree_order(&self, target: &NodeRef) -> Option<NodeRef> {
        // Collect all nodes in pre-order from root
        let mut nodes = Vec::new();
        self.collect_preorder(&self.root, &mut nodes);

        // Find target and return the next node
        for (i, node) in nodes.iter().enumerate() {
            if self.is_same_node(node, target) {
                return nodes.get(i + 1).cloned();
            }
        }
        None
    }

    /// Collects all nodes in pre-order (depth-first) starting from the given node
    fn collect_preorder(&self, node: &NodeRef, result: &mut Vec<NodeRef>) {
        result.push(node.clone());
        for child in node.read().child_nodes() {
            self.collect_preorder(&child, result);
        }
    }

    /// Returns the previous node in reverse tree order
    ///
    /// This implementation traverses from root to find the target node,
    /// then returns the previous node in pre-order sequence.
    fn previous_in_tree_order(&self, target: &NodeRef) -> Option<NodeRef> {
        // Check if this is the root - no previous node
        if self.is_same_node(&self.root, target) {
            return None;
        }

        // Collect all nodes in pre-order from root
        let mut nodes = Vec::new();
        self.collect_preorder(&self.root, &mut nodes);

        // Find target and return the previous node
        for (i, node) in nodes.iter().enumerate() {
            if self.is_same_node(node, target) {
                if i > 0 {
                    return Some(nodes[i - 1].clone());
                }
                return None;
            }
        }
        None
    }

    /// Returns the last descendant of a node (for reverse traversal)
    fn last_descendant(&self, node: &NodeRef) -> NodeRef {
        let mut current = node.clone();
        loop {
            let children = current.read().child_nodes();
            if children.is_empty() {
                return current;
            }
            current = children.last().unwrap().clone();
        }
    }

    /// Skips a node and its entire subtree, returning the next node after the subtree
    ///
    /// This is used when a filter returns Reject - we skip the node and ALL descendants.
    fn skip_subtree(&self, node: &NodeRef) -> Option<NodeRef> {
        // Get the last descendant of this node (the deepest rightmost node)
        let last = self.last_descendant(node);

        // Return the next node in pre-order after the last descendant
        self.next_in_tree_order(&last)
    }

    /// Skips a node and its subtree in backward direction
    ///
    /// For backward traversal with Reject filter.
    fn skip_subtree_backwards(&self, node: &NodeRef) -> Option<NodeRef> {
        // When going backwards and rejecting a node, we want to skip to
        // the node before this one in pre-order (not its descendants)
        self.previous_in_tree_order(node)
    }

    /// Checks if a node is within the root subtree
    fn is_in_root_subtree(&self, node: &NodeRef) -> bool {
        if self.is_same_node(&self.root, node) {
            return true;
        }

        let mut current = node.read().parent_node();
        while let Some(parent) = current {
            if self.is_same_node(&self.root, &parent) {
                return true;
            }
            current = parent.read().parent_node();
        }

        false
    }

    /// Checks if two nodes are the same (pointer equality)
    fn is_same_node(&self, node1: &NodeRef, node2: &NodeRef) -> bool {
        let ptr1 = &**node1.read() as *const dyn dom_core::Node;
        let ptr2 = &**node2.read() as *const dyn dom_core::Node;
        ptr1 == ptr2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_result_values() {
        assert_eq!(FilterResult::Accept as u32, 1);
        assert_eq!(FilterResult::Reject as u32, 2);
        assert_eq!(FilterResult::Skip as u32, 3);
    }

    #[test]
    fn test_constants() {
        assert_eq!(SHOW_ALL, 0xFFFFFFFF);
        assert_eq!(SHOW_ELEMENT, 0x1);
        assert_eq!(SHOW_TEXT, 0x4);
        assert_eq!(SHOW_COMMENT, 0x80);
        assert_eq!(SHOW_DOCUMENT, 0x100);
    }
}
