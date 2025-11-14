//! Tests for string interning (Atom) system
//!
//! This test suite verifies:
//! - String deduplication works correctly
//! - Atoms are comparable and hashable
//! - Thread-safe concurrent access
//! - Pre-interned strings are available
//! - Atom equality and resolution

use dom_types::AtomTable;
use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

#[test]
fn test_atom_table_creation() {
    let table = AtomTable::new();
    assert!(
        table.len() > 0,
        "AtomTable should have pre-interned strings"
    );
}

#[test]
fn test_string_deduplication() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("hello");
    let atom2 = table.get_or_intern("hello");
    let atom3 = table.get_or_intern("world");

    assert_eq!(atom1, atom2, "Same string should produce same atom");
    assert_ne!(
        atom1, atom3,
        "Different strings should produce different atoms"
    );
}

#[test]
fn test_atom_resolution() {
    let table = AtomTable::new();
    let atom = table.get_or_intern("test_string");
    let resolved = table.resolve(&atom);

    assert_eq!(
        resolved, "test_string",
        "Atom should resolve to original string"
    );
}

#[test]
fn test_atom_equality() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("equal");
    let atom2 = table.get_or_intern("equal");
    let atom3 = table.get_or_intern("different");

    assert_eq!(atom1, atom2, "Atoms for same string should be equal");
    assert_ne!(
        atom1, atom3,
        "Atoms for different strings should not be equal"
    );
}

#[test]
fn test_atom_hashing() {
    let table = AtomTable::new();
    let mut set = HashSet::new();

    let atom1 = table.get_or_intern("one");
    let atom2 = table.get_or_intern("two");
    let atom3 = table.get_or_intern("one"); // duplicate

    set.insert(atom1);
    set.insert(atom2);
    set.insert(atom3);

    assert_eq!(set.len(), 2, "HashSet should contain only unique atoms");
}

#[test]
fn test_pre_interned_tag_names() {
    let table = AtomTable::new();

    // Common tag names should be pre-interned
    let div = table.get_or_intern("div");
    let span = table.get_or_intern("span");
    let p = table.get_or_intern("p");
    let a = table.get_or_intern("a");
    let img = table.get_or_intern("img");
    let body = table.get_or_intern("body");
    let head = table.get_or_intern("head");
    let html = table.get_or_intern("html");
    let button = table.get_or_intern("button");
    let input = table.get_or_intern("input");

    // Verify they resolve correctly
    assert_eq!(table.resolve(&div), "div");
    assert_eq!(table.resolve(&span), "span");
    assert_eq!(table.resolve(&p), "p");
    assert_eq!(table.resolve(&a), "a");
    assert_eq!(table.resolve(&img), "img");
    assert_eq!(table.resolve(&body), "body");
    assert_eq!(table.resolve(&head), "head");
    assert_eq!(table.resolve(&html), "html");
    assert_eq!(table.resolve(&button), "button");
    assert_eq!(table.resolve(&input), "input");
}

#[test]
fn test_pre_interned_attributes() {
    let table = AtomTable::new();

    // Common attributes should be pre-interned
    let class = table.get_or_intern("class");
    let id = table.get_or_intern("id");
    let style = table.get_or_intern("style");
    let href = table.get_or_intern("href");
    let src = table.get_or_intern("src");
    let type_attr = table.get_or_intern("type");
    let value = table.get_or_intern("value");
    let name = table.get_or_intern("name");

    // Verify they resolve correctly
    assert_eq!(table.resolve(&class), "class");
    assert_eq!(table.resolve(&id), "id");
    assert_eq!(table.resolve(&style), "style");
    assert_eq!(table.resolve(&href), "href");
    assert_eq!(table.resolve(&src), "src");
    assert_eq!(table.resolve(&type_attr), "type");
    assert_eq!(table.resolve(&value), "value");
    assert_eq!(table.resolve(&name), "name");
}

#[test]
fn test_pre_interned_namespaces() {
    let table = AtomTable::new();

    // Namespace strings should be pre-interned
    let xml = table.get_or_intern("http://www.w3.org/XML/1998/namespace");
    let xmlns = table.get_or_intern("http://www.w3.org/2000/xmlns/");
    let html = table.get_or_intern("http://www.w3.org/1999/xhtml");
    let svg = table.get_or_intern("http://www.w3.org/2000/svg");
    let mathml = table.get_or_intern("http://www.w3.org/1998/Math/MathML");

    // Verify they resolve correctly
    assert_eq!(table.resolve(&xml), "http://www.w3.org/XML/1998/namespace");
    assert_eq!(table.resolve(&xmlns), "http://www.w3.org/2000/xmlns/");
    assert_eq!(table.resolve(&html), "http://www.w3.org/1999/xhtml");
    assert_eq!(table.resolve(&svg), "http://www.w3.org/2000/svg");
    assert_eq!(table.resolve(&mathml), "http://www.w3.org/1998/Math/MathML");
}

