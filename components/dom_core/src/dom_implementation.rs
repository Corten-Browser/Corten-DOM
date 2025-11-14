//! DOMImplementation interface
//!
//! Provides methods for creating documents and document types independent of any particular document instance.

use crate::document::{Document, DocumentRef};
use crate::document_type::{DocumentType, DocumentTypeRef};
use crate::node::Node;
use dom_types::DomException;
use parking_lot::RwLock;
use std::sync::Arc;

/// DOMImplementation provides factory methods for creating documents
#[derive(Debug, Clone)]
pub struct DOMImplementation;

impl DOMImplementation {
    /// Create a new DOMImplementation instance
    pub fn new() -> Self {
        Self
    }

    /// Test if the DOM implementation implements a specific feature
    ///
    /// # Arguments
    /// * `feature` - The name of the feature to test
    /// * `version` - The version of the feature to test
    ///
    /// # Returns
    /// Always returns `true` for supported features, `false` otherwise
    ///
    /// # Notes
    /// This method is deprecated in modern DOM specifications but included
    /// for compatibility. Supported features include:
    /// - "Core" (DOM Level 1-3)
    /// - "XML" (DOM Level 1-3)
    /// - "Events" (DOM Level 2-3)
    /// - "UIEvents" (DOM Level 2-3)
    /// - "MouseEvents" (DOM Level 2-3)
    /// - "HTML" (DOM Level 2)
    pub fn has_feature(&self, feature: &str, _version: &str) -> bool {
        // Modern implementations should always return true for supported features
        // or false for unsupported ones, ignoring version
        matches!(
            feature.to_lowercase().as_str(),
            "core" | "xml" | "events" | "uievents" | "mouseevents" | "html"
        )
    }

    /// Create a new DocumentType node
    ///
    /// # Arguments
    /// * `qualified_name` - The qualified name of the document type (e.g., "html")
    /// * `public_id` - The public identifier (e.g., "-//W3C//DTD HTML 4.01//EN")
    /// * `system_id` - The system identifier (e.g., "http://www.w3.org/TR/html4/strict.dtd")
    ///
    /// # Returns
    /// A new DocumentType node
    ///
    /// # Errors
    /// * `InvalidCharacterError` - If the qualified name contains invalid characters
    /// * `NamespaceError` - If the qualified name is malformed
    ///
    /// # Example
    /// ```ignore
    /// let dom_impl = DOMImplementation::new();
    /// let doctype = dom_impl.create_document_type(
    ///     "html",
    ///     "-//W3C//DTD HTML 4.01//EN",
    ///     "http://www.w3.org/TR/html4/strict.dtd"
    /// ).unwrap();
    /// ```
    pub fn create_document_type(
        &self,
        qualified_name: impl Into<String>,
        public_id: impl Into<String>,
        system_id: impl Into<String>,
    ) -> Result<DocumentTypeRef, DomException> {
        let name = qualified_name.into();

        // Validate qualified name
        if !is_valid_name(&name) {
            return Err(DomException::InvalidCharacterError);
        }

        // Check for namespace prefix (should not have prefix for document type)
        if name.contains(':') {
            return Err(DomException::NamespaceError);
        }

        let doctype = DocumentType::new(name, public_id, system_id);
        Ok(Arc::new(RwLock::new(doctype)))
    }

