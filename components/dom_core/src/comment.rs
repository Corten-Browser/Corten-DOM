//! Comment node implementation

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// Comment node
#[derive(Clone)]
pub struct Comment {
    /// Base node data
    node_data: NodeData,

    /// Comment text
    data: String,
}

impl Comment {
    /// Creates a new comment node
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            node_data: NodeData::new(NodeType::Comment, "#comment"),
            data: data.into(),
        }
    }

    /// Gets the comment data
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Sets the comment data
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Gets the length of the comment
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

impl Node for Comment {
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
        // Comment nodes have no children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Comment nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Comment nodes cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // Comment nodes cannot have children
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
    fn test_comment_creation() {
        let comment = Comment::new("This is a comment");
        assert_eq!(comment.data(), "This is a comment");
        assert_eq!(comment.node_type(), NodeType::Comment);
        assert_eq!(comment.node_name(), "#comment");
    }

    #[test]
    fn test_comment_modification() {
        let mut comment = Comment::new("Initial comment");

        comment.set_data("Updated comment");
        assert_eq!(comment.data(), "Updated comment");
    }

    #[test]
    fn test_comment_no_children() {
        let mut comment = Comment::new("test");
        let child = Comment::new("child");
        let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

        let result = comment.append_child(child_ref);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, DomException::HierarchyRequestError));
        }
    }
}
