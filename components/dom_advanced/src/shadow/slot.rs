//! Slot element and distribution implementation

use dom_core::{ElementRef, NodeRef};
use parking_lot::RwLock;
use std::sync::Arc;

/// Slot assignment mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotAssignmentMode {
    /// Manual assignment using assign()
    Manual,
    /// Automatic assignment based on slot attribute
    Named,
}

/// A slot element for content distribution
#[derive(Clone)]
pub struct SlotElement {
    inner: Arc<RwLock<SlotElementInner>>,
}

struct SlotElementInner {
    element: ElementRef,
    name: Option<String>,
    assigned_nodes: Vec<NodeRef>,
    fallback_nodes: Vec<NodeRef>,
}

impl SlotElement {
    /// Create a new slot element
    pub fn new(element: ElementRef) -> Self {
        let name = element.read().get_attribute("name").map(|s| s.to_string());

        Self {
            inner: Arc::new(RwLock::new(SlotElementInner {
                element,
                name,
                assigned_nodes: Vec::new(),
                fallback_nodes: Vec::new(),
            })),
        }
    }

    /// Get the slot name
    pub fn name(&self) -> Option<String> {
        let inner = self.inner.read();
        inner.name.clone()
    }

    /// Set the slot name
    pub fn set_name(&self, name: Option<String>) {
        let mut inner = self.inner.write();
        inner.name = name;

        // Update the element's name attribute
        if let Some(ref n) = inner.name {
            let _ = inner.element.write().set_attribute("name", n);
        } else {
            let _ = inner.element.write().remove_attribute("name");
        }
    }

    /// Get assigned nodes (slottables)
    pub fn assigned_nodes(&self) -> Vec<NodeRef> {
        let inner = self.inner.read();
        inner.assigned_nodes.clone()
    }

    /// Get assigned elements (elements only)
    pub fn assigned_elements(&self) -> Vec<ElementRef> {
        let inner = self.inner.read();
        // Simplified: proper implementation would need better NodeRef to ElementRef conversion
        Vec::new()
    }

    /// Manually assign nodes to this slot
    pub fn assign(&self, nodes: Vec<NodeRef>) {
        let mut inner = self.inner.write();
        inner.assigned_nodes = nodes;
    }

    /// Add a fallback node
    pub fn add_fallback(&self, node: NodeRef) {
        let mut inner = self.inner.write();
        if !inner.fallback_nodes.iter().any(|n| Arc::ptr_eq(n, &node)) {
            inner.fallback_nodes.push(node);
        }
    }

    /// Get fallback nodes
    pub fn fallback_nodes(&self) -> Vec<NodeRef> {
        let inner = self.inner.read();
        inner.fallback_nodes.clone()
    }

    /// Get the element reference
    pub fn element(&self) -> ElementRef {
        let inner = self.inner.read();
        inner.element.clone()
    }

    /// Distribute content to this slot
    ///
    /// This implements the slot distribution algorithm
    pub fn distribute(&self, available_nodes: &[NodeRef]) {
        let mut inner = self.inner.write();

        // Clear current assignments
        inner.assigned_nodes.clear();

        // Find nodes that match this slot
        for node in available_nodes {
            let should_assign = {
                let node_guard = node.read();
                if let Some(element) = node_guard.as_any().downcast_ref::<dom_core::Element>() {
                    // Check if element's slot attribute matches this slot's name
                    match (element.get_attribute("slot"), &inner.name) {
                        (Some(slot_attr), Some(slot_name)) => slot_attr == *slot_name,
                        (None, None) => true, // Default slot
                        _ => false,
                    }
                } else {
                    // Text nodes go to default slot (unnamed)
                    inner.name.is_none()
                }
            };

            if should_assign {
                inner.assigned_nodes.push(node.clone());
            }
        }
    }

