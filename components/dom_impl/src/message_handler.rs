//! Message bus integration for DOM component communication
//!
//! This module provides a simplified message handling interface for direct
//! DOM operations. It complements the more comprehensive `messages` module
//! by providing a streamlined API for common operations.

use dom_types::NodeId;
use serde::{Deserialize, Serialize};

/// Simplified messages for direct DOM component communication
///
/// These messages provide a streamlined interface for common DOM operations,
/// complementing the more comprehensive `DomComponentMessage` enum in the
/// `messages` module.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectDomMessage {
    /// Create a new document
    CreateDocument {
        /// Optional URL for the document
        url: Option<String>,
    },

    /// Get element by ID
    GetElementById {
        /// Document containing the element
        document_id: NodeId,
        /// Element ID attribute to search for
        id: String,
    },

    /// Query selector to find elements matching a CSS selector
    QuerySelector {
        /// Root node to search from
        root_id: NodeId,
        /// CSS selector string
        selector: String,
    },

    /// Query selector all to find all elements matching a CSS selector
    QuerySelectorAll {
        /// Root node to search from
        root_id: NodeId,
        /// CSS selector string
        selector: String,
    },

    /// Append child node to parent
    AppendChild {
        /// Parent node ID
        parent_id: NodeId,
        /// Child node ID to append
        child_id: NodeId,
    },

    /// Remove child node from parent
    RemoveChild {
        /// Parent node ID
        parent_id: NodeId,
        /// Child node ID to remove
        child_id: NodeId,
    },

    /// Insert child before reference node
    InsertBefore {
        /// Parent node ID
        parent_id: NodeId,
        /// New child node ID
        new_child_id: NodeId,
        /// Reference node ID (insert before this)
        ref_child_id: Option<NodeId>,
    },

    /// Replace child node
    ReplaceChild {
        /// Parent node ID
        parent_id: NodeId,
        /// New child node ID
        new_child_id: NodeId,
        /// Old child node ID to replace
        old_child_id: NodeId,
    },

    /// Set attribute on element
    SetAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
        /// Attribute value
        value: String,
    },

    /// Get attribute from element
    GetAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
    },

    /// Remove attribute from element
    RemoveAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
    },

    /// Check if element has attribute
    HasAttribute {
        /// Element ID
        element_id: NodeId,
        /// Attribute name
        name: String,
    },

    /// Set text content of node
    SetTextContent {
        /// Node ID
        node_id: NodeId,
        /// Text content
        text: String,
    },

    /// Get text content of node
    GetTextContent {
        /// Node ID
        node_id: NodeId,
    },

    /// Dispatch event on target
    DispatchEvent {
        /// Target node ID
        target_id: NodeId,
        /// Event type (e.g., "click", "submit", "input")
        event_type: String,
    },

    /// Create a new element
    CreateElement {
        /// Document ID
        document_id: NodeId,
        /// Tag name
        tag_name: String,
    },

    /// Create a text node
    CreateTextNode {
        /// Document ID
        document_id: NodeId,
        /// Text content
        text: String,
    },

    /// Clone a node
    CloneNode {
        /// Node ID to clone
        node_id: NodeId,
        /// Whether to deep clone (include descendants)
        deep: bool,
    },
}

/// Responses from direct DOM message handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DirectDomResponse {
    /// Operation succeeded with optional node ID result
    Success {
        /// Node ID if the operation produced one (e.g., CreateDocument, CreateElement)
        node_id: Option<NodeId>,
    },

    /// Query result with multiple node IDs
    QueryResult {
        /// List of matching node IDs
        node_ids: Vec<NodeId>,
    },

    /// Attribute value result
    AttributeValue {
        /// Attribute value if it exists
        value: Option<String>,
    },

    /// Text content result
    TextContent {
        /// Text content of the node
        text: String,
    },

    /// Boolean result (e.g., for HasAttribute)
    BooleanResult {
        /// The boolean result
        result: bool,
    },

    /// Error response
    Error {
        /// Error message describing what went wrong
        message: String,
        /// Error code for programmatic handling
        code: DomErrorCode,
    },
}

/// DOM error codes for programmatic error handling
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DomErrorCode {
    /// Node not found in the DOM tree
    NodeNotFound = 1,
    /// Invalid node ID provided
    InvalidNodeId = 2,
    /// Operation not supported on this node type
    InvalidNodeType = 3,
    /// Hierarchy request error (e.g., inserting node into itself)
    HierarchyRequestError = 4,
    /// Wrong document error (nodes from different documents)
    WrongDocumentError = 5,
    /// Invalid selector syntax
    InvalidSelector = 6,
    /// Invalid character in attribute name or value
    InvalidCharacter = 7,
    /// Operation not implemented
    NotImplemented = 99,
    /// Unknown error
    Unknown = 100,
}

