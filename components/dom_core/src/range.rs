//! Basic Range implementation for Document.createRange()
//!
//! This provides a minimal Range type for dom_core. The full Range implementation
//! with all DOM4 features is in the dom_advanced crate.

use crate::node::NodeRef;
use crate::text::Text;
use dom_types::DomException;
use parking_lot::RwLock;
use std::sync::Arc;

/// A basic Range represents a fragment of a document
///
/// This is a minimal implementation for Document.createRange().
/// For full Range functionality, see the dom_advanced crate.
#[derive(Debug, Clone)]
pub struct Range {
    /// The node that contains the start of the range
    start_container: NodeRef,
    /// The offset within the start container
    start_offset: usize,
    /// The node that contains the end of the range
    end_container: NodeRef,
    /// The offset within the end container
    end_offset: usize,
}

/// Thread-safe reference to a Range
pub type RangeRef = Arc<RwLock<Range>>;

impl Range {
    /// Create a new collapsed Range
    ///
    /// Creates a new Range with both boundary points set to the start
    /// of the document (or a dummy node if no document element exists).
    ///
    /// # Arguments
    /// * `document_node` - Optional node to use as the initial container.
    ///                     If None, creates a dummy text node.
    ///
    /// # Example
    /// ```
    /// use dom_core::{Document, Range};
    ///
    /// let doc = Document::new();
    /// let range = Range::new(None);
    /// assert!(range.collapsed());
    /// ```
    pub fn new(document_node: Option<NodeRef>) -> Self {
        let container = document_node.unwrap_or_else(|| {
            // Create a dummy text node for initialization
            let text = Text::new("");
            Arc::new(RwLock::new(Box::new(text) as Box<dyn crate::node::Node>))
        });

        Self {
            start_container: container.clone(),
            start_offset: 0,
            end_container: container,
            end_offset: 0,
        }
    }

    /// Get the start container node
    pub fn start_container(&self) -> &NodeRef {
        &self.start_container
    }

    /// Get the start offset
    pub fn start_offset(&self) -> usize {
        self.start_offset
    }

    /// Get the end container node
    pub fn end_container(&self) -> &NodeRef {
        &self.end_container
    }

    /// Get the end offset
    pub fn end_offset(&self) -> usize {
        self.end_offset
    }

    /// Check if the range is collapsed (start equals end)
    pub fn collapsed(&self) -> bool {
        Arc::ptr_eq(&self.start_container, &self.end_container)
            && self.start_offset == self.end_offset
    }

    /// Set the start boundary point
    ///
    /// # Errors
    /// Returns an error if the offset is invalid for the node type.
    pub fn set_start(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        let length = self.get_node_length(&node);
        if offset > length {
            return Err(DomException::InvalidModificationError);
        }

        self.start_container = node;
        self.start_offset = offset;

        // If start is after end, collapse to start
        if Arc::ptr_eq(&self.start_container, &self.end_container)
            && self.start_offset > self.end_offset
        {
            self.end_offset = self.start_offset;
        }

        Ok(())
    }

    /// Set the end boundary point
    ///
    /// # Errors
    /// Returns an error if the offset is invalid for the node type.
    pub fn set_end(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        let length = self.get_node_length(&node);
        if offset > length {
            return Err(DomException::InvalidModificationError);
        }

        self.end_container = node;
        self.end_offset = offset;

        // If end is before start, collapse to end
        if Arc::ptr_eq(&self.start_container, &self.end_container)
            && self.end_offset < self.start_offset
        {
            self.start_offset = self.end_offset;
        }

        Ok(())
    }

    /// Collapse the range to one of its boundary points
    ///
    /// # Arguments
    /// * `to_start` - If true, collapse to start; otherwise collapse to end
    pub fn collapse(&mut self, to_start: bool) {
        if to_start {
            self.end_container = self.start_container.clone();
            self.end_offset = self.start_offset;
        } else {
            self.start_container = self.end_container.clone();
            self.start_offset = self.end_offset;
        }
    }

    /// Get the length of a node for boundary validation
    fn get_node_length(&self, node: &NodeRef) -> usize {
        use dom_types::NodeType;

        let node_guard = node.read();
        match node_guard.node_type() {
            NodeType::Text | NodeType::Comment | NodeType::ProcessingInstruction => {
                node_guard.text_content().map(|s| s.len()).unwrap_or(0)
            }
            _ => node_guard.child_nodes().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_creation() {
        let range = Range::new(None);
        assert!(range.collapsed());
        assert_eq!(range.start_offset(), 0);
        assert_eq!(range.end_offset(), 0);
    }

    #[test]
    fn test_range_collapse() {
        let text = Text::new("Hello World");
        let node: NodeRef = Arc::new(RwLock::new(Box::new(text) as Box<dyn crate::node::Node>));

        let mut range = Range::new(Some(node.clone()));
        range.set_start(node.clone(), 0).unwrap();
        range.set_end(node, 5).unwrap();

        assert!(!range.collapsed());

        range.collapse(true);
        assert!(range.collapsed());
        assert_eq!(range.start_offset(), 0);
        assert_eq!(range.end_offset(), 0);
    }

    #[test]
    fn test_range_set_boundaries() {
        let text = Text::new("Hello");
        let node: NodeRef = Arc::new(RwLock::new(Box::new(text) as Box<dyn crate::node::Node>));

        let mut range = Range::new(Some(node.clone()));
        range.set_start(node.clone(), 1).unwrap();
        range.set_end(node, 4).unwrap();

        assert_eq!(range.start_offset(), 1);
        assert_eq!(range.end_offset(), 4);
        assert!(!range.collapsed());
    }

    #[test]
    fn test_range_invalid_offset() {
        let text = Text::new("Hi");
        let node: NodeRef = Arc::new(RwLock::new(Box::new(text) as Box<dyn crate::node::Node>));

        let mut range = Range::new(Some(node.clone()));

        // Offset 10 is beyond "Hi" (length 2)
        let result = range.set_start(node, 10);
        assert!(result.is_err());
    }
}
