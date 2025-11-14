//! Element node implementation

use crate::attr::{Attr, AttrRef};
use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use indexmap::IndexMap;
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

/// Element node implementation
#[derive(Debug)]
pub struct Element {
    /// Base node data
    node_data: NodeData,

    /// Element tag name (always uppercase)
    tag_name: String,

    /// Element namespace (e.g., "http://www.w3.org/1999/xhtml")
    namespace: Option<String>,

    /// Element attributes (preserves insertion order)
    attributes: IndexMap<String, String>,

    /// CSS class list (space-separated class names)
    class_list: Vec<String>,

    /// Element ID (if any)
    id: Option<String>,

    /// Self-reference for attribute owner tracking (weak to avoid cycles)
    self_ref: Option<Weak<RwLock<Element>>>,
}

/// Thread-safe reference to an Element
pub type ElementRef = Arc<RwLock<Element>>;

impl Element {
    /// Creates a new element with the given tag name
    pub fn new(tag_name: impl Into<String>) -> Self {
        let tag = tag_name.into().to_uppercase();
        Self {
            node_data: NodeData::new(NodeType::Element, tag.clone()),
            tag_name: tag,
            namespace: None,
            attributes: IndexMap::new(),
            class_list: Vec::new(),
            id: None,
            self_ref: None,
        }
    }

    /// Creates a new element with namespace
    pub fn new_with_namespace(tag_name: impl Into<String>, namespace: impl Into<String>) -> Self {
        let tag = tag_name.into().to_uppercase();
        Self {
            node_data: NodeData::new(NodeType::Element, tag.clone()),
            tag_name: tag,
            namespace: Some(namespace.into()),
            attributes: IndexMap::new(),
            class_list: Vec::new(),
            id: None,
            self_ref: None,
        }
    }

    /// Sets the self-reference for this element (called after wrapping in Arc<RwLock<>>)
    pub fn set_self_ref(&mut self, self_ref: Weak<RwLock<Element>>) {
        self.self_ref = Some(self_ref);
    }

    /// Gets the tag name (always uppercase)
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }

    /// Gets the namespace URI
    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Gets an attribute value
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Sets an attribute
    pub fn set_attribute(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), DomException> {
        let name = name.into();
        let value = value.into();

        // Validate attribute name
        if !is_valid_attribute_name(&name) {
            return Err(DomException::InvalidCharacterError);
        }

        // Handle special attributes
        if name == "class" {
            self.update_class_list(&value);
        } else if name == "id" {
            self.id = Some(value.clone());
        }

        self.attributes.insert(name, value);
        Ok(())
    }

    /// Removes an attribute
    pub fn remove_attribute(&mut self, name: &str) -> Result<(), DomException> {
        if name == "class" {
            self.class_list.clear();
        } else if name == "id" {
            self.id = None;
        }

        self.attributes.shift_remove(name);
        Ok(())
    }

    /// Checks if an attribute exists
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    /// Gets all attributes
    pub fn attributes(&self) -> &IndexMap<String, String> {
        &self.attributes
    }

    /// Gets an attribute node by name
    pub fn get_attribute_node(&self, name: &str) -> Option<AttrRef> {
        // Check if attribute exists in the attributes map
        let value = self.attributes.get(name)?;

        // Create a new Attr with the name and value
        let mut attr = Attr::new(name, value);

        // Set the owner element weak reference if we have self_ref
        if let Some(ref self_weak) = self.self_ref {
            attr.set_owner_element(Some(self_weak.clone()));
        }

        let attr_ref = Arc::new(RwLock::new(attr));

        Some(attr_ref)
    }

    /// Sets an attribute node
    ///
    /// # Errors
    /// Returns `DomException::InvalidStateError` if the attribute is already
    /// owned by a different element.
    pub fn set_attribute_node(
        &mut self,
        attr: AttrRef,
    ) -> Result<Option<AttrRef>, DomException> {
        let attr_guard = attr.read();
        let attr_name = attr_guard.name().to_string();
        let attr_value = attr_guard.value().to_string();

        // Check if attr already has an owner element
        if let Some(owner) = attr_guard.owner_element() {
            // Check if it's a different element by comparing weak pointers
            if let Some(ref self_weak) = self.self_ref {
                // Try to upgrade self_weak to compare
                if let Some(self_arc) = self_weak.upgrade() {
                    // Compare Arc pointers
                    if !Arc::ptr_eq(&owner, &self_arc) {
                        return Err(DomException::InvalidStateError);
                    }
                }
            } else {
                // If we don't have self_ref but attr has an owner, it must be a different element
                return Err(DomException::InvalidStateError);
            }
        }
        drop(attr_guard);

        // Get the old attribute value if it exists
        let old_attr = if self.attributes.contains_key(&attr_name) {
            // Create an Attr node for the old attribute
            let old_value = self.attributes.get(&attr_name).unwrap();
            let mut old_attr_node = Attr::new(&attr_name, old_value);
            // Set owner element on old attr if we have self_ref
            if let Some(ref self_weak) = self.self_ref {
                old_attr_node.set_owner_element(Some(self_weak.clone()));
            }
            Some(Arc::new(RwLock::new(old_attr_node)))
        } else {
            None
        };

        // Set the attribute using the existing set_attribute method
        self.set_attribute(&attr_name, &attr_value)?;

        // Set the owner element on the new attr
        if let Some(ref self_weak) = self.self_ref {
            attr.write().set_owner_element(Some(self_weak.clone()));
        }

        Ok(old_attr)
    }

    /// Gets the class list
    pub fn class_list(&self) -> &[String] {
        &self.class_list
    }

    /// Gets the element ID
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Gets elements by tag name (returns descendants matching tag)
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<ElementRef> {
        let mut result = Vec::new();
        let target = tag_name.to_uppercase();

        for child in &self.node_data.children {
            if child.read().node_type() == NodeType::Element {
                let child_element = child.read();
                if child_element.node_name() == target || target == "*" {
                    // We need to return ElementRef, but we have NodeRef
                    // This is a simplified version - in a full implementation,
                    // we'd need proper type conversion
                    result.push(Arc::new(RwLock::new(self.clone())));
                }

                // Recursively search children
                // In full implementation, need proper Element extraction
            }
        }

        result
    }

    /// Gets elements by class name
    pub fn get_elements_by_class_name(&self, class_names: &str) -> Vec<ElementRef> {
        let _target_classes: Vec<&str> = class_names.split_whitespace().collect();
        let mut result = Vec::new();

        for child in &self.node_data.children {
            if child.read().node_type() == NodeType::Element {
                // Check if element has all target classes
                // Simplified implementation
                result.push(Arc::new(RwLock::new(self.clone())));
            }
        }

        result
    }

    /// Updates the class list from a space-separated string
    fn update_class_list(&mut self, class_str: &str) {
        self.class_list = class_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
    }

    /// Helper to get this element as a NodeRef
    fn as_node_ref(&self) -> NodeRef {
        Arc::new(RwLock::new(Box::new(self.clone()) as Box<dyn Node>))
    }
}

