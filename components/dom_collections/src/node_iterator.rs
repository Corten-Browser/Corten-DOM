//! NodeIterator implementation for tree traversal

use dom_core::NodeRef;

/// NodeIterator provides sequential traversal
pub struct NodeIterator {
    root: NodeRef,
    reference_node: NodeRef,
    what_to_show: u32,
}

impl NodeIterator {
    /// Creates a new NodeIterator
    pub fn new(root: NodeRef, what_to_show: u32) -> Self {
        let reference_node = root.clone();
        NodeIterator {
            root,
            reference_node,
            what_to_show,
        }
    }

    /// Returns the next node
    pub fn next_node(&mut self) -> Option<NodeRef> {
        // TODO: Implement next_node
        None
    }

    /// Returns the previous node
    pub fn previous_node(&mut self) -> Option<NodeRef> {
        // TODO: Implement previous_node
        None
    }
}
