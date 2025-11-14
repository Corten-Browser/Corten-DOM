# Component: dom_collections

## Component Identification
- **Name**: dom_collections
- **Type**: Feature (Level 2)
- **Version**: 0.1.0
- **Dependencies**: dom-types, dom-core

## Responsibility
Implement live collections (HTMLCollection, NodeList, DOMTokenList) and tree traversal (NodeIterator, TreeWalker).

## Key Implementation Sections from Spec
- Collections Interface (lines 338-385)
- Live Collection Implementation (lines 635-681)
- Traversal API (lines 57-62)

## Core Components to Implement

### 1. HTMLCollection (Live)
```rust
pub struct HTMLCollection {
    root: WeakNodeRef,
    filter: Box<dyn Fn(&Element) -> bool + Send + Sync>,
    cached_items: RefCell<Vec<ElementRef>>,
    version: u64,
}

impl HTMLCollection {
    pub fn length(&self) -> usize;
    pub fn item(&self, index: usize) -> Option<ElementRef>;
    pub fn named_item(&self, name: &str) -> Option<ElementRef>;
}
```

**Key Feature**: Collection is LIVE - updates when DOM changes.

### 2. NodeList (Live and Static)
```rust
pub enum NodeList {
    Live {
        root: WeakNodeRef,
        filter: Box<dyn Fn(&dyn Node) -> bool + Send + Sync>,
    },
    Static {
        nodes: Vec<NodeRef>,
    },
}
```

### 3. DOMTokenList (for classes)
```rust
pub struct DOMTokenList {
    element: WeakElementRef,
    attribute_name: String,
}

impl DOMTokenList {
    pub fn add(&mut self, tokens: &[&str]) -> Result<(), DomException>;
    pub fn remove(&mut self, tokens: &[&str]) -> Result<(), DomException>;
    pub fn toggle(&mut self, token: &str) -> Result<bool, DomException>;
    pub fn contains(&self, token: &str) -> bool;
}
```

### 4. NodeIterator & TreeWalker
```rust
pub struct NodeIterator {
    root: NodeRef,
    reference_node: NodeRef,
    what_to_show: u32,
    filter: Option<Box<dyn NodeFilter>>,
}

impl NodeIterator {
    pub fn next_node(&mut self) -> Option<NodeRef>;
    pub fn previous_node(&mut self) -> Option<NodeRef>;
}
```

## TDD Requirements

**Example Test (RED first)**:
```rust
#[test]
fn test_html_collection_is_live() {
    let doc = Document::new();
    let root = doc.create_element("div").unwrap();
    let collection = root.get_elements_by_tag_name("span");

    // Initially empty
    assert_eq!(collection.length(), 0);

    // Add element
    let span = doc.create_element("span").unwrap();
    root.append_child(span);

    // Collection updates automatically (LIVE)
    assert_eq!(collection.length(), 1);
}
```

## Quality Gates
- ✅ All tests pass (100%)
- ✅ Coverage ≥ 80%
- ✅ Live collections update correctly
- ✅ Performance: Collection access <1μs

## Success Criteria
1. HTMLCollection and NodeList work correctly
2. Live collections update when DOM changes
3. TreeWalker/NodeIterator traverse correctly
4. All tests pass, coverage ≥ 80%

## Estimated Effort
- **LOC**: ~3,500-4,500
- **Time**: 10-12 hours
