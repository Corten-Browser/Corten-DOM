//! Unit tests for CDATASection node

use dom_core::{CDATASection, Node};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_cdata_creation() {
    let cdata = CDATASection::new("Sample CDATA content");
    assert_eq!(cdata.data(), "Sample CDATA content");
    assert_eq!(cdata.length(), 20);
}

#[test]
fn test_cdata_empty() {
    let cdata = CDATASection::new("");
    assert_eq!(cdata.data(), "");
    assert_eq!(cdata.length(), 0);
}

#[test]
fn test_cdata_node_type() {
    let cdata = CDATASection::new("test");
    assert_eq!(cdata.node_type(), NodeType::CDataSection);
}

#[test]
fn test_cdata_node_name() {
    let cdata = CDATASection::new("test");
    assert_eq!(cdata.node_name(), "#cdata-section");
}

#[test]
fn test_cdata_node_value() {
    let cdata = CDATASection::new("CDATA value");
    assert_eq!(cdata.node_value(), Some("CDATA value"));
}

#[test]
fn test_cdata_set_node_value() {
    let mut cdata = CDATASection::new("initial");
    cdata.set_node_value(Some("updated".to_string()));
    assert_eq!(cdata.node_value(), Some("updated"));
    assert_eq!(cdata.data(), "updated");
}

#[test]
fn test_cdata_set_node_value_none() {
    let mut cdata = CDATASection::new("initial");
    cdata.set_node_value(None);
    assert_eq!(cdata.node_value(), Some(""));
    assert_eq!(cdata.data(), "");
}

#[test]
fn test_cdata_text_content() {
    let cdata = CDATASection::new("Content here");
    assert_eq!(cdata.text_content(), Some("Content here".to_string()));
}

#[test]
fn test_cdata_set_text_content() {
    let mut cdata = CDATASection::new("old");
    cdata.set_text_content("new content".to_string());
    assert_eq!(cdata.text_content(), Some("new content".to_string()));
    assert_eq!(cdata.data(), "new content");
}

#[test]
fn test_cdata_set_data() {
    let mut cdata = CDATASection::new("original");
    cdata.set_data("modified");
    assert_eq!(cdata.data(), "modified");
    assert_eq!(cdata.length(), 8);
}

#[test]
fn test_cdata_append_data() {
    let mut cdata = CDATASection::new("Hello");
    cdata.append_data(", World!");
    assert_eq!(cdata.data(), "Hello, World!");
    assert_eq!(cdata.length(), 13);
}

#[test]
fn test_cdata_insert_data() {
    let mut cdata = CDATASection::new("Hello!");
    let result = cdata.insert_data(5, " World");
    assert!(result.is_ok());
    assert_eq!(cdata.data(), "Hello World!");
}

#[test]
fn test_cdata_insert_data_invalid_offset() {
    let mut cdata = CDATASection::new("Hello");
    let result = cdata.insert_data(10, " World");
    assert!(result.is_err());
    assert_eq!(result, Err(DomException::InvalidModificationError));
}

#[test]
fn test_cdata_delete_data() {
    let mut cdata = CDATASection::new("Hello, World!");
    let result = cdata.delete_data(5, 7);
    assert!(result.is_ok());
    assert_eq!(cdata.data(), "Hello!");
}

#[test]
fn test_cdata_delete_data_invalid_offset() {
    let mut cdata = CDATASection::new("Hello");
    let result = cdata.delete_data(10, 5);
    assert!(result.is_err());
    assert_eq!(result, Err(DomException::InvalidModificationError));
}

#[test]
fn test_cdata_delete_data_past_end() {
    let mut cdata = CDATASection::new("Hello");
    let result = cdata.delete_data(2, 100);
    assert!(result.is_ok());
    assert_eq!(cdata.data(), "He");
}

#[test]
fn test_cdata_replace_data() {
    let mut cdata = CDATASection::new("Hello, World!");
    let result = cdata.replace_data(7, 5, "Rust");
    assert!(result.is_ok());
    assert_eq!(cdata.data(), "Hello, Rust!");
}

#[test]
fn test_cdata_replace_data_invalid_offset() {
    let mut cdata = CDATASection::new("Hello");
    let result = cdata.replace_data(10, 5, "test");
    assert!(result.is_err());
    assert_eq!(result, Err(DomException::InvalidModificationError));
}

