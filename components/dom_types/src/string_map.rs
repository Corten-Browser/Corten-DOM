//! DOMStringMap for element.dataset attribute access.
//!
//! This module defines the [`DOMStringMap`] type which provides named access
//! to custom data attributes (data-*) on elements.

use std::collections::HashMap;

use crate::NodeId;

/// Provides access to custom data attributes (data-*) on an element.
///
/// `DOMStringMap` is typically accessed via `element.dataset` and provides
/// a way to read and write custom data attributes. The attribute names are
/// automatically converted between camelCase (JavaScript) and kebab-case (HTML).
///
/// For example, `data-user-name` in HTML becomes `userName` in JavaScript.
///
/// # Examples
///
/// ```
/// use dom_types::DOMStringMap;
///
/// let mut dataset = DOMStringMap::new(1);
/// dataset.set("userName", "John");
/// assert_eq!(dataset.get("userName"), Some("John".to_string()));
/// assert!(dataset.contains("userName"));
///
/// // The attribute name is converted: userName -> data-user-name
/// assert_eq!(DOMStringMap::to_attribute_name("userName"), "data-user-name");
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DOMStringMap {
    /// Reference to the element this map belongs to.
    element_id: NodeId,
    /// Internal storage for the data attributes.
    /// In a real implementation, this would be backed by the element's attributes.
    data: HashMap<String, String>,
}

impl DOMStringMap {
    /// Creates a new `DOMStringMap` for the specified element.
    ///
    /// # Arguments
    ///
    /// * `element_id` - The NodeId of the element this map belongs to
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let dataset = DOMStringMap::new(42);
    /// assert!(dataset.is_empty());
    /// ```
    pub fn new(element_id: NodeId) -> Self {
        Self {
            element_id,
            data: HashMap::new(),
        }
    }

    /// Creates a `DOMStringMap` with initial data.
    ///
    /// # Arguments
    ///
    /// * `element_id` - The NodeId of the element this map belongs to
    /// * `data` - Initial data attributes in camelCase format
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    /// use std::collections::HashMap;
    ///
    /// let mut initial = HashMap::new();
    /// initial.insert("userName".to_string(), "John".to_string());
    /// let dataset = DOMStringMap::with_data(42, initial);
    /// assert_eq!(dataset.get("userName"), Some("John".to_string()));
    /// ```
    pub fn with_data(element_id: NodeId, data: HashMap<String, String>) -> Self {
        Self { element_id, data }
    }

    /// Returns the element ID this map is associated with.
    pub fn element_id(&self) -> NodeId {
        self.element_id
    }

