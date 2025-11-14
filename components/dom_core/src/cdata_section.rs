//! CDATASection node implementation
//!
//! CDATASection nodes represent CDATA sections in XML documents.
//! They allow special characters (<, >, &) to be included without escaping.
//!
//! # Example
//!
//! ```xml
//! <script><![CDATA[
//!   function test(a, b) {
//!     if (a < b && a > 0) {
//!       return true;
//!     }
//!   }
//! ]]></script>
//! ```

use crate::node::{Node, NodeData, NodeRef};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::sync::Arc;

/// CDATASection node implementation
///
/// Represents `<![CDATA[data]]>` sections in XML documents.
/// Inherits behavior from Text node but has different NodeType (CDataSection).
///
/// # Examples
///
/// ```
/// use dom_core::CDATASection;
///
/// let cdata = CDATASection::new("<script>alert('test');</script>");
/// assert_eq!(cdata.data(), "<script>alert('test');</script>");
/// ```
#[derive(Debug, Clone)]
pub struct CDATASection {
    /// Base node data
    node_data: NodeData,

    /// Text content (the data inside <![CDATA[...]]>)
    data: String,
}

/// Thread-safe reference to a CDATASection
pub type CDATASectionRef = Arc<RwLock<CDATASection>>;

impl CDATASection {
    /// Creates a new CDATA section with the specified data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let cdata = CDATASection::new("Sample CDATA content");
    /// assert_eq!(cdata.data(), "Sample CDATA content");
    /// ```
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            node_data: NodeData::new(NodeType::CDataSection, "#cdata-section"),
            data: data.into(),
        }
    }

    /// Gets the CDATA section data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let cdata = CDATASection::new("test data");
    /// assert_eq!(cdata.data(), "test data");
    /// ```
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Sets the CDATA section data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let mut cdata = CDATASection::new("old");
    /// cdata.set_data("new");
    /// assert_eq!(cdata.data(), "new");
    /// ```
    pub fn set_data(&mut self, data: impl Into<String>) {
        self.data = data.into();
    }

    /// Gets the length of the CDATA section data
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let cdata = CDATASection::new("Hello");
    /// assert_eq!(cdata.length(), 5);
    /// ```
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Appends data to the existing CDATA section content
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let mut cdata = CDATASection::new("Hello");
    /// cdata.append_data(", World!");
    /// assert_eq!(cdata.data(), "Hello, World!");
    /// ```
    pub fn append_data(&mut self, data: impl Into<String>) {
        self.data.push_str(&data.into());
    }

    /// Replaces data at specified offset
    ///
    /// # Arguments
    ///
    /// * `offset` - The character offset at which to start replacing
    /// * `count` - The number of characters to replace
    /// * `data` - The replacement data
    ///
    /// # Errors
    ///
    /// Returns `InvalidModificationError` if offset is greater than the length
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let mut cdata = CDATASection::new("Hello, World!");
    /// cdata.replace_data(7, 5, "Rust").unwrap();
    /// assert_eq!(cdata.data(), "Hello, Rust!");
    /// ```
    pub fn replace_data(
        &mut self,
        offset: usize,
        count: usize,
        data: impl Into<String>,
    ) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        self.data.replace_range(offset..end, &data.into());
        Ok(())
    }

    /// Deletes data at specified offset
    ///
    /// # Arguments
    ///
    /// * `offset` - The character offset at which to start deleting
    /// * `count` - The number of characters to delete
    ///
    /// # Errors
    ///
    /// Returns `InvalidModificationError` if offset is greater than the length
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let mut cdata = CDATASection::new("Hello, World!");
    /// cdata.delete_data(5, 7).unwrap();
    /// assert_eq!(cdata.data(), "Hello!");
    /// ```
    pub fn delete_data(&mut self, offset: usize, count: usize) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        self.data.replace_range(offset..end, "");
        Ok(())
    }

    /// Inserts data at specified offset
    ///
    /// # Arguments
    ///
    /// * `offset` - The character offset at which to insert
    /// * `data` - The data to insert
    ///
    /// # Errors
    ///
    /// Returns `InvalidModificationError` if offset is greater than the length
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let mut cdata = CDATASection::new("Hello!");
    /// cdata.insert_data(5, " World").unwrap();
    /// assert_eq!(cdata.data(), "Hello World!");
    /// ```
    pub fn insert_data(
        &mut self,
        offset: usize,
        data: impl Into<String>,
    ) -> Result<(), DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        self.data.insert_str(offset, &data.into());
        Ok(())
    }

    /// Extracts a substring from the CDATA section data
    ///
    /// # Arguments
    ///
    /// * `offset` - The character offset at which to start extracting
    /// * `count` - The number of characters to extract
    ///
    /// # Errors
    ///
    /// Returns `InvalidModificationError` if offset is greater than the length
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_core::CDATASection;
    ///
    /// let cdata = CDATASection::new("Hello, World!");
    /// let substr = cdata.substring_data(7, 5).unwrap();
    /// assert_eq!(substr, "World");
    /// ```
    pub fn substring_data(&self, offset: usize, count: usize) -> Result<String, DomException> {
        if offset > self.data.len() {
            return Err(DomException::InvalidModificationError);
        }

        let end = (offset + count).min(self.data.len());
        Ok(self.data[offset..end].to_string())
    }
}

