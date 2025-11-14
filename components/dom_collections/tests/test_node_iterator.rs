//! Comprehensive tests for NodeIterator implementation

use dom_collections::node_iterator::{
    FilterResult, NodeFilter, NodeIterator, SHOW_ALL, SHOW_COMMENT, SHOW_ELEMENT, SHOW_TEXT,
};
use dom_core::{Document, Element, Node};
use parking_lot::RwLock;
use std::sync::Arc;

/// Helper to create a test tree structure
/// Tree structure:
///   root (div)
///     ├── text1 ("Hello")
///     ├── span
///     │   └── text2 ("World")
///     ├── comment ("test comment")
///     └── p
///         └── text3 ("End")
fn create_test_tree() -> Arc<RwLock<Box<dyn Node>>> {
    let mut doc = Document::new();

    // Create root element
    let root = doc.create_element("div").unwrap();

    // Create and add text1
    let text1 = doc.create_text_node("Hello");
    root.write().append_child(text1).unwrap();

    // Create span with text2
    let span = doc.create_element("span").unwrap();
    let text2 = doc.create_text_node("World");
    span.write().append_child(text2).unwrap();

    // Convert span to NodeRef and add to root
    let span_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(span.read().clone()) as Box<dyn Node>));
    root.write().append_child(span_node).unwrap();

    // Create and add comment
    let comment = doc.create_comment("test comment");
    root.write().append_child(comment).unwrap();

    // Create p with text3
    let p = doc.create_element("p").unwrap();
    let text3 = doc.create_text_node("End");
    p.write().append_child(text3).unwrap();

    // Convert p to NodeRef and add to root
    let p_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(p.read().clone()) as Box<dyn Node>));
    root.write().append_child(p_node).unwrap();

    // Convert root to NodeRef and return
    let root_clone = root.read().clone();
    Arc::new(RwLock::new(Box::new(root_clone) as Box<dyn Node>))
}

#[test]
fn test_node_iterator_creation() {
    let root = create_test_tree();
    let iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Compare node names instead of NodeRef directly (NodeRef doesn't implement PartialEq)
    assert_eq!(iter.root().read().node_name(), root.read().node_name());
    assert_eq!(iter.what_to_show(), SHOW_ALL);
}

#[test]
fn test_next_node_show_all() {
    let root = create_test_tree();

    // Debug: Check tree structure
    let children_count = root.read().child_nodes().len();
    eprintln!("Root has {} children", children_count);
    for (i, child) in root.read().child_nodes().iter().enumerate() {
        eprintln!("  Child {}: {}", i, child.read().node_name());
    }

    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Should traverse in depth-first pre-order
    // Expected order: root(div), text1, span, text2, comment, p, text3

    let node1 = iter.next_node();
    assert!(node1.is_some(), "node1 should be Some");
    assert_eq!(node1.unwrap().read().node_name(), "DIV"); // root

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "#text"); // text1

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "SPAN"); // span

    let node4 = iter.next_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "#text"); // text2

    let node5 = iter.next_node();
    assert!(node5.is_some());
    assert_eq!(node5.unwrap().read().node_name(), "#comment"); // comment

    let node6 = iter.next_node();
    assert!(node6.is_some());
    assert_eq!(node6.unwrap().read().node_name(), "P"); // p

    let node7 = iter.next_node();
    assert!(node7.is_some());
    assert_eq!(node7.unwrap().read().node_name(), "#text"); // text3

    // Should reach end
    let node8 = iter.next_node();
    assert!(node8.is_none());

    // Should still return None
    let node9 = iter.next_node();
    assert!(node9.is_none());
}

#[test]
fn test_next_node_show_element() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ELEMENT, None);

    // Should only show elements: div, span, p

    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "SPAN");

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "P");

    let node4 = iter.next_node();
    assert!(node4.is_none());
}

#[test]
fn test_next_node_show_text() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_TEXT, None);

    // Should only show text nodes: text1, text2, text3

    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "#text");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "#text");

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "#text");

    let node4 = iter.next_node();
    assert!(node4.is_none());
}

#[test]
fn test_next_node_show_comment() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_COMMENT, None);

    // Should only show comment node

    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "#comment");

    let node2 = iter.next_node();
    assert!(node2.is_none());
}

