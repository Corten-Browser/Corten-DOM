//! Unit tests for Attr (Attribute) node implementation

use dom_core::node::{Node, NodeRef};
use dom_core::{Attr, Element};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_attr_basic_creation() {
    let attr = Attr::new("id", "main");
    assert_eq!(attr.name(), "id");
    assert_eq!(attr.value(), "main");
    assert_eq!(attr.local_name(), "id");
    assert!(attr.namespace_uri().is_none());
    assert!(attr.prefix().is_none());
}

#[test]
fn test_attr_value_setter() {
    let mut attr = Attr::new("class", "btn");
    assert_eq!(attr.value(), "btn");

    attr.set_value("btn-primary");
    assert_eq!(attr.value(), "btn-primary");

    attr.set_value("");
    assert_eq!(attr.value(), "");
}

#[test]
fn test_attr_empty_value() {
    let attr = Attr::new("disabled", "");
    assert_eq!(attr.name(), "disabled");
    assert_eq!(attr.value(), "");
}

#[test]
fn test_attr_node_trait_implementation() {
    let attr = Attr::new("href", "https://example.com");

    assert_eq!(attr.node_type(), NodeType::Attribute);
    assert_eq!(attr.node_name(), "href");
    assert_eq!(attr.node_value(), Some("https://example.com"));
}

#[test]
fn test_attr_node_value_setter() {
    let mut attr = Attr::new("title", "Original");
    assert_eq!(attr.node_value(), Some("Original"));

    attr.set_node_value(Some("Modified".to_string()));
    assert_eq!(attr.node_value(), Some("Modified"));
    assert_eq!(attr.value(), "Modified");
}

#[test]
fn test_attr_text_content() {
    let attr = Attr::new("data-id", "12345");

    // Attributes should return value as text content
    assert_eq!(attr.text_content(), Some("12345".to_string()));
}

#[test]
fn test_attr_no_children() {
    let attr = Attr::new("type", "text");

    // Attributes cannot have children
    assert_eq!(attr.child_nodes().len(), 0);
    assert!(attr.first_child().is_none());
    assert!(attr.last_child().is_none());
}

#[test]
fn test_attr_no_parent() {
    let attr = Attr::new("name", "username");

    // Attributes without owner element have no parent in tree
    assert!(attr.parent_node().is_none());
}

#[test]
fn test_attr_namespace_creation() {
    let attr = Attr::new_ns("http://www.w3.org/1999/xlink", "xlink:href", "#anchor").unwrap();

    assert_eq!(attr.name(), "xlink:href");
    assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/1999/xlink"));
    assert_eq!(attr.prefix(), Some("xlink"));
    assert_eq!(attr.local_name(), "href");
    assert_eq!(attr.value(), "#anchor");
}

#[test]
fn test_attr_namespace_without_prefix() {
    let attr = Attr::new_ns("http://www.w3.org/1999/xhtml", "title", "Document Title").unwrap();

    assert_eq!(attr.name(), "title");
    assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/1999/xhtml"));
    assert!(attr.prefix().is_none());
    assert_eq!(attr.local_name(), "title");
}

#[test]
fn test_attr_invalid_qualified_name() {
    // Invalid qualified name (prefix without colon)
    let result = Attr::new_ns("http://example.com", "invalid name", "value");
    assert!(result.is_err());
}

#[test]
fn test_attr_multiple_colons_in_qualified_name() {
    // Multiple colons should be invalid
    let result = Attr::new_ns("http://example.com", "pre:fix:name", "value");
    assert!(result.is_err());
}

#[test]
fn test_attr_owner_element_initially_none() {
    let attr = Attr::new("id", "test");
    assert!(attr.owner_element().is_none());
}

// Note: owner_element tracking is tested indirectly through Element's
// attribute management methods (get_attribute_node, set_attribute_node).
// The set_owner_element method is pub(crate) and called internally by Element.

#[test]
fn test_attr_clone_node() {
    let attr = Attr::new("data-value", "42");
    let attr_ref = Arc::new(RwLock::new(Box::new(attr) as Box<dyn Node>));

    let cloned = attr_ref.read().clone_node(false);

    assert_eq!(cloned.read().node_type(), NodeType::Attribute);
    assert_eq!(cloned.read().node_name(), "data-value");
    assert_eq!(cloned.read().node_value(), Some("42"));
}

#[test]
fn test_attr_cannot_append_children() {
    let mut attr = Attr::new("id", "test");
    let child_attr = Attr::new("child", "value");
    let child_ref = Arc::new(RwLock::new(Box::new(child_attr) as Box<dyn Node>));

    // Attributes cannot have children
    let result = attr.append_child(child_ref);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomException::HierarchyRequestError
    ));
}

#[test]
fn test_attr_case_sensitivity() {
    let attr = Attr::new("DataID", "123");

    // Attribute names should preserve case
    assert_eq!(attr.name(), "DataID");
}

#[test]
fn test_attr_special_characters_in_value() {
    let attr = Attr::new("onclick", "alert('Hello, \"World\"!')");
    assert_eq!(attr.value(), "alert('Hello, \"World\"!')");
}

#[test]
fn test_attr_svg_namespace() {
    let attr = Attr::new_ns("http://www.w3.org/2000/svg", "svg:width", "100").unwrap();

    assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/2000/svg"));
    assert_eq!(attr.prefix(), Some("svg"));
    assert_eq!(attr.local_name(), "width");
}

#[test]
fn test_attr_as_any_downcasting() {
    let attr = Attr::new("test", "value");
    let any = attr.as_any();

    // Should be able to downcast to Attr
    assert!(any.downcast_ref::<Attr>().is_some());
}