    /// Gets the value of a data attribute.
    ///
    /// The name should be in camelCase format (e.g., `userName` for `data-user-name`).
    ///
    /// # Arguments
    ///
    /// * `name` - The camelCase name of the data attribute
    ///
    /// # Returns
    ///
    /// The value of the data attribute, or `None` if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// dataset.set("userId", "123");
    /// assert_eq!(dataset.get("userId"), Some("123".to_string()));
    /// assert_eq!(dataset.get("nonExistent"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<String> {
        self.data.get(name).cloned()
    }

    /// Sets the value of a data attribute.
    ///
    /// The name should be in camelCase format. Setting a value will create
    /// or update the corresponding `data-*` attribute on the element.
    ///
    /// # Arguments
    ///
    /// * `name` - The camelCase name of the data attribute
    /// * `value` - The value to set
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// dataset.set("userName", "Alice");
    /// assert_eq!(dataset.get("userName"), Some("Alice".to_string()));
    ///
    /// // Overwrite existing value
    /// dataset.set("userName", "Bob");
    /// assert_eq!(dataset.get("userName"), Some("Bob".to_string()));
    /// ```
    pub fn set(&mut self, name: &str, value: &str) {
        self.data.insert(name.to_string(), value.to_string());
    }

    /// Deletes a data attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - The camelCase name of the data attribute to delete
    ///
    /// # Returns
    ///
    /// `true` if the attribute existed and was removed, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// dataset.set("userId", "123");
    /// assert!(dataset.delete("userId"));
    /// assert!(!dataset.delete("userId")); // Already deleted
    /// ```
    pub fn delete(&mut self, name: &str) -> bool {
        self.data.remove(name).is_some()
    }

    /// Checks if a data attribute exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The camelCase name of the data attribute
    ///
    /// # Returns
    ///
    /// `true` if the attribute exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// dataset.set("userName", "John");
    /// assert!(dataset.contains("userName"));
    /// assert!(!dataset.contains("userAge"));
    /// ```
    pub fn contains(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    /// Returns the number of data attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// assert_eq!(dataset.len(), 0);
    /// dataset.set("a", "1");
    /// dataset.set("b", "2");
    /// assert_eq!(dataset.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if there are no data attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let dataset = DOMStringMap::new(1);
    /// assert!(dataset.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over all data attribute names.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// let mut dataset = DOMStringMap::new(1);
    /// dataset.set("userName", "John");
    /// dataset.set("userAge", "30");
    /// let names: Vec<_> = dataset.keys().collect();
    /// assert_eq!(names.len(), 2);
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    /// Returns an iterator over all data attribute values.
    pub fn values(&self) -> impl Iterator<Item = &String> {
        self.data.values()
    }

    /// Returns an iterator over all data attribute name-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }

    /// Converts a camelCase name to a data-kebab-case attribute name.
    ///
    /// This is the conversion used when storing JavaScript dataset properties
    /// as HTML attributes.
    ///
    /// # Arguments
    ///
    /// * `name` - The camelCase property name
    ///
    /// # Returns
    ///
    /// The corresponding `data-*` attribute name in kebab-case.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// assert_eq!(DOMStringMap::to_attribute_name("userName"), "data-user-name");
    /// assert_eq!(DOMStringMap::to_attribute_name("userId"), "data-user-id");
    /// assert_eq!(DOMStringMap::to_attribute_name("simple"), "data-simple");
    /// assert_eq!(DOMStringMap::to_attribute_name("XMLParser"), "data--x-m-l-parser");
    /// ```
    pub fn to_attribute_name(name: &str) -> String {
        let mut result = String::from("data-");
        for ch in name.chars() {
            if ch.is_ascii_uppercase() {
                result.push('-');
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch);
            }
        }
        result
    }

    /// Converts a data-kebab-case attribute name to a camelCase property name.
    ///
    /// This is the conversion used when reading HTML data attributes as
    /// JavaScript dataset properties.
    ///
    /// # Arguments
    ///
    /// * `attr_name` - The `data-*` attribute name in kebab-case
    ///
    /// # Returns
    ///
    /// The corresponding camelCase property name, or `None` if the attribute
    /// name doesn't start with "data-".
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMStringMap;
    ///
    /// assert_eq!(DOMStringMap::from_attribute_name("data-user-name"), Some("userName".to_string()));
    /// assert_eq!(DOMStringMap::from_attribute_name("data-user-id"), Some("userId".to_string()));
    /// assert_eq!(DOMStringMap::from_attribute_name("data-simple"), Some("simple".to_string()));
    /// assert_eq!(DOMStringMap::from_attribute_name("class"), None);
    /// ```
    pub fn from_attribute_name(attr_name: &str) -> Option<String> {
        if !attr_name.starts_with("data-") {
            return None;
        }

        let name_part = &attr_name[5..]; // Skip "data-"
        let mut result = String::new();
        let mut capitalize_next = false;

        for ch in name_part.chars() {
            if ch == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(ch);
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dataset = DOMStringMap::new(42);
        assert_eq!(dataset.element_id(), 42);
        assert!(dataset.is_empty());
    }

    #[test]
    fn test_with_data() {
        let mut data = HashMap::new();
        data.insert("userName".to_string(), "John".to_string());
        let dataset = DOMStringMap::with_data(1, data);
        assert_eq!(dataset.get("userName"), Some("John".to_string()));
    }

    #[test]
    fn test_get_set() {
        let mut dataset = DOMStringMap::new(1);
        assert_eq!(dataset.get("key"), None);

        dataset.set("key", "value");
        assert_eq!(dataset.get("key"), Some("value".to_string()));

        dataset.set("key", "new_value");
        assert_eq!(dataset.get("key"), Some("new_value".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut dataset = DOMStringMap::new(1);
        dataset.set("key", "value");

        assert!(dataset.delete("key"));
        assert!(!dataset.delete("key"));
        assert_eq!(dataset.get("key"), None);
    }

    #[test]
    fn test_contains() {
        let mut dataset = DOMStringMap::new(1);
        assert!(!dataset.contains("key"));

        dataset.set("key", "value");
        assert!(dataset.contains("key"));

        dataset.delete("key");
        assert!(!dataset.contains("key"));
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut dataset = DOMStringMap::new(1);
        assert!(dataset.is_empty());
        assert_eq!(dataset.len(), 0);

        dataset.set("a", "1");
        assert!(!dataset.is_empty());
        assert_eq!(dataset.len(), 1);

        dataset.set("b", "2");
        assert_eq!(dataset.len(), 2);
    }

    #[test]
    fn test_to_attribute_name() {
        assert_eq!(DOMStringMap::to_attribute_name("userName"), "data-user-name");
        assert_eq!(DOMStringMap::to_attribute_name("userId"), "data-user-id");
        assert_eq!(DOMStringMap::to_attribute_name("simple"), "data-simple");
        assert_eq!(DOMStringMap::to_attribute_name("aBC"), "data-a-b-c");
        assert_eq!(DOMStringMap::to_attribute_name(""), "data-");
    }

    #[test]
    fn test_from_attribute_name() {
        assert_eq!(
            DOMStringMap::from_attribute_name("data-user-name"),
            Some("userName".to_string())
        );
        assert_eq!(
            DOMStringMap::from_attribute_name("data-user-id"),
            Some("userId".to_string())
        );
        assert_eq!(
            DOMStringMap::from_attribute_name("data-simple"),
            Some("simple".to_string())
        );
        assert_eq!(DOMStringMap::from_attribute_name("class"), None);
        assert_eq!(DOMStringMap::from_attribute_name("id"), None);
        assert_eq!(
            DOMStringMap::from_attribute_name("data-"),
            Some("".to_string())
        );
    }

    #[test]
    fn test_roundtrip_conversion() {
        let names = ["userName", "userId", "myCustomData", "x", "aBC"];
        for name in names {
            let attr = DOMStringMap::to_attribute_name(name);
            let back = DOMStringMap::from_attribute_name(&attr).unwrap();
            assert_eq!(back, name);
        }
    }

    #[test]
    fn test_keys_values_iter() {
        let mut dataset = DOMStringMap::new(1);
        dataset.set("a", "1");
        dataset.set("b", "2");

        let keys: Vec<_> = dataset.keys().collect();
        assert_eq!(keys.len(), 2);

        let values: Vec<_> = dataset.values().collect();
        assert_eq!(values.len(), 2);

        let pairs: Vec<_> = dataset.iter().collect();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn test_serialization() {
        let mut dataset = DOMStringMap::new(1);
        dataset.set("key", "value");

        let json = serde_json::to_string(&dataset).unwrap();
        let deserialized: DOMStringMap = serde_json::from_str(&json).unwrap();
        assert_eq!(dataset, deserialized);
    }
}
