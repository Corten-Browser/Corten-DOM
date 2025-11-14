//! Validation tests for the requested querySelector enhancements
//! Tests the exact validation criteria from the task

use dom_core::{Document, Element, Node};
use dom_selectors::Selectable;

#[test]
fn test_tag_name_selector() {
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
fn test_class_selector() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("class", "classname").unwrap();
    root.write()
        .append_child(elem.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector(".classname");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_id_selector() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("id", "myid").unwrap();
    root.write()
        .append_child(elem.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("#myid");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_attribute_selector_with_value() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let input = doc.create_element("input").unwrap();

    input.write().set_attribute("type", "text").unwrap();
    root.write()
        .append_child(input.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("[type='text']");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_combined_div_classname() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("class", "classname").unwrap();
    root.write()
        .append_child(elem.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("div.classname");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_combined_id_classname() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("id", "myid").unwrap();
    elem.write().set_attribute("class", "classname").unwrap();
    root.write()
        .append_child(elem.clone().read().clone_node(false))
        .unwrap();

    let result = root.read().query_selector("#myid.classname");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_combined_div_classname_id() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let elem = doc.create_element("div").unwrap();

    elem.write().set_attribute("id", "myid").unwrap();
    elem.write().set_attribute("class", "classname").unwrap();
    root.write()
        .append_child(elem.clone().read().clone_node(false))
        .unwrap();

    // Test exact validation criteria: div.classname#id
    let result = root.read().query_selector("div.classname#myid");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_matches_with_combinations() {
    let elem = Element::new("button");
    let elem_ref = std::sync::Arc::new(parking_lot::RwLock::new(elem));
    elem_ref.write().set_attribute("class", "btn primary").unwrap();
    elem_ref.write().set_attribute("id", "submit").unwrap();

    // Test matching combinations
    assert!(elem_ref.read().matches("button").unwrap());
    assert!(elem_ref.read().matches(".btn").unwrap());
    assert!(elem_ref.read().matches("#submit").unwrap());
    assert!(elem_ref.read().matches("button.btn").unwrap());
    assert!(elem_ref.read().matches("button#submit").unwrap());
    assert!(elem_ref.read().matches("button.btn#submit").unwrap());
}
