//! Tests for Element attribute node methods

use dom_core::attr::Attr;
use dom_core::element::{Element, ElementRef};
use dom_types::DomException;
use parking_lot::RwLock;
use std::sync::Arc;

/// Helper function to create an element with proper self-reference
fn create_element_with_ref(tag_name: &str) -> ElementRef {
    let element = Element::new(tag_name);
    let element_ref = Arc::new(RwLock::new(element));
    element_ref
        .write()
        .set_self_ref(Arc::downgrade(&element_ref));
    element_ref
}

#[test]
fn test_get_attribute_node_nonexistent() {
    let element_ref = create_element_with_ref("div");
    let element = element_ref.read();
    let attr = element.get_attribute_node("id");
    assert!(attr.is_none());
}

#[test]
fn test_get_attribute_node_existing() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("id", "main").unwrap();

    let element = element_ref.read();
    let attr = element.get_attribute_node("id");
    assert!(attr.is_some());
}

#[test]
fn test_get_attribute_node_correct_name_value() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("class", "button").unwrap();

    let element = element_ref.read();
    let attr = element.get_attribute_node("class").unwrap();
    let attr_lock = attr.read();
    assert_eq!(attr_lock.name(), "class");
    assert_eq!(attr_lock.value(), "button");
}

#[test]
fn test_get_attribute_node_owner_element_set() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("title", "Test").unwrap();

    let element = element_ref.read();
    let attr = element.get_attribute_node("title").unwrap();
    let attr_lock = attr.read();

    // Check that owner element is set
    let owner = attr_lock.owner_element();
    assert!(owner.is_some());

    // Verify the owner element is the correct one
    let owner_elem = owner.unwrap();
    let owner_elem_lock = owner_elem.read();
    assert_eq!(owner_elem_lock.tag_name(), "DIV");
}

#[test]
fn test_set_attribute_node_adds_new() {
    let element_ref = create_element_with_ref("div");

    let attr = Arc::new(RwLock::new(Attr::new("id", "test")));
    let result = element_ref.write().set_attribute_node(attr.clone());

    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // No old attribute

    // Verify attribute was set
    assert_eq!(element_ref.read().get_attribute("id"), Some("test"));
}

#[test]
fn test_set_attribute_node_replaces_existing() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("class", "old").unwrap();

    let new_attr = Arc::new(RwLock::new(Attr::new("class", "new")));
    let result = element_ref.write().set_attribute_node(new_attr.clone());

    assert!(result.is_ok());
    assert_eq!(element_ref.read().get_attribute("class"), Some("new"));
}

#[test]
fn test_set_attribute_node_returns_old_attr() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("data-value", "original").unwrap();

    let new_attr = Arc::new(RwLock::new(Attr::new("data-value", "updated")));
    let result = element_ref.write().set_attribute_node(new_attr.clone());

    assert!(result.is_ok());
    let old_attr = result.unwrap();
    assert!(old_attr.is_some());

    let old_attr_ref = old_attr.unwrap();
    let old_attr_lock = old_attr_ref.read();
    assert_eq!(old_attr_lock.name(), "data-value");
    assert_eq!(old_attr_lock.value(), "original");
}

#[test]
fn test_set_attribute_node_sets_owner_element() {
    let element_ref = create_element_with_ref("button");

    let attr = Arc::new(RwLock::new(Attr::new("type", "submit")));
    element_ref.write().set_attribute_node(attr.clone()).unwrap();

    // Check that owner element is set on the attr
    let attr_lock = attr.read();
    let owner = attr_lock.owner_element();
    assert!(owner.is_some());

    let owner_elem = owner.unwrap();
    let owner_elem_lock = owner_elem.read();
    assert_eq!(owner_elem_lock.tag_name(), "BUTTON");
}

#[test]
fn test_set_attribute_node_fails_if_owned_by_other_element() {
    let element_ref1 = create_element_with_ref("div");
    let element_ref2 = create_element_with_ref("span");

    // Create attr and attach to element1
    let attr = Arc::new(RwLock::new(Attr::new("id", "shared")));
    element_ref1.write().set_attribute_node(attr.clone()).unwrap();

    // Try to attach same attr to element2 - should fail
    let result = element_ref2.write().set_attribute_node(attr.clone());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DomException::InvalidStateError);
}

