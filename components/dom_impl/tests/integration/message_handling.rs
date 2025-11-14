//! Integration tests for message handling

use browser_dom_impl::messages::{DocumentType, Modifiers};
use browser_dom_impl::*;
use std::collections::HashMap;

fn create_simple_parsed_tree() -> ParsedNode {
    ParsedNode {
        node_type: ParsedNodeType::Element,
        tag_name: Some("div".to_string()),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("id".to_string(), "root".to_string());
            attrs
        },
        text_content: None,
        children: vec![],
    }
}

fn create_complex_parsed_tree() -> ParsedNode {
    ParsedNode {
        node_type: ParsedNodeType::Element,
        tag_name: Some("html".to_string()),
        attributes: HashMap::new(),
        text_content: None,
        children: vec![
            ParsedNode {
                node_type: ParsedNodeType::Element,
                tag_name: Some("head".to_string()),
                attributes: HashMap::new(),
                text_content: None,
                children: vec![ParsedNode {
                    node_type: ParsedNodeType::Element,
                    tag_name: Some("title".to_string()),
                    attributes: HashMap::new(),
                    text_content: None,
                    children: vec![ParsedNode {
                        node_type: ParsedNodeType::Text,
                        tag_name: None,
                        attributes: HashMap::new(),
                        text_content: Some("Test Page".to_string()),
                        children: vec![],
                    }],
                }],
            },
            ParsedNode {
                node_type: ParsedNodeType::Element,
                tag_name: Some("body".to_string()),
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("class".to_string(), "main".to_string());
                    attrs
                },
                text_content: None,
                children: vec![
                    ParsedNode {
                        node_type: ParsedNodeType::Element,
                        tag_name: Some("h1".to_string()),
                        attributes: {
                            let mut attrs = HashMap::new();
                            attrs.insert("id".to_string(), "header".to_string());
                            attrs
                        },
                        text_content: None,
                        children: vec![ParsedNode {
                            node_type: ParsedNodeType::Text,
                            tag_name: None,
                            attributes: HashMap::new(),
                            text_content: Some("Welcome".to_string()),
                            children: vec![],
                        }],
                    },
                    ParsedNode {
                        node_type: ParsedNodeType::Element,
                        tag_name: Some("p".to_string()),
                        attributes: HashMap::new(),
                        text_content: None,
                        children: vec![ParsedNode {
                            node_type: ParsedNodeType::Text,
                            tag_name: None,
                            attributes: HashMap::new(),
                            text_content: Some("This is a test.".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        ],
    }
}

#[test]
fn test_parsed_document_creates_document() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ParsedDocument {
        request_id: 1,
        root: create_simple_parsed_tree(),
        doctype: None,
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomTreeReady {
            request_id,
            document_id,
        } => {
            assert_eq!(request_id, 1);
            assert_eq!(document_id, 1);

            // Verify document exists
            assert!(component.get_document(1).is_some());
        }
        _ => panic!("Expected DomTreeReady response"),
    }
}

#[test]
fn test_parsed_document_with_complex_tree() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ParsedDocument {
        request_id: 42,
        root: create_complex_parsed_tree(),
        doctype: Some(DocumentType {
            name: "html".to_string(),
            public_id: None,
            system_id: None,
        }),
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomTreeReady {
            request_id,
            document_id,
        } => {
            assert_eq!(request_id, 42);
            assert_eq!(document_id, 1);

            let document = component.get_document(document_id).unwrap();
            let doc_lock = document.read();

            // Verify document has root element
            assert!(doc_lock.document_element().is_some());
        }
        _ => panic!("Expected DomTreeReady response"),
    }
}

#[test]
fn test_script_manipulation_set_attribute() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::SetAttribute {
            element_id: 100,
            name: "class".to_string(),
            value: "active".to_string(),
        },
        node_id: 100,
        params: OperationParams::default(),
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomMutated {
            mutations,
            affected_nodes,
        } => {
            assert_eq!(mutations.len(), 1);
            assert_eq!(mutations[0].mutation_type, MutationType::Attributes);
            assert_eq!(mutations[0].target, 100);
            assert_eq!(mutations[0].attribute_name, Some("class".to_string()));
            assert_eq!(affected_nodes, vec![100]);
        }
        _ => panic!("Expected DomMutated response"),
    }
}

#[test]
fn test_script_manipulation_append_child() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::AppendChild {
            parent_id: 50,
            child_id: 51,
        },
        node_id: 50,
        params: OperationParams::default(),
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomMutated {
            mutations,
            affected_nodes,
        } => {
            assert_eq!(mutations.len(), 1);
            assert_eq!(mutations[0].mutation_type, MutationType::ChildList);
            assert_eq!(mutations[0].target, 50);
            assert_eq!(mutations[0].added_nodes, vec![51]);
            assert!(mutations[0].removed_nodes.is_empty());
            assert_eq!(affected_nodes, vec![50, 51]);
        }
        _ => panic!("Expected DomMutated response"),
    }
}

