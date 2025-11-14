use dom_collections::DOMTokenList;
use dom_core::Document;

#[test]
fn test_dom_token_list_length() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    element
        .write()
        .set_attribute("class", "foo bar baz")
        .unwrap();

    let token_list = DOMTokenList::new(element.clone(), "class");

    assert_eq!(token_list.length(), 3);
}

#[test]
fn test_dom_token_list_item() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    element
        .write()
        .set_attribute("class", "foo bar baz")
        .unwrap();

    let token_list = DOMTokenList::new(element.clone(), "class");

    assert_eq!(token_list.item(0), Some("foo".to_string()));
    assert_eq!(token_list.item(1), Some("bar".to_string()));
    assert_eq!(token_list.item(2), Some("baz".to_string()));
    assert_eq!(token_list.item(3), None);
}

#[test]
fn test_dom_token_list_contains() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    element.write().set_attribute("class", "foo bar").unwrap();

    let token_list = DOMTokenList::new(element.clone(), "class");

    assert!(token_list.contains("foo"));
    assert!(token_list.contains("bar"));
    assert!(!token_list.contains("baz"));
}

#[test]
fn test_dom_token_list_empty() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();

    let token_list = DOMTokenList::new(element.clone(), "class");

    assert_eq!(token_list.length(), 0);
    assert!(!token_list.contains("foo"));
}

#[test]
fn test_dom_token_list_add() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    element.write().set_attribute("class", "foo").unwrap();

    let mut token_list = DOMTokenList::new(element.clone(), "class");

    let result = token_list.add(&["bar", "baz"]);
    assert!(result.is_ok());
}

#[test]
fn test_dom_token_list_remove() {
    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    element
        .write()
        .set_attribute("class", "foo bar baz")
        .unwrap();

    let mut token_list = DOMTokenList::new(element.clone(), "class");

    let result = token_list.remove(&["bar"]);
    assert!(result.is_ok());
}
