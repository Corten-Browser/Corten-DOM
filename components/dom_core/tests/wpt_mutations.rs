//! WPT-Inspired Tree Mutation Tests
//!
//! These tests are translated from Web Platform Tests (WPT) focusing on tree mutations.
//! Original tests: https://github.com/web-platform-tests/wpt/tree/master/dom/nodes

use dom_core::{Document, Node};
use std::sync::Arc;
use parking_lot::RwLock;

/// Helper to create a document
fn create_document() -> Document {
    Document::new()
}

// ============================================================================
// Parent-Child Relationship Tests
// ============================================================================

/// WPT: dom/nodes/Node-appendChild.html
/// Test: appendChild should update child count
#[test]
fn test_append_child_updates_count() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();
    let child = doc.create_element("span").unwrap();

    assert_eq!(parent.read().child_nodes().len(), 0);

    let child_wrapped = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));
    parent.write().append_child(child_wrapped.clone()).unwrap();

    assert_eq!(parent.read().child_nodes().len(), 1);
}

/// WPT: dom/nodes/Node-removeChild.html
/// Test: removeChild should update child count
#[test]
fn test_remove_child_updates_count() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();
    let child = doc.create_element("span").unwrap();

    let child_wrapped = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));
    parent.write().append_child(child_wrapped.clone()).unwrap();
    assert_eq!(parent.read().child_nodes().len(), 1);

    // Remove child
    parent.write().remove_child(child_wrapped.clone()).unwrap();
    assert_eq!(parent.read().child_nodes().len(), 0);
}

/// WPT: dom/nodes/Node-appendChild.html
/// Test: Appending to new parent should remove from old parent
#[test]
fn test_reparent_updates_counts() {
    let mut doc = create_document();
    let parent1 = doc.create_element("div").unwrap();
    let parent2 = doc.create_element("section").unwrap();
    let child = doc.create_element("span").unwrap();

    let child_wrapped = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));

    // Append to first parent
    parent1.write().append_child(child_wrapped.clone()).unwrap();
    assert_eq!(parent1.read().child_nodes().len(), 1);

    // Append to second parent (should remove from first)
    parent2.write().append_child(child_wrapped.clone()).unwrap();
    assert_eq!(parent1.read().child_nodes().len(), 0);
    assert_eq!(parent2.read().child_nodes().len(), 1);
}

// ============================================================================
// Sibling Navigation Tests
// ============================================================================

/// WPT: dom/nodes/Node-properties.html
/// Test: First child should be correct after multiple appends
#[test]
fn test_first_child_after_multiple_appends() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();

    let child1 = doc.create_element("span").unwrap();
    let child2 = doc.create_element("p").unwrap();
    let child3 = doc.create_element("a").unwrap();

    let c1 = Arc::new(RwLock::new(Box::new(child1.read().clone()) as Box<dyn Node>));
    let c2 = Arc::new(RwLock::new(Box::new(child2.read().clone()) as Box<dyn Node>));
    let c3 = Arc::new(RwLock::new(Box::new(child3.read().clone()) as Box<dyn Node>));

    parent.write().append_child(c1).unwrap();
    parent.write().append_child(c2).unwrap();
    parent.write().append_child(c3).unwrap();

    let first = parent.read().first_child().unwrap();
    assert_eq!(first.read().node_name(), "SPAN");
}

/// WPT: dom/nodes/Node-properties.html
/// Test: Last child should be correct after multiple appends
#[test]
fn test_last_child_after_multiple_appends() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();

    let child1 = doc.create_element("span").unwrap();
    let child2 = doc.create_element("p").unwrap();
    let child3 = doc.create_element("a").unwrap();

    let c1 = Arc::new(RwLock::new(Box::new(child1.read().clone()) as Box<dyn Node>));
    let c2 = Arc::new(RwLock::new(Box::new(child2.read().clone()) as Box<dyn Node>));
    let c3 = Arc::new(RwLock::new(Box::new(child3.read().clone()) as Box<dyn Node>));

    parent.write().append_child(c1).unwrap();
    parent.write().append_child(c2).unwrap();
    parent.write().append_child(c3).unwrap();

    let last = parent.read().last_child().unwrap();
    assert_eq!(last.read().node_name(), "A");
}

/// WPT: dom/nodes/Node-insertBefore.html
/// Test: insertBefore at beginning should update firstChild
#[test]
fn test_insert_before_updates_first_child() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();

    let existing = doc.create_element("span").unwrap();
    let new_child = doc.create_element("p").unwrap();

    let existing_wrapped = Arc::new(RwLock::new(Box::new(existing.read().clone()) as Box<dyn Node>));
    let new_wrapped = Arc::new(RwLock::new(Box::new(new_child.read().clone()) as Box<dyn Node>));

    parent.write().append_child(existing_wrapped.clone()).unwrap();
    parent.write().insert_before(new_wrapped, Some(existing_wrapped)).unwrap();

    let first = parent.read().first_child().unwrap();
    assert_eq!(first.read().node_name(), "P");
}

