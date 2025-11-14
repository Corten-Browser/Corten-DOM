//! ShadowRoot implementation

use super::slot::SlotAssignmentMode;
use dom_core::{ElementRef, Node, NodeRef};
use dom_types::DomException;
use std::sync::Weak;
use parking_lot::RwLock;
use std::sync::Arc;

/// Mode for shadow root encapsulation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowRootMode {
    /// Shadow root is accessible via element.shadowRoot
    Open,
    /// Shadow root is not accessible from outside
    Closed,
}

/// A shadow root for encapsulated DOM subtrees
#[derive(Clone)]
pub struct ShadowRoot {
    inner: Arc<RwLock<ShadowRootInner>>,
}

struct ShadowRootInner {
    host: Weak<parking_lot::RwLock<dom_core::Element>>,
    mode: ShadowRootMode,
    delegates_focus: bool,
    slot_assignment: SlotAssignmentMode,
    // The shadow root acts as a document fragment
    children: Vec<NodeRef>,
}

impl ShadowRoot {
    /// Create a new shadow root
    pub fn new(
        host: ElementRef,
        mode: ShadowRootMode,
        delegates_focus: bool,
        slot_assignment: SlotAssignmentMode,
    ) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ShadowRootInner {
                host: Arc::downgrade(&host),
                mode,
                delegates_focus,
                slot_assignment,
                children: Vec::new(),
            })),
        }
    }

    /// Get the host element
    pub fn host(&self) -> Result<ElementRef, DomException> {
        let inner = self.inner.read();
        inner.host.upgrade().ok_or(DomException::InvalidStateError)
    }

    /// Get the shadow root mode
    pub fn mode(&self) -> ShadowRootMode {
        let inner = self.inner.read();
        inner.mode
    }

    /// Check if focus delegation is enabled
    pub fn delegates_focus(&self) -> bool {
        let inner = self.inner.read();
        inner.delegates_focus
    }

    /// Get the slot assignment mode
    pub fn slot_assignment(&self) -> SlotAssignmentMode {
        let inner = self.inner.read();
        inner.slot_assignment
    }

    /// Get the shadow root as a node reference
    ///
    /// In a real implementation, ShadowRoot would implement Node traits
    pub fn as_node(&self) -> NodeRef {
        // Placeholder: create a pseudo-node for the shadow root
        // In reality, this would be a proper Node implementation
        let host = self.host().unwrap();
        // Convert ElementRef to NodeRef by boxing the element
        let element_clone = host.read().clone();
        Arc::new(RwLock::new(Box::new(element_clone) as Box<dyn Node>))
    }

    /// Append a child to the shadow root
    pub fn append_child(&self, child: NodeRef) -> Result<(), DomException> {
        let mut inner = self.inner.write();

        // Check if child is already in the children list
        if inner.children.iter().any(|c| Arc::ptr_eq(c, &child)) {
            return Ok(());
        }

        inner.children.push(child);
        Ok(())
    }

    /// Get all children of the shadow root
    pub fn children(&self) -> Vec<NodeRef> {
        let inner = self.inner.read();
        inner.children.clone()
    }

    /// Get the first child
    pub fn first_child(&self) -> Option<NodeRef> {
        let inner = self.inner.read();
        inner.children.first().cloned()
    }

    /// Get the last child
    pub fn last_child(&self) -> Option<NodeRef> {
        let inner = self.inner.read();
        inner.children.last().cloned()
    }

    /// Query for an element within the shadow root
    pub fn query_selector(&self, _selector: &str) -> Option<ElementRef> {
        // Simplified: would need full CSS selector implementation
        None
    }

    /// Query for all elements within the shadow root
    pub fn query_selector_all(&self, _selector: &str) -> Vec<ElementRef> {
        // Simplified: would need full CSS selector implementation
        Vec::new()
    }

    /// Get element by ID within the shadow root
    pub fn get_element_by_id(&self, id: &str) -> Option<ElementRef> {
        let inner = self.inner.read();
        for child in &inner.children {
            let node_guard = child.read();
            if let Some(element) = node_guard.as_any().downcast_ref::<dom_core::Element>() {
                if let Some(elem_id) = element.get_attribute("id") {
                    if elem_id == id {
                        // Create an ElementRef from the child
                        // Note: This is a simplified conversion
                        drop(node_guard);
                        // We can't directly convert NodeRef to ElementRef here
                        // Return None for now - a proper implementation would need better type handling
                        return None;
                    }
                }
            }
        }
        None
    }
}

impl std::fmt::Debug for ShadowRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.read();
        f.debug_struct("ShadowRoot")
            .field("mode", &inner.mode)
            .field("delegates_focus", &inner.delegates_focus)
            .field("slot_assignment", &inner.slot_assignment)
            .field("children_count", &inner.children.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Document;

    #[test]
    fn test_shadow_root_creation() {
        let mut doc = Document::new();
        let div = doc.create_element("div").unwrap();

        let shadow = ShadowRoot::new(
            div.clone(),
            ShadowRootMode::Open,
            false,
            SlotAssignmentMode::Named,
        );

        assert_eq!(shadow.mode(), ShadowRootMode::Open);
        assert!(!shadow.delegates_focus());
        assert_eq!(shadow.slot_assignment(), SlotAssignmentMode::Named);
    }

    #[test]
    fn test_shadow_root_host() {
        let mut doc = Document::new();
        let div = doc.create_element("div").unwrap();

        let shadow = ShadowRoot::new(
            div.clone(),
            ShadowRootMode::Open,
            false,
            SlotAssignmentMode::Named,
        );

        let host = shadow.host().unwrap();
        assert!(Arc::ptr_eq(&host, &div));
    }

    #[test]
    fn test_shadow_root_append_child() {
        let mut doc = Document::new();
        let host = doc.create_element("div").unwrap();
        let child = doc.create_element("span").unwrap();

        let shadow = ShadowRoot::new(
            host,
            ShadowRootMode::Open,
            false,
            SlotAssignmentMode::Named,
        );

        // Convert ElementRef to NodeRef
        let child_node = {
            let element_clone = child.read().clone();
            Arc::new(parking_lot::RwLock::new(Box::new(element_clone) as Box<dyn dom_core::Node>))
        };
        shadow.append_child(child_node).unwrap();

        let children = shadow.children();
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_shadow_root_modes() {
        let mut doc = Document::new();
        let div = doc.create_element("div").unwrap();

        let open_shadow = ShadowRoot::new(
            div.clone(),
            ShadowRootMode::Open,
            false,
            SlotAssignmentMode::Named,
        );
        assert_eq!(open_shadow.mode(), ShadowRootMode::Open);

        let closed_shadow = ShadowRoot::new(
            div,
            ShadowRootMode::Closed,
            false,
            SlotAssignmentMode::Named,
        );
        assert_eq!(closed_shadow.mode(), ShadowRootMode::Closed);
    }
}
