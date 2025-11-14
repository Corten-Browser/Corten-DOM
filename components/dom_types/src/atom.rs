//! String interning (Atom) system for optimizing repeated string usage in DOM
//!
//! This module provides a thread-safe string interning system that:
//! - Deduplicates repeated strings to save memory
//! - Provides fast equality comparisons (compare IDs instead of strings)
//! - Pre-interns common DOM strings for efficiency
//!
//! # Example
//!
//! ```
//! use dom_types::AtomTable;
//!
//! let table = AtomTable::new();
//! let atom1 = table.get_or_intern("div");
//! let atom2 = table.get_or_intern("div");
//!
//! // Same string produces same atom
//! assert_eq!(atom1, atom2);
//!
//! // Resolve atom back to string
//! assert_eq!(table.resolve(&atom1), "div");
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// An interned string represented by a unique ID
///
/// Atoms are lightweight handles to strings stored in an `AtomTable`.
/// They support fast equality comparison and hashing based on their ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atom {
    id: usize,
}

impl Atom {
    /// Create a new atom with the given ID
    ///
    /// This is an internal constructor. Users should create atoms via
    /// `AtomTable::get_or_intern`.
    fn new(id: usize) -> Self {
        Self { id }
    }

    /// Get the internal ID of this atom
    ///
    /// This is primarily for internal use and debugging.
    pub fn id(&self) -> usize {
        self.id
    }
}

/// Thread-safe table for managing string interning
///
/// The `AtomTable` maintains bidirectional mappings between strings and atom IDs,
/// ensuring that each unique string is stored only once. It comes pre-populated
/// with common DOM strings for efficiency.
///
/// # Thread Safety
///
/// `AtomTable` uses `Arc<RwLock<_>>` internally, making it safe to share across
/// threads. Multiple threads can intern and resolve strings concurrently.
///
/// # Example
///
/// ```
/// use dom_types::AtomTable;
/// use std::sync::Arc;
/// use std::thread;
///
/// let table = Arc::new(AtomTable::new());
/// let table_clone = Arc::clone(&table);
///
/// let handle = thread::spawn(move || {
///     table_clone.get_or_intern("hello")
/// });
///
/// let atom = handle.join().unwrap();
/// assert_eq!(table.resolve(&atom), "hello");
/// ```
#[derive(Clone)]
pub struct AtomTable {
    /// Maps strings to their atom IDs
    string_to_id: Arc<RwLock<HashMap<String, usize>>>,
    /// Maps atom IDs to their strings
    id_to_string: Arc<RwLock<Vec<Arc<str>>>>,
}

impl AtomTable {
    /// Create a new `AtomTable` with pre-interned common DOM strings
    ///
    /// The table is initialized with commonly used DOM strings including:
    /// - Tag names (div, span, p, a, img, body, head, html, button, input)
    /// - Attributes (class, id, style, href, src, type, value, name)
    /// - Namespaces (XML, XMLNS, HTML, SVG, MathML)
    ///
    /// # Example
    ///
    /// ```
    /// use dom_types::AtomTable;
    ///
    /// let table = AtomTable::new();
    /// assert!(table.len() > 0); // Contains pre-interned strings
    /// ```
    pub fn new() -> Self {
        let string_to_id = Arc::new(RwLock::new(HashMap::new()));
        let id_to_string = Arc::new(RwLock::new(Vec::new()));

        let table = Self {
            string_to_id,
            id_to_string,
        };

        // Pre-intern common DOM strings
        table.pre_intern_common_strings();

        table
    }

    /// Pre-intern commonly used DOM strings
    fn pre_intern_common_strings(&self) {
        // Common tag names
        let tag_names = [
            "div", "span", "p", "a", "img", "body", "head", "html", "button", "input",
        ];

        // Common attributes
        let attributes = [
            "class", "id", "style", "href", "src", "type", "value", "name",
        ];

        // Namespace URIs
        let namespaces = [
            "http://www.w3.org/XML/1998/namespace",
            "http://www.w3.org/2000/xmlns/",
            "http://www.w3.org/1999/xhtml",
            "http://www.w3.org/2000/svg",
            "http://www.w3.org/1998/Math/MathML",
        ];

        // Intern all common strings
        for s in tag_names
            .iter()
            .chain(attributes.iter())
            .chain(namespaces.iter())
        {
            self.get_or_intern(s);
        }
    }

