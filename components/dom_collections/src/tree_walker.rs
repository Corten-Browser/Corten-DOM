//! TreeWalker implementation for tree navigation
//!
//! Provides bidirectional tree navigation according to DOM Level 4.
//!
//! # Example
//!
//! ```rust
//! use dom_collections::tree_walker::TreeWalker;
//! use dom_collections::node_iterator::{SHOW_ALL, SHOW_ELEMENT};
//! use dom_core::{Document, Node};
//! use parking_lot::RwLock;
//! use std::sync::Arc;
//!
//! let mut doc = Document::new();
//! let root = doc.create_element("div").unwrap();
//! let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
//!
//! // Navigate with TreeWalker
//! let mut walker = TreeWalker::new(root_node.clone(), SHOW_ALL, None);
//!
//! // Go to first child
//! if let Some(child) = walker.first_child() {
//!     println!("First child: {}", child.read().node_name());
//! }
//!
//! // Go to next sibling
//! if let Some(sibling) = walker.next_sibling() {
//!     println!("Next sibling: {}", sibling.read().node_name());
//! }
//!
//! // Go back to parent
//! if let Some(parent) = walker.parent_node() {
//!     println!("Parent: {}", parent.read().node_name());
//! }
//! ```

use crate::node_iterator::{FilterResult, NodeFilter, SHOW_ELEMENT, SHOW_TEXT, SHOW_COMMENT, SHOW_DOCUMENT};
use dom_core::NodeRef;
use dom_types::NodeType;

/// TreeWalker provides bidirectional tree navigation
///
/// Allows navigating a tree with directional methods (parent, first_child, etc.)
/// and sequential methods (previous_node, next_node).
///
/// # DOM Level 4 Specification
///
/// TreeWalker provides tree navigation with:
/// - Settable current_node pointer (can be moved by user)
/// - Directional navigation (parent_node, first_child, last_child, previous_sibling, next_sibling)
/// - Sequential navigation (previous_node, next_node)
/// - what_to_show bitmask filter for node types
/// - Optional NodeFilter callback for custom filtering
pub struct TreeWalker {
    /// Root node of the traversal
    root: NodeRef,
    /// Current node in the traversal (settable)
    current_node: NodeRef,
    /// Bitmask filter for node types
    what_to_show: u32,
    /// Optional custom filter callback
    filter: NodeFilter,
}

impl TreeWalker {
    /// Creates a new TreeWalker
    ///
    /// # Arguments
    ///
    /// * `root` - The root node to start traversal from
    /// * `what_to_show` - Bitmask of node types to include (use SHOW_* constants)
    /// * `filter` - Optional custom filter callback
    ///
    /// # Example
    ///
    /// ```rust
    /// use dom_collections::tree_walker::TreeWalker;
    /// use dom_collections::node_iterator::SHOW_ELEMENT;
    /// # use dom_core::{Document, Node};
    /// # use parking_lot::RwLock;
    /// # use std::sync::Arc;
    /// # let mut doc = Document::new();
    /// # let root = doc.create_element("div").unwrap();
    /// # let root_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(root.read().clone())));
    ///
    /// let mut walker = TreeWalker::new(root_node, SHOW_ELEMENT, None);
    /// ```
    pub fn new(root: NodeRef, what_to_show: u32, filter: NodeFilter) -> Self {
        let current_node = root.clone();
        TreeWalker {
            root,
            current_node,
            what_to_show,
            filter,
        }
    }

    /// Returns the current node
    pub fn current_node(&self) -> NodeRef {
        self.current_node.clone()
    }

    /// Sets the current node
    ///
    /// Allows jumping to any node in the tree (not restricted to root subtree).
    pub fn set_current_node(&mut self, node: NodeRef) {
        self.current_node = node;
    }

    /// Returns the root node
    pub fn root(&self) -> NodeRef {
        self.root.clone()
    }

    /// Returns the what_to_show bitmask
    pub fn what_to_show(&self) -> u32 {
        self.what_to_show
    }

