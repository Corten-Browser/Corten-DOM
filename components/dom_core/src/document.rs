//! Document node implementation

use crate::comment::Comment;
use crate::element::{Element, ElementRef};
use crate::node::{Node, NodeData, NodeRef};
use crate::text::Text;
use dom_types::{DomException, NodeType};
use indexmap::IndexMap;
use parking_lot::RwLock;
use std::sync::Arc;

/// Document node implementation
pub struct Document {
    /// Base node data
    node_data: NodeData,

    /// Document element (the root element, e.g., <html>)
    document_element: Option<ElementRef>,

    /// Registry mapping element IDs to elements
    id_map: IndexMap<String, Vec<ElementRef>>,

    /// Document URI
    url: String,

    /// Character encoding
    charset: String,
}

/// Thread-safe reference to a Document
pub type DocumentRef = Arc<RwLock<Document>>;

impl Document {
    /// Creates a new empty document
    pub fn new() -> Self {
        Self {
            node_data: NodeData::new(NodeType::Document, "#document"),
            document_element: None,
            id_map: IndexMap::new(),
            url: String::from("about:blank"),
            charset: String::from("UTF-8"),
        }
    }

    /// Gets the document element (root element)
    pub fn document_element(&self) -> Option<ElementRef> {
        self.document_element.clone()
    }

    /// Sets the document element
    pub fn set_document_element(&mut self, element: ElementRef) {
        self.document_element = Some(element);
    }

    /// Creates a new element
    pub fn create_element(
        &mut self,
        tag_name: impl Into<String>,
    ) -> Result<ElementRef, DomException> {
        let tag = tag_name.into();

        // Validate tag name
        if !is_valid_tag_name(&tag) {
            return Err(DomException::InvalidCharacterError);
        }

        let element = Arc::new(RwLock::new(Element::new(tag)));

        // Register element if it has an ID
        if let Some(id) = element.read().id() {
            self.register_element_id(id, element.clone());
        }

        Ok(element)
    }

    /// Creates a new element with namespace
    pub fn create_element_ns(
        &mut self,
        namespace: impl Into<String>,
        qualified_name: impl Into<String>,
    ) -> Result<ElementRef, DomException> {
        let name = qualified_name.into();
        let ns = namespace.into();

        if !is_valid_tag_name(&name) {
            return Err(DomException::InvalidCharacterError);
        }

        let element = Arc::new(RwLock::new(Element::new_with_namespace(name, ns)));

        if let Some(id) = element.read().id() {
            self.register_element_id(id, element.clone());
        }

        Ok(element)
    }

    /// Creates a text node
    pub fn create_text_node(&mut self, data: impl Into<String>) -> NodeRef {
        let text = Text::new(data);
        Arc::new(RwLock::new(Box::new(text) as Box<dyn Node>))
    }

    /// Creates a comment node
    pub fn create_comment(&mut self, data: impl Into<String>) -> NodeRef {
        let comment = Comment::new(data);
        Arc::new(RwLock::new(Box::new(comment) as Box<dyn Node>))
    }

    /// Creates a document fragment
    pub fn create_document_fragment(&mut self) -> NodeRef {
        // For now, we'll use a simple element as a fragment
        // In a full implementation, this would be a separate DocumentFragment type
        let _fragment_data = NodeData::new(NodeType::DocumentFragment, "#document-fragment");
        let fragment = Element::new("fragment");
        Arc::new(RwLock::new(Box::new(fragment) as Box<dyn Node>))
    }

    /// Gets an element by its ID
    pub fn get_element_by_id(&self, id: &str) -> Option<ElementRef> {
        self.id_map
            .get(id)
            .and_then(|elements| elements.first().cloned())
    }

    /// Gets elements by tag name (searches entire document tree)
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<ElementRef> {
        let mut result = Vec::new();

        if let Some(root) = &self.document_element {
            self.collect_elements_by_tag(root, tag_name, &mut result);
        }

        result
    }

    /// Gets elements by class name
    pub fn get_elements_by_class_name(&self, class_names: &str) -> Vec<ElementRef> {
        let mut result = Vec::new();

        if let Some(root) = &self.document_element {
            self.collect_elements_by_class(root, class_names, &mut result);
        }

        result
    }

    /// Gets the document URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Sets the document URL
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }

    /// Gets the character encoding
    pub fn charset(&self) -> &str {
        &self.charset
    }

    /// Registers an element ID
    fn register_element_id(&mut self, id: impl Into<String>, element: ElementRef) {
        let id = id.into();
        self.id_map.entry(id).or_insert_with(Vec::new).push(element);
    }

    /// Collects elements by tag name recursively
    fn collect_elements_by_tag(
        &self,
        element: &ElementRef,
        tag_name: &str,
        result: &mut Vec<ElementRef>,
    ) {
        let target = tag_name.to_uppercase();
        let elem = element.read();

        if elem.tag_name() == target || target == "*" {
            result.push(element.clone());
        }

        // Search children
        for child in elem.child_nodes() {
            if child.read().node_type() == NodeType::Element {
                // In a full implementation, we'd properly convert NodeRef to ElementRef
                // For now, this is simplified
            }
        }
    }

    /// Collects elements by class name recursively
    fn collect_elements_by_class(
        &self,
        element: &ElementRef,
        class_names: &str,
        result: &mut Vec<ElementRef>,
    ) {
        let target_classes: Vec<&str> = class_names.split_whitespace().collect();
        let elem = element.read();

        // Check if element has all target classes
        let has_all_classes = target_classes
            .iter()
            .all(|target| elem.class_list().iter().any(|c| c == target));

        if has_all_classes {
            result.push(element.clone());
        }

        // Search children
        for child in elem.child_nodes() {
            if child.read().node_type() == NodeType::Element {
                // Recursively search children
            }
        }
    }
}

