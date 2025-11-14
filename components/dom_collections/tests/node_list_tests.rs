use dom_collections::NodeList;
use dom_core::{Document, Node};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_static_node_list() {
    let mut doc = Document::new();
    let elem1 = doc.create_element("div").unwrap();
    let elem2 = doc.create_element("span").unwrap();

    // Create node refs from elements
    let node1: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(elem1.read().clone())));
    let node2: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(elem2.read().clone())));

    let nodes = vec![node1, node2];
    let node_list = NodeList::new_static(nodes);

    assert_eq!(node_list.length(), 2);

    let item0 = node_list.item(0);
    assert!(item0.is_some());

    let item1 = node_list.item(1);
    assert!(item1.is_some());

    let item2 = node_list.item(2);
    assert!(item2.is_none());
}

#[test]
fn test_static_node_list_empty() {
    let node_list = NodeList::new_static(vec![]);
    assert_eq!(node_list.length(), 0);
    assert!(node_list.item(0).is_none());
}

#[test]
fn test_live_node_list() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let root_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(root.read().clone())));

    let node_list = NodeList::new_live(root_node.clone(), |_| true);

    // Live list should have at least the root
    assert!(node_list.length() >= 1);
}