/// Handler for processing direct DOM messages
///
/// This handler provides a simplified interface for DOM operations.
/// For production use, it should be connected to the actual DOM
/// implementation via the `DomComponent`.
pub struct DomMessageHandler {
    /// Next node ID to assign for new nodes
    next_node_id: NodeId,
}

impl Default for DomMessageHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DomMessageHandler {
    /// Create a new message handler
    pub fn new() -> Self {
        Self { next_node_id: 1 }
    }

    /// Handle a direct DOM message and return a response
    ///
    /// This is a stub implementation that demonstrates the message handling
    /// interface. In production, this would connect to the actual DOM
    /// implementation through `DomComponent`.
    pub fn handle(&mut self, msg: DirectDomMessage) -> DirectDomResponse {
        match msg {
            DirectDomMessage::CreateDocument { .. } => {
                let node_id = self.allocate_node_id();
                DirectDomResponse::Success {
                    node_id: Some(node_id),
                }
            }

            DirectDomMessage::GetElementById { .. } => {
                // Stub: Would search document for element with matching ID
                DirectDomResponse::QueryResult { node_ids: vec![] }
            }

            DirectDomMessage::QuerySelector { .. } => {
                // Stub: Would use CSS selector engine to find first match
                DirectDomResponse::QueryResult { node_ids: vec![] }
            }

            DirectDomMessage::QuerySelectorAll { .. } => {
                // Stub: Would use CSS selector engine to find all matches
                DirectDomResponse::QueryResult { node_ids: vec![] }
            }

            DirectDomMessage::AppendChild { .. } => {
                // Stub: Would perform appendChild operation
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::RemoveChild { .. } => {
                // Stub: Would perform removeChild operation
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::InsertBefore { .. } => {
                // Stub: Would perform insertBefore operation
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::ReplaceChild { .. } => {
                // Stub: Would perform replaceChild operation
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::SetAttribute { .. } => {
                // Stub: Would set attribute on element
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::GetAttribute { .. } => {
                // Stub: Would get attribute from element
                DirectDomResponse::AttributeValue { value: None }
            }

            DirectDomMessage::RemoveAttribute { .. } => {
                // Stub: Would remove attribute from element
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::HasAttribute { .. } => {
                // Stub: Would check if element has attribute
                DirectDomResponse::BooleanResult { result: false }
            }

            DirectDomMessage::SetTextContent { .. } => {
                // Stub: Would set text content
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::GetTextContent { .. } => {
                // Stub: Would get text content
                DirectDomResponse::TextContent {
                    text: String::new(),
                }
            }

            DirectDomMessage::DispatchEvent { .. } => {
                // Stub: Would dispatch event through event system
                DirectDomResponse::Success { node_id: None }
            }

            DirectDomMessage::CreateElement { .. } => {
                let node_id = self.allocate_node_id();
                DirectDomResponse::Success {
                    node_id: Some(node_id),
                }
            }

            DirectDomMessage::CreateTextNode { .. } => {
                let node_id = self.allocate_node_id();
                DirectDomResponse::Success {
                    node_id: Some(node_id),
                }
            }

            DirectDomMessage::CloneNode { .. } => {
                let node_id = self.allocate_node_id();
                DirectDomResponse::Success {
                    node_id: Some(node_id),
                }
            }
        }
    }

    /// Allocate a new node ID
    fn allocate_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    /// Process multiple messages in batch
    pub fn handle_batch(&mut self, messages: Vec<DirectDomMessage>) -> Vec<DirectDomResponse> {
        messages.into_iter().map(|msg| self.handle(msg)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_document() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::CreateDocument {
            url: Some("https://example.com".to_string()),
        });

        match response {
            DirectDomResponse::Success { node_id } => {
                assert!(node_id.is_some());
                assert_eq!(node_id.unwrap(), 1);
            }
            _ => panic!("Expected Success response"),
        }
    }

    #[test]
    fn test_get_element_by_id() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::GetElementById {
            document_id: 1,
            id: "test".to_string(),
        });

        match response {
            DirectDomResponse::QueryResult { node_ids } => {
                assert!(node_ids.is_empty());
            }
            _ => panic!("Expected QueryResult response"),
        }
    }

    #[test]
    fn test_query_selector() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::QuerySelector {
            root_id: 1,
            selector: ".active".to_string(),
        });

        match response {
            DirectDomResponse::QueryResult { node_ids } => {
                assert!(node_ids.is_empty());
            }
            _ => panic!("Expected QueryResult response"),
        }
    }

    #[test]
    fn test_append_child() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::AppendChild {
            parent_id: 1,
            child_id: 2,
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_remove_child() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::RemoveChild {
            parent_id: 1,
            child_id: 2,
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_set_attribute() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::SetAttribute {
            element_id: 1,
            name: "class".to_string(),
            value: "active".to_string(),
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_get_attribute() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::GetAttribute {
            element_id: 1,
            name: "class".to_string(),
        });

        match response {
            DirectDomResponse::AttributeValue { value } => {
                assert!(value.is_none());
            }
            _ => panic!("Expected AttributeValue response"),
        }
    }

    #[test]
    fn test_has_attribute() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::HasAttribute {
            element_id: 1,
            name: "class".to_string(),
        });

        match response {
            DirectDomResponse::BooleanResult { result } => {
                assert!(!result);
            }
            _ => panic!("Expected BooleanResult response"),
        }
    }

    #[test]
    fn test_dispatch_event() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::DispatchEvent {
            target_id: 1,
            event_type: "click".to_string(),
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_create_element() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::CreateElement {
            document_id: 1,
            tag_name: "div".to_string(),
        });

        match response {
            DirectDomResponse::Success { node_id } => {
                assert!(node_id.is_some());
            }
            _ => panic!("Expected Success response"),
        }
    }

    #[test]
    fn test_create_text_node() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::CreateTextNode {
            document_id: 1,
            text: "Hello".to_string(),
        });

