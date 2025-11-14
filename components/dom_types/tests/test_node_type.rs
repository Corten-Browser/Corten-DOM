use dom_types::NodeType;

#[test]
fn test_node_type_element() {
    assert_eq!(NodeType::Element as u16, 1);
}

#[test]
fn test_node_type_attribute() {
    assert_eq!(NodeType::Attribute as u16, 2);
}

#[test]
fn test_node_type_text() {
    assert_eq!(NodeType::Text as u16, 3);
}

#[test]
fn test_node_type_cdata_section() {
    assert_eq!(NodeType::CDataSection as u16, 4);
}

#[test]
fn test_node_type_processing_instruction() {
    assert_eq!(NodeType::ProcessingInstruction as u16, 7);
}

#[test]
fn test_node_type_comment() {
    assert_eq!(NodeType::Comment as u16, 8);
}

#[test]
fn test_node_type_document() {
    assert_eq!(NodeType::Document as u16, 9);
}

#[test]
fn test_node_type_document_type() {
    assert_eq!(NodeType::DocumentType as u16, 10);
}

#[test]
fn test_node_type_document_fragment() {
    assert_eq!(NodeType::DocumentFragment as u16, 11);
}

#[test]
fn test_node_type_debug() {
    assert_eq!(format!("{:?}", NodeType::Element), "Element");
    assert_eq!(format!("{:?}", NodeType::Text), "Text");
    assert_eq!(format!("{:?}", NodeType::Document), "Document");
}

#[test]
fn test_node_type_clone() {
    let node_type = NodeType::Element;
    #[allow(clippy::clone_on_copy)]
    let cloned = node_type.clone();
    assert_eq!(node_type, cloned);
}

#[test]
fn test_node_type_copy() {
    let node_type = NodeType::Element;
    let copied = node_type;
    assert_eq!(node_type, copied);
}

#[test]
fn test_node_type_equality() {
    assert_eq!(NodeType::Element, NodeType::Element);
    assert_ne!(NodeType::Element, NodeType::Text);
    assert_ne!(NodeType::Document, NodeType::DocumentFragment);
}

#[test]
fn test_node_type_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(NodeType::Element);
    set.insert(NodeType::Element); // Duplicate
    set.insert(NodeType::Text);

    assert_eq!(set.len(), 2); // Only unique values
    assert!(set.contains(&NodeType::Element));
    assert!(set.contains(&NodeType::Text));
    assert!(!set.contains(&NodeType::Document));
}

#[test]
fn test_node_type_serialization() {
    let node_type = NodeType::Element;
    let json = serde_json::to_string(&node_type).expect("Serialization failed");
    assert!(!json.is_empty());
}

#[test]
fn test_node_type_deserialization() {
    let json = serde_json::to_string(&NodeType::Element).expect("Serialization failed");
    let deserialized: NodeType = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(deserialized, NodeType::Element);
}

#[test]
fn test_all_node_types_roundtrip() {
    let types = vec![
        NodeType::Element,
        NodeType::Attribute,
        NodeType::Text,
        NodeType::CDataSection,
        NodeType::ProcessingInstruction,
        NodeType::Comment,
        NodeType::Document,
        NodeType::DocumentType,
        NodeType::DocumentFragment,
    ];

    for node_type in types {
        let json = serde_json::to_string(&node_type).expect("Serialization failed");
        let deserialized: NodeType = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(node_type, deserialized);
    }
}
