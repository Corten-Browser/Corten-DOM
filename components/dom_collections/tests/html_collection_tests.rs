use dom_collections::HTMLCollection;
use dom_core::{Document, ElementRef, Node};
use parking_lot::RwLock;
use std::sync::Arc;

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
