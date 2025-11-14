//! Integration tests verifying all component APIs are accessible

use browser_dom_impl::messages::Modifiers;
use browser_dom_impl::*;
use std::sync::Arc;

#[test]
fn test_dom_types_accessible() {
    // Verify NodeId and DocumentId types are accessible
    use dom_types::{DocumentId, NodeId};

    let node_id: NodeId = 42;
    let doc_id: DocumentId = 100;

    assert_eq!(node_id, 42);
    assert_eq!(doc_id, 100);

    // Verify NodeType enum is accessible
    use dom_types::NodeType;
    let node_type = NodeType::Element;
    assert_eq!(node_type, NodeType::Element);

    // Verify DomException is accessible
    use dom_types::DomException;
    let error = DomException::HierarchyRequestError;
    matches!(error, DomException::HierarchyRequestError);
}

#[test]
fn test_dom_core_accessible() {
    // Verify Document is accessible
    let document = Document::new();
    assert!(document.document_element().is_none());

    // Verify Element is accessible
    let element = Element::new("div");
    assert_eq!(element.tag_name(), "DIV");

    // Verify Text is accessible
    let text = Text::new("Hello");
    assert_eq!(text.data(), "Hello");

    // Verify Comment is accessible
    let comment = Comment::new("Test comment");
    assert_eq!(comment.data(), "Test comment");

    // Verify DocumentFragment is accessible
    let fragment = DocumentFragment::new();
    assert!(fragment.child_nodes().is_empty());
}

#[test]
fn test_element_operations() {
    let mut element = Element::new("div");

    // Test set_attribute
    assert!(element.set_attribute("class", "test").is_ok());
    assert_eq!(element.get_attribute("class"), Some("test"));

    // Test has_attribute
    assert!(element.has_attribute("class"));
    assert!(!element.has_attribute("id"));

    // Test remove_attribute
    element.remove_attribute("class");
    assert!(!element.has_attribute("class"));
}

#[test]
fn test_document_operations() {
    let mut document = Document::new();

    // Create an element
    let element = Element::new("html");
    let element_ref = Arc::new(parking_lot::RwLock::new(element));

    // Set document element
    document.set_document_element(element_ref.clone());

    // Verify document element is set
    assert!(document.document_element().is_some());

    // Verify we can access the element
    let doc_elem = document.document_element().unwrap();
    let elem_lock = doc_elem.read();
    assert_eq!(elem_lock.tag_name(), "HTML");
}

#[test]
fn test_text_node_operations() {
    let mut text = Text::new("Initial");

    assert_eq!(text.data(), "Initial");

    text.set_data("Modified");
    assert_eq!(text.data(), "Modified");

    assert_eq!(text.length(), 8);
}

#[test]
fn test_comment_node_operations() {
    let mut comment = Comment::new("Comment");

    assert_eq!(comment.data(), "Comment");

    comment.set_data("New comment");
    assert_eq!(comment.data(), "New comment");
}

#[test]
fn test_document_fragment_operations() {
    let fragment = DocumentFragment::new();

    // Initially empty
    assert!(fragment.child_nodes().is_empty());
}

#[test]
fn test_config_and_component_integration() {
    // Create custom config
    let config = DomConfig {
        max_tree_depth: 128,
        max_children: 1000,
        enable_mutation_observers: true,
        enable_shadow_dom: false,
        gc_threshold: 10000,
        arena_capacity: 5000,
    };

    // Create component
    let component = DomComponent::new(config.clone());

    // Verify config is accessible
    assert_eq!(component.config().max_tree_depth, 128);
    assert_eq!(component.config().max_children, 1000);
    assert!(component.config().enable_mutation_observers);
    assert!(!component.config().enable_shadow_dom);
}

#[test]
fn test_all_message_types_accessible() {
    use std::collections::HashMap;

    // Test all message variants can be constructed
    let _parsed_doc = DomComponentMessage::ParsedDocument {
        request_id: 1,
        root: ParsedNode {
            node_type: ParsedNodeType::Element,
            tag_name: Some("div".to_string()),
            attributes: HashMap::new(),
            text_content: None,
            children: vec![],
        },
        doctype: None,
    };

    let _script_manip = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::SetAttribute {
            element_id: 1,
            name: "class".to_string(),
            value: "test".to_string(),
        },
        node_id: 1,
        params: OperationParams::default(),
    };

    let _style_inval = DomComponentMessage::StyleInvalidation {
        node_id: 1,
        reason: InvalidationReason::ClassChanged,
    };

    let _user_interact = DomComponentMessage::UserInteraction {
        event_type: "click".to_string(),
        target_id: 1,
        event_data: EventData {
            mouse_x: Some(10),
            mouse_y: Some(20),
            key: None,
            modifiers: Modifiers::default(),
            extra: HashMap::new(),
        },
    };

    let _query = DomComponentMessage::Query {
        request_id: 1,
        query: QueryType::GetElementById {
            id: "test".to_string(),
        },
    };
}

#[test]
fn test_all_response_types_accessible() {
    // Test all response variants can be constructed
    let _tree_ready = DomComponentResponse::DomTreeReady {
        request_id: 1,
        document_id: 1,
    };

    let _dom_mutated = DomComponentResponse::DomMutated {
        mutations: vec![],
        affected_nodes: vec![],
    };

    let _tree_changed = DomComponentResponse::DomTreeChanged {
        change_type: TreeChangeType::NodeInserted,
        affected_subtree: 1,
        needs_style_recalc: true,
    };

    let _layout_inval = DomComponentResponse::LayoutInvalidated {
        root_id: 1,
        invalidation_type: LayoutInvalidationType::Full,
    };

    let _query_result = DomComponentResponse::QueryResult {
        request_id: 1,
        result: QueryResultType::NodeIds(vec![1, 2, 3]),
    };

    let _error = DomComponentResponse::Error {
        message: "Test error".to_string(),
        code: 500,
    };
}

#[test]
fn test_cross_component_element_creation() {
    // This test verifies that Element from dom-core works with the integration layer

    let mut component = DomComponent::new(DomConfig::default());

    // Create a parsed tree
    let parsed = ParsedNode {
        node_type: ParsedNodeType::Element,
        tag_name: Some("div".to_string()),
        attributes: {
            let mut attrs = std::collections::HashMap::new();
            attrs.insert("id".to_string(), "test-div".to_string());
            attrs.insert("class".to_string(), "container".to_string());
            attrs
        },
        text_content: None,
        children: vec![ParsedNode {
            node_type: ParsedNodeType::Text,
            tag_name: None,
            attributes: std::collections::HashMap::new(),
            text_content: Some("Test content".to_string()),
            children: vec![],
        }],
    };

    // Send to component
    let msg = DomComponentMessage::ParsedDocument {
        request_id: 99,
        root: parsed,
        doctype: None,
    };

    let response = component.handle_message(msg);

    // Verify document was created
    match response {
        DomComponentResponse::DomTreeReady { document_id, .. } => {
            let document = component.get_document(document_id).unwrap();
            let doc_lock = document.read();

            // Verify document has root element
            assert!(doc_lock.document_element().is_some());

            // Verify root element is a div
            let root = doc_lock.document_element().unwrap();
            let root_lock = root.read();
            assert_eq!(root_lock.tag_name(), "DIV");

            // Verify attributes were set
            assert_eq!(root_lock.get_attribute("id"), Some("test-div"));
            assert_eq!(root_lock.get_attribute("class"), Some("container"));
        }
        _ => panic!("Expected DomTreeReady response"),
    }
}
