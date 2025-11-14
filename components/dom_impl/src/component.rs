//! Main DOM component for message bus integration

use crate::{
    config::DomConfig, messages::*, Document, DocumentRef, Element, ElementRef, Node, NodeRef, Text,
};
use dom_types::{DocumentId, NodeId};
use std::collections::HashMap;
use std::sync::{Arc, Weak};

// Re-use RwLock from parking_lot via re-export
type RwLock<T> = parking_lot::RwLock<T>;

/// Weak reference to a node for the registry
pub type WeakNodeRef = Weak<RwLock<Box<dyn Node>>>;

/// Main DOM component that handles messages from other browser components
pub struct DomComponent {
    /// Map of document IDs to document references
    documents: HashMap<DocumentId, DocumentRef>,

    /// Global node registry (weak references to prevent memory leaks)
    node_registry: HashMap<NodeId, WeakNodeRef>,

    /// Configuration
    config: DomConfig,

    /// Next node ID to assign
    next_node_id: NodeId,

    /// Next document ID to assign
    next_document_id: DocumentId,
}

impl DomComponent {
    /// Create a new DOM component with the given configuration
    pub fn new(config: DomConfig) -> Self {
        Self {
            documents: HashMap::new(),
            node_registry: HashMap::new(),
            config,
            next_node_id: 1,
            next_document_id: 1,
        }
    }

    /// Handle an incoming message from the browser message bus
    pub fn handle_message(&mut self, msg: DomComponentMessage) -> DomComponentResponse {
        match msg {
            DomComponentMessage::ParsedDocument {
                request_id,
                root,
                doctype,
            } => self.handle_parsed_document(request_id, root, doctype),

            DomComponentMessage::ScriptManipulation {
                operation,
                node_id,
                params,
            } => self.handle_script_manipulation(operation, node_id, params),

            DomComponentMessage::StyleInvalidation { node_id, reason } => {
                self.handle_style_invalidation(node_id, reason)
            }

            DomComponentMessage::UserInteraction {
                event_type,
                target_id,
                event_data,
            } => self.handle_user_interaction(event_type, target_id, event_data),

            DomComponentMessage::Query { request_id, query } => {
                self.handle_query(request_id, query)
            }
        }
    }

    /// Handle a parsed document from the HTML parser
    fn handle_parsed_document(
        &mut self,
        request_id: u64,
        root: ParsedNode,
        _doctype: Option<DocumentType>,
    ) -> DomComponentResponse {
        // Create a new document
        let document = Document::new();
        let document_ref = Arc::new(RwLock::new(document));

        // Build the DOM tree from the parsed nodes
        if let Some(root_element) = self.build_dom_tree(&document_ref, root) {
            // Set the document element
            let mut doc = document_ref.write();
            doc.set_document_element(root_element);
        }

        // Assign document ID and store
        let document_id = self.next_document_id;
        self.next_document_id += 1;
        self.documents.insert(document_id, document_ref);

        DomComponentResponse::DomTreeReady {
            request_id,
            document_id,
        }
    }

    /// Build DOM tree from parsed nodes
    fn build_dom_tree(
        &mut self,
        _document: &DocumentRef,
        parsed: ParsedNode,
    ) -> Option<ElementRef> {
        match parsed.node_type {
            ParsedNodeType::Element => {
                let tag_name = parsed.tag_name.as_ref()?;

                // Create element (delegate to dom-core)
                let mut element = Element::new(tag_name.clone());

                // Set attributes
                for (name, value) in parsed.attributes {
                    let _ = element.set_attribute(&name, &value);
                }

                let element_ref = Arc::new(RwLock::new(element));

                // Register element with node ID (simplified - just track the ID)
                let _node_id = self.next_node_id;
                self.next_node_id += 1;

                // Process children recursively
                for child_parsed in parsed.children {
                    match child_parsed.node_type {
                        ParsedNodeType::Element => {
                            if let Some(child_element) =
                                self.build_dom_tree(_document, child_parsed)
                            {
                                // Convert ElementRef to NodeRef
                                let child_node: NodeRef =
                                    Arc::new(RwLock::new(Box::new(child_element.read().clone())));

                                {
                                    let mut elem = element_ref.write();
                                    let _ = elem.append_child(child_node);
                                }
                            }
                        }
                        ParsedNodeType::Text => {
                            if let Some(text_content) = child_parsed.text_content {
                                let text = Text::new(text_content);
                                let text_node: NodeRef =
                                    Arc::new(RwLock::new(Box::new(text) as Box<dyn Node>));

                                {
                                    let mut elem = element_ref.write();
                                    let _ = elem.append_child(text_node);
                                }
                            }
                        }
                        _ => {
                            // Skip other node types for now
                        }
                    }
                }

                Some(element_ref)
            }
            _ => None,
        }
    }

