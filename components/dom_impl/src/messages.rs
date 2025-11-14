//! Message types for communication with other browser components

use dom_types::{DocumentId, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Messages received by the DOM component from other browser components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomComponentMessage {
    /// HTML Parser has parsed a document and is sending the parse tree
    ParsedDocument {
        /// Unique request identifier
        request_id: u64,
        /// Root node of the parsed tree
        root: ParsedNode,
        /// Document type declaration (if present)
        doctype: Option<DocumentType>,
    },

    /// JavaScript runtime is requesting a DOM manipulation
    ScriptManipulation {
        /// The DOM operation to perform
        operation: DomOperation,
        /// Target node ID
        node_id: NodeId,
        /// Operation parameters
        params: OperationParams,
    },

    /// CSS engine has detected a style change requiring invalidation
    StyleInvalidation {
        /// Node that needs style recalculation
        node_id: NodeId,
        /// Reason for invalidation
        reason: InvalidationReason,
    },

    /// User interaction event from the browser shell
    UserInteraction {
        /// Event type (e.g., "click", "input", "keypress")
        event_type: String,
        /// Target node ID
        target_id: NodeId,
        /// Event data
        event_data: EventData,
    },

    /// Query DOM state
    Query {
        /// Request ID for matching response
        request_id: u64,
        /// Query type
        query: QueryType,
    },
}

/// Responses sent by the DOM component to other browser components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomComponentResponse {
    /// DOM tree has been created and is ready
    DomTreeReady {
        /// Request ID from ParsedDocument message
        request_id: u64,
        /// Document ID assigned to the new document
        document_id: DocumentId,
    },

    /// DOM has been mutated (sent to JavaScript for MutationObserver)
    DomMutated {
        /// List of mutation records
        mutations: Vec<MutationRecord>,
        /// IDs of all affected nodes
        affected_nodes: Vec<NodeId>,
    },

    /// DOM tree structure changed (sent to CSS engine)
    DomTreeChanged {
        /// Type of change
        change_type: TreeChangeType,
        /// Root of affected subtree
        affected_subtree: NodeId,
        /// Whether style recalculation is needed
        needs_style_recalc: bool,
    },

    /// Layout needs to be invalidated (sent to rendering engine)
    LayoutInvalidated {
        /// Root node for layout invalidation
        root_id: NodeId,
        /// Type of layout invalidation
        invalidation_type: LayoutInvalidationType,
    },

    /// Response to a query
    QueryResult {
        /// Request ID from query message
        request_id: u64,
        /// Query result
        result: QueryResultType,
    },

    /// Error occurred
    Error {
        /// Error message
        message: String,
        /// Error code
        code: u32,
    },
}

/// Parsed node from HTML parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNode {
    /// Node type
    pub node_type: ParsedNodeType,
    /// Tag name (for elements)
    pub tag_name: Option<String>,
    /// Attributes (for elements)
    pub attributes: HashMap<String, String>,
    /// Text content (for text nodes)
    pub text_content: Option<String>,
    /// Child nodes
    pub children: Vec<ParsedNode>,
}

/// Type of parsed node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParsedNodeType {
    /// Element node
    Element,
    /// Text node
    Text,
    /// Comment node
    Comment,
    /// Document node
    Document,
}

/// Document type declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentType {
    /// Document type name
    pub name: String,
    /// Public ID
    pub public_id: Option<String>,
    /// System ID
    pub system_id: Option<String>,
}

/// DOM operation requested by JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomOperation {
    /// Append a child node
    AppendChild {
        /// Parent node ID
        parent_id: NodeId,
        /// Child node ID
        child_id: NodeId,
    },

    /// Remove a child node
    RemoveChild {
        /// Parent node ID
        parent_id: NodeId,
        /// Child node ID
        child_id: NodeId,
    },

    /// Set an attribute
    SetAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
        /// Attribute value
        value: String,
    },

    /// Remove an attribute
    RemoveAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
    },

    /// Set text content
    SetTextContent {
        /// Node ID
        node_id: NodeId,
        /// New text content
        text: String,
    },

    /// Create a new element
    CreateElement {
        /// Tag name
        tag_name: String,
        /// Namespace URI (if any)
        namespace: Option<String>,
    },

    /// Create a text node
    CreateTextNode {
        /// Text content
        text: String,
    },
}

/// Parameters for DOM operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationParams {
    /// Additional parameters
    pub extra: HashMap<String, String>,
}

/// Reason for style invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvalidationReason {
    /// Class attribute changed
    ClassChanged,
    /// ID attribute changed
    IdChanged,
    /// Style attribute changed
    StyleChanged,
    /// Other attribute changed
    AttributeChanged(String),
    /// Pseudoclass state changed
    PseudoClassChanged,
}

