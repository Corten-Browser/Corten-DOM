//! DocumentFragment implementation

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// A DocumentFragment is a minimal document object that has no parent
#[derive(Clone)]
pub struct DocumentFragment {
    node_data: NodeData,
    children: Vec<NodeRef>,
}

impl DocumentFragment {
    /// Create a new DocumentFragment
    pub fn new() -> Self {
        Self {
            node_data: NodeData::new(NodeType::DocumentFragment, "#document-fragment"),
            children: Vec::new(),
        }
    }

    /// Get the children
    pub fn children(&self) -> &[NodeRef] {
        &self.children
    }

    /// Append a child node
    pub fn append_child(&mut self, child: NodeRef) -> Result<(), DomException> {
        if !self.children.iter().any(|c| Arc::ptr_eq(c, &child)) {
            self.children.push(child);
        }
        Ok(())
    }

    /// Get text content of all child nodes
    pub fn text_content(&self) -> Option<String> {
        if self.children.is_empty() {
            return None;
        }

        let content: String = self
            .children
            .iter()
            .filter_map(|child| child.read().text_content())
            .collect::<Vec<_>>()
            .join("");

        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}

impl Default for DocumentFragment {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for DocumentFragment {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn node_value(&self) -> Option<&str> {
        None
    }

    fn set_node_value(&mut self, _value: Option<String>) {
        // DocumentFragment has no value
    }

    fn text_content(&self) -> Option<String> {
        Self::text_content(self)
    }

    fn set_text_content(&mut self, _text: String) {
        // Clear children and replace with text node would go here
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        self.children.clone()
    }

    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        Self::append_child(self, child.clone())?;
        Ok(child)
    }

    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        let index = self
            .children
            .iter()
            .position(|c| Arc::ptr_eq(c, &child))
            .ok_or(DomException::NotFoundError)?;

        self.children.remove(index);
        Ok(child)
    }

    fn insert_before(
        &mut self,
        new_child: NodeRef,
        ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        if let Some(ref_node) = ref_child {
            let index = self
                .children
                .iter()
                .position(|c| Arc::ptr_eq(c, &ref_node))
                .ok_or(DomException::NotFoundError)?;

            self.children.insert(index, new_child.clone());
        } else {
            self.children.push(new_child.clone());
        }

        Ok(new_child)
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
        std::ptr::addr_eq(self_ptr, other_ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Text;

    #[test]
    fn test_fragment_creation() {
        let fragment = DocumentFragment::new();
        assert_eq!(fragment.node_type(), NodeType::DocumentFragment);
        assert_eq!(fragment.node_name(), "#document-fragment");
    }

    #[test]
    fn test_fragment_append_child() {
        let mut fragment = DocumentFragment::new();
        let text = Text::new("Hello");
        let text_ref = Arc::new(RwLock::new(Box::new(text) as Box<dyn Node>));

        fragment.append_child(text_ref.clone()).unwrap();

        assert_eq!(fragment.children().len(), 1);
    }

    #[test]
    fn test_fragment_text_content() {
        let mut fragment = DocumentFragment::new();
        let text1 = Text::new("Hello");
        let text2 = Text::new(" World");

        let text1_ref = Arc::new(RwLock::new(Box::new(text1) as Box<dyn Node>));
        let text2_ref = Arc::new(RwLock::new(Box::new(text2) as Box<dyn Node>));

        fragment.append_child(text1_ref).unwrap();
        fragment.append_child(text2_ref).unwrap();

        assert_eq!(fragment.text_content(), Some("Hello World".to_string()));
    }
}
