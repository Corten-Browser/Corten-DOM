//! Range API implementation
//!
//! Represents a fragment of a document that can contain nodes and parts of text nodes.

use dom_core::NodeRef;
use dom_types::{DomException, NodeType};
use std::cmp::Ordering;

/// A Range represents a fragment of a document
#[derive(Debug, Clone)]
pub struct Range {
    start_container: NodeRef,
    start_offset: usize,
    end_container: NodeRef,
    end_offset: usize,
}

impl Range {
    /// Create a new collapsed Range
    ///
    /// Note: In a real implementation, Range would be associated with a document.
    /// For simplicity, we create a detached range that can be positioned later.
    pub fn new(_document: &dom_core::Document) -> Self {
        // Create a dummy node reference for initialization
        // In practice, this should be set via set_start/set_end
        let dummy_text = dom_core::Text::new("");
        let node_ref = std::sync::Arc::new(parking_lot::RwLock::new(
            Box::new(dummy_text) as Box<dyn dom_core::Node>
        ));

        Self {
            start_container: node_ref.clone(),
            start_offset: 0,
            end_container: node_ref,
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
        self.start_container.ptr_eq(&self.end_container) && self.start_offset == self.end_offset
    }

    /// Get the common ancestor container of both boundary points
    pub fn common_ancestor_container(&self) -> NodeRef {
        if self.start_container.ptr_eq(&self.end_container) {
            return self.start_container.clone();
        }

        // Find common ancestor by traversing up from start
        let mut ancestors = Vec::new();
        let mut current = self.start_container.clone();
        while let Some(parent) = current.parent_node() {
            ancestors.push(parent.clone());
            current = parent;
        }

        // Traverse up from end until we find a common ancestor
        let mut current = self.end_container.clone();
        while let Some(parent) = current.parent_node() {
            if ancestors.iter().any(|a| a.ptr_eq(&parent)) {
                return parent;
            }
            current = parent;
        }

        // Should not reach here if both nodes are in the same document
        self.start_container.clone()
    }

    /// Set the start boundary point
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is invalid for the node type.
    pub fn set_start(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        self.validate_boundary_point(&node, offset)?;
        self.start_container = node;
        self.start_offset = offset;

        // If start is after end, collapse to start
        if self.compare_boundary_points(&self.start_container, self.start_offset, &self.end_container, self.end_offset) == Ordering::Greater {
            self.end_container = self.start_container.clone();
            self.end_offset = self.start_offset;
        }

        Ok(())
    }

    /// Set the end boundary point
    ///
    /// # Errors
    ///
    /// Returns an error if the offset is invalid for the node type.
    pub fn set_end(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        self.validate_boundary_point(&node, offset)?;
        self.end_container = node;
        self.end_offset = offset;

        // If end is before start, collapse to end
        if self.compare_boundary_points(&self.start_container, self.start_offset, &self.end_container, self.end_offset) == Ordering::Greater {
            self.start_container = self.end_container.clone();
            self.start_offset = self.end_offset;
        }

        Ok(())
    }

    /// Set both boundary points to the same position
    pub fn collapse(&mut self, to_start: bool) {
        if to_start {
            self.end_container = self.start_container.clone();
            self.end_offset = self.start_offset;
        } else {
            self.start_container = self.end_container.clone();
            self.start_offset = self.end_offset;
        }
    }

    /// Select the contents of a node
    ///
    /// # Errors
    ///
    /// Returns an error if the node cannot contain a range.
    pub fn select_node_contents(&mut self, node: NodeRef) -> Result<(), DomException> {
        match node.read().node_type() {
            NodeType::DocumentType => {
                return Err(DomException::NotSupportedError);
            }
            _ => {}
        }

        let length = self.get_node_length(&node);

        self.start_container = node.clone();
        self.start_offset = 0;
        self.end_container = node;
        self.end_offset = length;

        Ok(())
    }

    /// Extract the contents of the range into a DocumentFragment
    ///
    /// This removes the contents from the document.
    pub fn extract_contents(&mut self) -> Result<dom_core::DocumentFragment, DomException> {
        // For now, return a simple implementation
        // A full implementation would need to handle partial text nodes
        let fragment = dom_core::DocumentFragment::new();

        if self.collapsed() {
            return Ok(fragment);
        }

        // Simplified: only handle same container for now
        if std::sync::Arc::ptr_eq(&self.start_container, &self.end_container) {
            let node_type = self.start_container.read().node_type();
            if let NodeType::Text = node_type {
                let text_content = self.start_container.read().text_content();
                if let Some(content) = text_content {
                    let extracted = content
                        .chars()
                        .skip(self.start_offset)
                        .take(self.end_offset - self.start_offset)
                        .collect::<String>();

                    // Create a text node with extracted content
                    let text_node = dom_core::Text::new(&extracted);
                    let text_ref = std::sync::Arc::new(parking_lot::RwLock::new(
                        Box::new(text_node) as Box<dyn dom_core::Node>
                    ));
                    fragment.append_child(text_ref)?;

                    // Remove extracted text from original node
                    let remaining = format!(
                        "{}{}",
                        content.chars().take(self.start_offset).collect::<String>(),
                        content.chars().skip(self.end_offset).collect::<String>()
                    );
                    self.start_container.write().set_text_content(remaining);

                    // Collapse to start
                    self.collapse(true);
                }
            }
        }

        Ok(fragment)
    }

    /// Delete the contents of the range
    pub fn delete_contents(&mut self) -> Result<(), DomException> {
        if self.collapsed() {
            return Ok(());
        }

        // Simplified: only handle same container text nodes
        if std::sync::Arc::ptr_eq(&self.start_container, &self.end_container) {
            let node_type = self.start_container.read().node_type();
            if let NodeType::Text = node_type {
                let text_content = self.start_container.read().text_content();
                if let Some(content) = text_content {
                    let remaining = format!(
                        "{}{}",
                        content.chars().take(self.start_offset).collect::<String>(),
                        content.chars().skip(self.end_offset).collect::<String>()
                    );
                    self.start_container.write().set_text_content(remaining);

                    // Collapse to start
                    self.collapse(true);
                }
            }
        }

        Ok(())
    }

    /// Clone the contents of the range into a DocumentFragment
    pub fn clone_contents(&self) -> Result<dom_core::DocumentFragment, DomException> {
        let mut fragment = dom_core::DocumentFragment::new();

        if self.collapsed() {
            return Ok(fragment);
        }

        // Simplified: only handle same container text nodes
        if std::sync::Arc::ptr_eq(&self.start_container, &self.end_container) {
            let node_type = self.start_container.read().node_type();
            if let NodeType::Text = node_type {
                let text_content = self.start_container.read().text_content();
                if let Some(content) = text_content {
                    let cloned = content
                        .chars()
                        .skip(self.start_offset)
                        .take(self.end_offset - self.start_offset)
                        .collect::<String>();

                    let text_node = dom_core::Text::new(&cloned);
                    let text_ref = std::sync::Arc::new(parking_lot::RwLock::new(
                        Box::new(text_node) as Box<dyn dom_core::Node>
                    ));
                    fragment.append_child(text_ref)?;
                }
            }
        }

        Ok(fragment)
    }

    // Helper methods

    fn validate_boundary_point(&self, node: &NodeRef, offset: usize) -> Result<(), DomException> {
        match node.read().node_type() {
            NodeType::DocumentType => {
                return Err(DomException::NotSupportedError);
            }
            _ => {}
        }

        let length = self.get_node_length(node);
        if offset > length {
            return Err(DomException::InvalidModificationError);
        }

        Ok(())
    }

    fn get_node_length(&self, node: &NodeRef) -> usize {
        let node_guard = node.read();
        match node_guard.node_type() {
            NodeType::Text | NodeType::Comment | NodeType::ProcessingInstruction => {
                node_guard.text_content().map(|s| s.len()).unwrap_or(0)
            }
            _ => node_guard.child_nodes().len(),
        }
    }

    fn compare_boundary_points(
        &self,
        a_node: &NodeRef,
        a_offset: usize,
        b_node: &NodeRef,
        b_offset: usize,
    ) -> Ordering {
        if a_node.ptr_eq(b_node) {
            return a_offset.cmp(&b_offset);
        }

        // Simplified: just compare by document order
        // A full implementation would need proper tree position comparison
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::{Document, Node, Text};
    use std::sync::Arc;
    use parking_lot::RwLock;

    fn create_text_node_ref(content: &str) -> NodeRef {
        let text = Text::new(content);
        Arc::new(RwLock::new(Box::new(text) as Box<dyn Node>))
    }

    #[test]
    fn test_range_creation() {
        let doc = Document::new();
        let range = Range::new(&doc);

        assert!(range.collapsed());
        assert_eq!(range.start_offset(), 0);
        assert_eq!(range.end_offset(), 0);
    }

    #[test]
    fn test_range_collapse() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");
        let mut range = Range::new(&doc);

        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref, 5).unwrap();

        assert!(!range.collapsed());

        range.collapse(true);
        assert!(range.collapsed());
        assert_eq!(range.start_offset(), 0);
        assert_eq!(range.end_offset(), 0);
    }

    #[test]
    fn test_range_extract_text() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");
        let mut range = Range::new(&doc);

        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref.clone(), 5).unwrap();

        let fragment = range.extract_contents().unwrap();
        assert_eq!(fragment.text_content(), Some("Hello".to_string()));

        // Original text should be modified
        assert_eq!(text_ref.read().text_content(), Some(" World".to_string()));

        // Range should be collapsed
        assert!(range.collapsed());
    }

    #[test]
    fn test_range_delete_contents() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");
        let mut range = Range::new(&doc);

        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref.clone(), 6).unwrap();

        range.delete_contents().unwrap();

        assert_eq!(text_ref.read().text_content(), Some("World".to_string()));
        assert!(range.collapsed());
    }

    #[test]
    fn test_range_clone_contents() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");
        let mut range = Range::new(&doc);

        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref.clone(), 5).unwrap();

        let fragment = range.clone_contents().unwrap();
        assert_eq!(fragment.text_content(), Some("Hello".to_string()));

        // Original text should be unchanged
        assert_eq!(text_ref.read().text_content(), Some("Hello World".to_string()));
    }
}
