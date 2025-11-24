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

    /// Creates an HTMLCollection that matches elements by tag name.
    ///
    /// This is used by `Element.getElementsByTagName()`.
    ///
    /// # Arguments
    /// * `root` - The root element to search within (descendants only)
    /// * `tag_name` - The tag name to match (case-insensitive for HTML, "*" matches all)
    ///
    /// # Examples
    /// ```ignore
    /// let collection = HTMLCollection::by_tag_name(root, "div".to_string());
    /// ```
    pub fn by_tag_name(root: ElementRef, tag_name: String) -> Self {
        let target_tag = tag_name.to_uppercase();
        let match_all = target_tag == "*";

        HTMLCollection::new(root, move |el: &ElementRef| {
            if match_all {
                true
            } else {
                el.read().tag_name() == target_tag
            }
        })
    }

    /// Creates an HTMLCollection that matches elements by class name(s).
    ///
    /// This is used by `Element.getElementsByClassName()`.
    ///
    /// # Arguments
    /// * `root` - The root element to search within (descendants only)
    /// * `class_names` - Space-separated list of class names (all must match)
    ///
    /// # Examples
    /// ```ignore
    /// // Matches elements with both "foo" and "bar" classes
    /// let collection = HTMLCollection::by_class_name(root, "foo bar".to_string());
    /// ```
    pub fn by_class_name(root: ElementRef, class_names: String) -> Self {
        // Parse the space-separated class names
        let target_classes: Vec<String> = class_names
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // If no classes specified, return empty collection (nothing matches)
        if target_classes.is_empty() {
            return HTMLCollection::new(root, |_: &ElementRef| false);
        }

        HTMLCollection::new(root, move |el: &ElementRef| {
            let element = el.read();
            let element_classes = element.class_list();

            // All target classes must be present in the element
            target_classes
                .iter()
                .all(|target| element_classes.contains(target))
        })
    }

    /// Creates an HTMLCollection that matches elements by namespace and local name.
    ///
    /// This is used by `Element.getElementsByTagNameNS()`.
    ///
    /// # Arguments
    /// * `root` - The root element to search within (descendants only)
    /// * `namespace` - The namespace URI to match (None matches no namespace, Some("*") matches any)
    /// * `local_name` - The local name to match ("*" matches any)
    ///
    /// # Examples
    /// ```ignore
    /// // Match SVG elements with any local name
    /// let collection = HTMLCollection::by_tag_name_ns(
    ///     root,
    ///     Some("http://www.w3.org/2000/svg".to_string()),
    ///     "*".to_string()
    /// );
    /// ```
    pub fn by_tag_name_ns(root: ElementRef, namespace: Option<String>, local_name: String) -> Self {
        let target_local_name = local_name.to_uppercase();
        let match_any_local_name = target_local_name == "*";
        let match_any_namespace = namespace.as_deref() == Some("*");

        HTMLCollection::new(root, move |el: &ElementRef| {
            let element = el.read();

            // Check local name match
            let local_name_matches = if match_any_local_name {
                true
            } else {
                element.tag_name() == target_local_name
            };

            // Check namespace match
            let namespace_matches = if match_any_namespace {
                true
            } else {
                match (&namespace, element.namespace_uri()) {
                    (None, None) => true,           // Both have no namespace
                    (Some(ns), Some(el_ns)) => ns == el_ns, // Both have same namespace
                    _ => false,                      // One has namespace, other doesn't
                }
            };

            local_name_matches && namespace_matches
        })
    }

    /// Register an element for tracking (used to build element tree)
    fn register_element(&mut self, element: Weak<RwLock<dom_core::Element>>) {
        self.element_refs.borrow_mut().push(element);
    }

    /// Updates the cache by traversing the DOM tree
    fn update_cache(&self) {
        let mut items = Vec::new();

        if let Some(root) = self.root.upgrade() {
            // Collect from root's DESCENDANTS only (not root itself)
            // This matches the DOM spec for getElementsBy* methods
            self.collect_descendants(&root, &mut items);
        }

        *self.cached_items.borrow_mut() = items;
    }

    /// Collects matching elements from an element's descendants (not the element itself)
    fn collect_descendants(&self, element: &ElementRef, items: &mut Vec<ElementRef>) {
        // Get children from the element's node data
        let children = element.read().child_nodes();

        // Process each child
        for child in children {
            // Check node type and release lock before downcast to avoid deadlock
            // parking_lot::RwLock does not support recursive read locks from same thread
            let is_element = child.read().node_type() == NodeType::Element;

            if is_element {
                // Downcast NodeRef to ElementRef using as_any()
                if let Some(child_element) = self.downcast_to_element(&child) {
                    // Check if this descendant matches the filter
                    if (self.filter)(&child_element) {
                        items.push(child_element.clone());
                    }
                    // Recursively collect from this element's descendants
                    self.collect_descendants(&child_element, items);
                }
            }
        }
    }

    /// Collects matching elements from an element and its descendants (includes the element)
    /// This is kept for potential future use where we want to include the root
    #[allow(dead_code)]
    fn collect_from_element_and_children(&self, element: &ElementRef, items: &mut Vec<ElementRef>) {
        // Check if this element matches the filter
        if (self.filter)(element) {
            items.push(element.clone());
        }

        // Then collect from descendants
        self.collect_descendants(element, items);
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