    /// Handle script manipulation
    fn handle_script_manipulation(
        &mut self,
        operation: DomOperation,
        _node_id: NodeId,
        _params: OperationParams,
    ) -> DomComponentResponse {
        match operation {
            DomOperation::AppendChild {
                parent_id,
                child_id,
            } => {
                // In a real implementation, we would:
                // 1. Look up parent and child from node_registry
                // 2. Perform appendChild operation
                // 3. Generate mutation records
                // For now, return a placeholder response

                DomComponentResponse::DomMutated {
                    mutations: vec![MutationRecord {
                        mutation_type: MutationType::ChildList,
                        target: parent_id,
                        added_nodes: vec![child_id],
                        removed_nodes: vec![],
                        previous_sibling: None,
                        next_sibling: None,
                        attribute_name: None,
                        old_value: None,
                    }],
                    affected_nodes: vec![parent_id, child_id],
                }
            }

            DomOperation::SetAttribute {
                element_id,
                name,
                value,
            } => {
                // Look up element and set attribute (delegating to dom-core)
                // Generate mutation record

                DomComponentResponse::DomMutated {
                    mutations: vec![MutationRecord {
                        mutation_type: MutationType::Attributes,
                        target: element_id,
                        added_nodes: vec![],
                        removed_nodes: vec![],
                        previous_sibling: None,
                        next_sibling: None,
                        attribute_name: Some(name),
                        old_value: Some(value),
                    }],
                    affected_nodes: vec![element_id],
                }
            }

            DomOperation::RemoveChild {
                parent_id,
                child_id,
            } => DomComponentResponse::DomMutated {
                mutations: vec![MutationRecord {
                    mutation_type: MutationType::ChildList,
                    target: parent_id,
                    added_nodes: vec![],
                    removed_nodes: vec![child_id],
                    previous_sibling: None,
                    next_sibling: None,
                    attribute_name: None,
                    old_value: None,
                }],
                affected_nodes: vec![parent_id, child_id],
            },

            DomOperation::RemoveAttribute { element_id, name } => {
                DomComponentResponse::DomMutated {
                    mutations: vec![MutationRecord {
                        mutation_type: MutationType::Attributes,
                        target: element_id,
                        added_nodes: vec![],
                        removed_nodes: vec![],
                        previous_sibling: None,
                        next_sibling: None,
                        attribute_name: Some(name),
                        old_value: None,
                    }],
                    affected_nodes: vec![element_id],
                }
            }

            DomOperation::SetTextContent { node_id, text: _ } => DomComponentResponse::DomMutated {
                mutations: vec![MutationRecord {
                    mutation_type: MutationType::CharacterData,
                    target: node_id,
                    added_nodes: vec![],
                    removed_nodes: vec![],
                    previous_sibling: None,
                    next_sibling: None,
                    attribute_name: None,
                    old_value: None,
                }],
                affected_nodes: vec![node_id],
            },

            DomOperation::CreateElement { tag_name, .. } => {
                // Create element and register
                let _element = Element::new(tag_name);
                let element_id = self.next_node_id;
                self.next_node_id += 1;

                // Return success (simplified)
                DomComponentResponse::QueryResult {
                    request_id: 0,
                    result: QueryResultType::NodeIds(vec![element_id]),
                }
            }

            DomOperation::CreateTextNode { .. } => {
                let node_id = self.next_node_id;
                self.next_node_id += 1;

                DomComponentResponse::QueryResult {
                    request_id: 0,
                    result: QueryResultType::NodeIds(vec![node_id]),
                }
            }
        }
    }

    /// Handle style invalidation
    fn handle_style_invalidation(
        &mut self,
        node_id: NodeId,
        _reason: InvalidationReason,
    ) -> DomComponentResponse {
        // Notify rendering engine that layout needs invalidation
        DomComponentResponse::LayoutInvalidated {
            root_id: node_id,
            invalidation_type: LayoutInvalidationType::Incremental,
        }
    }

    /// Handle user interaction
    fn handle_user_interaction(
        &mut self,
        _event_type: String,
        target_id: NodeId,
        _event_data: EventData,
    ) -> DomComponentResponse {
        // In a real implementation, we would dispatch events through the event system
        // For now, return a simple response indicating the tree might have changed

        DomComponentResponse::DomTreeChanged {
            change_type: TreeChangeType::SubtreeModified,
            affected_subtree: target_id,
            needs_style_recalc: false,
        }
    }

    /// Handle query
    fn handle_query(&mut self, request_id: u64, query: QueryType) -> DomComponentResponse {
        match query {
            QueryType::GetElementById { id: _ } => {
                // In a real implementation, search through documents for element with this ID
                // For now, return not found

                DomComponentResponse::QueryResult {
                    request_id,
                    result: QueryResultType::NotFound,
                }
            }

            QueryType::QuerySelector {
                selector: _,
                context: _,
            } => {
                // In a real implementation, use dom-selectors to query
                // For now, return empty results

                DomComponentResponse::QueryResult {
                    request_id,
                    result: QueryResultType::NodeIds(vec![]),
                }
            }

            QueryType::GetNodeProperties { node_id: _ } => {
                // In a real implementation, look up node and return properties
                // For now, return empty

                DomComponentResponse::QueryResult {
                    request_id,
                    result: QueryResultType::NodeProperties(HashMap::new()),
                }
            }
        }
    }