#[test]
fn test_previous_node_show_all() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Move to end first
    while iter.next_node().is_some() {}

    // Now traverse backwards
    // Expected reverse order: text3, p, comment, text2, span, text1, div

    let node1 = iter.previous_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "#text"); // text3

    let node2 = iter.previous_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "P");

    let node3 = iter.previous_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "#comment");

    let node4 = iter.previous_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "#text"); // text2

    let node5 = iter.previous_node();
    assert!(node5.is_some());
    assert_eq!(node5.unwrap().read().node_name(), "SPAN");

    let node6 = iter.previous_node();
    assert!(node6.is_some());
    assert_eq!(node6.unwrap().read().node_name(), "#text"); // text1

    let node7 = iter.previous_node();
    assert!(node7.is_some());
    assert_eq!(node7.unwrap().read().node_name(), "DIV");

    // Should reach beginning
    let node8 = iter.previous_node();
    assert!(node8.is_none());
}

#[test]
fn test_previous_node_before_first_next() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Calling previous_node before next_node should return None
    let node = iter.previous_node();
    assert!(node.is_none());
}

#[test]
fn test_custom_filter_accept() {
    let root = create_test_tree();

    // Filter that only accepts SPAN elements
    let filter: NodeFilter = Some(Arc::new(|node| {
        if let Some(element) = node.read().as_any().downcast_ref::<Element>() {
            if element.tag_name() == "SPAN" {
                return FilterResult::Accept;
            }
        }
        FilterResult::Skip
    }));

    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, filter);

    // Should only return span
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "SPAN");

    let node2 = iter.next_node();
    assert!(node2.is_none());
}

#[test]
fn test_custom_filter_reject() {
    let root = create_test_tree();

    // Filter that rejects DIV (and all descendants)
    let filter: NodeFilter = Some(Arc::new(|node| {
        if let Some(element) = node.read().as_any().downcast_ref::<Element>() {
            if element.tag_name() == "DIV" {
                return FilterResult::Reject;
            }
        }
        FilterResult::Accept
    }));

    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, filter);

    // Should return nothing because root is rejected
    let node1 = iter.next_node();
    assert!(node1.is_none());
}

#[test]
fn test_custom_filter_skip() {
    let root = create_test_tree();

    // Filter that skips SPAN but accepts its descendants
    let filter: NodeFilter = Some(Arc::new(|node| {
        if let Some(element) = node.read().as_any().downcast_ref::<Element>() {
            if element.tag_name() == "SPAN" {
                return FilterResult::Skip;
            }
        }
        FilterResult::Accept
    }));

    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, filter);

    // Should skip SPAN but include its text child
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "#text"); // text1

    // SPAN is skipped, but its child is included
    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "#text"); // text2 (child of SPAN)
}

#[test]
fn test_mixed_filter_and_what_to_show() {
    let root = create_test_tree();

    // SHOW_ELEMENT + filter that only accepts P elements
    let filter: NodeFilter = Some(Arc::new(|node| {
        if let Some(element) = node.read().as_any().downcast_ref::<Element>() {
            if element.tag_name() == "P" {
                return FilterResult::Accept;
            }
        }
        FilterResult::Skip
    }));

    let mut iter = NodeIterator::new(root.clone(), SHOW_ELEMENT, filter);

    // Should only return P element
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "P");

    let node2 = iter.next_node();
    assert!(node2.is_none());
}

#[test]
fn test_bidirectional_iteration() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ELEMENT, None);

    // Forward: div, span, p
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "SPAN");

    // Backward
    let node3 = iter.previous_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "SPAN");

    let node4 = iter.previous_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "DIV");

    // Forward again
    let node5 = iter.next_node();
    assert!(node5.is_some());
    assert_eq!(node5.unwrap().read().node_name(), "DIV");
}

#[test]
fn test_reference_node_getter() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Reference node should initially be root
    assert_eq!(
        iter.reference_node().read().node_name(),
        root.read().node_name()
    );

    // After next_node, reference_node should update
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(
        iter.reference_node().read().node_name(),
        node1.unwrap().read().node_name()
    );
}

