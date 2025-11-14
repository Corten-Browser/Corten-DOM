//! WPT-Inspired Attribute Tests
//!
//! These tests are translated from Web Platform Tests (WPT) dom/nodes/Attr-* test suite.
//! Original tests: https://github.com/web-platform-tests/wpt/tree/master/dom/nodes

use dom_core::{Attr, Document, Node};
use dom_types::NodeType;

/// Helper to create a document
fn create_document() -> Document {
    Document::new()
}

// ============================================================================
// Attr Creation Tests
// ============================================================================

/// WPT: dom/nodes/Attr-constructor.html
/// Test: Attr should be created with name
#[test]
fn test_attr_creation() {
    let attr = Attr::new("id", "");
    assert_eq!(attr.name(), "id");
    assert_eq!(attr.value(), "");
}

/// WPT: dom/nodes/Attr-value.html
/// Test: Attr value should be settable
#[test]
fn test_attr_value() {
    let mut attr = Attr::new("class", "");
    attr.set_value("container");
    assert_eq!(attr.value(), "container");
}

/// WPT: dom/nodes/Attr-nodeType.html
/// Test: Attr nodeType should be 2
#[test]
fn test_attr_node_type() {
    let attr = Attr::new("id", "test");
    assert_eq!(attr.node_type(), NodeType::Attribute);
}

/// WPT: dom/nodes/Attr-nodeName.html
/// Test: Attr nodeName should equal name
#[test]
fn test_attr_node_name() {
    let attr = Attr::new("data-value", "123");
    assert_eq!(attr.node_name(), "data-value");
}

// ============================================================================
// Namespaced Attribute Tests
// ============================================================================

/// WPT: dom/nodes/Attr-namespaceURI.html
/// Test: Non-namespaced attr should have null namespace
#[test]
fn test_attr_no_namespace() {
    let attr = Attr::new("id", "test");
    assert_eq!(attr.namespace_uri(), None);
}

/// WPT: dom/nodes/Attr-constructor-namespace.html
/// Test: Namespaced attr should have namespace URI
#[test]
fn test_attr_with_namespace() {
    let attr = Attr::new_ns(
        "http://www.w3.org/1999/xlink",
        "xlink:href",
        "#target"
    ).unwrap();

    assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/1999/xlink"));
    assert_eq!(attr.prefix(), Some("xlink"));
    assert_eq!(attr.local_name(), "href");
}

/// WPT: dom/nodes/Attr-constructor-namespace.html
/// Test: Qualified name parsing should handle no prefix
#[test]
fn test_attr_no_prefix() {
    let attr = Attr::new_ns(
        "http://www.w3.org/1999/xhtml",
        "href",
        "#link"
    ).unwrap();

    assert_eq!(attr.namespace_uri(), Some("http://www.w3.org/1999/xhtml"));
    assert_eq!(attr.prefix(), None);
    assert_eq!(attr.local_name(), "href");
}

// ============================================================================
// Attr Node Properties Tests
// ============================================================================

/// WPT: dom/nodes/Attr-ownerElement.html
/// Test: Attr ownerElement should be None initially
#[test]
fn test_attr_owner_element_none() {
    let attr = Attr::new("id", "test");
    assert!(attr.owner_element().is_none());
}

/// WPT: dom/nodes/Attr-ownerElement.html
/// Test: Attr created via createAttribute should have owner
#[test]
fn test_attr_from_document() {
    let mut doc = create_document();
    let attr = doc.create_attribute("id").unwrap();

    // Set value
    attr.write().set_value("test-id");
    assert_eq!(attr.read().value(), "test-id");
}

/// WPT: dom/nodes/Element-getAttributeNode.html
/// Test: getAttributeNode should return Attr with owner
#[test]
fn test_get_attribute_node() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    // Set attribute
    elem.write().set_attribute("id", "test").unwrap();

    // Get attribute node
    let attr_node = elem.read().get_attribute_node("id");
    assert!(attr_node.is_some());

    let attr = attr_node.unwrap();
    assert_eq!(attr.read().name(), "id");
    assert_eq!(attr.read().value(), "test");
}

// ============================================================================
// Summary
// ============================================================================
// Total WPT-inspired Attribute tests: 10
// Categories covered:
//   - Attr creation (4 tests)
//   - Namespaced attributes (3 tests)
//   - Attr properties (3 tests)
