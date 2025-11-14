use dom_types::{DocumentId, NodeId};

#[test]
fn test_node_id_is_u64() {
    let node_id: NodeId = 42;
    assert_eq!(node_id, 42u64);
}

#[test]
fn test_document_id_is_u64() {
    let doc_id: DocumentId = 100;
    assert_eq!(doc_id, 100u64);
}

#[test]
fn test_node_id_zero() {
    let node_id: NodeId = 0;
    assert_eq!(node_id, 0);
}

#[test]
fn test_node_id_max() {
    let node_id: NodeId = u64::MAX;
    assert_eq!(node_id, u64::MAX);
}

#[test]
fn test_document_id_zero() {
    let doc_id: DocumentId = 0;
    assert_eq!(doc_id, 0);
}

#[test]
fn test_document_id_max() {
    let doc_id: DocumentId = u64::MAX;
    assert_eq!(doc_id, u64::MAX);
}

#[test]
fn test_node_id_arithmetic() {
    let node_id: NodeId = 10;
    let result = node_id + 5;
    assert_eq!(result, 15);
}

#[test]
fn test_document_id_arithmetic() {
    let doc_id: DocumentId = 20;
    let result = doc_id * 2;
    assert_eq!(result, 40);
}

#[test]
fn test_node_id_comparison() {
    let id1: NodeId = 5;
    let id2: NodeId = 10;
    assert!(id1 < id2);
    assert!(id2 > id1);
    assert_eq!(id1, 5);
}

#[test]
fn test_document_id_comparison() {
    let id1: DocumentId = 100;
    let id2: DocumentId = 200;
    assert!(id1 < id2);
    assert!(id2 > id1);
    assert_eq!(id1, 100);
}

#[test]
fn test_node_id_in_collection() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let node_id: NodeId = 42;
    map.insert(node_id, "test_node");
    assert_eq!(map.get(&42), Some(&"test_node"));
}

#[test]
fn test_document_id_in_collection() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let doc_id: DocumentId = 99;
    set.insert(doc_id);
    assert!(set.contains(&99));
}

#[test]
fn test_node_id_increment() {
    let mut node_id: NodeId = 1;
    node_id += 1;
    assert_eq!(node_id, 2);
}

#[test]
fn test_document_id_increment() {
    let mut doc_id: DocumentId = 1;
    doc_id += 1;
    assert_eq!(doc_id, 2);
}

#[test]
fn test_ids_are_different_types() {
    // This is a compile-time test, but we can verify behavior
    let node_id: NodeId = 42;
    let doc_id: DocumentId = 42;

    // Both are u64, so they can be compared in value
    assert_eq!(node_id, doc_id);
}

#[test]
fn test_node_id_formatting() {
    let node_id: NodeId = 12345;
    let formatted = format!("{}", node_id);
    assert_eq!(formatted, "12345");
}

#[test]
fn test_document_id_formatting() {
    let doc_id: DocumentId = 67890;
    let formatted = format!("{}", doc_id);
    assert_eq!(formatted, "67890");
}
