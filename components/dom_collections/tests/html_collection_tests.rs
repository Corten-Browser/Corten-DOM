use dom_collections::HTMLCollection;
use dom_core::{Document, Element, ElementRef, Node};
use parking_lot::RwLock;
use std::sync::Arc;

// =============================================================================
// getElementsByTagName Tests
// =============================================================================

#[test]
fn test_by_tag_name_basic() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Add some elements
    let span1 = doc.create_element("span").unwrap();
    let span2 = doc.create_element("span").unwrap();
    let p = doc.create_element("p").unwrap();

    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        let p_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(p.read().clone())));
        root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
        root.write().append_child(p_node).unwrap();
    }

    let collection = HTMLCollection::by_tag_name(root.clone(), "span".to_string());

    assert_eq!(collection.length(), 2);
    assert_eq!(collection.item(0).unwrap().read().tag_name(), "SPAN");
    assert_eq!(collection.item(1).unwrap().read().tag_name(), "SPAN");
}

#[test]
fn test_by_tag_name_case_insensitive() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Test various case combinations
    let collection_lower = HTMLCollection::by_tag_name(root.clone(), "span".to_string());
    let collection_upper = HTMLCollection::by_tag_name(root.clone(), "SPAN".to_string());
    let collection_mixed = HTMLCollection::by_tag_name(root.clone(), "SpAn".to_string());

    assert_eq!(collection_lower.length(), 1);
    assert_eq!(collection_upper.length(), 1);
    assert_eq!(collection_mixed.length(), 1);
}

#[test]
fn test_by_tag_name_wildcard() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    let p = doc.create_element("p").unwrap();
    let section = doc.create_element("section").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        let p_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(p.read().clone())));
        let section_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(section.read().clone())));
        root.write().append_child(span_node).unwrap();
        root.write().append_child(p_node).unwrap();
        root.write().append_child(section_node).unwrap();
    }

    // "*" should match all elements
    let collection = HTMLCollection::by_tag_name(root.clone(), "*".to_string());

    assert_eq!(collection.length(), 3);
}

#[test]
fn test_by_tag_name_no_matches() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    let collection = HTMLCollection::by_tag_name(root.clone(), "article".to_string());

    assert_eq!(collection.length(), 0);
}

#[test]
fn test_by_tag_name_nested_elements() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create nested structure: root > child_div > span
    let child_div = doc.create_element("div").unwrap();
    let nested_span = doc.create_element("span").unwrap();
    {
        let nested_span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(nested_span.read().clone())));
        child_div.write().append_child(nested_span_node).unwrap();
    }

    // Also add a direct span child
    let direct_span = doc.create_element("span").unwrap();

    {
        let child_div_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(child_div.read().clone())));
        let direct_span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(direct_span.read().clone())));
        root.write().append_child(child_div_node).unwrap();
        root.write().append_child(direct_span_node).unwrap();
    }

    let collection = HTMLCollection::by_tag_name(root.clone(), "span".to_string());

    // Should find both spans (nested and direct)
    assert_eq!(collection.length(), 2);
}

// =============================================================================
// getElementsByClassName Tests
// =============================================================================

#[test]
fn test_by_class_name_single_class() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span1 = doc.create_element("span").unwrap();
    span1.write().set_attribute("class", "highlight").unwrap();

    let span2 = doc.create_element("span").unwrap();
    span2.write().set_attribute("class", "normal").unwrap();

    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
    }

    let collection = HTMLCollection::by_class_name(root.clone(), "highlight".to_string());

    assert_eq!(collection.length(), 1);
}

#[test]
fn test_by_class_name_multiple_classes_required() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Element with both classes
    let span1 = doc.create_element("span").unwrap();
    span1.write().set_attribute("class", "foo bar baz").unwrap();

    // Element with only one of the classes
    let span2 = doc.create_element("span").unwrap();
    span2.write().set_attribute("class", "foo").unwrap();

    // Element with extra classes
    let span3 = doc.create_element("span").unwrap();
    span3.write().set_attribute("class", "bar foo extra").unwrap();

    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        let span3_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span3.read().clone())));
        root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
        root.write().append_child(span3_node).unwrap();
    }

    // Searching for "foo bar" should match elements with BOTH classes
    let collection = HTMLCollection::by_class_name(root.clone(), "foo bar".to_string());

    // Should find span1 and span3 (both have foo and bar)
    assert_eq!(collection.length(), 2);
}

#[test]
fn test_by_class_name_order_independent() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    span.write().set_attribute("class", "alpha beta gamma").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Order shouldn't matter
    let collection1 = HTMLCollection::by_class_name(root.clone(), "alpha beta".to_string());
    let collection2 = HTMLCollection::by_class_name(root.clone(), "beta alpha".to_string());
    let collection3 = HTMLCollection::by_class_name(root.clone(), "gamma alpha beta".to_string());

    assert_eq!(collection1.length(), 1);
    assert_eq!(collection2.length(), 1);
    assert_eq!(collection3.length(), 1);
}

