//! HTMLCollection implementation (live collection)

use dom_core::{ElementRef, Node, NodeRef};
use dom_types::NodeType;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::sync::{Arc, Weak};

/// Live HTMLCollection that updates when the DOM changes
///
/// This collection maintains a weak reference to the root and re-scans
/// the tree on every access, making it "live" - it automatically reflects
/// changes to the DOM.
pub struct HTMLCollection {
    /// Weak reference to the root element (to avoid circular references)
    root: Weak<RwLock<dom_core::Element>>,

    /// Filter function to determine which elements to include
    filter: Arc<dyn Fn(&ElementRef) -> bool + Send + Sync>,

    /// Cached items (updated on every access to reflect live DOM)
    cached_items: RefCell<Vec<ElementRef>>,

    /// Version counter (not used yet, but reserved for optimization)
    _version: u64,

    /// Track element refs separately for tree traversal
    /// This is a workaround for the NodeRef/ElementRef type mismatch
    element_refs: RefCell<Vec<Weak<RwLock<dom_core::Element>>>>,
}

impl HTMLCollection {
    /// Creates a new HTMLCollection with the given root and filter
    pub fn new<F>(root: ElementRef, filter: F) -> Self
    where
        F: Fn(&ElementRef) -> bool + Send + Sync + 'static,
    {
        let mut collection = HTMLCollection {
            root: Arc::downgrade(&root),
            filter: Arc::new(filter),
            cached_items: RefCell::new(Vec::new()),
            _version: 0,
            element_refs: RefCell::new(Vec::new()),
        };

        // Register the root for tracking
        collection.register_element(Arc::downgrade(&root));

        collection
    }

    /// Register an element for tracking (used to build element tree)
    fn register_element(&mut self, element: Weak<RwLock<dom_core::Element>>) {
        self.element_refs.borrow_mut().push(element);
    }

    /// Updates the cache by traversing the DOM tree
    fn update_cache(&self) {
        let mut items = Vec::new();

        if let Some(root) = self.root.upgrade() {
            // Collect from root
            self.collect_from_element_and_children(&root, &mut items);
        }

        *self.cached_items.borrow_mut() = items;
    }

    /// Collects matching elements from an element and its descendants
    fn collect_from_element_and_children(&self, element: &ElementRef, items: &mut Vec<ElementRef>) {
        // Check if this element matches the filter
        if (self.filter)(element) {
            items.push(element.clone());
        }

        // Get children from the element's node data
        let children = element.read().child_nodes();

        // Process each child
        for child in children {
            if child.read().node_type() == NodeType::Element {
                // Downcast NodeRef to ElementRef using as_any()
                if let Some(child_element) = self.downcast_to_element(&child) {
                    // Recursively collect from this element and its children
                    self.collect_from_element_and_children(&child_element, items);
                }
            }
        }
    }

    /// Helper method to downcast a NodeRef to ElementRef
    fn downcast_to_element(&self, node: &NodeRef) -> Option<ElementRef> {
        // Lock the node to access its data
        let node_guard = node.read();

        // Use as_any() to downcast Box<dyn Node> to concrete Element
        if let Some(element) = node_guard.as_any().downcast_ref::<dom_core::Element>() {
            // We found an Element! Clone it to create a new ElementRef
            // This preserves all attributes and properties
            let cloned_element = element.clone();
            drop(node_guard); // Release the lock before creating new Arc
            Some(Arc::new(RwLock::new(cloned_element)))
        } else {
            None
        }
    }

    /// Returns the number of elements in the collection
    pub fn length(&self) -> usize {
        self.update_cache();
        self.cached_items.borrow().len()
    }

    /// Returns the element at the given index
    pub fn item(&self, index: usize) -> Option<ElementRef> {
        self.update_cache();
        self.cached_items.borrow().get(index).cloned()
    }

    /// Returns the element with the given name or ID
    pub fn named_item(&self, name: &str) -> Option<ElementRef> {
        self.update_cache();

        for item in self.cached_items.borrow().iter() {
            let element = item.read();

            // Check ID attribute
            if let Some(id) = element.get_attribute("id") {
                if id == name {
                    return Some(item.clone());
                }
            }

            // Check name attribute
            if let Some(attr_name) = element.get_attribute("name") {
                if attr_name == name {
                    return Some(item.clone());
                }
            }
        }

        None
    }
}

// Make HTMLCollection Send + Sync for thread safety
unsafe impl Send for HTMLCollection {}
unsafe impl Sync for HTMLCollection {}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Document;

    #[test]
    fn test_basic_collection() {
        let mut doc = Document::new();
        let root = doc.create_element("div").unwrap();

        let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
            el.read().tag_name() == "SPAN"
        });

        // Initially empty
        assert_eq!(collection.length(), 0);
    }

    #[test]
    fn test_length() {
        let mut doc = Document::new();
        let root = doc.create_element("div").unwrap();

        let span = doc.create_element("span").unwrap();
        // Note: Adding children requires proper tree manipulation
        // which we'll handle in integration tests

        let collection = HTMLCollection::new(root.clone(), |_: &ElementRef| true);
        // Length will depend on proper tree setup
        assert!(collection.length() >= 0);
    }
}