    /// Moves to the parent node
    ///
    /// Returns None if current_node is root or has no parent.
    /// Updates current_node to the parent if successful.
    pub fn parent_node(&mut self) -> Option<NodeRef> {
        // Can't go above root
        if self.is_same_node(&self.root, &self.current_node) {
            return None;
        }

        // Get parent
        let parent = self.current_node.read().parent_node()?;

        // Check if parent is root (stop at root boundary)
        if self.is_same_node(&self.root, &parent) {
            // Accept root if it passes filter
            match self.accept_node(&parent) {
                FilterResult::Accept => {
                    self.current_node = parent.clone();
                    return Some(parent);
                }
                _ => {
                    // Root doesn't pass filter, can't go higher
                    return None;
                }
            }
        }

        // Parent is not root, continue up if it doesn't pass filter
        let mut node = parent;
        loop {
            match self.accept_node(&node) {
                FilterResult::Accept => {
                    self.current_node = node.clone();
                    return Some(node);
                }
                FilterResult::Reject | FilterResult::Skip => {
                    // Move to next parent
                    if self.is_same_node(&self.root, &node) {
                        return None;
                    }
                    let next_parent = node.read().parent_node()?;
                    node = next_parent;
                }
            }
        }
    }

    /// Moves to the first child
    ///
    /// Returns None if current_node has no children that pass the filter.
    /// Updates current_node to the first child if successful.
    pub fn first_child(&mut self) -> Option<NodeRef> {
        let children = self.current_node.read().child_nodes();

        for child in children.iter() {
            match self.accept_node(child) {
                FilterResult::Accept => {
                    self.current_node = child.clone();
                    return Some(child.clone());
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    if let Some(descendant) = self.first_child_of(child) {
                        self.current_node = descendant.clone();
                        return Some(descendant);
                    }
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    continue;
                }
            }
        }

        None
    }

    /// Moves to the last child
    ///
    /// Returns None if current_node has no children that pass the filter.
    /// Updates current_node to the last child if successful.
    pub fn last_child(&mut self) -> Option<NodeRef> {
        let children = self.current_node.read().child_nodes();

        for child in children.iter().rev() {
            match self.accept_node(child) {
                FilterResult::Accept => {
                    self.current_node = child.clone();
                    return Some(child.clone());
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    if let Some(descendant) = self.last_child_of(child) {
                        self.current_node = descendant.clone();
                        return Some(descendant);
                    }
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    continue;
                }
            }
        }

        None
    }

    /// Moves to the previous sibling
    ///
    /// Returns None if current_node has no previous sibling that passes the filter.
    /// Updates current_node to the previous sibling if successful.
    pub fn previous_sibling(&mut self) -> Option<NodeRef> {
        // Get parent
        let parent = self.current_node.read().parent_node()?;
        let siblings = parent.read().child_nodes();

        // Find current node in siblings
        let current_ptr = {
            let guard = self.current_node.read();
            &**guard as *const dyn dom_core::Node
        };

        for (i, sibling) in siblings.iter().enumerate() {
            let sibling_ptr = {
                let guard = sibling.read();
                &**guard as *const dyn dom_core::Node
            };
            if sibling_ptr == current_ptr && i > 0 {
                // Found current node, check previous siblings
                for prev_sibling in siblings[..i].iter().rev() {
                    match self.accept_node(prev_sibling) {
                        FilterResult::Accept => {
                            self.current_node = prev_sibling.clone();
                            return Some(prev_sibling.clone());
                        }
                        FilterResult::Skip => {
                            // Skip this node but check its descendants
                            if let Some(descendant) = self.last_child_of(prev_sibling) {
                                self.current_node = descendant.clone();
                                return Some(descendant);
                            }
                        }
                        FilterResult::Reject => {
                            // Skip this node and all its descendants
                            continue;
                        }
                    }
                }
                break;
            }
        }

        None
    }

