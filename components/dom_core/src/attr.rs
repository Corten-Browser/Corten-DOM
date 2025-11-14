//! Attr (Attribute) node implementation

use crate::element::Element;
use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

/// Attribute node implementation
#[derive(Debug, Clone)]
pub struct Attr {
    /// Base node data
    node_data: NodeData,

    /// Attribute name (qualified name if namespaced)
    name: String,

    /// Attribute value
    value: String,

    /// Namespace URI (if namespaced attribute)
    namespace_uri: Option<String>,

    /// Namespace prefix (e.g., "xlink" in "xlink:href")
    prefix: Option<String>,

    /// Local name (e.g., "href" in "xlink:href")
    local_name: String,

    /// Owner element (weak reference to prevent cycles)
    owner_element: Option<Weak<RwLock<Element>>>,
}

/// Thread-safe reference to an Attr
pub type AttrRef = Arc<RwLock<Attr>>;

impl Attr {
    /// Creates a new attribute
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let local_name = name.clone();

        Self {
            node_data: NodeData::new(NodeType::Attribute, name.clone()),
            name,
            value: value.into(),
            namespace_uri: None,
            prefix: None,
            local_name,
            owner_element: None,
        }
    }

    /// Creates a new namespaced attribute
    ///
    /// # Arguments
    /// * `namespace_uri` - The namespace URI
    /// * `qualified_name` - The qualified name (e.g., "prefix:localName" or just "localName")
    /// * `value` - The attribute value
    ///
    /// # Errors
    /// Returns `DomException::InvalidCharacterError` if:
    /// - The qualified name contains invalid characters
    /// - The qualified name has multiple colons
    /// - The qualified name starts or ends with a colon
    pub fn new_ns(
        namespace_uri: impl Into<String>,
        qualified_name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DomException> {
        let qualified_name = qualified_name.into();
        let namespace_uri = namespace_uri.into();

        // Validate qualified name
        if !is_valid_qualified_name(&qualified_name) {
            return Err(DomException::InvalidCharacterError);
        }

        // Parse qualified name into prefix and local name
        let (prefix, local_name) = parse_qualified_name(&qualified_name);
        let local_name_string = local_name.to_string();

        Ok(Self {
            node_data: NodeData::new(NodeType::Attribute, qualified_name.clone()),
            name: qualified_name,
            value: value.into(),
            namespace_uri: Some(namespace_uri),
            prefix,
            local_name: local_name_string,
            owner_element: None,
        })
    }

    /// Gets the attribute name (qualified name if namespaced)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the attribute value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the attribute value
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }

    /// Gets the namespace URI
    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace_uri.as_deref()
    }

    /// Gets the prefix
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Gets the local name
    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    /// Gets the owner element
    pub fn owner_element(&self) -> Option<Arc<RwLock<Element>>> {
        self.owner_element.as_ref().and_then(|weak| weak.upgrade())
    }

    /// Sets the owner element (called by Element when attribute is added)
    pub(crate) fn set_owner_element(&mut self, element: Option<Weak<RwLock<Element>>>) {
        self.owner_element = element;
    }
}

impl Node for Attr {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn node_value(&self) -> Option<&str> {
        Some(&self.value)
    }

    fn set_node_value(&mut self, value: Option<String>) {
        if let Some(val) = value {
            self.value = val;
        } else {
            self.value.clear();
        }
    }

    fn text_content(&self) -> Option<String> {
        Some(self.value.clone())
    }

    fn set_text_content(&mut self, text: String) {
        self.value = text;
    }

    fn parent_node(&self) -> Option<NodeRef> {
        // Attributes are not part of the document tree
        // They're associated with elements but don't have a parent in the tree sense
        None
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        // Attributes cannot have children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Attributes cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // Attributes cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // Attributes cannot have children
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
        std::ptr::addr_eq(self_ptr, other_ptr)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Validates a qualified name according to XML naming rules
fn is_valid_qualified_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Check for invalid patterns
    if name.starts_with(':') || name.ends_with(':') {
        return false;
    }

    // Count colons - should be 0 or 1
    let colon_count = name.chars().filter(|&c| c == ':').count();
    if colon_count > 1 {
        return false;
    }

    // Check for whitespace
    if name.contains(char::is_whitespace) {
        return false;
    }

    true
}

/// Parses a qualified name into prefix and local name
///
/// Returns (prefix, local_name) tuple where prefix is None if no colon found
fn parse_qualified_name(qualified_name: &str) -> (Option<String>, &str) {
    if let Some(colon_pos) = qualified_name.find(':') {
        let prefix = &qualified_name[..colon_pos];
        let local_name = &qualified_name[colon_pos + 1..];
        (Some(prefix.to_string()), local_name)
    } else {
        (None, qualified_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_creation() {
        let attr = Attr::new("id", "main");
        assert_eq!(attr.name(), "id");
        assert_eq!(attr.value(), "main");
        assert_eq!(attr.local_name(), "id");
    }

    #[test]
    fn test_attr_value_modification() {
        let mut attr = Attr::new("class", "btn");
        assert_eq!(attr.value(), "btn");

        attr.set_value("btn-primary");
        assert_eq!(attr.value(), "btn-primary");
    }

    #[test]
    fn test_attr_namespace() {
        let attr = Attr::new_ns("http://www.w3.org/1999/xlink", "xlink:href", "#anchor").unwrap();

        assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/1999/xlink"));
        assert_eq!(attr.prefix(), Some("xlink"));
        assert_eq!(attr.local_name(), "href");
    }

    #[test]
    fn test_qualified_name_parsing() {
        let (prefix, local_name) = parse_qualified_name("xlink:href");
        assert_eq!(prefix, Some("xlink".to_string()));
        assert_eq!(local_name, "href");

        let (prefix, local_name) = parse_qualified_name("title");
        assert_eq!(prefix, None);
        assert_eq!(local_name, "title");
    }

    #[test]
    fn test_invalid_qualified_names() {
        assert!(!is_valid_qualified_name(""));
        assert!(!is_valid_qualified_name(":href"));
        assert!(!is_valid_qualified_name("xlink:"));
        assert!(!is_valid_qualified_name("a:b:c"));
        assert!(!is_valid_qualified_name("invalid name"));
    }

    #[test]
    fn test_valid_qualified_names() {
        assert!(is_valid_qualified_name("id"));
        assert!(is_valid_qualified_name("xlink:href"));
        assert!(is_valid_qualified_name("svg:width"));
        assert!(is_valid_qualified_name("data-value"));
    }

    #[test]
    fn test_attr_node_type() {
        let attr = Attr::new("test", "value");
        assert_eq!(attr.node_type(), NodeType::Attribute);
    }
}
