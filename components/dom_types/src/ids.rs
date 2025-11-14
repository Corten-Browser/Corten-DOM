//! Type aliases for DOM node and document identifiers.
//!
//! This module defines type aliases for unique identifiers used throughout
//! the DOM implementation to reference nodes and documents.

/// Unique identifier for a DOM node.
///
/// Each node in the DOM tree has a unique `NodeId` that can be used to
/// reference it efficiently. The ID is a 64-bit unsigned integer, providing
/// a large range of unique identifiers.
///
/// # Examples
///
/// ```
/// use dom_types::NodeId;
/// use std::collections::HashMap;
///
/// let mut nodes: HashMap<NodeId, String> = HashMap::new();
/// let id: NodeId = 42;
/// nodes.insert(id, "div".to_string());
///
/// assert_eq!(nodes.get(&42), Some(&"div".to_string()));
/// ```
pub type NodeId = u64;

/// Unique identifier for a document.
///
/// Each document has a unique `DocumentId` that distinguishes it from other
/// documents. Like `NodeId`, it's a 64-bit unsigned integer.
///
/// # Examples
///
/// ```
/// use dom_types::DocumentId;
///
/// let main_doc: DocumentId = 1;
/// let iframe_doc: DocumentId = 2;
///
/// assert_ne!(main_doc, iframe_doc);
/// ```
pub type DocumentId = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_type() {
        let id: NodeId = 123;
        assert_eq!(id, 123u64);
    }

    #[test]
    fn test_document_id_type() {
        let id: DocumentId = 456;
        assert_eq!(id, 456u64);
    }

    #[test]
    fn test_id_operations() {
        let mut node_id: NodeId = 1;
        node_id += 1;
        assert_eq!(node_id, 2);

        let doc_id: DocumentId = 10;
        let result = doc_id * 2;
        assert_eq!(result, 20);
    }
}
