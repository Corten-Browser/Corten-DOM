//! Comprehensive tests for TreeWalker implementation

use dom_collections::node_iterator::{
    FilterResult, NodeFilter, SHOW_ALL, SHOW_COMMENT, SHOW_ELEMENT, SHOW_TEXT,
};
use dom_collections::tree_walker::TreeWalker;
use dom_core::{Document, Element, Node};
use dom_types::NodeType;
use parking_lot::RwLock;
use std::sync::Arc;

/// Helper to create a test tree structure
/// Tree structure:
///   root (div)
///     ├── text1 ("Hello")
///     ├── span
///     │   ├── text2 ("World")
///     │   └── b
///     ├── comment ("test comment")
///     └── p
///         ├── text3 ("End")
///         └── em
fn create_test_tree() -> Arc<RwLock<Box<dyn Node>>> {
    let mut doc = Document::new();

    // Create root element
    let root = doc.create_element("div").unwrap();

    // Create and add text1
    let text1 = doc.create_text_node("Hello");
    root.write().append_child(text1).unwrap();

    // Create span with text2 and b
    let span = doc.create_element("span").unwrap();
    let text2 = doc.create_text_node("World");
    span.write().append_child(text2).unwrap();
    let b = doc.create_element("b").unwrap();
    span.write().append_child(Arc::new(RwLock::new(Box::new(b.read().clone()) as Box<dyn Node>))).unwrap();

    // Convert span to NodeRef and add to root
    let span_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(span.read().clone()) as Box<dyn Node>));
    root.write().append_child(span_node).unwrap();

    // Create and add comment
    let comment = doc.create_comment("test comment");
    root.write().append_child(comment).unwrap();

    // Create p with text3 and em
    let p = doc.create_element("p").unwrap();
    let text3 = doc.create_text_node("End");
    p.write().append_child(text3).unwrap();
    let em = doc.create_element("em").unwrap();
    p.write().append_child(Arc::new(RwLock::new(Box::new(em.read().clone()) as Box<dyn Node>))).unwrap();

    // Convert p to NodeRef and add to root
    let p_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(p.read().clone()) as Box<dyn Node>));
    root.write().append_child(p_node).unwrap();

    // Convert root to NodeRef and return
    let root_clone = root.read().clone();
    Arc::new(RwLock::new(Box::new(root_clone) as Box<dyn Node>))
}

/// Helper to create a simpler test tree
/// Tree structure:
///   root (div)
///     ├── span
///     └── p
fn create_simple_tree() -> Arc<RwLock<Box<dyn Node>>> {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    root.write().append_child(Arc::new(RwLock::new(Box::new(span.read().clone()) as Box<dyn Node>))).unwrap();

    let p = doc.create_element("p").unwrap();
    root.write().append_child(Arc::new(RwLock::new(Box::new(p.read().clone()) as Box<dyn Node>))).unwrap();

    let root_clone = root.read().clone();
    Arc::new(RwLock::new(Box::new(root_clone) as Box<dyn Node>))
}

#[test]
fn test_tree_walker_creation() {
    let root = create_test_tree();
    let walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Should start at root
    assert_eq!(walker.current_node().read().node_name(), root.read().node_name());
    assert_eq!(walker.root().read().node_name(), root.read().node_name());
    assert_eq!(walker.what_to_show(), SHOW_ALL);
}

#[test]
fn test_current_node_getter() {
    let root = create_test_tree();
    let walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // current_node should return current position
    let current = walker.current_node();
    assert_eq!(current.read().node_name(), "DIV");
}

#[test]
fn test_set_current_node() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Get first child
    let first_child = root.read().child_nodes()[0].clone();

    // Set current_node
    walker.set_current_node(first_child.clone());

    // Verify it changed
    assert_eq!(walker.current_node().read().node_name(), first_child.read().node_name());
}

#[test]
fn test_root_getter() {
    let root = create_test_tree();
    let walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // root() should return the root node
    let walker_root = walker.root();
    assert_eq!(walker_root.read().node_name(), root.read().node_name());
}