    /// Get an existing atom for a string or create a new one
    ///
    /// This method checks if the string is already interned. If so, it returns
    /// the existing atom. Otherwise, it creates a new atom and stores the string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to intern
    ///
    /// # Returns
    ///
    /// An `Atom` representing the interned string
    ///
    /// # Example
    ///
    /// ```
    /// use dom_types::AtomTable;
    ///
    /// let table = AtomTable::new();
    /// let atom1 = table.get_or_intern("hello");
    /// let atom2 = table.get_or_intern("hello");
    /// assert_eq!(atom1, atom2); // Same string produces same atom
    /// ```
    pub fn get_or_intern(&self, s: &str) -> Atom {
        // Fast path: check if string is already interned (read lock only)
        {
            let string_to_id = self.string_to_id.read().unwrap();
            if let Some(&id) = string_to_id.get(s) {
                return Atom::new(id);
            }
        }

        // Slow path: intern the string (write lock required)
        let mut string_to_id = self.string_to_id.write().unwrap();
        let mut id_to_string = self.id_to_string.write().unwrap();

        // Double-check in case another thread interned it while we waited for the lock
        if let Some(&id) = string_to_id.get(s) {
            return Atom::new(id);
        }

        // Create new atom
        let id = id_to_string.len();
        let arc_str: Arc<str> = Arc::from(s);

        id_to_string.push(Arc::clone(&arc_str));
        string_to_id.insert(s.to_string(), id);

        Atom::new(id)
    }

    /// Resolve an atom back to its string
    ///
    /// # Arguments
    ///
    /// * `atom` - The atom to resolve
    ///
    /// # Returns
    ///
    /// A string slice representing the atom's value
    ///
    /// # Panics
    ///
    /// Panics if the atom ID is invalid (should never happen with properly
    /// created atoms from this table)
    ///
    /// # Example
    ///
    /// ```
    /// use dom_types::AtomTable;
    ///
    /// let table = AtomTable::new();
    /// let atom = table.get_or_intern("world");
    /// assert_eq!(table.resolve(&atom), "world");
    /// ```
    pub fn resolve(&self, atom: &Atom) -> &str {
        let id_to_string = self.id_to_string.read().unwrap();
        // Safety: We need to extend the lifetime here. This is safe because:
        // 1. Atoms are only created by this table
        // 2. Strings are stored in Arc<str> and never removed
        // 3. The table is never dropped while atoms exist (enforced by API design)
        unsafe {
            let s: &str = &id_to_string[atom.id];
            // Extend lifetime from guard's lifetime to 'static
            // This is safe because Arc<str> ensures the string lives as long as needed
            std::mem::transmute::<&str, &str>(s)
        }
    }

    /// Get the number of interned strings in the table
    ///
    /// # Returns
    ///
    /// The count of unique strings currently interned
    ///
    /// # Example
    ///
    /// ```
    /// use dom_types::AtomTable;
    ///
    /// let table = AtomTable::new();
    /// let initial_len = table.len();
    /// table.get_or_intern("new_string");
    /// assert_eq!(table.len(), initial_len + 1);
    /// ```
    pub fn len(&self) -> usize {
        let id_to_string = self.id_to_string.read().unwrap();
        id_to_string.len()
    }

    /// Check if the table is empty
    ///
    /// # Returns
    ///
    /// `true` if no strings are interned, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use dom_types::AtomTable;
    ///
    /// let table = AtomTable::new();
    /// assert!(!table.is_empty()); // Has pre-interned strings
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for AtomTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_creation() {
        let atom = Atom::new(42);
        assert_eq!(atom.id(), 42);
    }

    #[test]
    fn test_atom_equality() {
        let atom1 = Atom::new(1);
        let atom2 = Atom::new(1);
        let atom3 = Atom::new(2);

        assert_eq!(atom1, atom2);
        assert_ne!(atom1, atom3);
    }

    #[test]
    fn test_atom_clone() {
        let atom1 = Atom::new(5);
        let atom2 = atom1.clone();
        assert_eq!(atom1, atom2);
    }

    #[test]
    fn test_atom_copy() {
        let atom1 = Atom::new(5);
        let atom2 = atom1; // Copy
        assert_eq!(atom1, atom2);
    }

    #[test]
    fn test_atom_table_default() {
        let table = AtomTable::default();
        assert!(table.len() > 0);
    }

    #[test]
    fn test_basic_interning() {
        let table = AtomTable::new();
        let atom = table.get_or_intern("test");
        assert_eq!(table.resolve(&atom), "test");
    }

    #[test]
    fn test_deduplication() {
        let table = AtomTable::new();
        let atom1 = table.get_or_intern("dup");
        let atom2 = table.get_or_intern("dup");
        assert_eq!(atom1, atom2);
    }
}