impl Node for CDATASection {
    fn node_type(&self) -> NodeType {
        self.node_data.node_type
    }

    fn node_name(&self) -> &str {
        &self.node_data.node_name
    }

    fn node_value(&self) -> Option<&str> {
        Some(&self.data)
    }

    fn set_node_value(&mut self, value: Option<String>) {
        if let Some(val) = value {
            self.data = val;
        } else {
            self.data.clear();
        }
    }

    fn text_content(&self) -> Option<String> {
        Some(self.data.clone())
    }

    fn set_text_content(&mut self, text: String) {
        self.data = text;
    }

    fn parent_node(&self) -> Option<NodeRef> {
        self.node_data.get_parent()
    }

    fn child_nodes(&self) -> Vec<NodeRef> {
        // CDATA sections cannot have children
        Vec::new()
    }

    fn append_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // CDATA sections cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn remove_child(&mut self, _child: NodeRef) -> Result<NodeRef, DomException> {
        // CDATA sections cannot have children
        Err(DomException::NotFoundError)
    }

    fn insert_before(
        &mut self,
        _new_child: NodeRef,
        _ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException> {
        // CDATA sections cannot have children
        Err(DomException::HierarchyRequestError)
    }

    fn clone_node(&self, _deep: bool) -> NodeRef {
        let cloned = self.clone();
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
        std::ptr::addr_eq(self_ptr, other_ptr)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdata_creation() {
        let cdata = CDATASection::new("Test content");
        assert_eq!(cdata.data(), "Test content");
        assert_eq!(cdata.node_type(), NodeType::CDataSection);
        assert_eq!(cdata.node_name(), "#cdata-section");
    }

    #[test]
    fn test_cdata_operations() {
        let mut cdata = CDATASection::new("Hello, World!");

        // Insert
        cdata.insert_data(7, "beautiful ").unwrap();
        assert_eq!(cdata.data(), "Hello, beautiful World!");

        // Delete
        cdata.delete_data(7, 10).unwrap();
        assert_eq!(cdata.data(), "Hello, World!");

        // Replace
        cdata.replace_data(0, 5, "Hi").unwrap();
        assert_eq!(cdata.data(), "Hi, World!");
    }

    #[test]
    fn test_cdata_substring() {
        let cdata = CDATASection::new("Hello, World!");

        let substr = cdata.substring_data(0, 5).unwrap();
        assert_eq!(substr, "Hello");

        let substr = cdata.substring_data(7, 5).unwrap();
        assert_eq!(substr, "World");
    }

    #[test]
    fn test_cdata_no_children() {
        let mut cdata = CDATASection::new("test");
        let child = CDATASection::new("child");
        let child_ref = Arc::new(RwLock::new(Box::new(child) as Box<dyn Node>));

        let result = cdata.append_child(child_ref);
        assert!(result.is_err());
        assert!(matches!(result, Err(DomException::HierarchyRequestError)));
    }
}
