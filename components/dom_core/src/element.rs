//! Element node implementation

use crate::attr::{Attr, AttrRef};
use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use indexmap::IndexMap;
use parking_lot::RwLock;
use std::sync::{Arc, Weak};

/// Key for namespaced attributes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NamespacedAttrKey {
    namespace: Option<String>,
    local_name: String,
}

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

    /// Namespaced attributes (namespace + localName -> qualified_name + value)
    namespaced_attributes: IndexMap<NamespacedAttrKey, (String, String)>,

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
            namespaced_attributes: IndexMap::new(),
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
            namespaced_attributes: IndexMap::new(),
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

    // ==================== Namespaced Attribute Operations ====================

    /// Gets a namespaced attribute value
    ///
    /// # Arguments
    /// * `namespace` - The namespace URI (None for no namespace)
    /// * `local_name` - The local name of the attribute
    ///
    /// # Returns
    /// The attribute value if found, None otherwise
    pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String> {
        let key = NamespacedAttrKey {
            namespace: namespace.map(|s| s.to_string()),
            local_name: local_name.to_string(),
        };

        self.namespaced_attributes.get(&key).map(|(_, value)| value.clone())
    }

    /// Sets a namespaced attribute
    ///
    /// # Arguments
    /// * `namespace` - The namespace URI (None for no namespace)
    /// * `qualified_name` - The qualified name (may include prefix like "prefix:localName")
    /// * `value` - The attribute value
    ///
    /// # Errors
    /// Returns `DomException::InvalidCharacterError` if the qualified name is invalid
    /// Returns `DomException::NamespaceError` if there's a namespace/prefix mismatch
    pub fn set_attribute_ns(
        &mut self,
        namespace: Option<&str>,
        qualified_name: &str,
        value: &str,
    ) -> Result<(), DomException> {
        // Validate qualified name
        if !is_valid_qualified_name(qualified_name) {
            return Err(DomException::InvalidCharacterError);
        }

        // Parse qualified name
        let (prefix, local_name) = parse_qualified_name(qualified_name);

        // Namespace validation
        // If prefix is Some, namespace must be Some
        if prefix.is_some() && namespace.is_none() {
            return Err(DomException::NamespaceError);
        }

        // "xml" prefix must be used with XML namespace
        if prefix.as_deref() == Some("xml")
            && namespace != Some("http://www.w3.org/XML/1998/namespace")
        {
            return Err(DomException::NamespaceError);
        }

        // "xmlns" prefix must be used with XMLNS namespace
        if prefix.as_deref() == Some("xmlns")
            && namespace != Some("http://www.w3.org/2000/xmlns/")
        {
            return Err(DomException::NamespaceError);
        }

        // "xmlns" local name without prefix requires XMLNS namespace
        if local_name == "xmlns" && prefix.is_none()
            && namespace != Some("http://www.w3.org/2000/xmlns/")
        {
            return Err(DomException::NamespaceError);
        }

        let key = NamespacedAttrKey {
            namespace: namespace.map(|s| s.to_string()),
            local_name: local_name.to_string(),
        };

        self.namespaced_attributes
            .insert(key, (qualified_name.to_string(), value.to_string()));

        // Also store in regular attributes for compatibility
        self.attributes
            .insert(qualified_name.to_string(), value.to_string());

        Ok(())
    }

    /// Removes a namespaced attribute
    ///
    /// # Arguments
    /// * `namespace` - The namespace URI (None for no namespace)
    /// * `local_name` - The local name of the attribute
    pub fn remove_attribute_ns(
        &mut self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<(), DomException> {
        let key = NamespacedAttrKey {
            namespace: namespace.map(|s| s.to_string()),
            local_name: local_name.to_string(),
        };

        // Get the qualified name before removing
        if let Some((qualified_name, _)) = self.namespaced_attributes.shift_remove(&key) {
            // Also remove from regular attributes
            self.attributes.shift_remove(&qualified_name);
        }

        Ok(())
    }

    /// Checks if a namespaced attribute exists
    ///
    /// # Arguments
    /// * `namespace` - The namespace URI (None for no namespace)
    /// * `local_name` - The local name of the attribute
    pub fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool {
        let key = NamespacedAttrKey {
            namespace: namespace.map(|s| s.to_string()),
            local_name: local_name.to_string(),
        };

        self.namespaced_attributes.contains_key(&key)
    }

    /// Gets a namespaced attribute node
    ///
    /// # Arguments
    /// * `namespace` - The namespace URI (None for no namespace)
    /// * `local_name` - The local name of the attribute
    pub fn get_attribute_node_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<AttrRef> {
        let key = NamespacedAttrKey {
            namespace: namespace.map(|s| s.to_string()),
            local_name: local_name.to_string(),
        };

        let (qualified_name, value) = self.namespaced_attributes.get(&key)?;

        // Create a namespaced Attr
        let attr = if let Some(ns) = namespace {
            Attr::new_ns(ns, qualified_name, value).ok()?
        } else {
            Attr::new(qualified_name, value)
        };

        let attr_ref = Arc::new(RwLock::new(attr));

        // Set the owner element weak reference if we have self_ref
        if let Some(ref self_weak) = self.self_ref {
            attr_ref.write().set_owner_element(Some(self_weak.clone()));
        }

        Some(attr_ref)
    }

    /// Sets a namespaced attribute node
    ///
    /// # Arguments
    /// * `attr` - The attribute node to set
    ///
    /// # Errors
    /// Returns `DomException::InvalidStateError` if the attribute is already
    /// owned by a different element
    pub fn set_attribute_node_ns(
        &mut self,
        attr: AttrRef,
    ) -> Result<Option<AttrRef>, DomException> {
        let attr_guard = attr.read();
        let namespace = attr_guard.namespace_uri().map(|s| s.to_string());
        let local_name = attr_guard.local_name().to_string();
        let qualified_name = attr_guard.name().to_string();
        let value = attr_guard.value().to_string();

        // Check if attr already has an owner element
        if let Some(owner) = attr_guard.owner_element() {
            if let Some(ref self_weak) = self.self_ref {
                if let Some(self_arc) = self_weak.upgrade() {
                    if !Arc::ptr_eq(&owner, &self_arc) {
                        return Err(DomException::InvalidStateError);
                    }
                }
            } else {
                return Err(DomException::InvalidStateError);
            }
        }
        drop(attr_guard);

        let key = NamespacedAttrKey {
            namespace: namespace.clone(),
            local_name: local_name.clone(),
        };

        // Get old attribute if it exists
        let old_attr = if let Some((old_qname, old_value)) = self.namespaced_attributes.get(&key) {
            let old_attr_node = if let Some(ref ns) = namespace {
                Attr::new_ns(ns, old_qname, old_value).ok()
            } else {
                Some(Attr::new(old_qname, old_value))
            };

            old_attr_node.map(|mut a| {
                if let Some(ref self_weak) = self.self_ref {
                    a.set_owner_element(Some(self_weak.clone()));
                }
                Arc::new(RwLock::new(a))
            })
        } else {
            None
        };

        // Set the new attribute
        self.namespaced_attributes
            .insert(key, (qualified_name.clone(), value.clone()));
        self.attributes.insert(qualified_name, value);

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
        // IMPORTANT: Don't try to read through self_ref here, as this method
        // may be called while holding a write lock on self (e.g., in append_child).
        // parking_lot RwLock doesn't support recursive locking, so we must
        // just clone self directly.
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

    fn text_content(&self) -> Option<String> {
        // For Element nodes, text_content returns the concatenation of all Text descendant text content
        let mut result = String::new();
        for child in &self.node_data.children {
            let child_guard = child.read();
            match child_guard.node_type() {
                NodeType::Text | NodeType::CDataSection => {
                    if let Some(text) = child_guard.text_content() {
                        result.push_str(&text);
                    }
                }
                NodeType::Element => {
                    if let Some(text) = child_guard.text_content() {
                        result.push_str(&text);
                    }
                }
                _ => {}
            }
        }
        Some(result)
    }

    fn set_text_content(&mut self, text: String) {
        // Clear all children first
        self.node_data.children.clear();

        // If text is not empty, create a Text node child
        if !text.is_empty() {
            let text_node = crate::Text::new(&text);
            let text_ref: NodeRef = Arc::new(RwLock::new(Box::new(text_node) as Box<dyn Node>));
            self.node_data.add_child(text_ref);
        }
    }

    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        // 1. Check for circular reference - can't append ourselves
        {
            let child_node = child.read();
            let self_ptr = self as *const _ as *const dyn Node;
            let child_ptr = &**child_node as *const dyn Node;

            if std::ptr::addr_eq(self_ptr, child_ptr) {
                return Err(DomException::HierarchyRequestError);
            }
        }

        // 2. Check if child is one of our ancestors (would create cycle)
        // We walk UP through our parent chain instead of DOWN through child's descendants
        // to avoid deadlock (we may hold locks on nodes that are descendants of child)
        {
            let child_ptr = &**child.read() as *const dyn Node;
            let mut current = self.node_data.get_parent();
            while let Some(ancestor) = current {
                let ancestor_guard = ancestor.read();
                let ancestor_ptr = &**ancestor_guard as *const dyn Node;
                if std::ptr::addr_eq(ancestor_ptr, child_ptr) {
                    return Err(DomException::HierarchyRequestError);
                }
                current = ancestor_guard.parent_node();
            }
        }

        // 3. Remove from old parent if exists
        let old_parent = child.read().parent_node();
        if let Some(parent) = old_parent {
            parent.write().remove_child(child.clone())?;
        }

        // 4. Add to children
        self.node_data.add_child(child.clone());

        // 5. Set parent using self_node_ref (the actual NodeRef that wraps us)
        if let Some(self_ref) = self.node_data.get_self_node_ref() {
            child
                .write()
                .node_data_mut()
                .set_parent(Some(Arc::downgrade(&self_ref)));
        }

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
        // 1. Check for circular reference - can't insert ourselves
        {
            let child_node = new_child.read();
            let self_ptr = self as *const _ as *const dyn Node;
            let child_ptr = &**child_node as *const dyn Node;

            if std::ptr::addr_eq(self_ptr, child_ptr) {
                return Err(DomException::HierarchyRequestError);
            }
        }

        // 2. Check if child is one of our ancestors (would create cycle)
        {
            let child_ptr = &**new_child.read() as *const dyn Node;
            let mut current = self.node_data.get_parent();
            while let Some(ancestor) = current {
                let ancestor_guard = ancestor.read();
                let ancestor_ptr = &**ancestor_guard as *const dyn Node;
                if std::ptr::addr_eq(ancestor_ptr, child_ptr) {
                    return Err(DomException::HierarchyRequestError);
                }
                current = ancestor_guard.parent_node();
            }
        }

        // 3. Remove from old parent if exists
        let old_parent = new_child.read().parent_node();
        if let Some(parent) = old_parent {
            parent.write().remove_child(new_child.clone())?;
        }

        // 4. Insert before reference child
        self.node_data
            .insert_child_before(new_child.clone(), ref_child.as_ref())?;

        // 5. Set parent using self_node_ref (the actual NodeRef that wraps us)
        if let Some(self_ref) = self.node_data.get_self_node_ref() {
            new_child
                .write()
                .node_data_mut()
                .set_parent(Some(Arc::downgrade(&self_ref)));
        }

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

        // Use addr_eq to compare only the data addresses, not vtable pointers
        if std::ptr::addr_eq(self_ptr, other_ptr) {
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
            namespaced_attributes: self.namespaced_attributes.clone(),
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

/// Validates a qualified name according to XML naming rules
fn is_valid_qualified_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Check for invalid patterns
    if name.starts_with(':') || name.ends_with(':') {
        return false;
    }

    // Count colons - should be 0 or 1
    let colon_count = name.chars().filter(|&c| c == ':').count();
    if colon_count > 1 {
        return false;
    }

    // Check for whitespace
    if name.contains(char::is_whitespace) {
        return false;
    }

    true
}

/// Parses a qualified name into prefix and local name
///
/// Returns (prefix, local_name) tuple where prefix is None if no colon found
fn parse_qualified_name(qualified_name: &str) -> (Option<String>, &str) {
    if let Some(colon_pos) = qualified_name.find(':') {
        let prefix = &qualified_name[..colon_pos];
        let local_name = &qualified_name[colon_pos + 1..];
        (Some(prefix.to_string()), local_name)
    } else {
        (None, qualified_name)
    }
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

    // ==================== Namespaced Attribute Tests ====================

    #[test]
    fn test_set_attribute_ns_basic() {
        let mut elem = Element::new("svg");

        elem.set_attribute_ns(
            Some("http://www.w3.org/1999/xlink"),
            "xlink:href",
            "#target",
        )
        .unwrap();

        assert!(elem.has_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"));
        assert_eq!(
            elem.get_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"),
            Some("#target".to_string())
        );
    }

    #[test]
    fn test_set_attribute_ns_no_namespace() {
        let mut elem = Element::new("div");

        elem.set_attribute_ns(None, "data-value", "123").unwrap();

        assert!(elem.has_attribute_ns(None, "data-value"));
        assert_eq!(
            elem.get_attribute_ns(None, "data-value"),
            Some("123".to_string())
        );
    }

    #[test]
    fn test_remove_attribute_ns() {
        let mut elem = Element::new("svg");

        elem.set_attribute_ns(
            Some("http://www.w3.org/1999/xlink"),
            "xlink:href",
            "#target",
        )
        .unwrap();

        assert!(elem.has_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"));

        elem.remove_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href")
            .unwrap();

        assert!(!elem.has_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"));
        assert_eq!(
            elem.get_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"),
            None
        );
    }

    #[test]
    fn test_set_attribute_ns_prefix_without_namespace_error() {
        let mut elem = Element::new("div");

        // Prefix requires namespace
        let result = elem.set_attribute_ns(None, "foo:bar", "value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::NamespaceError);
    }

    #[test]
    fn test_set_attribute_ns_xml_namespace() {
        let mut elem = Element::new("div");

        // xml prefix must use XML namespace
        let result = elem.set_attribute_ns(
            Some("http://wrong-namespace.com"),
            "xml:lang",
            "en",
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::NamespaceError);

        // Correct XML namespace
        let result = elem.set_attribute_ns(
            Some("http://www.w3.org/XML/1998/namespace"),
            "xml:lang",
            "en",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_attribute_ns_invalid_qualified_name() {
        let mut elem = Element::new("div");

        // Invalid: starts with colon
        let result = elem.set_attribute_ns(Some("http://example.com"), ":invalid", "value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::InvalidCharacterError);

        // Invalid: multiple colons
        let result = elem.set_attribute_ns(Some("http://example.com"), "a:b:c", "value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::InvalidCharacterError);
    }

    #[test]
    fn test_get_attribute_node_ns() {
        let mut elem = Element::new("svg");

        elem.set_attribute_ns(
            Some("http://www.w3.org/1999/xlink"),
            "xlink:href",
            "#target",
        )
        .unwrap();

        let attr = elem
            .get_attribute_node_ns(Some("http://www.w3.org/1999/xlink"), "href")
            .unwrap();

        let attr_guard = attr.read();
        assert_eq!(attr_guard.value(), "#target");
        assert_eq!(
            attr_guard.namespace_uri(),
            Some("http://www.w3.org/1999/xlink")
        );
        assert_eq!(attr_guard.prefix(), Some("xlink"));
        assert_eq!(attr_guard.local_name(), "href");
    }

    #[test]
    fn test_get_attribute_node_ns_not_found() {
        let elem = Element::new("div");

        let result = elem.get_attribute_node_ns(Some("http://example.com"), "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_set_attribute_node_ns() {
        let mut elem = Element::new("svg");

        let attr = Attr::new_ns(
            "http://www.w3.org/1999/xlink",
            "xlink:href",
            "#new-target",
        )
        .unwrap();
        let attr_ref = Arc::new(RwLock::new(attr));

        let result = elem.set_attribute_node_ns(attr_ref);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // No old attribute

        assert_eq!(
            elem.get_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"),
            Some("#new-target".to_string())
        );
    }

    #[test]
    fn test_set_attribute_node_ns_replaces_existing() {
        let mut elem = Element::new("svg");

        // Set initial attribute
        elem.set_attribute_ns(
            Some("http://www.w3.org/1999/xlink"),
            "xlink:href",
            "#original",
        )
        .unwrap();

        // Create new attribute to replace
        let new_attr = Attr::new_ns(
            "http://www.w3.org/1999/xlink",
            "xlink:href",
            "#replaced",
        )
        .unwrap();
        let new_attr_ref = Arc::new(RwLock::new(new_attr));

        let result = elem.set_attribute_node_ns(new_attr_ref);
        assert!(result.is_ok());

        let old_attr = result.unwrap();
        assert!(old_attr.is_some());
        assert_eq!(old_attr.unwrap().read().value(), "#original");

        assert_eq!(
            elem.get_attribute_ns(Some("http://www.w3.org/1999/xlink"), "href"),
            Some("#replaced".to_string())
        );
    }

    // ==================== Helper Function Tests ====================

    #[test]
    fn test_is_valid_qualified_name() {
        assert!(is_valid_qualified_name("id"));
        assert!(is_valid_qualified_name("xlink:href"));
        assert!(is_valid_qualified_name("data-value"));

        assert!(!is_valid_qualified_name(""));
        assert!(!is_valid_qualified_name(":invalid"));
        assert!(!is_valid_qualified_name("invalid:"));
        assert!(!is_valid_qualified_name("a:b:c"));
        assert!(!is_valid_qualified_name("invalid name"));
    }

    #[test]
    fn test_parse_qualified_name() {
        let (prefix, local) = parse_qualified_name("xlink:href");
        assert_eq!(prefix, Some("xlink".to_string()));
        assert_eq!(local, "href");

        let (prefix, local) = parse_qualified_name("id");
        assert_eq!(prefix, None);
        assert_eq!(local, "id");
    }
}