    /// Moves to the next sibling
    ///
    /// Returns None if current_node has no next sibling that passes the filter.
    /// Updates current_node to the next sibling if successful.
    pub fn next_sibling(&mut self) -> Option<NodeRef> {
        // Get parent
        let parent = self.current_node.read().parent_node()?;
        let siblings = parent.read().child_nodes();

        // Find current node in siblings
        let current_ptr = {
            let guard = self.current_node.read();
            &**guard as *const dyn dom_core::Node
        };

        for (i, sibling) in siblings.iter().enumerate() {
            let sibling_ptr = {
                let guard = sibling.read();
                &**guard as *const dyn dom_core::Node
            };
            if sibling_ptr == current_ptr && i + 1 < siblings.len() {
                // Found current node, check next siblings
                for next_sibling in siblings[i + 1..].iter() {
                    match self.accept_node(next_sibling) {
                        FilterResult::Accept => {
                            self.current_node = next_sibling.clone();
                            return Some(next_sibling.clone());
                        }
                        FilterResult::Skip => {
                            // Skip this node but check its descendants
                            if let Some(descendant) = self.first_child_of(next_sibling) {
                                self.current_node = descendant.clone();
                                return Some(descendant);
                            }
                        }
                        FilterResult::Reject => {
                            // Skip this node and all its descendants
                            continue;
                        }
                    }
                }
                break;
            }
        }

        None
    }

    /// Moves to the previous node in tree order
    ///
    /// Traverses the tree backwards from current_node.
    /// Returns None when reaching the beginning.
    /// Updates current_node to the previous node if successful.
    pub fn previous_node(&mut self) -> Option<NodeRef> {
        // Check if at root
        if self.is_same_node(&self.root, &self.current_node) {
            return None;
        }

        let mut node = self.previous_in_tree_order(&self.current_node)?;

        loop {
            match self.accept_node(&node) {
                FilterResult::Accept => {
                    self.current_node = node.clone();
                    return Some(node);
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    node = self.skip_subtree_backwards(&node)?;
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    node = self.previous_in_tree_order(&node)?;
                }
            }
        }
    }