    /// Create a new XML Document
    ///
    /// # Arguments
    /// * `namespace_uri` - The namespace URI of the document element (None for HTML)
    /// * `qualified_name` - The qualified name of the document element (empty for no root)
    /// * `doctype` - The document type node (None if no doctype)
    ///
    /// # Returns
    /// A new Document
    ///
    /// # Errors
    /// * `InvalidCharacterError` - If the qualified name contains invalid characters
    /// * `NamespaceError` - If the namespace/qualified name combination is invalid
    ///
    /// # Example
    /// ```ignore
    /// let dom_impl = DOMImplementation::new();
    /// let doc = dom_impl.create_document(
    ///     Some("http://www.w3.org/1999/xhtml"),
    ///     "html",
    ///     None
    /// ).unwrap();
    /// ```
    pub fn create_document(
        &self,
        namespace_uri: Option<impl Into<String>>,
        qualified_name: impl Into<String>,
        _doctype: Option<DocumentTypeRef>,
    ) -> Result<DocumentRef, DomException> {
        let name = qualified_name.into();

        // Create new document
        let mut doc = Document::new();

        // If qualified name is provided, create root element
        if !name.is_empty() {
            // Validate qualified name
            if !is_valid_name(&name) {
                return Err(DomException::InvalidCharacterError);
            }

            // Create root element with namespace if provided
            let root = if let Some(ns) = namespace_uri {
                let ns_str = ns.into();

                // Validate namespace/qualified name combination
                if name.contains(':') {
                    let parts: Vec<&str> = name.split(':').collect();
                    if parts.len() != 2 {
                        return Err(DomException::NamespaceError);
                    }

                    // Check for invalid combinations
                    let prefix = parts[0];
                    if prefix == "xml" && ns_str != "http://www.w3.org/XML/1998/namespace" {
                        return Err(DomException::NamespaceError);
                    }
                    if prefix == "xmlns" {
                        return Err(DomException::NamespaceError);
                    }
                }

                doc.create_element_ns(ns_str, &name)?
            } else {
                doc.create_element(&name)?
            };

            doc.set_document_element(root);
        }

        Ok(Arc::new(RwLock::new(doc)))
    }

    /// Create a new HTML Document
    ///
    /// This is a convenience method for creating HTML documents.
    ///
    /// # Arguments
    /// * `title` - The title of the document (optional)
    ///
    /// # Returns
    /// A new HTML Document with basic structure (html, head, body)
    ///
    /// # Example
    /// ```ignore
    /// let dom_impl = DOMImplementation::new();
    /// let doc = dom_impl.create_html_document(Some("My Page")).unwrap();
    /// ```
    pub fn create_html_document(
        &self,
        title: Option<impl Into<String>>,
    ) -> Result<DocumentRef, DomException> {
        let mut doc = Document::new();

        // Create HTML structure
        let html = doc.create_element("html")?;
        let head = doc.create_element("head")?;
        let body = doc.create_element("body")?;

        // Add title if provided
        if let Some(title_text) = title {
            let title_elem = doc.create_element("title")?;
            let text = crate::text::Text::new(title_text.into());
            let text_node =
                Arc::new(RwLock::new(Box::new(text) as Box<dyn crate::node::Node>));
            title_elem
                .write()
                .append_child(text_node)
                .map_err(|_| DomException::HierarchyRequestError)?;
            head.write()
                .append_child(
                    Arc::new(RwLock::new(Box::new(title_elem.read().clone())
                        as Box<dyn crate::node::Node>)),
                )
                .map_err(|_| DomException::HierarchyRequestError)?;
        }

        // Build document structure
        html.write()
            .append_child(Arc::new(RwLock::new(Box::new(head.read().clone())
                as Box<dyn crate::node::Node>)))
            .map_err(|_| DomException::HierarchyRequestError)?;
        html.write()
            .append_child(Arc::new(RwLock::new(Box::new(body.read().clone())
                as Box<dyn crate::node::Node>)))
            .map_err(|_| DomException::HierarchyRequestError)?;

        doc.set_document_element(html);

        Ok(Arc::new(RwLock::new(doc)))
    }
}

impl Default for DOMImplementation {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate an XML name according to XML 1.0 specification
fn is_valid_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let chars: Vec<char> = name.chars().collect();

    // First character must be letter, underscore, or colon
    let first = chars[0];
    if !first.is_alphabetic() && first != '_' && first != ':' {
        return false;
    }