#[test]
fn test_detach() {
    let root = create_test_tree();
    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, None);

    // Detach (deprecated but should not crash)
    iter.detach();

    // Should still work after detach (detach is a no-op in modern DOM)
    let node = iter.next_node();
    assert!(node.is_some());
}

#[test]
fn test_empty_tree() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let root_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(root.read().clone())));

    let mut iter = NodeIterator::new(root_node.clone(), SHOW_ALL, None);

    // Should return just the root
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    // No more nodes
    let node2 = iter.next_node();
    assert!(node2.is_none());
}

#[test]
fn test_single_level_tree() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Add two children
    let child1 = doc.create_element("span").unwrap();
    let child1_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(child1.read().clone())));
    root.write().append_child(child1_node).unwrap();

    let child2 = doc.create_element("p").unwrap();
    let child2_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(child2.read().clone())));
    root.write().append_child(child2_node).unwrap();

    let root_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(root.read().clone())));

    let mut iter = NodeIterator::new(root_node.clone(), SHOW_ELEMENT, None);

    // Should return: div, span, p
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "SPAN");

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "P");

    let node4 = iter.next_node();
    assert!(node4.is_none());
}

#[test]
fn test_deep_tree() {
    // Create deeper tree: div -> ul -> li -> span -> text
    let mut doc = Document::new();
    let div = doc.create_element("div").unwrap();
    let ul = doc.create_element("ul").unwrap();
    let li = doc.create_element("li").unwrap();
    let span = doc.create_element("span").unwrap();
    let text = doc.create_text_node("Deep");

    span.write().append_child(text).unwrap();

    let span_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(span.read().clone())));
    li.write().append_child(span_node).unwrap();

    let li_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(li.read().clone())));
    ul.write().append_child(li_node).unwrap();

    let ul_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(ul.read().clone())));
    div.write().append_child(ul_node).unwrap();

    let div_node: Arc<RwLock<Box<dyn Node>>> =
        Arc::new(RwLock::new(Box::new(div.read().clone())));

    let mut iter = NodeIterator::new(div_node.clone(), SHOW_ELEMENT, None);

    // Should traverse: div, ul, li, span
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "UL");

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "LI");

    let node4 = iter.next_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "SPAN");

    let node5 = iter.next_node();
    assert!(node5.is_none());
}

#[test]
fn test_iterator_with_filter_result_combinations() {
    let root = create_test_tree();

    // Complex filter: Accept DIV, Reject SPAN (and descendants), Skip P
    let filter: NodeFilter = Some(Arc::new(|node| {
        if let Some(element) = node.read().as_any().downcast_ref::<Element>() {
            match element.tag_name() {
                "DIV" => return FilterResult::Accept,
                "SPAN" => return FilterResult::Reject, // Skip SPAN and children
                "P" => return FilterResult::Skip,      // Skip P but check children
                _ => {}
            }
        }
        FilterResult::Accept
    }));

    let mut iter = NodeIterator::new(root.clone(), SHOW_ALL, filter);

    // Should get: DIV, text1, comment, text3 (P skipped but text3 included)
    let node1 = iter.next_node();
    assert!(node1.is_some());
    assert_eq!(node1.unwrap().read().node_name(), "DIV");

    let node2 = iter.next_node();
    assert!(node2.is_some());
    assert_eq!(node2.unwrap().read().node_name(), "#text"); // text1

    // SPAN is rejected (with descendants), so text2 is skipped

    let node3 = iter.next_node();
    assert!(node3.is_some());
    assert_eq!(node3.unwrap().read().node_name(), "#comment");

    // P is skipped, but its child text3 is included
    let node4 = iter.next_node();
    assert!(node4.is_some());
    assert_eq!(node4.unwrap().read().node_name(), "#text"); // text3

    let node5 = iter.next_node();
    assert!(node5.is_none());
}

#[test]
fn test_constants() {
    // Verify constants match DOM specification
    assert_eq!(SHOW_ALL, 0xFFFFFFFF);
    assert_eq!(SHOW_ELEMENT, 0x1);
    assert_eq!(SHOW_TEXT, 0x4);
    assert_eq!(SHOW_COMMENT, 0x80);
}
