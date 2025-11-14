//! Text node implementation

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// Text node containing character data
#[derive(Clone)]
pub struct Text {
    /// Base node data
    node_data: NodeData,

    /// Text content
    data: String,
}

impl Text {
    /// Creates a new text node
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            node_data: NodeData::new(NodeType::Text, "#text"),
            data: data.into(),
        }
    }

    /// Gets the text data
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Sets the text data
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Gets the length of the text
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Appends data to the text
    pub fn append_data(&mut self, data: &str) {
        self.data.push_str(data);
    }

    /// Inserts data at an offset
    pub fn insert_data(&mut self, offset: usize, data: &str) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        self.data.insert_str(offset, data);
        Ok(())
    }

    /// Deletes data
    pub fn delete_data(&mut self, offset: usize, count: usize) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        self.data.replace_range(offset..end, "");
        Ok(())
    }

    /// Replaces data
    pub fn replace_data(
        &mut self,
        offset: usize,
        count: usize,
        data: &str,
    ) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        self.data.replace_range(offset..end, data);
        Ok(())
    }

    /// Extracts a substring
    pub fn substring_data(&self, offset: usize, count: usize) -> Result<String, DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        Ok(self.data[offset..end].to_string())
    }
}

impl Node for Text {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn node_value(&self) -> Option<&str> {
        Some(&self.data)
    }

    fn set_node_value(&mut self, value: Option<String>) {
        if let Some(val) = value {
            self.data = val;
        } else {
            self.data.clear();
        }
    }

    fn text_content(&self) -> Option<String> {
        Some(self.data.clone())
    }

    fn set_text_content(&mut self, text: String) {
        self.data = text;
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        // Text nodes have no children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Text nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Text nodes cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // Text nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn clone_node(&self, _deep: bool) -> NodeRef {
        let cloned = self.clone();
        Arc::new(RwLock::new(Box::new(cloned) as Box<dyn Node>))
    }

    fn node_data(&self) -> &NodeData {
        &self.node_data
    }

    fn node_data_mut(&mut self) -> &mut NodeData {
        &mut self.node_data
    }

    fn contains(&self, other: &dyn Node) -> bool {
        let self_ptr = self as *const _ as *const dyn Node;
        let other_ptr = other as *const dyn Node;
        self_ptr == other_ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello, world!");
        assert_eq!(text.data(), "Hello, world!");
        assert_eq!(text.node_type(), NodeType::Text);
        assert_eq!(text.node_name(), "#text");
    }

    #[test]
    fn test_text_modification() {
        let mut text = Text::new("Hello");

        text.append_data(", world!");
        assert_eq!(text.data(), "Hello, world!");

        text.set_data("New text");
        assert_eq!(text.data(), "New text");
    }

    #[test]
    fn test_text_operations() {
        let mut text = Text::new("Hello, world!");

        // Insert
        text.insert_data(7, "beautiful ").unwrap();
        assert_eq!(text.data(), "Hello, beautiful world!");

        // Delete
        text.delete_data(7, 10).unwrap();
        assert_eq!(text.data(), "Hello, world!");

        // Replace
        text.replace_data(0, 5, "Hi").unwrap();
        assert_eq!(text.data(), "Hi, world!");
    }

    #[test]
    fn test_substring() {
        let text = Text::new("Hello, world!");

        let substr = text.substring_data(0, 5).unwrap();
        assert_eq!(substr, "Hello");

        let substr = text.substring_data(7, 5).unwrap();
        assert_eq!(substr, "world");
    }

    #[test]
    fn test_text_no_children() {
        let mut text = Text::new("Hello");
        let child = Text::new("child");
        let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

        let result = text.append_child(child_ref);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, DomException::HierarchyRequestError));
        }
    }
}