impl Node for Element {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        self.node_data.children.clone()
    }

    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        // 1. Check for circular reference by checking if child contains any of our ancestors
        // We simplify by not allowing the node to append itself or its descendants
        {
            let child_node = child.read();
            let self_ptr = self as *const _ as *const dyn Node;
            let child_ptr = &**child_node as *const dyn Node;

            if self_ptr == child_ptr {
                return Err(DomException::HierarchyRequestError);
            }
        }

        // 2. Remove from old parent if exists
        let old_parent = child.read().parent_node();
        if let Some(parent) = old_parent {
            parent.write().remove_child(child.clone())?;
        }

        // 3. Add to children
        self.node_data.add_child(child.clone());

        // 4. Set parent (create weak reference)
        let self_ref = self.as_node_ref();
        child
            .write()
            .node_data_mut()
            .set_parent(Some(Arc::downgrade(&self_ref)));

        Ok(child)
    }

    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        // Remove from children list
        let removed = self.node_data.remove_child(&child)?;

        // Clear parent reference
        removed.write().node_data_mut().set_parent(None);

        Ok(removed)
    }

    fn insert_before(
        &mut self,
        new_child: NodeRef,
        ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // 1. Check for circular reference (simple check)
        {
            let child_node = new_child.read();
            let self_ptr = self as *const _ as *const dyn Node;
            let child_ptr = &**child_node as *const dyn Node;

            if self_ptr == child_ptr {
                return Err(DomException::HierarchyRequestError);
            }
        }

        // 2. Remove from old parent if exists
        let old_parent = new_child.read().parent_node();
        if let Some(parent) = old_parent {
            parent.write().remove_child(new_child.clone())?;
        }

        // 3. Insert before reference child
        self.node_data
            .insert_child_before(new_child.clone(), ref_child.as_ref())?;

        // 4. Set parent
        let self_ref = self.as_node_ref();
        new_child
            .write()
            .node_data_mut()
            .set_parent(Some(Arc::downgrade(&self_ref)));

        Ok(new_child)
    }

    fn clone_node(&self, deep: bool) -> NodeRef {
        let mut cloned = self.clone();
        cloned.node_data.parent = None;

        if !deep {
            cloned.node_data.children.clear();
        } else {
            // Deep clone: clone all children recursively
            let cloned_children: Vec<NodeRef> = self
                .node_data
                .children
                .iter()
                .map(|child| child.read().clone_node(true))
                .collect();
            cloned.node_data.children = cloned_children;
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Self {
            node_data: self.node_data.clone(),
            tag_name: self.tag_name.clone(),
            namespace: self.namespace.clone(),
            attributes: self.attributes.clone(),
            class_list: self.class_list.clone(),
            id: self.id.clone(),
            self_ref: None, // Don't clone self-reference
        }
    }
}

/// Validates an attribute name
fn is_valid_attribute_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Attribute names must start with a letter or underscore
    let first_char = name.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return false;
    }

    // Subsequent characters can be letters, digits, hyphens, underscores
    name.chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let elem = Element::new("div");
        assert_eq!(elem.tag_name(), "DIV");
        assert_eq!(elem.node_type(), NodeType::Element);
    }

    #[test]
    fn test_attribute_operations() {
        let mut elem = Element::new("div");

        elem.set_attribute("id", "test").unwrap();
        assert_eq!(elem.get_attribute("id"), Some("test"));
        assert!(elem.has_attribute("id"));

        elem.remove_attribute("id").unwrap();
        assert_eq!(elem.get_attribute("id"), None);
        assert!(!elem.has_attribute("id"));
    }

    #[test]
    fn test_class_handling() {
        let mut elem = Element::new("div");

        elem.set_attribute("class", "foo bar baz").unwrap();
        assert_eq!(elem.class_list().len(), 3);
        assert!(elem.class_list().contains(&"foo".to_string()));
        assert!(elem.class_list().contains(&"bar".to_string()));
    }

    #[test]
    fn test_invalid_attribute_name() {
        let mut elem = Element::new("div");

        let result = elem.set_attribute("123invalid", "value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::InvalidCharacterError);
    }
}