#[test]
fn test_script_manipulation_remove_child() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::RemoveChild {
            parent_id: 60,
            child_id: 61,
        },
        node_id: 60,
        params: OperationParams::default(),
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomMutated {
            mutations,
            affected_nodes,
        } => {
            assert_eq!(mutations.len(), 1);
            assert_eq!(mutations[0].mutation_type, MutationType::ChildList);
            assert_eq!(mutations[0].target, 60);
            assert!(mutations[0].added_nodes.is_empty());
            assert_eq!(mutations[0].removed_nodes, vec![61]);
            assert_eq!(affected_nodes, vec![60, 61]);
        }
        _ => panic!("Expected DomMutated response"),
    }
}

#[test]
fn test_script_manipulation_set_text_content() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::ScriptManipulation {
        operation: DomOperation::SetTextContent {
            node_id: 70,
            text: "New text".to_string(),
        },
        node_id: 70,
        params: OperationParams::default(),
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomMutated {
            mutations,
            affected_nodes,
        } => {
            assert_eq!(mutations.len(), 1);
            assert_eq!(mutations[0].mutation_type, MutationType::CharacterData);
            assert_eq!(mutations[0].target, 70);
            assert_eq!(affected_nodes, vec![70]);
        }
        _ => panic!("Expected DomMutated response"),
    }
}

#[test]
fn test_style_invalidation() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::StyleInvalidation {
        node_id: 80,
        reason: InvalidationReason::ClassChanged,
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::LayoutInvalidated {
            root_id,
            invalidation_type,
        } => {
            assert_eq!(root_id, 80);
            matches!(invalidation_type, LayoutInvalidationType::Incremental);
        }
        _ => panic!("Expected LayoutInvalidated response"),
    }
}

#[test]
fn test_user_interaction() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::UserInteraction {
        event_type: "click".to_string(),
        target_id: 90,
        event_data: EventData {
            mouse_x: Some(100),
            mouse_y: Some(200),
            key: None,
            modifiers: Modifiers {
                shift: false,
                ctrl: true,
                alt: false,
                meta: false,
            },
            extra: HashMap::new(),
        },
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::DomTreeChanged {
            change_type,
            affected_subtree,
            needs_style_recalc,
        } => {
            matches!(change_type, TreeChangeType::SubtreeModified);
            assert_eq!(affected_subtree, 90);
            assert!(!needs_style_recalc);
        }
        _ => panic!("Expected DomTreeChanged response"),
    }
}

#[test]
fn test_query_get_element_by_id() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::Query {
        request_id: 100,
        query: QueryType::GetElementById {
            id: "test".to_string(),
        },
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::QueryResult { request_id, result } => {
            assert_eq!(request_id, 100);
            matches!(result, QueryResultType::NotFound);
        }
        _ => panic!("Expected QueryResult response"),
    }
}