    /// Get a document by ID
    pub fn get_document(&self, document_id: DocumentId) -> Option<&DocumentRef> {
        self.documents.get(&document_id)
    }

    /// Get configuration
    pub fn config(&self) -> &DomConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_parsed_tree() -> ParsedNode {
        ParsedNode {
            node_type: ParsedNodeType::Element,
            tag_name: Some("html".to_string()),
            attributes: HashMap::new(),
            text_content: None,
            children: vec![ParsedNode {
                node_type: ParsedNodeType::Element,
                tag_name: Some("body".to_string()),
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("class".to_string(), "main".to_string());
                    attrs
                },
                text_content: None,
                children: vec![ParsedNode {
                    node_type: ParsedNodeType::Text,
                    tag_name: None,
                    attributes: HashMap::new(),
                    text_content: Some("Hello World".to_string()),
                    children: vec![],
                }],
            }],
        }
    }

    #[test]
    fn test_component_creation() {
        let component = DomComponent::new(DomConfig::default());
        assert_eq!(component.documents.len(), 0);
        assert_eq!(component.next_node_id, 1);
        assert_eq!(component.next_document_id, 1);
    }

    #[test]
    fn test_handle_parsed_document() {
        let mut component = DomComponent::new(DomConfig::default());

        let parsed = create_test_parsed_tree();
        let msg = DomComponentMessage::ParsedDocument {
            request_id: 123,
            root: parsed,
            doctype: None,
        };

        let response = component.handle_message(msg);

        match response {
            DomComponentResponse::DomTreeReady {
                request_id,
                document_id,
            } => {
                assert_eq!(request_id, 123);
                assert_eq!(document_id, 1);
            }
            _ => panic!("Expected DomTreeReady response"),
        }

        // Verify document was created
        assert_eq!(component.documents.len(), 1);
        assert!(component.get_document(1).is_some());
    }

    #[test]
    fn test_handle_script_manipulation_set_attribute() {
        let mut component = DomComponent::new(DomConfig::default());

        let msg = DomComponentMessage::ScriptManipulation {
            operation: DomOperation::SetAttribute {
                element_id: 42,
                name: "class".to_string(),
                value: "active".to_string(),
            },
            node_id: 42,
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
                assert_eq!(mutations[0].target, 42);
                assert_eq!(affected_nodes, vec![42]);
            }
            _ => panic!("Expected DomMutated response"),
        }
    }

    #[test]
    fn test_handle_script_manipulation_append_child() {
        let mut component = DomComponent::new(DomConfig::default());

        let msg = DomComponentMessage::ScriptManipulation {
            operation: DomOperation::AppendChild {
                parent_id: 10,
                child_id: 20,
            },
            node_id: 10,
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
                assert_eq!(mutations[0].target, 10);
                assert_eq!(mutations[0].added_nodes, vec![20]);
                assert_eq!(affected_nodes, vec![10, 20]);
            }
            _ => panic!("Expected DomMutated response"),
        }
    }

    #[test]
    fn test_handle_style_invalidation() {
        let mut component = DomComponent::new(DomConfig::default());

        let msg = DomComponentMessage::StyleInvalidation {
            node_id: 100,
            reason: InvalidationReason::ClassChanged,
        };

        let response = component.handle_message(msg);

        match response {
            DomComponentResponse::LayoutInvalidated {
                root_id,
                invalidation_type,
            } => {
                assert_eq!(root_id, 100);
                matches!(invalidation_type, LayoutInvalidationType::Incremental);
            }
            _ => panic!("Expected LayoutInvalidated response"),
        }
    }

    #[test]
    fn test_handle_query() {
        let mut component = DomComponent::new(DomConfig::default());

        let msg = DomComponentMessage::Query {
            request_id: 456,
            query: QueryType::GetElementById {
                id: "test".to_string(),
            },
        };

        let response = component.handle_message(msg);

        match response {
            DomComponentResponse::QueryResult { request_id, result } => {
                assert_eq!(request_id, 456);
                matches!(result, QueryResultType::NotFound);
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
            root: create_test_parsed_tree(),
            doctype: None,
        };
        component.handle_message(msg1);

        // Create second document
        let msg2 = DomComponentMessage::ParsedDocument {
            request_id: 2,
            root: create_test_parsed_tree(),
            doctype: None,
        };
        component.handle_message(msg2);

        // Verify both documents exist
        assert_eq!(component.documents.len(), 2);
        assert!(component.get_document(1).is_some());
        assert!(component.get_document(2).is_some());
    }
}