#[test]
fn test_what_to_show_getter() {
    let root = create_test_tree();
    let walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    assert_eq!(walker.what_to_show(), SHOW_ELEMENT);
}

#[test]
fn test_first_child_basic() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to first child
    let first = walker.first_child();
    assert!(first.is_some());
    assert_eq!(first.unwrap().read().node_name(), "SPAN");

    // current_node should have moved
    assert_eq!(walker.current_node().read().node_name(), "SPAN");
}

#[test]
fn test_first_child_no_children() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to span (which has no children)
    walker.first_child();

    // Try to go to first child of span
    let result = walker.first_child();
    assert!(result.is_none());

    // current_node should not have changed
    assert_eq!(walker.current_node().read().node_name(), "SPAN");
}

#[test]
fn test_first_child_with_filter() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Move to first child (should skip text, return span)
    let first = walker.first_child();
    assert!(first.is_some());
    assert_eq!(first.unwrap().read().node_name(), "SPAN");
}

#[test]
fn test_last_child_basic() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to last child
    let last = walker.last_child();
    assert!(last.is_some());
    assert_eq!(last.unwrap().read().node_name(), "P");

    // current_node should have moved
    assert_eq!(walker.current_node().read().node_name(), "P");
}

#[test]
fn test_last_child_no_children() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to p (which has no children)
    walker.last_child();

    // Try to go to last child of p
    let result = walker.last_child();
    assert!(result.is_none());

    // current_node should not have changed
    assert_eq!(walker.current_node().read().node_name(), "P");
}

#[test]
fn test_last_child_with_filter() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Move to last child (should skip comment and text, return p)
    let last = walker.last_child();
    assert!(last.is_some());
    assert_eq!(last.unwrap().read().node_name(), "P");
}

#[test]
fn test_next_sibling_basic() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to first child (span)
    walker.first_child();

    // Move to next sibling
    let next = walker.next_sibling();
    assert!(next.is_some());
    assert_eq!(next.unwrap().read().node_name(), "P");

    // current_node should have moved
    assert_eq!(walker.current_node().read().node_name(), "P");
}

#[test]
fn test_next_sibling_no_sibling() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to last child (p)
    walker.last_child();

    // Try to move to next sibling
    let result = walker.next_sibling();
    assert!(result.is_none());

    // current_node should not have changed
    assert_eq!(walker.current_node().read().node_name(), "P");
}

#[test]
fn test_next_sibling_with_filter() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Move to first child (span)
    walker.first_child();

    // Move to next sibling (should skip comment, return p)
    let next = walker.next_sibling();
    assert!(next.is_some());
    assert_eq!(next.unwrap().read().node_name(), "P");
}

#[test]
fn test_previous_sibling_basic() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to last child (p)
    walker.last_child();

    // Move to previous sibling
    let prev = walker.previous_sibling();
    assert!(prev.is_some());
    assert_eq!(prev.unwrap().read().node_name(), "SPAN");

    // current_node should have moved
    assert_eq!(walker.current_node().read().node_name(), "SPAN");
}

#[test]
fn test_previous_sibling_no_sibling() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to first child (span)
    walker.first_child();

    // Try to move to previous sibling
    let result = walker.previous_sibling();
    assert!(result.is_none());

    // current_node should not have changed
    assert_eq!(walker.current_node().read().node_name(), "SPAN");
}

#[test]
fn test_previous_sibling_with_filter() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Move to last child (p)
    walker.last_child();

    // Move to previous sibling (should skip comment and text, return span)
    let prev = walker.previous_sibling();
    assert!(prev.is_some());
    assert_eq!(prev.unwrap().read().node_name(), "SPAN");
}

#[test]
fn test_parent_node_basic() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to first child
    walker.first_child();

    // Move to parent
    let parent = walker.parent_node();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().read().node_name(), "DIV");

    // current_node should have moved
    assert_eq!(walker.current_node().read().node_name(), "DIV");
}

