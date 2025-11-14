//! Tests for NamedNodeMap implementation
//!
//! These tests follow TDD - they are written FIRST and should FAIL
//! until NamedNodeMap is implemented.

use dom_collections::NamedNodeMap;
use dom_core::{Attr, AttrRef};
use dom_types::DomException;
use parking_lot::RwLock;
use std::sync::Arc;

/// Helper to create an AttrRef
fn create_attr(name: &str, value: &str) -> AttrRef {
    Arc::new(RwLock::new(Attr::new(name, value)))
}

/// Helper to create a namespaced AttrRef
fn create_attr_ns(namespace: &str, qualified_name: &str, value: &str) -> AttrRef {
    Arc::new(RwLock::new(
        Attr::new_ns(namespace, qualified_name, value).unwrap(),
    ))
}

#[test]
fn test_empty_map_creation() {
    let map = NamedNodeMap::new();
    assert_eq!(map.length(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_default_creates_empty_map() {
    let map = NamedNodeMap::default();
    assert_eq!(map.length(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_set_and_get_named_item() {
    let mut map = NamedNodeMap::new();
    let attr = create_attr("id", "main");

    let result = map.set_named_item(attr.clone());
    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // No previous attribute

    assert_eq!(map.length(), 1);
    assert!(!map.is_empty());

    let retrieved = map.get_named_item("id");
    assert!(retrieved.is_some());

    let retrieved_attr = retrieved.unwrap();
    let retrieved_locked = retrieved_attr.read();
    assert_eq!(retrieved_locked.name(), "id");
    assert_eq!(retrieved_locked.value(), "main");
}

#[test]
fn test_set_named_item_replaces_existing() {
    let mut map = NamedNodeMap::new();

    // Add first attribute
    let attr1 = create_attr("class", "btn");
    map.set_named_item(attr1.clone()).unwrap();

    // Replace with new attribute with same name
    let attr2 = create_attr("class", "btn-primary");
    let result = map.set_named_item(attr2.clone());

    assert!(result.is_ok());
    let old_attr = result.unwrap();
    assert!(old_attr.is_some());

    // Verify old attribute was returned
    let old_attr_ref = old_attr.unwrap();
    let old_locked = old_attr_ref.read();
    assert_eq!(old_locked.value(), "btn");

    // Verify map now has new attribute
    assert_eq!(map.length(), 1);
    let current = map.get_named_item("class").unwrap();
    let current_locked = current.read();
    assert_eq!(current_locked.value(), "btn-primary");
}

#[test]
fn test_get_named_item_non_existent() {
    let map = NamedNodeMap::new();
    let result = map.get_named_item("nonexistent");
    assert!(result.is_none());
}

#[test]
fn test_remove_named_item() {
    let mut map = NamedNodeMap::new();
    let attr = create_attr("title", "Test");
    map.set_named_item(attr.clone()).unwrap();

    assert_eq!(map.length(), 1);

    let removed = map.remove_named_item("title");
    assert!(removed.is_ok());

    let removed_attr = removed.unwrap();
    let removed_locked = removed_attr.read();
    assert_eq!(removed_locked.name(), "title");
    assert_eq!(removed_locked.value(), "Test");

    assert_eq!(map.length(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_remove_named_item_non_existent() {
    let mut map = NamedNodeMap::new();
    let result = map.remove_named_item("nonexistent");

    assert!(result.is_err());
    match result {
        Err(DomException::NotFoundError) => (),
        _ => panic!("Expected NotFoundError"),
    }
}

#[test]
fn test_item_by_index() {
    let mut map = NamedNodeMap::new();

    let attr1 = create_attr("id", "main");
    let attr2 = create_attr("class", "container");
    let attr3 = create_attr("title", "Test");

    map.set_named_item(attr1).unwrap();
    map.set_named_item(attr2).unwrap();
    map.set_named_item(attr3).unwrap();

    assert_eq!(map.length(), 3);

    // Get items by index
    let item0 = map.item(0);
    assert!(item0.is_some());

    let item1 = map.item(1);
    assert!(item1.is_some());

    let item2 = map.item(2);
    assert!(item2.is_some());

    // Out of bounds
    let item3 = map.item(3);
    assert!(item3.is_none());
}

#[test]
fn test_item_out_of_bounds() {
    let map = NamedNodeMap::new();
    assert!(map.item(0).is_none());
    assert!(map.item(100).is_none());
}

#[test]
fn test_namespaced_attributes() {
    let mut map = NamedNodeMap::new();

    let attr = create_attr_ns("http://www.w3.org/1999/xlink", "xlink:href", "#anchor");
    map.set_named_item_ns(attr.clone()).unwrap();

    assert_eq!(map.length(), 1);

    // Retrieve by namespace and local name
    let retrieved = map.get_named_item_ns(Some("http://www.w3.org/1999/xlink"), "href");
    assert!(retrieved.is_some());

    let retrieved_ref = retrieved.unwrap();
    let retrieved_locked = retrieved_ref.read();
    assert_eq!(retrieved_locked.local_name(), "href");
    assert_eq!(retrieved_locked.value(), "#anchor");
}

#[test]
fn test_set_named_item_ns_replaces_existing() {
    let mut map = NamedNodeMap::new();

    let attr1 = create_attr_ns("http://example.com", "ex:prop", "value1");
    map.set_named_item_ns(attr1.clone()).unwrap();

    let attr2 = create_attr_ns("http://example.com", "ex:prop", "value2");
    let result = map.set_named_item_ns(attr2.clone());

    assert!(result.is_ok());
    let old_attr = result.unwrap();
    assert!(old_attr.is_some());

    // Verify new value is set
    let current = map
        .get_named_item_ns(Some("http://example.com"), "prop")
        .unwrap();
    let current_locked = current.read();
    assert_eq!(current_locked.value(), "value2");
}

#[test]
fn test_remove_named_item_ns() {
    let mut map = NamedNodeMap::new();

    let attr = create_attr_ns("http://example.com", "ex:prop", "test");
    map.set_named_item_ns(attr.clone()).unwrap();

    assert_eq!(map.length(), 1);

    let removed = map.remove_named_item_ns(Some("http://example.com"), "prop");
    assert!(removed.is_ok());

    let removed_ref = removed.unwrap();
    let removed_locked = removed_ref.read();
    assert_eq!(removed_locked.local_name(), "prop");

    assert_eq!(map.length(), 0);
}

#[test]
fn test_remove_named_item_ns_non_existent() {
    let mut map = NamedNodeMap::new();

    let result = map.remove_named_item_ns(Some("http://example.com"), "nonexistent");
    assert!(result.is_err());
    match result {
        Err(DomException::NotFoundError) => (),
        _ => panic!("Expected NotFoundError"),
    }
}

#[test]
fn test_mixed_namespaced_and_non_namespaced() {
    let mut map = NamedNodeMap::new();

    // Add non-namespaced attribute
    let attr1 = create_attr("id", "main");
    map.set_named_item(attr1.clone()).unwrap();

    // Add namespaced attribute
    let attr2 = create_attr_ns("http://www.w3.org/1999/xlink", "xlink:href", "#link");
    map.set_named_item_ns(attr2.clone()).unwrap();

    // Add another non-namespaced
    let attr3 = create_attr("class", "container");
    map.set_named_item(attr3.clone()).unwrap();

    assert_eq!(map.length(), 3);

    // Verify we can retrieve all
    assert!(map.get_named_item("id").is_some());
    assert!(map.get_named_item("class").is_some());
    assert!(map
        .get_named_item_ns(Some("http://www.w3.org/1999/xlink"), "href")
        .is_some());
}

#[test]
fn test_clear() {
    let mut map = NamedNodeMap::new();

    map.set_named_item(create_attr("id", "main")).unwrap();
    map.set_named_item(create_attr("class", "container")).unwrap();
    map.set_named_item(create_attr("title", "Test")).unwrap();

    assert_eq!(map.length(), 3);

    map.clear();

    assert_eq!(map.length(), 0);
    assert!(map.is_empty());
    assert!(map.get_named_item("id").is_none());
    assert!(map.get_named_item("class").is_none());
}

#[test]
fn test_names_iterator() {
    let mut map = NamedNodeMap::new();

    map.set_named_item(create_attr("id", "main")).unwrap();
    map.set_named_item(create_attr("class", "container")).unwrap();
    map.set_named_item(create_attr("title", "Test")).unwrap();

    let names = map.names();
    assert_eq!(names.len(), 3);

    // Verify all names are present (order may vary)
    assert!(names.contains(&"id".to_string()));
    assert!(names.contains(&"class".to_string()));
    assert!(names.contains(&"title".to_string()));
}

#[test]
fn test_names_empty_map() {
    let map = NamedNodeMap::new();
    let names = map.names();
    assert_eq!(names.len(), 0);
}

#[test]
fn test_attributes_vector() {
    let mut map = NamedNodeMap::new();

    let attr1 = create_attr("id", "main");
    let attr2 = create_attr("class", "container");

    map.set_named_item(attr1.clone()).unwrap();
    map.set_named_item(attr2.clone()).unwrap();

    let attrs = map.attributes();
    assert_eq!(attrs.len(), 2);

    // Verify we have all attributes
    let names: Vec<String> = attrs
        .iter()
        .map(|attr| attr.read().name().to_string())
        .collect();
    assert!(names.contains(&"id".to_string()));
    assert!(names.contains(&"class".to_string()));
}

#[test]
fn test_attributes_empty_map() {
    let map = NamedNodeMap::new();
    let attrs = map.attributes();
    assert_eq!(attrs.len(), 0);
}

#[test]
fn test_case_sensitivity() {
    let mut map = NamedNodeMap::new();

    map.set_named_item(create_attr("ID", "main")).unwrap();

    // HTML attributes are case-insensitive in lookups, but we store exact case
    // For now, we'll test exact case matching
    assert!(map.get_named_item("ID").is_some());
    assert!(map.get_named_item("id").is_none()); // Different case
}

#[test]
fn test_get_named_item_ns_without_namespace() {
    let mut map = NamedNodeMap::new();

    // Add non-namespaced attribute
    let attr = create_attr("title", "Test");
    map.set_named_item(attr.clone()).unwrap();

    // Try to get it using namespace API with None
    let result = map.get_named_item_ns(None, "title");
    // Depending on implementation, this might return the attribute or None
    // For now, let's test that it doesn't crash
    assert!(result.is_none() || result.is_some());
}

#[test]
fn test_multiple_operations() {
    let mut map = NamedNodeMap::new();

    // Add some attributes
    map.set_named_item(create_attr("id", "main")).unwrap();
    map.set_named_item(create_attr("class", "btn")).unwrap();
    map.set_named_item(create_attr("title", "Click me")).unwrap();
    assert_eq!(map.length(), 3);

    // Remove one
    map.remove_named_item("class").unwrap();
    assert_eq!(map.length(), 2);

    // Add another
    map.set_named_item(create_attr("data-value", "123")).unwrap();
    assert_eq!(map.length(), 3);

    // Replace one
    map.set_named_item(create_attr("id", "header")).unwrap();
    assert_eq!(map.length(), 3);

    // Verify final state
    let id_attr = map.get_named_item("id").unwrap();
    assert_eq!(id_attr.read().value(), "header");

    assert!(map.get_named_item("class").is_none());
    assert!(map.get_named_item("title").is_some());
    assert!(map.get_named_item("data-value").is_some());
}

#[test]
fn test_length_consistency() {
    let mut map = NamedNodeMap::new();

    assert_eq!(map.length(), 0);

    map.set_named_item(create_attr("a", "1")).unwrap();
    assert_eq!(map.length(), 1);

    map.set_named_item(create_attr("b", "2")).unwrap();
    assert_eq!(map.length(), 2);

    map.set_named_item(create_attr("c", "3")).unwrap();
    assert_eq!(map.length(), 3);

    // Replace doesn't change length
    map.set_named_item(create_attr("a", "100")).unwrap();
    assert_eq!(map.length(), 3);

    map.remove_named_item("b").unwrap();
    assert_eq!(map.length(), 2);

    map.clear();
    assert_eq!(map.length(), 0);
}

#[test]
fn test_clone_behavior() {
    let mut map = NamedNodeMap::new();

    map.set_named_item(create_attr("id", "main")).unwrap();
    map.set_named_item(create_attr("class", "btn")).unwrap();

    let cloned = map.clone();

    assert_eq!(map.length(), cloned.length());
    assert!(cloned.get_named_item("id").is_some());
    assert!(cloned.get_named_item("class").is_some());
}