    /// Moves to the next node in tree order
    ///
    /// Traverses the tree forward from current_node.
    /// Returns None when reaching the end.
    /// Updates current_node to the next node if successful.
    pub fn next_node(&mut self) -> Option<NodeRef> {
        let mut node = self.next_in_tree_order(&self.current_node)?;

        loop {
            match self.accept_node(&node) {
                FilterResult::Accept => {
                    self.current_node = node.clone();
                    return Some(node);
                }
                FilterResult::Reject => {
                    // Skip this node and all its descendants
                    node = self.skip_subtree(&node)?;
                }
                FilterResult::Skip => {
                    // Skip this node but check its descendants
                    node = self.next_in_tree_order(&node)?;
                }
            }
        }
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

    /// Helper to find first accepted child of a node (for Skip case)
    fn first_child_of(&self, node: &NodeRef) -> Option<NodeRef> {
        let children = node.read().child_nodes();

        for child in children.iter() {
            match self.accept_node(child) {
                FilterResult::Accept => {
                    return Some(child.clone());
                }
                FilterResult::Skip => {
                    if let Some(descendant) = self.first_child_of(child) {
                        return Some(descendant);
                    }
                }
                FilterResult::Reject => {
                    continue;
                }
            }
        }

        None
    }

    /// Helper to find last accepted child of a node (for Skip case)
    fn last_child_of(&self, node: &NodeRef) -> Option<NodeRef> {
        let children = node.read().child_nodes();

        for child in children.iter().rev() {
            match self.accept_node(child) {
                FilterResult::Accept => {
                    return Some(child.clone());
                }
                FilterResult::Skip => {
                    if let Some(descendant) = self.last_child_of(child) {
                        return Some(descendant);
                    }
                }
                FilterResult::Reject => {
                    continue;
                }
            }
        }

        None
    }

    /// Returns the next node in tree order (depth-first pre-order)
    fn next_in_tree_order(&self, node: &NodeRef) -> Option<NodeRef> {
        // First check children
        let children = node.read().child_nodes();
        if !children.is_empty() {
            return Some(children[0].clone());
        }

        // No children, need to find next sibling or ancestor's sibling
        let mut current = node.clone();

        loop {
            // Check if we're at root - if so, we're done
            if self.is_same_node(&self.root, &current) {
                return None;
            }

            // Get parent
            let parent = current.read().parent_node()?;

            // Find current node in parent's children and get next sibling
            let siblings = parent.read().child_nodes();
            let current_ptr = {
                let guard = current.read();
                &**guard as *const dyn dom_core::Node
            };

            for (i, sibling) in siblings.iter().enumerate() {
                let sibling_ptr = {
                    let guard = sibling.read();
                    &**guard as *const dyn dom_core::Node
                };
                if sibling_ptr == current_ptr {
                    // Found current node, check if there's a next sibling
                    if i + 1 < siblings.len() {
                        return Some(siblings[i + 1].clone());
                    }
                    // No next sibling, continue up the tree
                    break;
                }
            }

            // Move up to parent and continue looking
            current = parent;
        }
    }

    /// Returns the previous node in reverse tree order
    fn previous_in_tree_order(&self, node: &NodeRef) -> Option<NodeRef> {
        // Check if this is the root
        if self.is_same_node(&self.root, node) {
            return None;
        }

        // Get parent
        let parent = node.read().parent_node()?;

        // Find previous sibling through parent's children
        let siblings = parent.read().child_nodes();
        let node_ptr = {
            let guard = node.read();
            &**guard as *const dyn dom_core::Node
        };

        for (i, sibling) in siblings.iter().enumerate() {
            let sibling_ptr = {
                let guard = sibling.read();
                &**guard as *const dyn dom_core::Node
            };
            if sibling_ptr == node_ptr {
                // Found current node
                if i > 0 {
                    // Has previous sibling - return its last descendant
                    return Some(self.last_descendant(&siblings[i - 1]));
                } else {
                    // No previous sibling, return parent
                    return Some(parent);
                }
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
    fn skip_subtree(&self, node: &NodeRef) -> Option<NodeRef> {
        let mut current = node.clone();

        loop {
            // Check if we're at root
            if self.is_same_node(&self.root, &current) {
                return None;
            }

            // Get parent
            let parent = current.read().parent_node()?;

            // Find next sibling through parent's children
            let siblings = parent.read().child_nodes();
            let current_ptr = {
                let guard = current.read();
                &**guard as *const dyn dom_core::Node
            };

            for (i, sibling) in siblings.iter().enumerate() {
                let sibling_ptr = {
                    let guard = sibling.read();
                    &**guard as *const dyn dom_core::Node
                };
                if sibling_ptr == current_ptr {
                    // Found current node
                    if i + 1 < siblings.len() {
                        return Some(siblings[i + 1].clone());
                    }
                    // No next sibling, continue up the tree
                    break;
                }
            }

            // Move up to parent
            current = parent;
        }
    }

    /// Skips a node and its subtree in backward direction
    fn skip_subtree_backwards(&self, node: &NodeRef) -> Option<NodeRef> {
        // Check if this is the root
        if self.is_same_node(&self.root, node) {
            return None;
        }

        // Get parent
        let parent = node.read().parent_node()?;

        // Find previous sibling through parent's children
        let siblings = parent.read().child_nodes();
        let node_ptr = {
            let guard = node.read();
            &**guard as *const dyn dom_core::Node
        };

        for (i, sibling) in siblings.iter().enumerate() {
            let sibling_ptr = {
                let guard = sibling.read();
                &**guard as *const dyn dom_core::Node
            };
            if sibling_ptr == node_ptr {
                // Found current node
                if i > 0 {
                    // Has previous sibling - return its last descendant
                    return Some(self.last_descendant(&siblings[i - 1]));
                } else {
                    // No previous sibling, return parent
                    return Some(parent);
                }
            }
        }

        None
    }

    /// Checks if two nodes are the same (pointer equality)
    fn is_same_node(&self, node1: &NodeRef, node2: &NodeRef) -> bool {
        let ptr1 = &**node1.read() as *const dyn dom_core::Node;
        let ptr2 = &**node2.read() as *const dyn dom_core::Node;
        ptr1 == ptr2
    }
}