#[test]
fn test_by_class_name_empty_string() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    span.write().set_attribute("class", "test").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Empty string should return empty collection
    let collection = HTMLCollection::by_class_name(root.clone(), "".to_string());

    assert_eq!(collection.length(), 0);
}

#[test]
fn test_by_class_name_whitespace_only() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    span.write().set_attribute("class", "test").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Whitespace only should return empty collection
    let collection = HTMLCollection::by_class_name(root.clone(), "   ".to_string());

    assert_eq!(collection.length(), 0);
}

#[test]
fn test_by_class_name_no_matches() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    span.write().set_attribute("class", "foo bar").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    let collection = HTMLCollection::by_class_name(root.clone(), "nonexistent".to_string());

    assert_eq!(collection.length(), 0);
}

// =============================================================================
// getElementsByTagNameNS Tests
// =============================================================================

#[test]
fn test_by_tag_name_ns_specific_namespace() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create SVG element with namespace
    let svg_ns = "http://www.w3.org/2000/svg";
    let svg_rect = Element::new_with_namespace("rect", svg_ns);
    let svg_rect_ref: ElementRef = Arc::new(RwLock::new(svg_rect));

    // Create HTML element (no namespace or HTML namespace)
    let html_rect = doc.create_element("rect").unwrap();

    {
        let svg_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_rect_ref.read().clone())));
        let html_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(html_rect.read().clone())));
        root.write().append_child(svg_rect_node).unwrap();
        root.write().append_child(html_rect_node).unwrap();
    }

    // Search for SVG namespace
    let collection = HTMLCollection::by_tag_name_ns(
        root.clone(),
        Some(svg_ns.to_string()),
        "rect".to_string(),
    );

    // Should only find the SVG rect
    assert_eq!(collection.length(), 1);
}

#[test]
fn test_by_tag_name_ns_any_namespace() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create elements in different namespaces
    let svg_ns = "http://www.w3.org/2000/svg";
    let svg_rect = Element::new_with_namespace("rect", svg_ns);
    let svg_rect_ref: ElementRef = Arc::new(RwLock::new(svg_rect));

    let html_rect = doc.create_element("rect").unwrap();

    {
        let svg_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_rect_ref.read().clone())));
        let html_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(html_rect.read().clone())));
        root.write().append_child(svg_rect_node).unwrap();
        root.write().append_child(html_rect_node).unwrap();
    }

    // "*" namespace matches any namespace
    let collection = HTMLCollection::by_tag_name_ns(
        root.clone(),
        Some("*".to_string()),
        "rect".to_string(),
    );

    // Should find both rects
    assert_eq!(collection.length(), 2);
}

#[test]
fn test_by_tag_name_ns_any_local_name() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let svg_ns = "http://www.w3.org/2000/svg";
    let svg_rect = Element::new_with_namespace("rect", svg_ns);
    let svg_rect_ref: ElementRef = Arc::new(RwLock::new(svg_rect));

    let svg_circle = Element::new_with_namespace("circle", svg_ns);
    let svg_circle_ref: ElementRef = Arc::new(RwLock::new(svg_circle));

    {
        let svg_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_rect_ref.read().clone())));
        let svg_circle_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_circle_ref.read().clone())));
        root.write().append_child(svg_rect_node).unwrap();
        root.write().append_child(svg_circle_node).unwrap();
    }

    // "*" local name matches any element name
    let collection = HTMLCollection::by_tag_name_ns(
        root.clone(),
        Some(svg_ns.to_string()),
        "*".to_string(),
    );

    // Should find both SVG elements
    assert_eq!(collection.length(), 2);
}

#[test]
fn test_by_tag_name_ns_null_namespace() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Element with no namespace
    let span = doc.create_element("span").unwrap();

    // Element with namespace
    let svg_ns = "http://www.w3.org/2000/svg";
    let svg_span = Element::new_with_namespace("span", svg_ns);
    let svg_span_ref: ElementRef = Arc::new(RwLock::new(svg_span));

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        let svg_span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_span_ref.read().clone())));
        root.write().append_child(span_node).unwrap();
        root.write().append_child(svg_span_node).unwrap();
    }

    // None namespace matches only elements with no namespace
    let collection = HTMLCollection::by_tag_name_ns(root.clone(), None, "span".to_string());

    // Should only find the HTML span (no namespace)
    assert_eq!(collection.length(), 1);
}

#[test]
fn test_by_tag_name_ns_both_wildcards() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    let p = doc.create_element("p").unwrap();

    let svg_ns = "http://www.w3.org/2000/svg";
    let svg_rect = Element::new_with_namespace("rect", svg_ns);
    let svg_rect_ref: ElementRef = Arc::new(RwLock::new(svg_rect));

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        let p_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(p.read().clone())));
        let svg_rect_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(svg_rect_ref.read().clone())));
        root.write().append_child(span_node).unwrap();
        root.write().append_child(p_node).unwrap();
        root.write().append_child(svg_rect_node).unwrap();
    }

    // Both "*" should match everything
    let collection = HTMLCollection::by_tag_name_ns(
        root.clone(),
        Some("*".to_string()),
        "*".to_string(),
    );

    // Should find all three elements
    assert_eq!(collection.length(), 3);
}