// ============================================================================
// Complex Tree Operations
// ============================================================================

/// WPT: dom/nodes/Node-appendChild.html
/// Test: Building a deep tree structure
#[test]
fn test_deep_tree_structure() {
    let mut doc = create_document();
    let root = doc.create_element("div").unwrap();
    let child = doc.create_element("ul").unwrap();
    let grandchild = doc.create_element("li").unwrap();

    let child_wrapped = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));
    let grandchild_wrapped = Arc::new(RwLock::new(Box::new(grandchild.read().clone()) as Box<dyn Node>));

    // Build tree: div > ul > li
    child_wrapped.write().append_child(grandchild_wrapped).unwrap();
    root.write().append_child(child_wrapped).unwrap();

    assert_eq!(root.read().child_nodes().len(), 1);
    assert_eq!(root.read().first_child().unwrap().read().node_name(), "UL");
}

/// WPT: dom/nodes/Node-replaceChild.html
/// Test: replaceChild should preserve sibling relationships
#[test]
fn test_replace_child_preserves_siblings() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();

    let child1 = doc.create_element("span").unwrap();
    let child2 = doc.create_element("p").unwrap();
    let child3 = doc.create_element("a").unwrap();
    let replacement = doc.create_element("section").unwrap();

    let c1 = Arc::new(RwLock::new(Box::new(child1.read().clone()) as Box<dyn Node>));
    let c2 = Arc::new(RwLock::new(Box::new(child2.read().clone()) as Box<dyn Node>));
    let c3 = Arc::new(RwLock::new(Box::new(child3.read().clone()) as Box<dyn Node>));
    let repl = Arc::new(RwLock::new(Box::new(replacement.read().clone()) as Box<dyn Node>));

    parent.write().append_child(c1).unwrap();
    parent.write().append_child(c2.clone()).unwrap();
    parent.write().append_child(c3).unwrap();

    // Replace middle child
    parent.write().replace_child(repl, c2).unwrap();

    let children = parent.read().child_nodes();
    assert_eq!(children.len(), 3);
    assert_eq!(children[0].read().node_name(), "SPAN");
    assert_eq!(children[1].read().node_name(), "SECTION");
    assert_eq!(children[2].read().node_name(), "A");
}

/// WPT: dom/nodes/Node-cloneNode.html
/// Test: Deep clone should preserve tree structure
#[test]
fn test_deep_clone_preserves_structure() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();
    let child1 = doc.create_element("span").unwrap();
    let child2 = doc.create_element("p").unwrap();

    let c1 = Arc::new(RwLock::new(Box::new(child1.read().clone()) as Box<dyn Node>));
    let c2 = Arc::new(RwLock::new(Box::new(child2.read().clone()) as Box<dyn Node>));

    parent.write().append_child(c1).unwrap();
    parent.write().append_child(c2).unwrap();

    // Deep clone
    let clone = parent.read().clone_node(true);

    assert_eq!(clone.read().node_name(), "DIV");
    assert_eq!(clone.read().child_nodes().len(), 2);
    assert_eq!(clone.read().child_nodes()[0].read().node_name(), "SPAN");
    assert_eq!(clone.read().child_nodes()[1].read().node_name(), "P");
}

/// WPT: dom/nodes/Node-insertBefore.html
/// Test: Insert multiple children in order
#[test]
fn test_insert_multiple_children_ordered() {
    let mut doc = create_document();
    let parent = doc.create_element("div").unwrap();

    let child1 = doc.create_element("a").unwrap();
    let child2 = doc.create_element("b").unwrap();
    let child3 = doc.create_element("c").unwrap();

    let c1 = Arc::new(RwLock::new(Box::new(child1.read().clone()) as Box<dyn Node>));
    let c2 = Arc::new(RwLock::new(Box::new(child2.read().clone()) as Box<dyn Node>));
    let c3 = Arc::new(RwLock::new(Box::new(child3.read().clone()) as Box<dyn Node>));

    // Insert in reverse order using insertBefore
    parent.write().insert_before(c3, None).unwrap(); // c
    parent.write().insert_before(c2, parent.read().first_child()).unwrap(); // b, c
    parent.write().insert_before(c1, parent.read().first_child()).unwrap(); // a, b, c

    let children = parent.read().child_nodes();
    assert_eq!(children[0].read().node_name(), "A");
    assert_eq!(children[1].read().node_name(), "B");
    assert_eq!(children[2].read().node_name(), "C");
}

// ============================================================================
// Summary
// ============================================================================
// Total WPT-inspired Mutation tests: 11
// Categories covered:
//   - Parent-child relationships (3 tests)
//   - Sibling navigation (3 tests)
//   - Complex tree operations (5 tests)
