//! NamedNodeMap implementation for managing element attributes
//!
//! NamedNodeMap is a collection of Attr nodes that provides efficient access
//! to attributes by name or by namespace and local name.

use dom_core::AttrRef;
use dom_types::DomException;
use std::collections::HashMap;

/// NamedNodeMap manages a collection of Attr nodes
///
/// This collection provides methods to:
/// - Access attributes by index (deterministic ordering)
/// - Access attributes by name
/// - Access attributes by namespace and local name
/// - Add, replace, and remove attributes
///
/// # Example
///
/// ```rust,no_run
/// use dom_collections::NamedNodeMap;
/// use dom_core::Attr;
/// use parking_lot::RwLock;
/// use std::sync::Arc;
///
/// let mut map = NamedNodeMap::new();
/// let attr = Arc::new(RwLock::new(Attr::new("id", "main")));
/// map.set_named_item(attr.clone()).unwrap();
///
/// assert_eq!(map.length(), 1);
/// assert!(map.get_named_item("id").is_some());
/// ```
#[derive(Debug, Clone)]
pub struct NamedNodeMap {
    /// Attributes stored by name for fast lookup
    attributes: HashMap<String, AttrRef>,

    /// Attributes stored by namespace and local name for namespaced lookup
    /// Key is (namespace_uri, local_name)
    namespaced_attributes: HashMap<(String, String), AttrRef>,

    /// Ordered list of attribute names for deterministic iteration
    /// Maintains insertion order
    ordered_names: Vec<String>,
}

