//! Tests for ProcessingInstruction node implementation

use dom_core::{Node, ProcessingInstruction};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_processing_instruction_creation() {
    let pi = ProcessingInstruction::new("xml-stylesheet", "type='text/css' href='style.css'");
    assert_eq!(pi.target(), "xml-stylesheet");
    assert_eq!(pi.data(), "type='text/css' href='style.css'");
}

#[test]
fn test_processing_instruction_node_type() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    assert_eq!(pi.node_type(), NodeType::ProcessingInstruction);
}

#[test]
fn test_processing_instruction_node_name() {
    let pi = ProcessingInstruction::new("xml-stylesheet", "href='style.css'");
    assert_eq!(pi.node_name(), "xml-stylesheet");
}

#[test]
fn test_processing_instruction_node_value() {
    let pi = ProcessingInstruction::new("xml", "version='1.0' encoding='UTF-8'");
    assert_eq!(pi.node_value(), Some("version='1.0' encoding='UTF-8'"));
}

#[test]
fn test_processing_instruction_set_node_value() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    pi.set_node_value(Some("version='2.0'".to_string()));
    assert_eq!(pi.node_value(), Some("version='2.0'"));
    assert_eq!(pi.data(), "version='2.0'");
}

#[test]
fn test_processing_instruction_set_node_value_none() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    pi.set_node_value(None);
    assert_eq!(pi.node_value(), Some(""));
    assert_eq!(pi.data(), "");
}

#[test]
fn test_processing_instruction_text_content() {
    let pi = ProcessingInstruction::new("php", "echo 'Hello World';");
    assert_eq!(pi.text_content(), Some("echo 'Hello World';".to_string()));
}

#[test]
fn test_processing_instruction_set_text_content() {
    let mut pi = ProcessingInstruction::new("php", "echo 'Hello';");
    pi.set_text_content("echo 'Goodbye';".to_string());
    assert_eq!(pi.text_content(), Some("echo 'Goodbye';".to_string()));
    assert_eq!(pi.data(), "echo 'Goodbye';");
}

#[test]
fn test_processing_instruction_set_data() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    pi.set_data("version='2.0'");
    assert_eq!(pi.data(), "version='2.0'");
}

#[test]
fn test_processing_instruction_target_getter() {
    let pi = ProcessingInstruction::new("xml-stylesheet", "");
    assert_eq!(pi.target(), "xml-stylesheet");
}

#[test]
fn test_processing_instruction_empty_data() {
    let pi = ProcessingInstruction::new("target", "");
    assert_eq!(pi.data(), "");
    assert_eq!(pi.node_value(), Some(""));
}

#[test]
fn test_processing_instruction_special_characters() {
    let pi = ProcessingInstruction::new("custom", "attr=\"value\" foo='bar' <>&");
    assert_eq!(pi.data(), "attr=\"value\" foo='bar' <>&");
}

#[test]
fn test_processing_instruction_xml_declaration() {
    let pi = ProcessingInstruction::new("xml", "version=\"1.0\" encoding=\"UTF-8\"");
    assert_eq!(pi.target(), "xml");
    assert_eq!(pi.data(), "version=\"1.0\" encoding=\"UTF-8\"");
}

#[test]
fn test_processing_instruction_xml_stylesheet() {
    let pi = ProcessingInstruction::new("xml-stylesheet", "type=\"text/css\" href=\"style.css\"");
    assert_eq!(pi.target(), "xml-stylesheet");
    assert_eq!(pi.data(), "type=\"text/css\" href=\"style.css\"");
}

#[test]
fn test_processing_instruction_php() {
    let pi = ProcessingInstruction::new("php", "echo \"Hello World\";");
    assert_eq!(pi.target(), "php");
    assert_eq!(pi.data(), "echo \"Hello World\";");
}

#[test]
fn test_processing_instruction_no_children() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    let child = ProcessingInstruction::new("child", "data");
    let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

    let result = pi.append_child(child_ref);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, DomException::HierarchyRequestError));
    }
}

#[test]
fn test_processing_instruction_child_nodes_empty() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    assert_eq!(pi.child_nodes().len(), 0);
}

#[test]
fn test_processing_instruction_remove_child_error() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    let child = ProcessingInstruction::new("child", "data");
    let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

    let result = pi.remove_child(child_ref);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, DomException::NotFoundError));
    }
}

#[test]
fn test_processing_instruction_insert_before_error() {
    let mut pi = ProcessingInstruction::new("xml", "version='1.0'");
    let child = ProcessingInstruction::new("child", "data");
    let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

    let result = pi.insert_before(child_ref, None);
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, DomException::HierarchyRequestError));
    }
}

#[test]
fn test_processing_instruction_clone_node() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    let cloned = pi.clone_node(false);

    let cloned_pi = cloned.read();
    assert_eq!(cloned_pi.node_type(), NodeType::ProcessingInstruction);
    assert_eq!(cloned_pi.node_name(), "xml");
    assert_eq!(cloned_pi.node_value(), Some("version='1.0'"));
}

#[test]
fn test_processing_instruction_clone_node_deep() {
    // Deep parameter doesn't matter for ProcessingInstruction (no children)
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    let cloned = pi.clone_node(true);

    let cloned_pi = cloned.read();
    assert_eq!(cloned_pi.node_type(), NodeType::ProcessingInstruction);
    assert_eq!(cloned_pi.node_name(), "xml");
    assert_eq!(cloned_pi.node_value(), Some("version='1.0'"));
}

#[test]
fn test_processing_instruction_parent_node_none() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    assert!(pi.parent_node().is_none());
}

#[test]
fn test_processing_instruction_as_any() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    let any = pi.as_any();
    assert!(any.downcast_ref::<ProcessingInstruction>().is_some());
}

#[test]
fn test_processing_instruction_contains_self() {
    let pi = ProcessingInstruction::new("xml", "version='1.0'");
    assert!(pi.contains(&pi));
}

#[test]
fn test_processing_instruction_long_data() {
    let long_data = "a".repeat(1000);
    let pi = ProcessingInstruction::new("custom", &long_data);
    assert_eq!(pi.data().len(), 1000);
}

#[test]
fn test_processing_instruction_multiline_data() {
    let data = "line1\nline2\nline3";
    let pi = ProcessingInstruction::new("custom", data);
    assert_eq!(pi.data(), data);
}
