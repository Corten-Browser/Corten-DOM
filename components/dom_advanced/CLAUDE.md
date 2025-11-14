# Component: dom_advanced

## Component Identification
- **Name**: dom_advanced
- **Type**: Feature (Level 2)
- **Version**: 0.1.0
- **Dependencies**: dom-types, dom-core, dom-events

## Responsibility
Implement advanced DOM features: MutationObserver, Range, Selection, and Shadow DOM.

## Key Spec Sections
- MutationObserver (lines 388-413, 685-715)
- Range API (lines 63-66)
- Shadow DOM (lines 78-82, Phase 6 details lines 1160-1166)

## Core Components

### 1. MutationObserver
```rust
pub struct MutationObserver {
    callback: Box<dyn Fn(&[MutationRecord], &MutationObserver) + Send + Sync>,
    observed_nodes: Vec<(WeakNodeRef, MutationObserverInit)>,
    record_queue: Vec<MutationRecord>,
}

pub struct MutationRecord {
    pub record_type: MutationType,
    pub target: NodeRef,
    pub added_nodes: NodeList,
    pub removed_nodes: NodeList,
    pub previous_sibling: Option<NodeRef>,
    pub next_sibling: Option<NodeRef>,
    pub attribute_name: Option<String>,
    pub old_value: Option<String>,
}

impl MutationObserver {
    pub fn new(callback: Box<dyn Fn(&[MutationRecord], &MutationObserver) + Send + Sync>) -> Self;
    pub fn observe(&mut self, target: NodeRef, options: MutationObserverInit) -> Result<(), DomException>;
    pub fn disconnect(&mut self);
    pub fn take_records(&mut self) -> Vec<MutationRecord>;
}
```

**Key Feature**: Observer is notified asynchronously via microtask queue.

### 2. Range API
```rust
pub struct Range {
    start_container: NodeRef,
    start_offset: usize,
    end_container: NodeRef,
    end_offset: usize,
    collapsed: bool,
}

impl Range {
    pub fn new(document: &Document) -> Self;
    pub fn set_start(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException>;
    pub fn set_end(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException>;
    pub fn collapse(&mut self, to_start: bool);
    pub fn extract_contents(&mut self) -> Result<DocumentFragmentRef, DomException>;
    pub fn delete_contents(&mut self) -> Result<(), DomException>;
}
```

### 3. Selection API
```rust
pub struct Selection {
    range: Option<Range>,
    direction: SelectionDirection,
}

impl Selection {
    pub fn add_range(&mut self, range: Range);
    pub fn remove_all_ranges(&mut self);
    pub fn get_range_at(&self, index: usize) -> Option<&Range>;
    pub fn collapse(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException>;
}
```

### 4. Shadow DOM
```rust
pub struct ShadowRoot {
    host: WeakElementRef,
    mode: ShadowRootMode,
    delegates_focus: bool,
    slot_assignment: SlotAssignmentMode,
}

pub enum ShadowRootMode {
    Open,
    Closed,
}

impl ShadowRoot {
    pub fn new(host: ElementRef, mode: ShadowRootMode) -> Self;
    pub fn host(&self) -> ElementRef;
    pub fn mode(&self) -> ShadowRootMode;
}

impl Element {
    pub fn attach_shadow(&mut self, mode: ShadowRootMode) -> Result<ShadowRootRef, DomException>;
    pub fn shadow_root(&self) -> Option<ShadowRootRef>;
}
```

## TDD Examples

### MutationObserver Test
```rust
#[test]
fn test_mutation_observer_detects_children() {
    let doc = Document::new();
    let div = doc.create_element("div").unwrap();

    let observed = Arc::new(Mutex::new(false));
    let observed_clone = observed.clone();

    let observer = MutationObserver::new(Box::new(move |records, _| {
        if records.iter().any(|r| r.record_type == MutationType::ChildList) {
            *observed_clone.lock().unwrap() = true;
        }
    }));

    observer.observe(div.clone(), MutationObserverInit {
        child_list: true,
        ..Default::default()
    }).unwrap();

    // Trigger mutation
    let span = doc.create_element("span").unwrap();
    div.append_child(span).unwrap();

    // Deliver mutations (microtask)
    deliver_mutations();

    assert!(*observed.lock().unwrap());
}
```

### Range Test
```rust
#[test]
fn test_range_extract_contents() {
    let doc = Document::new();
    let div = doc.create_element("div").unwrap();
    let text = doc.create_text_node("Hello World");
    div.append_child(text.clone());

    let mut range = Range::new(&doc);
    range.set_start(text.clone(), 0).unwrap();
    range.set_end(text.clone(), 5).unwrap();

    let fragment = range.extract_contents().unwrap();
    assert_eq!(fragment.text_content(), Some("Hello".to_string()));
    assert_eq!(text.text_content(), Some(" World".to_string()));
}
```

## Quality Gates
- ✅ MutationObserver fires correctly
- ✅ Range operations work (extract, delete)
- ✅ Shadow DOM encapsulation correct
- ✅ Coverage ≥ 80%

## Success Criteria
1. MutationObserver detects mutations
2. Range API manipulates content correctly
3. Shadow DOM attachment works
4. All tests pass

## Estimated Effort
- **LOC**: ~6,000-8,000
- **Time**: 16-20 hours