    // Subsequent characters can be letters, digits, dots, hyphens, underscores, or colons
    for c in &chars[1..] {
        if !c.is_alphanumeric() && *c != '.' && *c != '-' && *c != '_' && *c != ':' {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_implementation_creation() {
        let dom_impl = DOMImplementation::new();
        assert!(true); // Just test that it can be created
        let _ = dom_impl; // Use the variable
    }

    #[test]
    fn test_has_feature_supported() {
        let dom_impl = DOMImplementation::new();
        assert!(dom_impl.has_feature("Core", ""));
        assert!(dom_impl.has_feature("XML", ""));
        assert!(dom_impl.has_feature("Events", ""));
        assert!(dom_impl.has_feature("UIEvents", ""));
        assert!(dom_impl.has_feature("HTML", ""));
    }

    #[test]
    fn test_has_feature_unsupported() {
        let dom_impl = DOMImplementation::new();
        assert!(!dom_impl.has_feature("UnknownFeature", ""));
        assert!(!dom_impl.has_feature("FakeModule", "1.0"));
    }

    #[test]
    fn test_has_feature_case_insensitive() {
        let dom_impl = DOMImplementation::new();
        assert!(dom_impl.has_feature("core", ""));
        assert!(dom_impl.has_feature("CORE", ""));
        assert!(dom_impl.has_feature("xml", ""));
    }

    #[test]
    fn test_create_document_type() {
        let dom_impl = DOMImplementation::new();
        let doctype = dom_impl
            .create_document_type(
                "html",
                "-//W3C//DTD HTML 4.01//EN",
                "http://www.w3.org/TR/html4/strict.dtd",
            )
            .unwrap();

        assert_eq!(doctype.read().name(), "html");
        assert_eq!(doctype.read().public_id(), "-//W3C//DTD HTML 4.01//EN");
        assert_eq!(
            doctype.read().system_id(),
            "http://www.w3.org/TR/html4/strict.dtd"
        );
    }

    #[test]
    fn test_create_document_type_invalid_name() {
        let dom_impl = DOMImplementation::new();
        let result = dom_impl.create_document_type("123invalid", "", "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::InvalidCharacterError);
    }

    #[test]
    fn test_create_document() {
        let dom_impl = DOMImplementation::new();
        let doc = dom_impl
            .create_document(Some("http://www.w3.org/1999/xhtml"), "html", None)
            .unwrap();

        assert!(doc.read().document_element().is_some());
        let root = doc.read().document_element().unwrap();
        assert_eq!(root.read().tag_name(), "HTML");
    }

    #[test]
    fn test_create_document_no_root() {
        let dom_impl = DOMImplementation::new();
        let doc = dom_impl
            .create_document(None::<String>, "", None)
            .unwrap();

        assert!(doc.read().document_element().is_none());
    }

    #[test]
    fn test_create_html_document() {
        let dom_impl = DOMImplementation::new();
        let doc = dom_impl
            .create_html_document(Some("Test Page"))
            .unwrap();

        assert!(doc.read().document_element().is_some());
        let root = doc.read().document_element().unwrap();
        assert_eq!(root.read().tag_name(), "HTML");
    }

    #[test]
    fn test_create_html_document_no_title() {
        let dom_impl = DOMImplementation::new();
        let doc = dom_impl.create_html_document(None::<String>).unwrap();

        assert!(doc.read().document_element().is_some());
    }

    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("html"));
        assert!(is_valid_name("div"));
        assert!(is_valid_name("my-element"));
        assert!(is_valid_name("my_element"));
        assert!(is_valid_name("my:element"));
        assert!(is_valid_name("_element"));
        assert!(is_valid_name("element123"));

        assert!(!is_valid_name(""));
        assert!(!is_valid_name("123element"));
        assert!(!is_valid_name("-element"));
        assert!(!is_valid_name(".element"));
    }

    #[test]
    fn test_default() {
        let dom_impl = DOMImplementation::default();
        assert!(dom_impl.has_feature("Core", ""));
    }
}