/// Event data from user interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Mouse coordinates (if applicable)
    pub mouse_x: Option<i32>,
    /// Mouse coordinates (if applicable)
    pub mouse_y: Option<i32>,
    /// Keyboard key (if applicable)
    pub key: Option<String>,
    /// Modifier keys
    pub modifiers: Modifiers,
    /// Additional event-specific data
    pub extra: HashMap<String, String>,
}

/// Modifier keys state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Modifiers {
    /// Shift key pressed
    pub shift: bool,
    /// Control key pressed
    pub ctrl: bool,
    /// Alt key pressed
    pub alt: bool,
    /// Meta/Command key pressed
    pub meta: bool,
}

/// Type of query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    /// Query selector
    QuerySelector {
        /// CSS selector
        selector: String,
        /// Context node (None = document)
        context: Option<NodeId>,
    },

    /// Get element by ID
    GetElementById {
        /// Element ID attribute
        id: String,
    },

    /// Get element properties
    GetNodeProperties {
        /// Node ID
        node_id: NodeId,
    },
}

/// Query result type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryResultType {
    /// Node IDs matching query
    NodeIds(Vec<NodeId>),

    /// Node properties
    NodeProperties(HashMap<String, String>),

    /// No result found
    NotFound,
}

/// Mutation record for MutationObserver
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    /// Type of mutation
    pub mutation_type: MutationType,
    /// Target node
    pub target: NodeId,
    /// Added nodes
    pub added_nodes: Vec<NodeId>,
    /// Removed nodes
    pub removed_nodes: Vec<NodeId>,
    /// Previous sibling
    pub previous_sibling: Option<NodeId>,
    /// Next sibling
    pub next_sibling: Option<NodeId>,
    /// Attribute name (for attribute mutations)
    pub attribute_name: Option<String>,
    /// Old value
    pub old_value: Option<String>,
}

/// Type of mutation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationType {
    /// Child list changed
    ChildList,
    /// Attributes changed
    Attributes,
    /// Character data changed
    CharacterData,
}

/// Type of tree change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeChangeType {
    /// Node inserted
    NodeInserted,
    /// Node removed
    NodeRemoved,
    /// Subtree modified
    SubtreeModified,
}

/// Type of layout invalidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutInvalidationType {
    /// Full layout needed
    Full,
    /// Incremental layout
    Incremental,
    /// Only reflow
    Reflow,
    /// Only repaint
    Repaint,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_node_creation() {
        let node = ParsedNode {
            node_type: ParsedNodeType::Element,
            tag_name: Some("div".to_string()),
            attributes: HashMap::new(),
            text_content: None,
            children: vec![],
        };

        assert_eq!(node.node_type, ParsedNodeType::Element);
        assert_eq!(node.tag_name, Some("div".to_string()));
    }

    #[test]
    fn test_message_serialization() {
        let msg = DomComponentMessage::Query {
            request_id: 123,
            query: QueryType::GetElementById {
                id: "test".to_string(),
            },
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: DomComponentMessage = serde_json::from_str(&json).unwrap();

        if let DomComponentMessage::Query { request_id, .. } = deserialized {
            assert_eq!(request_id, 123);
        } else {
            panic!("Deserialization failed");
        }
    }

    #[test]
    fn test_response_serialization() {
        let response = DomComponentResponse::DomTreeReady {
            request_id: 456,
            document_id: 789,
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: DomComponentResponse = serde_json::from_str(&json).unwrap();

        if let DomComponentResponse::DomTreeReady {
            request_id,
            document_id,
        } = deserialized
        {
            assert_eq!(request_id, 456);
            assert_eq!(document_id, 789);
        } else {
            panic!("Deserialization failed");
        }
    }

    #[test]
    fn test_dom_operation() {
        let op = DomOperation::SetAttribute {
            element_id: 100,
            name: "class".to_string(),
            value: "active".to_string(),
        };

        match op {
            DomOperation::SetAttribute { element_id, .. } => {
                assert_eq!(element_id, 100);
            }
            _ => panic!("Wrong operation type"),
        }
    }

    #[test]
    fn test_event_data() {
        let event = EventData {
            mouse_x: Some(100),
            mouse_y: Some(200),
            key: None,
            modifiers: Modifiers {
                shift: true,
                ctrl: false,
                alt: false,
                meta: false,
            },
            extra: HashMap::new(),
        };

        assert_eq!(event.mouse_x, Some(100));
        assert!(event.modifiers.shift);
    }
}
