//! WPT-Inspired Element Interface Tests
//!
//! These tests are translated from Web Platform Tests (WPT) dom/nodes/Element-* test suite.
//! Original tests: https://github.com/web-platform-tests/wpt/tree/master/dom/nodes

use dom_core::Document;

/// Helper to create a document
fn create_document() -> Document {
    Document::new()
}

// ============================================================================
// Element Creation Tests
// ============================================================================

/// WPT: dom/nodes/Document-createElement.html
/// Test: createElement should create element with correct tag name
#[test]
fn test_create_element_tag_name() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();
    assert_eq!(elem.read().tag_name(), "DIV");
}

/// WPT: dom/nodes/Document-createElement.html
/// Test: createElement should uppercase tag names
#[test]
fn test_create_element_uppercase() {
    let mut doc = create_document();
    let elem = doc.create_element("myElement").unwrap();
    assert_eq!(elem.read().tag_name(), "MYELEMENT");
}

/// WPT: dom/nodes/Document-createElement.html
/// Test: createElement with invalid characters should fail
#[test]
fn test_create_element_invalid_name() {
    let mut doc = create_document();
    let result = doc.create_element("invalid name");
    assert!(result.is_err());
}

/// WPT: dom/nodes/Document-createElementNS.html
/// Test: createElementNS should create namespaced element
#[test]
fn test_create_element_ns() {
    let mut doc = create_document();
    let elem = doc.create_element_ns("http://www.w3.org/1999/xhtml", "div").unwrap();
    assert_eq!(elem.read().tag_name(), "DIV");
    assert_eq!(elem.read().namespace_uri(), Some("http://www.w3.org/1999/xhtml"));
}

// ============================================================================
// Attribute Tests
// ============================================================================

/// WPT: dom/nodes/Element-getAttribute.html
/// Test: getAttribute should return null for non-existent attribute
#[test]
fn test_get_attribute_non_existent() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();
    assert_eq!(elem.read().get_attribute("foo"), None);
}

/// WPT: dom/nodes/Element-setAttribute.html
/// Test: setAttribute should set attribute value
#[test]
fn test_set_attribute_basic() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("foo", "bar").unwrap();
    assert_eq!(elem.read().get_attribute("foo"), Some("bar"));
}

/// WPT: dom/nodes/Element-setAttribute.html
/// Test: setAttribute should update existing attribute
#[test]
fn test_set_attribute_update() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("foo", "bar").unwrap();
    elem.write().set_attribute("foo", "baz").unwrap();
    assert_eq!(elem.read().get_attribute("foo"), Some("baz"));
}

/// WPT: dom/nodes/Element-hasAttribute.html
/// Test: hasAttribute should return false for non-existent attribute
#[test]
fn test_has_attribute_false() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();
    assert!(!elem.read().has_attribute("foo"));
}

/// WPT: dom/nodes/Element-hasAttribute.html
/// Test: hasAttribute should return true after setAttribute
#[test]
fn test_has_attribute_true() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("foo", "bar").unwrap();
    assert!(elem.read().has_attribute("foo"));
}

/// WPT: dom/nodes/Element-removeAttribute.html
/// Test: removeAttribute should remove attribute
#[test]
fn test_remove_attribute() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("foo", "bar").unwrap();
    assert!(elem.read().has_attribute("foo"));

    elem.write().remove_attribute("foo").unwrap();
    assert!(!elem.read().has_attribute("foo"));
}

/// WPT: dom/nodes/Element-setAttribute.html
/// Test: setAttribute with invalid name should fail
#[test]
fn test_set_attribute_invalid_name() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    let result = elem.write().set_attribute("invalid name", "value");
    assert!(result.is_err());
}

// ============================================================================
// Element Properties Tests
// ============================================================================

/// WPT: dom/nodes/Element-tagName.html
/// Test: tagName should be uppercase
#[test]
fn test_tag_name_uppercase() {
    let mut doc = create_document();
    let elem = doc.create_element("customElement").unwrap();
    assert_eq!(elem.read().tag_name(), "CUSTOMELEMENT");
}

/// WPT: dom/nodes/Element-id.html
/// Test: id attribute should be tracked
#[test]
fn test_element_id() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("id", "test-id").unwrap();
    assert_eq!(elem.read().id(), Some("test-id"));
}

/// WPT: dom/nodes/Element-attributes.html
/// Test: attributes should preserve insertion order
#[test]
fn test_attributes_order() {
    let mut doc = create_document();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("z", "1").unwrap();
    elem.write().set_attribute("a", "2").unwrap();
    elem.write().set_attribute("m", "3").unwrap();

    let elem_read = elem.read();
    let attrs = elem_read.attributes();
    let keys: Vec<&String> = attrs.keys().collect();
    assert_eq!(keys, vec!["z", "a", "m"]);
}

// ============================================================================
// Summary
// ============================================================================
// Total WPT-inspired Element tests: 15
// Categories covered:
//   - Element creation (4 tests)
//   - Attribute operations (7 tests)
//   - Element properties (4 tests)