#[test]
fn test_query_selector() {
    let mut component = DomComponent::new(DomConfig::default());

    let msg = DomComponentMessage::Query {
        request_id: 200,
        query: QueryType::QuerySelector {
            selector: ".active".to_string(),
            context: None,
        },
    };

    let response = component.handle_message(msg);

    match response {
        DomComponentResponse::QueryResult { request_id, result } => {
            assert_eq!(request_id, 200);
            match result {
                QueryResultType::NodeIds(ids) => {
                    assert_eq!(ids.len(), 0);
                }
                _ => panic!("Expected NodeIds result"),
            }
        }
        _ => panic!("Expected QueryResult response"),
    }
}

#[test]
fn test_multiple_documents() {
    let mut component = DomComponent::new(DomConfig::default());

    // Create first document
    let msg1 = DomComponentMessage::ParsedDocument {
        request_id: 1,
        root: create_simple_parsed_tree(),
        doctype: None,
    };
    component.handle_message(msg1);

    // Create second document
    let msg2 = DomComponentMessage::ParsedDocument {
        request_id: 2,
        root: create_complex_parsed_tree(),
        doctype: None,
    };
    component.handle_message(msg2);

    // Create third document
    let msg3 = DomComponentMessage::ParsedDocument {
        request_id: 3,
        root: create_simple_parsed_tree(),
        doctype: None,
    };
    component.handle_message(msg3);

    // Verify all documents exist
    assert!(component.get_document(1).is_some());
    assert!(component.get_document(2).is_some());
    assert!(component.get_document(3).is_some());
    assert!(component.get_document(4).is_none());
}

#[test]
fn test_config_is_accessible() {
    let config = DomConfig {
        max_tree_depth: 256,
        max_children: 5000,
        enable_mutation_observers: false,
        enable_shadow_dom: true,
        gc_threshold: 50000,
        arena_capacity: 25000,
    };

    let component = DomComponent::new(config.clone());

    assert_eq!(component.config().max_tree_depth, 256);
    assert_eq!(component.config().max_children, 5000);
    assert!(!component.config().enable_mutation_observers);
    assert!(component.config().enable_shadow_dom);
}

#[test]
fn test_message_serialization_round_trip() {
    let msg = DomComponentMessage::ParsedDocument {
        request_id: 123,
        root: create_simple_parsed_tree(),
        doctype: Some(DocumentType {
            name: "html".to_string(),
            public_id: Some("-//W3C//DTD HTML 4.01//EN".to_string()),
            system_id: Some("http://www.w3.org/TR/html4/strict.dtd".to_string()),
        }),
    };

    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: DomComponentMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        DomComponentMessage::ParsedDocument {
            request_id,
            doctype,
            ..
        } => {
            assert_eq!(request_id, 123);
            assert!(doctype.is_some());
            let dt = doctype.unwrap();
            assert_eq!(dt.name, "html");
        }
        _ => panic!("Deserialization failed"),
    }
}

#[test]
fn test_response_serialization_round_trip() {
    let response = DomComponentResponse::DomMutated {
        mutations: vec![MutationRecord {
            mutation_type: MutationType::Attributes,
            target: 42,
            added_nodes: vec![],
            removed_nodes: vec![],
            previous_sibling: None,
            next_sibling: None,
            attribute_name: Some("class".to_string()),
            old_value: Some("old".to_string()),
        }],
        affected_nodes: vec![42],
    };

    let json = serde_json::to_string(&response).unwrap();
    let deserialized: DomComponentResponse = serde_json::from_str(&json).unwrap();

    match deserialized {
        DomComponentResponse::DomMutated {
            mutations,
            affected_nodes,
        } => {
            assert_eq!(mutations.len(), 1);
            assert_eq!(affected_nodes, vec![42]);
        }
        _ => panic!("Deserialization failed"),
    }
}
