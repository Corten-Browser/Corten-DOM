//! Unit tests for DocumentType node

use dom_core::node::{Node, NodeRef};
use dom_core::DocumentType;
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// Helper to create a DocumentType node reference
fn create_doctype_ref(name: &str, public_id: &str, system_id: &str) -> NodeRef {
    let doctype = DocumentType::new(name, public_id, system_id);
    Arc::new(RwLock::new(Box::new(doctype) as Box<dyn Node>))
}

#[test]
fn test_html5_doctype_simple() {
    // HTML5 simple doctype: <!DOCTYPE html>
    let doctype = DocumentType::new_simple("html");

    assert_eq!(doctype.name(), "html");
    assert_eq!(doctype.public_id(), "");
    assert_eq!(doctype.system_id(), "");
}

#[test]
fn test_xhtml_strict_doctype() {
    // XHTML 1.0 Strict doctype
    let doctype = DocumentType::new(
        "html",
        "-//W3C//DTD XHTML 1.0 Strict//EN",
        "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd",
    );

    assert_eq!(doctype.name(), "html");
    assert_eq!(doctype.public_id(), "-//W3C//DTD XHTML 1.0 Strict//EN");
    assert_eq!(
        doctype.system_id(),
        "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd"
    );
}

#[test]
fn test_xhtml11_doctype() {
    // XHTML 1.1 doctype
    let doctype = DocumentType::new(
        "html",
        "-//W3C//DTD XHTML 1.1//EN",
        "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd",
    );

    assert_eq!(doctype.name(), "html");
    assert_eq!(doctype.public_id(), "-//W3C//DTD XHTML 1.1//EN");
    assert_eq!(
        doctype.system_id(),
        "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd"
    );
}

#[test]
fn test_svg_doctype() {
    // SVG 1.1 doctype
    let doctype = DocumentType::new(
        "svg",
        "-//W3C//DTD SVG 1.1//EN",
        "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd",
    );

    assert_eq!(doctype.name(), "svg");
    assert_eq!(doctype.public_id(), "-//W3C//DTD SVG 1.1//EN");
    assert_eq!(
        doctype.system_id(),
        "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"
    );
}

#[test]
fn test_custom_xml_doctype() {
    // Custom XML doctype with only system ID
    let doctype = DocumentType::new("custom", "", "custom.dtd");

    assert_eq!(doctype.name(), "custom");
    assert_eq!(doctype.public_id(), "");
    assert_eq!(doctype.system_id(), "custom.dtd");
}

#[test]
fn test_doctype_node_type() {
    let doctype = DocumentType::new_simple("html");
    assert_eq!(doctype.node_type(), NodeType::DocumentType);
}

#[test]
fn test_doctype_node_name() {
    let doctype = DocumentType::new("html", "", "");
    assert_eq!(doctype.node_name(), "html");

    let svg_doctype = DocumentType::new("svg", "", "");
    assert_eq!(svg_doctype.node_name(), "svg");
}

#[test]
fn test_doctype_node_value_is_none() {
    let doctype = DocumentType::new_simple("html");
    assert_eq!(doctype.node_value(), None);
}

#[test]
fn test_doctype_text_content_is_none() {
    let doctype = DocumentType::new_simple("html");
    assert_eq!(doctype.text_content(), None);
}

#[test]
fn test_doctype_cannot_have_children() {
    let doctype_ref = create_doctype_ref("html", "", "");
    let child_ref = create_doctype_ref("child", "", "");

    let result = doctype_ref.write().append_child(child_ref);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomException::HierarchyRequestError
    ));
}

#[test]
fn test_doctype_child_nodes_empty() {
    let doctype = DocumentType::new_simple("html");
    assert_eq!(doctype.child_nodes().len(), 0);
}

#[test]
fn test_doctype_remove_child_fails() {
    let doctype_ref = create_doctype_ref("html", "", "");
    let child_ref = create_doctype_ref("child", "", "");

    let result = doctype_ref.write().remove_child(child_ref);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DomException::NotFoundError));
}

#[test]
fn test_doctype_insert_before_fails() {
    let doctype_ref = create_doctype_ref("html", "", "");
    let child_ref = create_doctype_ref("child", "", "");

    let result = doctype_ref.write().insert_before(child_ref, None);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomException::HierarchyRequestError
    ));
}

#[test]
fn test_doctype_clone_node() {
    let doctype = DocumentType::new(
        "html",
        "-//W3C//DTD XHTML 1.0 Strict//EN",
        "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd",
    );

    let cloned_ref = doctype.clone_node(false);
    let cloned = cloned_ref.read();

    assert_eq!(cloned.node_type(), NodeType::DocumentType);
    assert_eq!(cloned.node_name(), "html");
}

#[test]
fn test_doctype_empty_public_and_system_ids() {
    let doctype = DocumentType::new("html", "", "");

    assert_eq!(doctype.name(), "html");
    assert_eq!(doctype.public_id(), "");
    assert_eq!(doctype.system_id(), "");
}

#[test]
fn test_doctype_case_sensitivity() {
    // HTML doctypes should preserve case (though HTML is case-insensitive)
    let doctype_lower = DocumentType::new_simple("html");
    let doctype_upper = DocumentType::new_simple("HTML");

    assert_eq!(doctype_lower.name(), "html");
    assert_eq!(doctype_upper.name(), "HTML");
}

#[test]
fn test_doctype_parent_node_is_none() {
    let doctype = DocumentType::new_simple("html");
    assert!(doctype.parent_node().is_none());
}

#[test]
fn test_doctype_as_any_downcast() {
    let doctype = DocumentType::new_simple("html");
    let any = doctype.as_any();
    assert!(any.downcast_ref::<DocumentType>().is_some());
}