    /// Check if this slot has assigned content
    pub fn has_assigned_content(&self) -> bool {
        let inner = self.inner.read();
        !inner.assigned_nodes.is_empty()
    }
}

impl std::fmt::Debug for SlotElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.read();
        f.debug_struct("SlotElement")
            .field("name", &inner.name)
            .field("assigned_count", &inner.assigned_nodes.len())
            .field("fallback_count", &inner.fallback_nodes.len())
            .finish()
    }
}

/// Helper to find slots in a shadow root
pub fn find_slots_in_shadow_tree(root: &NodeRef) -> Vec<SlotElement> {
    let mut slots = Vec::new();

    // Recursively search for slot elements
    fn search_slots(node: &NodeRef, slots: &mut Vec<SlotElement>) {
        // Simplified: would need proper type checking
        // For now, just search children
        let children = node.read().child_nodes();
        for child in children {
            search_slots(&child, slots);
        }
    }

    search_slots(root, &mut slots);
    slots
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Document;

    #[test]
    fn test_slot_creation() {
        let doc = Document::new();
        let slot_elem = doc.create_element("slot").unwrap();

        let slot = SlotElement::new(slot_elem);

        assert_eq!(slot.name(), None);
        assert!(!slot.has_assigned_content());
    }

    #[test]
    fn test_slot_with_name() {
        let doc = Document::new();
        let slot_elem = doc.create_element("slot").unwrap();
        slot_elem.set_attribute("name", "header").unwrap();

        let slot = SlotElement::new(slot_elem);

        assert_eq!(slot.name(), Some("header".to_string()));
    }

    #[test]
    fn test_slot_assign() {
        let doc = Document::new();
        let slot_elem = doc.create_element("slot").unwrap();
        let content = doc.create_element("div").unwrap();

        let slot = SlotElement::new(slot_elem);
        slot.assign(vec![content.as_node().clone()]);

        assert_eq!(slot.assigned_nodes().len(), 1);
        assert!(slot.has_assigned_content());
    }

    #[test]
    fn test_slot_distribution() {
        let doc = Document::new();

        // Create slot with name "header"
        let slot_elem = doc.create_element("slot").unwrap();
        slot_elem.set_attribute("name", "header").unwrap();
        let slot = SlotElement::new(slot_elem);

        // Create content with matching slot attribute
        let header = doc.create_element("div").unwrap();
        header.set_attribute("slot", "header").unwrap();

        // Create content without slot attribute (should not be assigned)
        let body = doc.create_element("div").unwrap();

        let available = vec![header.as_node().clone(), body.as_node().clone()];

        slot.distribute(&available);

        // Only header should be assigned
        assert_eq!(slot.assigned_nodes().len(), 1);
        assert!(slot
            .assigned_nodes()[0]
            .ptr_eq(header.as_node()));
    }

    #[test]
    fn test_default_slot() {
        let doc = Document::new();

        // Create unnamed (default) slot
        let slot_elem = doc.create_element("slot").unwrap();
        let slot = SlotElement::new(slot_elem);

        // Create content without slot attribute (should go to default)
        let content = doc.create_element("div").unwrap();

        // Create content with named slot (should NOT go to default)
        let header = doc.create_element("div").unwrap();
        header.set_attribute("slot", "header").unwrap();

        let available = vec![content.as_node().clone(), header.as_node().clone()];

        slot.distribute(&available);

        // Only unnamed content should be assigned
        assert_eq!(slot.assigned_nodes().len(), 1);
        assert!(slot
            .assigned_nodes()[0]
            .ptr_eq(content.as_node()));
    }

    #[test]
    fn test_slot_fallback() {
        let doc = Document::new();
        let slot_elem = doc.create_element("slot").unwrap();
        let fallback = doc.create_element("span").unwrap();

        let slot = SlotElement::new(slot_elem);
        slot.add_fallback(fallback.as_node().clone());

        assert_eq!(slot.fallback_nodes().len(), 1);
    }
}