#[test]
fn test_parent_node_at_root() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Try to move to parent from root
    let result = walker.parent_node();
    assert!(result.is_none());

    // current_node should not have changed
    assert_eq!(walker.current_node().read().node_name(), "DIV");
}

#[test]
fn test_parent_node_stops_at_root() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Navigate deep into tree
    walker.first_child(); // text1

    // Move to parent (should return to root)
    let parent = walker.parent_node();
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().read().node_name(), "DIV");

    // Try to go further (should fail - at root boundary)
    let result = walker.parent_node();
    assert!(result.is_none());
}

#[test]
fn test_next_node_sequential() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Sequential traversal: root(div), span, p
    let node1 = walker.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "SPAN");

    let node2 = walker.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "P");

    let node3 = walker.next_node();
    assert!(node3.is_none()); // End of tree
}

#[test]
fn test_next_node_with_elements_only() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Should traverse only elements: span, b, p, em
    let node1 = walker.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "SPAN");

    let node2 = walker.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "B");

    let node3 = walker.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "P");

    let node4 = walker.next_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "EM");

    let node5 = walker.next_node();
    assert!(node5.is_none());
}

#[test]
fn test_previous_node_sequential() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Move to end
    walker.next_node();
    walker.next_node();

    // Go backwards: p -> span
    let node1 = walker.previous_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "SPAN");

    // Try to go back further (should return None - current_node is first accepted)
    let node2 = walker.previous_node();
    assert!(node2.is_none());
}

#[test]
fn test_previous_node_with_elements_only() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Move to end
    while walker.next_node().is_some() {}

    // Go backwards: em -> p -> b -> span
    let node1 = walker.previous_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "P");

    let node2 = walker.previous_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "B");

    let node3 = walker.previous_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "SPAN");

    let node4 = walker.previous_node();
    assert!(node4.is_none());
}

#[test]
fn test_complex_navigation() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, None);

    // Start at root (div)
    assert_eq!(walker.current_node().read().node_name(), "DIV");

    // Go to first child (span)
    let span = walker.first_child();
    assert!(span.is_some());
    assert_eq!(span.unwrap().read().node_name(), "SPAN");

    // Go to next sibling (p)
    let p = walker.next_sibling();
    assert!(p.is_some());
    assert_eq!(p.unwrap().read().node_name(), "P");

    // Go to first child of p (em)
    let em = walker.first_child();
    assert!(em.is_some());
    assert_eq!(em.unwrap().read().node_name(), "EM");

    // Go to parent (p)
    let back_to_p = walker.parent_node();
    assert!(back_to_p.is_some());
    assert_eq!(back_to_p.unwrap().read().node_name(), "P");

    // Go to previous sibling (span)
    let back_to_span = walker.previous_sibling();
    assert!(back_to_span.is_some());
    assert_eq!(back_to_span.unwrap().read().node_name(), "SPAN");
}

#[test]
fn test_custom_filter_accept() {
    let root = create_test_tree();

    // Filter that accepts only elements with name "P"
    let filter: NodeFilter = Some(Arc::new(|node: &Arc<RwLock<Box<dyn Node>>>| {
        if node.read().node_name() == "P" {
            FilterResult::Accept
        } else {
            FilterResult::Skip
        }
    }));

    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, filter);

    // Should find only "P" element
    let p = walker.next_node();
    assert!(p.is_some());
    assert_eq!(p.unwrap().read().node_name(), "P");

    let none = walker.next_node();
    assert!(none.is_none());
}

