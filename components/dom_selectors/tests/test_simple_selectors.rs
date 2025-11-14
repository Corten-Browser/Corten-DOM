//! Unit tests for simple CSS selectors (tag, class, ID)

use dom_core::{Document, Element, Node};
use dom_selectors::Selectable;

#[test]
fn test_query_selector_by_tag() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let span = doc.create_element("span").unwrap();

    root.write()
        .append_child(span.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("span");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_query_selector_by_id() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let span = doc.create_element("span").unwrap();

    span.write().set_attribute("id", "test-id").unwrap();
    root.write()
        .append_child(span.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("#test-id");
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
}

#[test]
fn test_query_selector_by_class() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let span = doc.create_element("span").unwrap();

    span.write().set_attribute("class", "test-class").unwrap();
    root.write()
        .append_child(span.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector(".test-class");
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
}

#[test]
fn test_query_selector_not_found() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let result = root.read().query_selector(".nonexistent");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_query_selector_all_by_tag() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    for _ in 0..3 {
        let span = doc.create_element("span").unwrap();
        root.write()
            .append_child(span.read().clone_node(false))
            .unwrap();
    }

    let result = root.read().query_selector_all("span");
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.length(), 3);
}

#[test]
fn test_query_selector_all_by_class() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    for i in 0..3 {
        let span = doc.create_element("span").unwrap();
        span.write().set_attribute("class", "item").unwrap();
        root.write()
            .append_child(span.read().clone_node(false))
            .unwrap();
    }

    let result = root.read().query_selector_all(".item");
    assert!(result.is_ok());
    let list = result.unwrap();
    assert_eq!(list.length(), 3);
}

#[test]
fn test_matches_tag() {
    let _doc = Document::new();
    let elem = Element::new("button");
    let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem));

    let result = elem_ref.read().matches("button");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_matches_class() {
    let elem = Element::new("button");
    let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem));
    elem_ref
        .write()
        .set_attribute("class", "btn primary")
        .unwrap();

    let result = elem_ref.read().matches(".btn");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_matches_id() {
    let elem = Element::new("button");
    let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem));
    elem_ref.write().set_attribute("id", "submit").unwrap();

    let result = elem_ref.read().matches("#submit");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_matches_not_matching() {
    let elem = Element::new("button");
    let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem));

    let result = elem_ref.read().matches("input");
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_invalid_selector() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let result = root.read().query_selector("###invalid");
    assert!(result.is_err());
}