// =============================================================================
// Live Collection Behavior Tests
// =============================================================================

#[test]
fn test_by_tag_name_is_live() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create collection BEFORE adding elements
    let collection = HTMLCollection::by_tag_name(root.clone(), "span".to_string());

    // Initially empty
    assert_eq!(collection.length(), 0);

    // Add element
    let span = doc.create_element("span").unwrap();
    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Collection should update automatically (LIVE)
    assert_eq!(collection.length(), 1);
}

#[test]
fn test_by_class_name_is_live() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create collection BEFORE adding elements
    let collection = HTMLCollection::by_class_name(root.clone(), "active".to_string());

    // Initially empty
    assert_eq!(collection.length(), 0);

    // Add element with matching class
    let span = doc.create_element("span").unwrap();
    span.write().set_attribute("class", "active").unwrap();
    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        root.write().append_child(span_node).unwrap();
    }

    // Collection should update automatically (LIVE)
    assert_eq!(collection.length(), 1);
}

// =============================================================================
// Original Tests (kept for backwards compatibility)
// =============================================================================

#[test]
fn test_html_collection_is_live() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create collection before adding elements
    let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
        el.read().tag_name() == "SPAN"
    });

    // Initially empty
    assert_eq!(collection.length(), 0);

    // Add span element
    let span1 = doc.create_element("span").unwrap();
    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        root.write().append_child(span1_node).unwrap();
    }

    // Collection updates automatically (LIVE behavior)
    assert_eq!(collection.length(), 1);

    // Add another span
    let span2 = doc.create_element("span").unwrap();
    {
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        root.write().append_child(span2_node).unwrap();
    }

    // Collection reflects the change
    assert_eq!(collection.length(), 2);
}

#[test]
fn test_html_collection_item() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span1 = doc.create_element("span").unwrap();
    let span2 = doc.create_element("span").unwrap();

    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
    }

    let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
        el.read().tag_name() == "SPAN"
    });

    assert_eq!(collection.length(), 2);

    let item0 = collection.item(0);
    assert!(item0.is_some());
    assert_eq!(item0.unwrap().read().tag_name(), "SPAN");

    let item1 = collection.item(1);
    assert!(item1.is_some());

    let item2 = collection.item(2);
    assert!(item2.is_none());
}

#[test]
fn test_html_collection_filters_correctly() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span = doc.create_element("span").unwrap();
    let p = doc.create_element("p").unwrap();
    let div = doc.create_element("div").unwrap();

    {
        let span_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span.read().clone())));
        let p_node: Arc<RwLock<Box<dyn Node>>> = Arc::new(RwLock::new(Box::new(p.read().clone())));
        let div_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(div.read().clone())));
        root.write().append_child(span_node).unwrap();
        root.write().append_child(p_node).unwrap();
        root.write().append_child(div_node).unwrap();
    }

    let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
        el.read().tag_name() == "SPAN"
    });

    // Only spans should be in the collection
    assert_eq!(collection.length(), 1);
}

#[test]
fn test_html_collection_named_item() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span1 = doc.create_element("span").unwrap();
    span1.write().set_attribute("id", "myspan").unwrap();

    let span2 = doc.create_element("span").unwrap();
    span2.write().set_attribute("name", "namedspan").unwrap();

    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
    }

    let collection = HTMLCollection::new(root.clone(), |_: &ElementRef| true);

    let by_id = collection.named_item("myspan");
    assert!(by_id.is_some());
    assert_eq!(by_id.unwrap().read().get_attribute("id"), Some("myspan"));

    let by_name = collection.named_item("namedspan");
    assert!(by_name.is_some());
    assert_eq!(
        by_name.unwrap().read().get_attribute("name"),
        Some("namedspan")
    );
}

#[test]
fn test_html_collection_nested_elements() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Create nested structure
    let child1 = doc.create_element("div").unwrap();
    let span1 = doc.create_element("span").unwrap();
    {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        child1.write().append_child(span1_node).unwrap();
    }

    let span2 = doc.create_element("span").unwrap();

    {
        let child1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(child1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        root.write().append_child(child1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
    }

    let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
        el.read().tag_name() == "SPAN"
    });

    // Should find both spans (one nested, one direct child)
    assert_eq!(collection.length(), 2);
}

#[test]
fn test_html_collection_updates_on_removal() {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    let span1 = doc.create_element("span").unwrap();
    let span2 = doc.create_element("span").unwrap();

    let child1_ref = {
        let span1_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span1.read().clone())));
        let span2_node: Arc<RwLock<Box<dyn Node>>> =
            Arc::new(RwLock::new(Box::new(span2.read().clone())));
        let c1 = root.write().append_child(span1_node).unwrap();
        root.write().append_child(span2_node).unwrap();
        c1
    };

    let collection = HTMLCollection::new(root.clone(), |el: &ElementRef| {
        el.read().tag_name() == "SPAN"
    });

    assert_eq!(collection.length(), 2);

    // Remove one span
    root.write().remove_child(child1_ref).unwrap();

    // Collection should reflect the removal (LIVE)
    assert_eq!(collection.length(), 1);
}
