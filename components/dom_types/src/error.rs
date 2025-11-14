//! DOM exception types.
//!
//! This module defines the [`DomException`] enum which represents all the error
//! types that can occur during DOM operations, as per the DOM Level 4 specification.

use thiserror::Error;

/// DOM exception types as defined in the DOM Level 4 specification.
///
/// These exceptions are thrown when DOM operations encounter error conditions.
/// Each variant corresponds to a specific type of DOM error.
///
/// # Examples
///
/// ```
/// use dom_types::DomException;
///
/// let error = DomException::NotFoundError;
/// println!("Error: {}", error);
/// ```
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomException {
    /// The operation would create an invalid hierarchy
    /// (e.g., inserting a node in an inappropriate location).
    #[error("Hierarchy request error")]
    HierarchyRequestError,

    /// The node being operated on belongs to a different document
    /// than the one attempting the operation.
    #[error("Wrong document error")]
    WrongDocumentError,

    /// The string contains invalid characters for the operation
    /// (e.g., invalid characters in an element name).
    #[error("Invalid character error")]
    InvalidCharacterError,

    /// The object cannot be modified at this time
    /// (e.g., attempting to modify a read-only node).
    #[error("No modification allowed error")]
    NoModificationAllowedError,

    /// The requested node or object was not found
    /// (e.g., attempting to remove a node that doesn't exist).
    #[error("Not found error")]
    NotFoundError,

    /// The operation is not supported by the implementation
    /// (e.g., creating a node type that isn't implemented).
    #[error("Not supported error")]
    NotSupportedError,

    /// The object is in an invalid state for the requested operation
    /// (e.g., using a node that has been removed from the tree).
    #[error("Invalid state error")]
    InvalidStateError,

    /// The string contains a syntax error.
    ///
    /// The string parameter contains details about the syntax error.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DomException;
    ///
    /// let error = DomException::SyntaxError("Invalid CSS selector".to_string());
    /// assert!(error.to_string().contains("Invalid CSS selector"));
    /// ```
    #[error("Syntax error: {0}")]
    SyntaxError(String),

    /// The modification attempted is not allowed
    /// (e.g., modifying attributes in an invalid way).
    #[error("Invalid modification error")]
    InvalidModificationError,

    /// The namespace is invalid or conflicts with existing namespaces
    /// (e.g., attempting to create a namespace with an invalid URI).
    #[error("Namespace error")]
    NamespaceError,

    /// The operation is not allowed for security reasons
    /// (e.g., cross-origin access violation).
    #[error("Security error")]
    SecurityError,
}

impl DomException {
    /// Creates a new `SyntaxError` with the given message.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DomException;
    ///
    /// let error = DomException::syntax_error("Invalid selector");
    /// assert!(error.to_string().contains("Invalid selector"));
    /// ```
    pub fn syntax_error(message: impl Into<String>) -> Self {
        DomException::SyntaxError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_error_trait() {
        let err = DomException::NotFoundError;
        let _: &dyn Error = &err;
    }

    #[test]
    fn test_syntax_error_helper() {
        let err = DomException::syntax_error("test");
        assert!(err.to_string().contains("test"));
    }

    #[test]
    fn test_clone() {
        let err = DomException::NotFoundError;
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }
}