#[test]
fn test_set_attribute_node_succeeds_if_already_owned_by_same_element() {
    let element_ref = create_element_with_ref("div");

    // Create attr and attach to element
    let attr = Arc::new(RwLock::new(Attr::new("id", "test")));
    element_ref.write().set_attribute_node(attr.clone()).unwrap();

    // Try to set the same attr again on the same element - should succeed
    let result = element_ref.write().set_attribute_node(attr.clone());

    assert!(result.is_ok());
}

#[test]
fn test_roundtrip_set_then_get_attribute_node() {
    let element_ref = create_element_with_ref("input");

    // Set attribute using Attr node
    let attr = Arc::new(RwLock::new(Attr::new("placeholder", "Enter text")));
    element_ref.write().set_attribute_node(attr.clone()).unwrap();

    // Get it back
    let element = element_ref.read();
    let retrieved_attr = element.get_attribute_node("placeholder").unwrap();
    let retrieved_lock = retrieved_attr.read();

    assert_eq!(retrieved_lock.name(), "placeholder");
    assert_eq!(retrieved_lock.value(), "Enter text");
}

#[test]
fn test_get_attribute_node_interacts_with_string_method() {
    let element_ref = create_element_with_ref("a");

    // Set attribute using string method
    element_ref.write().set_attribute("href", "https://example.com").unwrap();

    // Get it as Attr node
    let element = element_ref.read();
    let attr = element.get_attribute_node("href").unwrap();
    let attr_lock = attr.read();

    assert_eq!(attr_lock.name(), "href");
    assert_eq!(attr_lock.value(), "https://example.com");

    // Verify string method still works
    assert_eq!(element.get_attribute("href"), Some("https://example.com"));
}

#[test]
fn test_set_attribute_node_interacts_with_string_method() {
    let element_ref = create_element_with_ref("img");

    // Set attribute using Attr node
    let attr = Arc::new(RwLock::new(Attr::new("src", "image.png")));
    element_ref.write().set_attribute_node(attr.clone()).unwrap();

    // Verify string method can read it
    let element = element_ref.read();
    assert_eq!(element.get_attribute("src"), Some("image.png"));
    assert!(element.has_attribute("src"));
}

#[test]
fn test_get_attribute_node_multiple_attributes() {
    let element_ref = create_element_with_ref("div");
    {
        let mut element = element_ref.write();
        element.set_attribute("id", "main").unwrap();
        element.set_attribute("class", "container").unwrap();
        element.set_attribute("data-role", "wrapper").unwrap();
    }

    // Get each attribute as a node
    let element = element_ref.read();
    let id_attr = element.get_attribute_node("id").unwrap();
    let class_attr = element.get_attribute_node("class").unwrap();
    let data_attr = element.get_attribute_node("data-role").unwrap();

    assert_eq!(id_attr.read().value(), "main");
    assert_eq!(class_attr.read().value(), "container");
    assert_eq!(data_attr.read().value(), "wrapper");
}

#[test]
fn test_set_attribute_node_updates_special_attributes() {
    let element_ref = create_element_with_ref("div");

    // Set id attribute via Attr node
    let id_attr = Arc::new(RwLock::new(Attr::new("id", "test-id")));
    element_ref.write().set_attribute_node(id_attr.clone()).unwrap();

    // Verify id() method returns correct value
    assert_eq!(element_ref.read().id(), Some("test-id"));

    // Set class attribute via Attr node
    let class_attr = Arc::new(RwLock::new(Attr::new("class", "btn primary")));
    element_ref.write().set_attribute_node(class_attr.clone()).unwrap();

    // Verify class_list() is updated
    let element = element_ref.read();
    assert_eq!(element.class_list().len(), 2);
    assert!(element.class_list().contains(&"btn".to_string()));
    assert!(element.class_list().contains(&"primary".to_string()));
}

#[test]
fn test_remove_attribute_then_get_attribute_node() {
    let element_ref = create_element_with_ref("div");
    element_ref.write().set_attribute("temp", "value").unwrap();

    // Remove the attribute
    element_ref.write().remove_attribute("temp").unwrap();

    // Verify get_attribute_node returns None
    let element = element_ref.read();
    assert!(element.get_attribute_node("temp").is_none());
}
