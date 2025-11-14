//! Node type enumeration for DOM nodes.
//!
//! This module defines the [`NodeType`] enum which represents the different
//! types of nodes that can exist in a DOM tree, as per the DOM Level 4 specification.

/// Node type enumeration per DOM Level 4 specification.
///
/// Each variant has a numeric value matching the DOM standard:
/// - Element = 1
/// - Attribute = 2
/// - Text = 3
/// - CDataSection = 4
/// - ProcessingInstruction = 7
/// - Comment = 8
/// - Document = 9
/// - DocumentType = 10
/// - DocumentFragment = 11
///
/// # Examples
///
/// ```
/// use dom_types::NodeType;
///
/// let node_type = NodeType::Element;
/// assert_eq!(node_type as u16, 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u16)]
pub enum NodeType {
    /// Element node (e.g., `<div>`, `<p>`)
    Element = 1,

    /// Attribute node (deprecated in DOM4, but included for compatibility)
    Attribute = 2,

    /// Text node containing character data
    Text = 3,

    /// CDATA section node
    CDataSection = 4,

    /// Processing instruction node
    ProcessingInstruction = 7,

    /// Comment node
    Comment = 8,

    /// Document node (root of the tree)
    Document = 9,

    /// Document type node (DTD)
    DocumentType = 10,

    /// Document fragment node
    DocumentFragment = 11,
}

impl NodeType {
    /// Returns the numeric value of the node type as defined in the DOM specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::NodeType;
    ///
    /// assert_eq!(NodeType::Element.as_u16(), 1);
    /// assert_eq!(NodeType::Text.as_u16(), 3);
    /// ```
    pub fn as_u16(self) -> u16 {
        self as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_values() {
        assert_eq!(NodeType::Element as u16, 1);
        assert_eq!(NodeType::Attribute as u16, 2);
        assert_eq!(NodeType::Text as u16, 3);
        assert_eq!(NodeType::CDataSection as u16, 4);
        assert_eq!(NodeType::ProcessingInstruction as u16, 7);
        assert_eq!(NodeType::Comment as u16, 8);
        assert_eq!(NodeType::Document as u16, 9);
        assert_eq!(NodeType::DocumentType as u16, 10);
        assert_eq!(NodeType::DocumentFragment as u16, 11);
    }

    #[test]
    fn test_as_u16_method() {
        assert_eq!(NodeType::Element.as_u16(), 1);
        assert_eq!(NodeType::Text.as_u16(), 3);
        assert_eq!(NodeType::Document.as_u16(), 9);
    }
}
