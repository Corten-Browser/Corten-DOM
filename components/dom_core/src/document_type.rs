//! DocumentType node implementation
//!
//! This module provides the `DocumentType` struct which represents a document type
//! declaration (DOCTYPE) in XML and HTML documents.
//!
//! # Examples
//!
//! ```
//! use dom_core::DocumentType;
//!
//! // HTML5 doctype
//! let html5 = DocumentType::new_simple("html");
//! assert_eq!(html5.name(), "html");
//! assert_eq!(html5.public_id(), "");
//!
//! // XHTML 1.0 Strict doctype
//! let xhtml = DocumentType::new(
//!     "html",
//!     "-//W3C//DTD XHTML 1.0 Strict//EN",
//!     "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd",
//! );
//! assert_eq!(xhtml.name(), "html");
//! assert_eq!(xhtml.public_id(), "-//W3C//DTD XHTML 1.0 Strict//EN");
//! ```

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// DocumentType node implementation.
///
/// Represents a DOCTYPE declaration in XML/HTML documents.
/// Examples:
/// - `<!DOCTYPE html>` (HTML5)
/// - `<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "...">` (XHTML)
/// - `<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "...">` (SVG)
///
/// DocumentType nodes cannot have children according to the DOM specification.
#[derive(Debug, Clone)]
pub struct DocumentType {
    /// Base node data
    node_data: NodeData,

    /// Document type name (e.g., "html", "svg")
    name: String,

    /// Public identifier (e.g., "-//W3C//DTD XHTML 1.0 Strict//EN")
    public_id: String,

    /// System identifier (e.g., "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd")
    system_id: String,
}

/// Thread-safe reference to a DocumentType
pub type DocumentTypeRef = Arc<RwLock<DocumentType>>;

impl DocumentType {
    /// Creates a new document type with name, public ID, and system ID.
    ///
    /// # Arguments
    ///
    /// * `name` - The document type name (e.g., "html", "svg")
    /// * `public_id` - The public identifier (empty string if not used)
    /// * `system_id` - The system identifier (empty string if not used)
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::DocumentType;
    ///
    /// let doctype = DocumentType::new(
    ///     "html",
    ///     "-//W3C//DTD XHTML 1.0 Strict//EN",
    ///     "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd",
    /// );
    /// ```
    pub fn new(
        name: impl Into<String>,
        public_id: impl Into<String>,
        system_id: impl Into<String>,
    ) -> Self {
        let name = name.into();
        Self {
            node_data: NodeData::new(NodeType::DocumentType, name.clone()),
            name,
            public_id: public_id.into(),
            system_id: system_id.into(),
        }
    }

    /// Creates a simple document type (HTML5 style) with only a name.
    ///
    /// This is equivalent to `<!DOCTYPE name>`.
    ///
    /// # Arguments
    ///
    /// * `name` - The document type name (e.g., "html")
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::DocumentType;
    ///
    /// let html5 = DocumentType::new_simple("html");
    /// assert_eq!(html5.name(), "html");
    /// assert_eq!(html5.public_id(), "");
    /// assert_eq!(html5.system_id(), "");
    /// ```
    pub fn new_simple(name: impl Into<String>) -> Self {
        Self::new(name, "", "")
    }

    /// Gets the document type name.
    ///
    /// # Returns
    ///
    /// The name of the document type (e.g., "html", "svg").
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::DocumentType;
    ///
    /// let doctype = DocumentType::new_simple("html");
    /// assert_eq!(doctype.name(), "html");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the public identifier.
    ///
    /// # Returns
    ///
    /// The public ID string, or an empty string if not specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::DocumentType;
    ///
    /// let doctype = DocumentType::new(
    ///     "html",
    ///     "-//W3C//DTD XHTML 1.0 Strict//EN",
    ///     "",
    /// );
    /// assert_eq!(doctype.public_id(), "-//W3C//DTD XHTML 1.0 Strict//EN");
    /// ```
    pub fn public_id(&self) -> &str {
        &self.public_id
    }

    /// Gets the system identifier.
    ///
    /// # Returns
    ///
    /// The system ID string, or an empty string if not specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::DocumentType;
    ///
    /// let doctype = DocumentType::new(
    ///     "html",
    ///     "",
    ///     "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd",
    /// );
    /// assert_eq!(doctype.system_id(), "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd");
    /// ```
    pub fn system_id(&self) -> &str {
        &self.system_id
    }
}

impl Node for DocumentType {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn node_value(&self) -> Option<&str> {
        // DocumentType nodes have no value
        None
    }

    fn set_node_value(&mut self, _value: Option<String>) {
        // DocumentType nodes cannot have their value set
        // This is a no-op per DOM specification
    }

    fn text_content(&self) -> Option<String> {
        // DocumentType nodes have no text content
        None
    }

    fn set_text_content(&mut self, _text: String) {
        // DocumentType nodes cannot have text content
        // This is a no-op per DOM specification
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        // DocumentType nodes have no children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // DocumentType nodes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // DocumentType nodes cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // DocumentType nodes cannot have children
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
        // DocumentType nodes cannot contain other nodes
        // Check only if it's the same node
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
    fn test_document_type_creation() {
        let doctype = DocumentType::new("html", "-//W3C//DTD XHTML 1.0//EN", "xhtml1.dtd");
        assert_eq!(doctype.name(), "html");
        assert_eq!(doctype.public_id(), "-//W3C//DTD XHTML 1.0//EN");
        assert_eq!(doctype.system_id(), "xhtml1.dtd");
        assert_eq!(doctype.node_type(), NodeType::DocumentType);
    }

    #[test]
    fn test_document_type_simple() {
        let doctype = DocumentType::new_simple("html");
        assert_eq!(doctype.name(), "html");
        assert_eq!(doctype.public_id(), "");
        assert_eq!(doctype.system_id(), "");
    }

    #[test]
    fn test_document_type_no_value() {
        let doctype = DocumentType::new_simple("html");
        assert_eq!(doctype.node_value(), None);
        assert_eq!(doctype.text_content(), None);
    }

    #[test]
    fn test_document_type_no_children() {
        let mut doctype = DocumentType::new_simple("html");
        let child = DocumentType::new_simple("child");
        let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

        let result = doctype.append_child(child_ref);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomException::HierarchyRequestError
        ));
    }
}