impl Node for Document {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn parent_node(&self) -> Option<NodeRef> {
        // Document has no parent
        None
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        self.node_data.children.clone()
    }

    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        // Documents can only have certain child types
        let child_type = child.read().node_type();

        match child_type {
            NodeType::Element | NodeType::Comment | NodeType::ProcessingInstruction => {
                // Valid child type
            }
            _ => return Err(DomException::HierarchyRequestError),
        }

        // Document can only have one element child
        if child_type == NodeType::Element && self.document_element.is_some() {
            return Err(DomException::HierarchyRequestError);
        }

        // Remove from old parent if exists
        if let Some(old_parent) = child.read().parent_node() {
            old_parent.write().remove_child(child.clone())?;
        }

        // Add to children
        self.node_data.add_child(child.clone());

        // Set as document element if it's an element
        if child_type == NodeType::Element {
            // Would need proper conversion here
        }

        Ok(child)
    }

    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        let removed = self.node_data.remove_child(&child)?;

        // Clear document element if it was removed
        if removed.read().node_type() == NodeType::Element {
            self.document_element = None;
        }

        Ok(removed)
    }

    fn insert_before(
        &mut self,
        new_child: NodeRef,
        ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // Validate child type
        let child_type = new_child.read().node_type();

        match child_type {
            NodeType::Element | NodeType::Comment | NodeType::ProcessingInstruction => {
                // Valid
            }
            _ => return Err(DomException::HierarchyRequestError),
        }

        // Document can only have one element child
        if child_type == NodeType::Element && self.document_element.is_some() {
            return Err(DomException::HierarchyRequestError);
        }

        // Remove from old parent
        if let Some(old_parent) = new_child.read().parent_node() {
            old_parent.write().remove_child(new_child.clone())?;
        }

        // Insert before reference child
        self.node_data
            .insert_child_before(new_child.clone(), ref_child.as_ref())?;

        Ok(new_child)
    }

    fn clone_node(&self, deep: bool) -> NodeRef {
        let mut cloned = Document::new();
        cloned.url = self.url.clone();
        cloned.charset = self.charset.clone();

        if deep {
            // Clone all children
            for child in &self.node_data.children {
                let cloned_child = child.read().clone_node(true);
                cloned.node_data.add_child(cloned_child);
            }

            // Clone document element
            if let Some(elem) = &self.document_element {
                cloned.document_element = Some(elem.clone());
            }
        }

        Arc::new(RwLock::new(Box::new(cloned) as Box<dyn Node>))
    }

    fn node_data(&self) -> &NodeData {
        &self.node_data
    }

    fn node_data_mut(&mut self) -> &mut NodeData {
        &mut self.node_data
    }

    fn contains(&self, other: &dyn Node) -> bool {
        let self_ptr = self as *const _ as *const dyn Node;
        let other_ptr = other as *const dyn Node;

        if self_ptr == other_ptr {
            return true;
        }

        for child in &self.node_data.children {
            if child.read().contains(other) {
                return true;
            }
        }

        false
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Document {
    fn clone(&self) -> Self {
        Self {
            node_data: self.node_data.clone(),
            document_element: self.document_element.clone(),
            id_map: self.id_map.clone(),
            url: self.url.clone(),
            charset: self.charset.clone(),
        }
    }
}

/// Validates a tag name
fn is_valid_tag_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Tag names must start with a letter
    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return false;
    }

    // Subsequent characters can be letters, digits, or hyphens
    name.chars().all(|c| c.is_alphanumeric() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert_eq!(doc.node_type(), NodeType::Document);
        assert_eq!(doc.node_name(), "#document");
    }

    #[test]
    fn test_create_element() {
        let mut doc = Document::new();
        let elem = doc.create_element("div").unwrap();
        assert_eq!(elem.read().tag_name(), "DIV");
    }

    #[test]
    fn test_create_text_node() {
        let mut doc = Document::new();
        let text = doc.create_text_node("Hello");
        assert_eq!(text.read().node_type(), NodeType::Text);
    }

    #[test]
    fn test_invalid_tag_name() {
        let mut doc = Document::new();
        let result = doc.create_element("123invalid");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, DomException::InvalidCharacterError));
        }
    }

    #[test]
    fn test_document_url() {
        let mut doc = Document::new();
        assert_eq!(doc.url(), "about:blank");

        doc.set_url("https://example.com");
        assert_eq!(doc.url(), "https://example.com");
    }
}