        match response {
            DirectDomResponse::Success { node_id } => {
                assert!(node_id.is_some());
            }
            _ => panic!("Expected Success response"),
        }
    }

    #[test]
    fn test_clone_node() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::CloneNode {
            node_id: 1,
            deep: true,
        });

        match response {
            DirectDomResponse::Success { node_id } => {
                assert!(node_id.is_some());
            }
            _ => panic!("Expected Success response"),
        }
    }

    #[test]
    fn test_node_id_allocation() {
        let mut handler = DomMessageHandler::new();

        // Create multiple documents to test ID allocation
        let r1 = handler.handle(DirectDomMessage::CreateDocument { url: None });
        let r2 = handler.handle(DirectDomMessage::CreateDocument { url: None });
        let r3 = handler.handle(DirectDomMessage::CreateDocument { url: None });

        match (r1, r2, r3) {
            (
                DirectDomResponse::Success { node_id: Some(id1) },
                DirectDomResponse::Success { node_id: Some(id2) },
                DirectDomResponse::Success { node_id: Some(id3) },
            ) => {
                assert_eq!(id1, 1);
                assert_eq!(id2, 2);
                assert_eq!(id3, 3);
            }
            _ => panic!("Expected Success responses with node IDs"),
        }
    }

    #[test]
    fn test_batch_handling() {
        let mut handler = DomMessageHandler::new();

        let messages = vec![
            DirectDomMessage::CreateDocument { url: None },
            DirectDomMessage::CreateElement {
                document_id: 1,
                tag_name: "div".to_string(),
            },
            DirectDomMessage::SetAttribute {
                element_id: 2,
                name: "id".to_string(),
                value: "test".to_string(),
            },
        ];

        let responses = handler.handle_batch(messages);
        assert_eq!(responses.len(), 3);
    }

    #[test]
    fn test_message_serialization() {
        let msg = DirectDomMessage::SetAttribute {
            element_id: 42,
            name: "class".to_string(),
            value: "active".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: DirectDomMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_response_serialization() {
        let response = DirectDomResponse::Success { node_id: Some(123) };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: DirectDomResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_error_response_serialization() {
        let response = DirectDomResponse::Error {
            message: "Node not found".to_string(),
            code: DomErrorCode::NodeNotFound,
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: DirectDomResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, deserialized);
    }

    #[test]
    fn test_default_handler() {
        let handler = DomMessageHandler::default();
        assert_eq!(handler.next_node_id, 1);
    }

    #[test]
    fn test_insert_before() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::InsertBefore {
            parent_id: 1,
            new_child_id: 2,
            ref_child_id: Some(3),
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_replace_child() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::ReplaceChild {
            parent_id: 1,
            new_child_id: 2,
            old_child_id: 3,
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_get_text_content() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::GetTextContent { node_id: 1 });

        match response {
            DirectDomResponse::TextContent { text } => {
                assert!(text.is_empty());
            }
            _ => panic!("Expected TextContent response"),
        }
    }

    #[test]
    fn test_set_text_content() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::SetTextContent {
            node_id: 1,
            text: "Hello World".to_string(),
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_remove_attribute() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::RemoveAttribute {
            element_id: 1,
            name: "class".to_string(),
        });

        assert!(matches!(response, DirectDomResponse::Success { .. }));
    }

    #[test]
    fn test_query_selector_all() {
        let mut handler = DomMessageHandler::new();

        let response = handler.handle(DirectDomMessage::QuerySelectorAll {
            root_id: 1,
            selector: "div".to_string(),
        });

        match response {
            DirectDomResponse::QueryResult { node_ids } => {
                assert!(node_ids.is_empty());
            }
            _ => panic!("Expected QueryResult response"),
        }
    }
}
