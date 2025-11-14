//! ProcessingInstruction node implementation

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// ProcessingInstruction node implementation
///
/// Represents processing instructions in XML documents like:
/// - `<?xml version="1.0" encoding="UTF-8"?>`
/// - `<?xml-stylesheet type="text/css" href="style.css"?>`
/// - `<?php echo "Hello World";?>`
///
/// Processing instructions have a target (e.g., "xml", "xml-stylesheet", "php")
/// and data (e.g., "version='1.0'", "type='text/css' href='style.css'").
///
/// # Examples
///
/// ```
/// use dom_core::ProcessingInstruction;
///
/// let pi = ProcessingInstruction::new("xml-stylesheet", "type='text/css' href='style.css'");
/// assert_eq!(pi.target(), "xml-stylesheet");
/// assert_eq!(pi.data(), "type='text/css' href='style.css'");
/// ```
#[derive(Debug, Clone)]
pub struct ProcessingInstruction {
    /// Base node data
    node_data: NodeData,

    /// Processing instruction target (e.g., "xml-stylesheet")
    target: String,

    /// Processing instruction data (e.g., "type='text/css' href='style.css'")
    data: String,
}

/// Thread-safe reference to a ProcessingInstruction
pub type ProcessingInstructionRef = Arc<RwLock<ProcessingInstruction>>;

impl ProcessingInstruction {
    /// Creates a new processing instruction
    ///
    /// # Arguments
    ///
    /// * `target` - The processing instruction target (e.g., "xml", "xml-stylesheet")
    /// * `data` - The processing instruction data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::ProcessingInstruction;
    ///
    /// let pi = ProcessingInstruction::new("xml", "version='1.0' encoding='UTF-8'");
    /// ```
    pub fn new(target: impl Into<String>, data: impl Into<String>) -> Self {
        let target_string = target.into();
        Self {
            node_data: NodeData::new(NodeType::ProcessingInstruction, &target_string),
            target: target_string,
            data: data.into(),
        }
    }

    /// Gets the processing instruction target
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::ProcessingInstruction;
    ///
    /// let pi = ProcessingInstruction::new("xml-stylesheet", "href='style.css'");
    /// assert_eq!(pi.target(), "xml-stylesheet");
    /// ```
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Gets the processing instruction data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::ProcessingInstruction;
    ///
    /// let pi = ProcessingInstruction::new("xml", "version='1.0'");
    /// assert_eq!(pi.data(), "version='1.0'");
    /// ```
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Sets the processing instruction data
    ///
    /// # Arguments
    ///
    /// * `data` - The new processing instruction data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::ProcessingInstruction;
    ///
    /// let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    /// pi.set_data("version='2.0'");
    /// assert_eq!(pi.data(), "version='2.0'");
    /// ```
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }
}

impl Node for ProcessingInstruction {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        // Node name for ProcessingInstruction is the target
        &self.target
    }

    fn node_value(&self) -> Option<&str> {
        // Node value for ProcessingInstruction is the data
        Some(&self.data)
    }

    fn set_node_value(&mut self, value: Option<String>) {
        // Setting node value updates the data
        if let Some(val) = value {
            self.data = val;
        } else {
            self.data.clear();
        }
    }

    fn text_content(&self) -> Option<String> {
        // Text content for ProcessingInstruction is the data
        Some(self.data.clone())
    }

    fn set_text_content(&mut self, text: String) {
        // Setting text content updates the data
        self.data = text;
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        // ProcessingInstruction nodes have no children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // ProcessingInstruction nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // ProcessingInstruction nodes cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // ProcessingInstruction nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn clone_node(&self, _deep: bool) -> NodeRef {
        // Clone this processing instruction
        // The `deep` parameter is ignored as ProcessingInstruction has no children
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
        // ProcessingInstruction only contains itself
        let self_ptr = self as *const _ as *const dyn Node;
        let other_ptr = other as *const dyn Node;
        std::ptr::addr_eq(self_ptr, other_ptr)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_instruction_creation() {
        let pi = ProcessingInstruction::new("xml-stylesheet", "type='text/css' href='style.css'");
        assert_eq!(pi.target(), "xml-stylesheet");
        assert_eq!(pi.data(), "type='text/css' href='style.css'");
        assert_eq!(pi.node_type(), NodeType::ProcessingInstruction);
        assert_eq!(pi.node_name(), "xml-stylesheet");
    }

    #[test]
    fn test_processing_instruction_modification() {
        let mut pi = ProcessingInstruction::new("xml", "version='1.0'");

        pi.set_data("version='2.0'");
        assert_eq!(pi.data(), "version='2.0'");
    }

    #[test]
    fn test_processing_instruction_no_children() {
        let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
        let child = ProcessingInstruction::new("child", "data");
        let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

        let result = pi.append_child(child_ref);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, DomException::HierarchyRequestError));
        }
    }

    #[test]
    fn test_xml_declaration() {
        let pi = ProcessingInstruction::new("xml", "version=\"1.0\" encoding=\"UTF-8\"");
        assert_eq!(pi.target(), "xml");
        assert_eq!(pi.data(), "version=\"1.0\" encoding=\"UTF-8\"");
    }
}