impl NamedNodeMap {
    /// Creates a new empty NamedNodeMap
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    ///
    /// let map = NamedNodeMap::new();
    /// assert_eq!(map.length(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            namespaced_attributes: HashMap::new(),
            ordered_names: Vec::new(),
        }
    }

    /// Returns the number of attributes in the map
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    ///
    /// let map = NamedNodeMap::new();
    /// assert_eq!(map.length(), 0);
    /// ```
    pub fn length(&self) -> usize {
        self.attributes.len() + self.namespaced_attributes.len()
    }

    /// Returns true if the map contains no attributes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    ///
    /// let map = NamedNodeMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty() && self.namespaced_attributes.is_empty()
    }

    /// Gets an attribute by index
    ///
    /// Attributes are returned in insertion order (for non-namespaced attributes)
    /// or sorted order (for deterministic results).
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the attribute to retrieve
    ///
    /// # Returns
    ///
    /// The attribute at the specified index, or None if index is out of bounds
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(Attr::new("id", "main")));
    /// map.set_named_item(attr.clone()).unwrap();
    ///
    /// assert!(map.item(0).is_some());
    /// assert!(map.item(1).is_none());
    /// ```
    pub fn item(&self, index: usize) -> Option<AttrRef> {
        if index >= self.ordered_names.len() {
            return None;
        }

        let name = &self.ordered_names[index];

        // First try to get from regular attributes
        if let Some(attr) = self.attributes.get(name) {
            return Some(attr.clone());
        }

        // Then try namespaced attributes
        // For namespaced attributes, the name in ordered_names is the qualified name
        // We need to find the matching attribute by comparing qualified names
        for ((_ns, _local), attr) in &self.namespaced_attributes {
            let attr_locked = attr.read();
            if attr_locked.name() == name {
                return Some(attr.clone());
            }
        }

        None
    }

    /// Gets an attribute by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the attribute to retrieve
    ///
    /// # Returns
    ///
    /// The attribute with the specified name, or None if not found
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(Attr::new("id", "main")));
    /// map.set_named_item(attr.clone()).unwrap();
    ///
    /// assert!(map.get_named_item("id").is_some());
    /// assert!(map.get_named_item("class").is_none());
    /// ```
    pub fn get_named_item(&self, name: &str) -> Option<AttrRef> {
        self.attributes.get(name).cloned()
    }

    /// Gets an attribute by namespace and local name
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace URI (or None for no namespace)
    /// * `local_name` - The local name of the attribute
    ///
    /// # Returns
    ///
    /// The attribute with the specified namespace and local name, or None if not found
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(
    ///     Attr::new_ns("http://www.w3.org/1999/xlink", "xlink:href", "#link").unwrap()
    /// ));
    /// map.set_named_item_ns(attr.clone()).unwrap();
    ///
    /// let retrieved = map.get_named_item_ns(Some("http://www.w3.org/1999/xlink"), "href");
    /// assert!(retrieved.is_some());
    /// ```
    pub fn get_named_item_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<AttrRef> {
        if let Some(ns) = namespace {
            let key = (ns.to_string(), local_name.to_string());
            self.namespaced_attributes.get(&key).cloned()
        } else {
            None
        }
    }

    /// Sets an attribute (adds or replaces)
    ///
    /// If an attribute with the same name already exists, it is replaced and returned.
    /// Otherwise, the new attribute is added and None is returned.
    ///
    /// # Arguments
    ///
    /// * `attr` - The attribute to set
    ///
    /// # Returns
    ///
    /// The previously existing attribute with the same name, or None
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr1 = Arc::new(RwLock::new(Attr::new("class", "btn")));
    /// let result = map.set_named_item(attr1.clone()).unwrap();
    /// assert!(result.is_none()); // No previous attribute
    ///
    /// let attr2 = Arc::new(RwLock::new(Attr::new("class", "btn-primary")));
    /// let result = map.set_named_item(attr2.clone()).unwrap();
    /// assert!(result.is_some()); // Returns old attribute
    /// ```
    pub fn set_named_item(&mut self, attr: AttrRef) -> Result<Option<AttrRef>, DomException> {
        let attr_locked = attr.read();
        let name = attr_locked.name().to_string();
        let namespace = attr_locked.namespace_uri().map(|s| s.to_string());
        drop(attr_locked);

        // Check if this is a namespaced attribute
        if namespace.is_some() {
            // Don't store in regular attributes map
            return self.set_named_item_ns(attr);
        }

        // Add to ordered names if not already present
        if !self.attributes.contains_key(&name) {
            self.ordered_names.push(name.clone());
        }

        // Store in regular attributes map
        let old_attr = self.attributes.insert(name, attr);

        Ok(old_attr)
    }

    /// Sets a namespaced attribute (adds or replaces)
    ///
    /// If an attribute with the same namespace and local name already exists,
    /// it is replaced and returned. Otherwise, the new attribute is added and None is returned.
    ///
    /// # Arguments
    ///
    /// * `attr` - The namespaced attribute to set
    ///
    /// # Returns
    ///
    /// The previously existing attribute with the same namespace and local name, or None
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(
    ///     Attr::new_ns("http://www.w3.org/1999/xlink", "xlink:href", "#link").unwrap()
    /// ));
    /// let result = map.set_named_item_ns(attr.clone()).unwrap();
    /// assert!(result.is_none()); // No previous attribute
    /// ```
    pub fn set_named_item_ns(&mut self, attr: AttrRef) -> Result<Option<AttrRef>, DomException> {
        let attr_locked = attr.read();
        let name = attr_locked.name().to_string();
        let namespace = attr_locked
            .namespace_uri()
            .ok_or(DomException::NamespaceError)?
            .to_string();
        let local_name = attr_locked.local_name().to_string();
        drop(attr_locked);

        let key = (namespace, local_name);

        // Add to ordered names if not already present
        if !self.namespaced_attributes.contains_key(&key) {
            self.ordered_names.push(name);
        }

        // Store in namespaced attributes map
        let old_attr = self.namespaced_attributes.insert(key, attr);

        Ok(old_attr)
    }

    /// Removes an attribute by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the attribute to remove
    ///
    /// # Returns
    ///
    /// The removed attribute, or an error if not found
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the attribute doesn't exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(Attr::new("id", "main")));
    /// map.set_named_item(attr.clone()).unwrap();
    ///
    /// let removed = map.remove_named_item("id").unwrap();
    /// assert_eq!(map.length(), 0);
    /// ```
    pub fn remove_named_item(&mut self, name: &str) -> Result<AttrRef, DomException> {
        let attr = self
            .attributes
            .remove(name)
            .ok_or(DomException::NotFoundError)?;

        // Remove from ordered names
        self.ordered_names.retain(|n| n != name);

        Ok(attr)
    }

    /// Removes an attribute by namespace and local name
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace URI (or None for no namespace)
    /// * `local_name` - The local name of the attribute
    ///
    /// # Returns
    ///
    /// The removed attribute, or an error if not found
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the attribute doesn't exist
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// let attr = Arc::new(RwLock::new(
    ///     Attr::new_ns("http://www.w3.org/1999/xlink", "xlink:href", "#link").unwrap()
    /// ));
    /// map.set_named_item_ns(attr.clone()).unwrap();
    ///
    /// let removed = map.remove_named_item_ns(Some("http://www.w3.org/1999/xlink"), "href").unwrap();
    /// assert_eq!(map.length(), 0);
    /// ```
    pub fn remove_named_item_ns(
        &mut self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Result<AttrRef, DomException> {
        let ns = namespace.ok_or(DomException::NotFoundError)?;
        let key = (ns.to_string(), local_name.to_string());

        let attr = self
            .namespaced_attributes
            .remove(&key)
            .ok_or(DomException::NotFoundError)?;

        // Remove from ordered names by qualified name
        let attr_locked = attr.read();
        let qualified_name = attr_locked.name();
        self.ordered_names.retain(|n| n != qualified_name);
        drop(attr_locked);

        Ok(attr)
    }

    /// Returns an iterator over attribute names (for deterministic ordering)
    ///
    /// # Returns
    ///
    /// A vector of attribute names in insertion order
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// map.set_named_item(Arc::new(RwLock::new(Attr::new("id", "main")))).unwrap();
    /// map.set_named_item(Arc::new(RwLock::new(Attr::new("class", "btn")))).unwrap();
    ///
    /// let names = map.names();
    /// assert_eq!(names.len(), 2);
    /// ```
    pub fn names(&self) -> Vec<String> {
        self.ordered_names.clone()
    }

    /// Returns all attributes as a vector
    ///
    /// # Returns
    ///
    /// A vector of all attributes in the map
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// map.set_named_item(Arc::new(RwLock::new(Attr::new("id", "main")))).unwrap();
    ///
    /// let attrs = map.attributes();
    /// assert_eq!(attrs.len(), 1);
    /// ```
    pub fn attributes(&self) -> Vec<AttrRef> {
        let mut attrs = Vec::new();

        // Add regular attributes in order
        for name in &self.ordered_names {
            if let Some(attr) = self.attributes.get(name) {
                attrs.push(attr.clone());
            } else {
                // Try namespaced attributes
                for ((_, _), attr) in &self.namespaced_attributes {
                    let attr_locked = attr.read();
                    if attr_locked.name() == name {
                        attrs.push(attr.clone());
                        break;
                    }
                }
            }
        }

        attrs
    }

    /// Clears all attributes from the map
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use dom_collections::NamedNodeMap;
    /// use dom_core::Attr;
    /// use parking_lot::RwLock;
    /// use std::sync::Arc;
    ///
    /// let mut map = NamedNodeMap::new();
    /// map.set_named_item(Arc::new(RwLock::new(Attr::new("id", "main")))).unwrap();
    /// assert_eq!(map.length(), 1);
    ///
    /// map.clear();
    /// assert_eq!(map.length(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.attributes.clear();
        self.namespaced_attributes.clear();
        self.ordered_names.clear();
    }
}

impl Default for NamedNodeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::Attr;
    use parking_lot::RwLock;
    use std::sync::Arc;

    fn create_attr(name: &str, value: &str) -> AttrRef {
        Arc::new(RwLock::new(Attr::new(name, value)))
    }

    #[test]
    fn test_new() {
        let map = NamedNodeMap::new();
        assert_eq!(map.length(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_default() {
        let map = NamedNodeMap::default();
        assert_eq!(map.length(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut map = NamedNodeMap::new();
        let attr = create_attr("id", "main");

        let result = map.set_named_item(attr.clone());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        assert_eq!(map.length(), 1);
        assert!(!map.is_empty());

        let retrieved = map.get_named_item("id");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_clone() {
        let mut map = NamedNodeMap::new();
        map.set_named_item(create_attr("id", "main")).unwrap();

        let cloned = map.clone();
        assert_eq!(cloned.length(), 1);
        assert!(cloned.get_named_item("id").is_some());
    }
}