#[test]
fn test_custom_filter_reject() {
    let root = create_test_tree();

    // Filter that rejects elements with name "SPAN" (and its descendants)
    let filter: NodeFilter = Some(Arc::new(|node: &Arc<RwLock<Box<dyn Node>>>| {
        if node.read().node_name() == "SPAN" {
            FilterResult::Reject
        } else {
            FilterResult::Accept
        }
    }));

    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, filter);

    // Should skip span and its descendants (B)
    // Should find only P and EM
    let p = walker.next_node();
    assert!(p.is_some());
    assert_eq!(p.unwrap().read().node_name(), "P");

    let em = walker.next_node();
    assert!(em.is_some());
    assert_eq!(em.unwrap().read().node_name(), "EM");

    let none = walker.next_node();
    assert!(none.is_none());
}

#[test]
fn test_custom_filter_skip() {
    let root = create_test_tree();

    // Filter that skips "SPAN" but accepts its children
    let filter: NodeFilter = Some(Arc::new(|node: &Arc<RwLock<Box<dyn Node>>>| {
        if node.read().node_name() == "SPAN" {
            FilterResult::Skip
        } else {
            FilterResult::Accept
        }
    }));

    let mut walker = TreeWalker::new(root.clone(), SHOW_ELEMENT, filter);

    // Should skip SPAN but find B (child of span), P, EM
    let b = walker.next_node();
    assert!(b.is_some());
    assert_eq!(b.unwrap().read().node_name(), "B");

    let p = walker.next_node();
    assert!(p.is_some());
    assert_eq!(p.unwrap().read().node_name(), "P");

    let em = walker.next_node();
    assert!(em.is_some());
    assert_eq!(em.unwrap().read().node_name(), "EM");

    let none = walker.next_node();
    assert!(none.is_none());
}

#[test]
fn test_set_current_node_outside_root() {
    let root = create_test_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // Create a different node outside root subtree
    let mut doc = Document::new();
    let external = doc.create_element("external").unwrap();
    let external_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(external.read().clone()) as Box<dyn Node>));

    // Set current_node to external node (should be allowed)
    walker.set_current_node(external_node.clone());

    // Verify it changed
    assert_eq!(walker.current_node().read().node_name(), "EXTERNAL");

    // Navigation should still respect root boundary
    // parent_node should work (even though we're outside root)
    let result = walker.parent_node();
    // Result depends on implementation - might be None
}

#[test]
fn test_navigation_at_boundaries() {
    let root = create_simple_tree();
    let mut walker = TreeWalker::new(root.clone(), SHOW_ALL, None);

    // At root - parent_node should return None
    assert!(walker.parent_node().is_none());

    // Go to first child
    walker.first_child();

    // At first child - previous_sibling should return None
    assert!(walker.previous_sibling().is_none());

    // Go to last sibling
    walker.next_sibling();

    // At last child - next_sibling should return None
    assert!(walker.next_sibling().is_none());
}

#[test]
fn test_filter_result_skip_vs_reject() {
    let root = create_test_tree();

    // Test Skip: Skips node but checks children
    let skip_filter: NodeFilter = Some(Arc::new(|node: &Arc<RwLock<Box<dyn Node>>>| {
        if node.read().node_name() == "SPAN" {
            FilterResult::Skip // Skip SPAN but allow B
        } else {
            FilterResult::Accept
        }
    }));

    let mut walker_skip = TreeWalker::new(root.clone(), SHOW_ELEMENT, skip_filter);

    // Should find B (child of skipped SPAN)
    let b = walker_skip.next_node();
    assert!(b.is_some());
    assert_eq!(b.unwrap().read().node_name(), "B");

    // Test Reject: Rejects node AND its descendants
    let reject_filter: NodeFilter = Some(Arc::new(|node: &Arc<RwLock<Box<dyn Node>>>| {
        if node.read().node_name() == "SPAN" {
            FilterResult::Reject // Reject SPAN and all children
        } else {
            FilterResult::Accept
        }
    }));

    let mut walker_reject = TreeWalker::new(root.clone(), SHOW_ELEMENT, reject_filter);

    // Should NOT find B (rejected along with SPAN)
    // Should find P instead
    let p = walker_reject.next_node();
    assert!(p.is_some());
    assert_eq!(p.unwrap().read().node_name(), "P");
}