#[test]
fn test_cdata_substring_data() {
    let cdata = CDATASection::new("Hello, World!");
    let result = cdata.substring_data(0, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello");
}

#[test]
fn test_cdata_substring_data_mid() {
    let cdata = CDATASection::new("Hello, World!");
    let result = cdata.substring_data(7, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "World");
}

#[test]
fn test_cdata_substring_data_invalid_offset() {
    let cdata = CDATASection::new("Hello");
    let result = cdata.substring_data(10, 5);
    assert!(result.is_err());
    assert_eq!(result, Err(DomException::InvalidModificationError));
}

#[test]
fn test_cdata_substring_data_past_end() {
    let cdata = CDATASection::new("Hello");
    let result = cdata.substring_data(2, 100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "llo");
}

#[test]
fn test_cdata_special_characters() {
    let cdata = CDATASection::new("<script>alert('test');</script>");
    assert_eq!(cdata.data(), "<script>alert('test');</script>");
}

#[test]
fn test_cdata_xml_special_chars() {
    let cdata = CDATASection::new("<greeting>Hello & goodbye</greeting>");
    assert_eq!(cdata.data(), "<greeting>Hello & goodbye</greeting>");
}

#[test]
fn test_cdata_javascript_code() {
    let js_code = "function test(a,b) { if (a < b && a > 0) { return true; } }";
    let cdata = CDATASection::new(js_code);
    assert_eq!(cdata.data(), js_code);
}

#[test]
fn test_cdata_comparison_operators() {
    let cdata = CDATASection::new("if (x < 10 && y > 5) { return x <= y; }");
    assert_eq!(cdata.data(), "if (x < 10 && y > 5) { return x <= y; }");
}

#[test]
fn test_cdata_multiline_content() {
    let content = "Line 1\nLine 2\nLine 3";
    let cdata = CDATASection::new(content);
    assert_eq!(cdata.data(), content);
    assert_eq!(cdata.length(), 20);
}

#[test]
fn test_cdata_unicode() {
    let cdata = CDATASection::new("Hello 世界 🌍");
    assert_eq!(cdata.data(), "Hello 世界 🌍");
}

#[test]
fn test_cdata_cannot_have_children() {
    let mut cdata = CDATASection::new("test");
    let child = CDATASection::new("child");
    let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

    let result = cdata.append_child(child_ref);
    assert!(result.is_err());
    assert!(matches!(result, Err(DomException::HierarchyRequestError)));
}

#[test]
fn test_cdata_child_nodes_empty() {
    let cdata = CDATASection::new("test");
    let children = cdata.child_nodes();
    assert_eq!(children.len(), 0);
}

#[test]
fn test_cdata_clone_node_shallow() {
    let cdata = CDATASection::new("Clone me");
    let cloned_ref = cdata.clone_node(false);
    let cloned = cloned_ref.read();

    assert_eq!(cloned.node_type(), NodeType::CDataSection);
    assert_eq!(cloned.node_value(), Some("Clone me"));
}

#[test]
fn test_cdata_clone_node_deep() {
    let cdata = CDATASection::new("Deep clone");
    let cloned_ref = cdata.clone_node(true);
    let cloned = cloned_ref.read();

    assert_eq!(cloned.node_type(), NodeType::CDataSection);
    assert_eq!(cloned.node_value(), Some("Deep clone"));
}

#[test]
fn test_cdata_parent_node() {
    let cdata = CDATASection::new("test");
    assert!(cdata.parent_node().is_none());
}

#[test]
fn test_cdata_xml_example() {
    let xml = r#"<example>
    <![CDATA[
        <greeting>Hello & goodbye</greeting>
        <data value="10 < 20 && 5 > 3"/>
    ]]>
</example>"#;

    let cdata = CDATASection::new(xml);
    assert_eq!(cdata.data(), xml);
}

#[test]
fn test_cdata_xhtml_script_example() {
    let script = r#"function matchwo(a,b) {
  if (a < b && a < 0) {
    return 1;
  }
  return 0;
}"#;

    let cdata = CDATASection::new(script);
    assert_eq!(cdata.data(), script);
}