#[test]
fn test_atom_clone() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("clone_test");
    let atom2 = atom1.clone();

    assert_eq!(atom1, atom2, "Cloned atoms should be equal");
    assert_eq!(table.resolve(&atom1), table.resolve(&atom2));
}

#[test]
fn test_atom_copy() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("copy_test");
    let atom2 = atom1; // Copy trait should allow this

    assert_eq!(atom1, atom2, "Copied atoms should be equal");
}

#[test]
fn test_atom_debug_format() {
    let table = AtomTable::new();
    let atom = table.get_or_intern("debug_test");
    let debug_str = format!("{:?}", atom);

    assert!(
        !debug_str.is_empty(),
        "Debug format should produce non-empty string"
    );
}

#[test]
fn test_thread_safety_concurrent_interning() {
    let table = Arc::new(AtomTable::new());
    let mut handles = vec![];

    // Spawn multiple threads that intern the same strings
    for i in 0..10 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let atom1 = table_clone.get_or_intern("concurrent");
            let atom2 = table_clone.get_or_intern(&format!("thread_{}", i));
            (atom1, atom2)
        });
        handles.push(handle);
    }

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All "concurrent" atoms should be equal
    let first_concurrent = results[0].0;
    for (atom, _) in &results {
        assert_eq!(
            *atom, first_concurrent,
            "All threads should get the same atom for 'concurrent'"
        );
    }

    // Verify all strings are resolvable
    for (atom1, atom2) in &results {
        assert_eq!(table.resolve(atom1), "concurrent");
        assert!(table.resolve(atom2).starts_with("thread_"));
    }
}

#[test]
fn test_thread_safety_concurrent_resolution() {
    let table = Arc::new(AtomTable::new());
    let atom = table.get_or_intern("resolve_me");
    let mut handles = vec![];

    // Spawn multiple threads that resolve the same atom
    for _ in 0..10 {
        let table_clone = Arc::clone(&table);
        let atom_clone = atom;
        let handle = thread::spawn(move || table_clone.resolve(&atom_clone).to_string());
        handles.push(handle);
    }

    // All resolutions should produce the same string
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for result in &results {
        assert_eq!(result, "resolve_me");
    }
}

#[test]
fn test_empty_string_interning() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("");
    let atom2 = table.get_or_intern("");

    assert_eq!(atom1, atom2, "Empty strings should produce same atom");
    assert_eq!(table.resolve(&atom1), "");
}

#[test]
fn test_long_string_interning() {
    let table = AtomTable::new();
    let long_string = "a".repeat(10000);
    let atom1 = table.get_or_intern(&long_string);
    let atom2 = table.get_or_intern(&long_string);

    assert_eq!(atom1, atom2, "Long strings should be deduplicated");
    assert_eq!(table.resolve(&atom1), long_string);
}

#[test]
fn test_unicode_string_interning() {
    let table = AtomTable::new();
    let unicode = "Hello 世界 🌍";
    let atom1 = table.get_or_intern(unicode);
    let atom2 = table.get_or_intern(unicode);

    assert_eq!(atom1, atom2, "Unicode strings should be deduplicated");
    assert_eq!(table.resolve(&atom1), unicode);
}

#[test]
fn test_atom_table_len() {
    let table = AtomTable::new();
    let initial_len = table.len();

    // Initial length should include pre-interned strings
    assert!(initial_len > 0, "Should have pre-interned strings");

    // Interning a new string should increase length
    let _atom = table.get_or_intern("new_unique_string_12345");
    assert_eq!(
        table.len(),
        initial_len + 1,
        "Length should increase after interning new string"
    );

    // Interning the same string should not increase length
    let _atom2 = table.get_or_intern("new_unique_string_12345");
    assert_eq!(
        table.len(),
        initial_len + 1,
        "Length should not increase for duplicate string"
    );
}

#[test]
fn test_atom_partial_eq() {
    let table = AtomTable::new();
    let atom1 = table.get_or_intern("test");
    let atom2 = table.get_or_intern("test");
    let atom3 = table.get_or_intern("other");

    assert!(atom1 == atom2, "Equal atoms should be equal");
    assert!(atom1 != atom3, "Different atoms should not be equal");
}
