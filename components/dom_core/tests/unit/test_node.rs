//! Unit tests for Node trait and base implementation

use dom_core::node::{Node, NodeData, NodeRef};
use dom_core::Element;
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

fn create_element_node(tag: &str) -> NodeRef {
    let node_ref: NodeRef = Arc::new(RwLock::new(Box::new(Element::new(tag)) as Box<dyn Node>));
    // Set self_node_ref so that append_child can set correct parent references
    node_ref.write().node_data_mut().set_self_node_ref(Arc::downgrade(&node_ref));
    node_ref
}

#[test]
fn test_node_type() {
    let node = create_element_node("div");
    assert_eq!(node.read().node_type(), NodeType::Element);
}

#[test]
fn test_node_name() {
    let node = create_element_node("div");
    assert_eq!(node.read().node_name(), "DIV");
}

#[test]
fn test_append_child() {
    let parent = create_element_node("div");
    let child = create_element_node("span");

    let result = parent.write().append_child(child.clone());
    assert!(result.is_ok());
    assert_eq!(parent.read().child_nodes().len(), 1);
}

#[test]
fn test_remove_child() {
    let parent = create_element_node("div");
    let child = create_element_node("span");

    parent.write().append_child(child.clone()).unwrap();
    assert_eq!(parent.read().child_nodes().len(), 1);

    let result = parent.write().remove_child(child.clone());
    assert!(result.is_ok());
    assert_eq!(parent.read().child_nodes().len(), 0);
}

#[test]
fn test_parent_child_relationship() {
    let parent = create_element_node("div");
    let child = create_element_node("span");

    parent.write().append_child(child.clone()).unwrap();

    let child_parent = child.read().parent_node();
    assert!(child_parent.is_some());

    // Verify parent is correct by checking node name
    let parent_node = child_parent.unwrap();
    assert_eq!(parent_node.read().node_name(), "DIV");
}

#[test]
fn test_prevent_circular_reference() {
    let node1 = create_element_node("div");
    let node2 = create_element_node("span");

    // node1 -> node2
    node1.write().append_child(node2.clone()).unwrap();

    // Try to make circular: node2 -> node1 (should fail)
    let result = node2.write().append_child(node1.clone());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DomException::HierarchyRequestError);
}

#[test]
fn test_first_child() {
    let parent = create_element_node("div");
    assert!(parent.read().first_child().is_none());

    let child1 = create_element_node("span");
    let child2 = create_element_node("p");

    parent.write().append_child(child1.clone()).unwrap();
    parent.write().append_child(child2.clone()).unwrap();

    let first = parent.read().first_child();
    assert!(first.is_some());
    assert_eq!(first.unwrap().read().node_name(), "SPAN");
}

#[test]
fn test_last_child() {
    let parent = create_element_node("div");
    assert!(parent.read().last_child().is_none());

    let child1 = create_element_node("span");
    let child2 = create_element_node("p");

    parent.write().append_child(child1.clone()).unwrap();
    parent.write().append_child(child2.clone()).unwrap();

    let last = parent.read().last_child();
    assert!(last.is_some());
    assert_eq!(last.unwrap().read().node_name(), "P");
}

#[test]
fn test_insert_before() {
    let parent = create_element_node("div");
    let child1 = create_element_node("span");
    let child2 = create_element_node("p");
    let child3 = create_element_node("a");

    parent.write().append_child(child1.clone()).unwrap();
    parent.write().append_child(child2.clone()).unwrap();

    // Insert child3 before child2
    parent
        .write()
        .insert_before(child3.clone(), Some(child2.clone()))
        .unwrap();

    let children = parent.read().child_nodes();
    assert_eq!(children.len(), 3);
    assert_eq!(children[0].read().node_name(), "SPAN");
    assert_eq!(children[1].read().node_name(), "A");
    assert_eq!(children[2].read().node_name(), "P");
}

#[test]
fn test_clone_node_shallow() {
    let parent = create_element_node("div");
    let child = create_element_node("span");
    parent.write().append_child(child.clone()).unwrap();

    let cloned = parent.read().clone_node(false);

    // Shallow clone should not have children
    assert_eq!(cloned.read().child_nodes().len(), 0);
    assert_eq!(cloned.read().node_name(), "DIV");
}

#[test]
fn test_clone_node_deep() {
    let parent = create_element_node("div");
    let child = create_element_node("span");
    parent.write().append_child(child.clone()).unwrap();

    let cloned = parent.read().clone_node(true);

    // Deep clone should have children
    assert_eq!(cloned.read().child_nodes().len(), 1);
    assert_eq!(cloned.read().node_name(), "DIV");
    assert_eq!(cloned.read().child_nodes()[0].read().node_name(), "SPAN");
}

#[test]
fn test_contains() {
    let grandparent = create_element_node("div");
    let parent = create_element_node("span");
    let child = create_element_node("a");

    grandparent.write().append_child(parent.clone()).unwrap();
    parent.write().append_child(child.clone()).unwrap();

    // Grandparent contains child
    assert!(grandparent.read().contains(&**child.read()));

    // Child does not contain grandparent
    assert!(!child.read().contains(&**grandparent.read()));
}

#[test]
fn test_node_reparenting() {
    let parent1 = create_element_node("div");
    let parent2 = create_element_node("span");
    let child = create_element_node("p");

    // Add child to parent1
    parent1.write().append_child(child.clone()).unwrap();
    assert_eq!(parent1.read().child_nodes().len(), 1);
    assert_eq!(parent2.read().child_nodes().len(), 0);

    // Move child to parent2
    parent2.write().append_child(child.clone()).unwrap();
    assert_eq!(parent1.read().child_nodes().len(), 0);
    assert_eq!(parent2.read().child_nodes().len(), 1);
}
