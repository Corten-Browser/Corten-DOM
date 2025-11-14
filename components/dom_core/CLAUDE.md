# Component: dom_core

## Component Identification
- **Name**: dom_core
- **Type**: Core (Level 1)
- **Version**: 0.1.0
- **Language**: Rust
- **Project Root**: /home/user/Corten-DOM
- **Component Root**: /home/user/Corten-DOM/components/dom_core

## Responsibility
Implement core DOM nodes (Node, Element, Document, Text, Comment) and tree operations (append, remove, insert, clone).

## Dependencies
- **Internal**: dom-types, dom-storage
- **External**: `dashmap`, `indexmap`, `smallvec`, `string_cache`, `html5ever`, `parking_lot`

## Development Workflow

### 1. Read Specification
Focus on these sections:
- Core Node Interface (lines 114-164)
- Element Interface (lines 186-238)
- Document Interface (lines 241-292)

### 2. Implement Node Trait (TDD)

**Phase 1: Node Trait and NodeData**

Test first (RED):
```rust
#[test]
fn test_node_type() {
    let node = create_element_node();
    assert_eq!(node.node_type(), NodeType::Element);
}

#[test]
fn test_append_child() {
    let mut parent = create_element_node();
    let child = create_text_node("Hello");
    let result = parent.append_child(child.clone());
    assert!(result.is_ok());
    assert_eq!(parent.child_nodes().len(), 1);
}
```

Then implement (GREEN):
```rust
pub trait Node: Send + Sync {
    fn node_type(&self) -> NodeType;
    fn node_name(&self) -> &str;
    fn node_value(&self) -> Option<&str>;
    fn set_node_value(&mut self, value: Option<String>);
    fn text_content(&self) -> Option<String>;
    fn parent_node(&self) -> Option<NodeRef>;
    fn child_nodes(&self) -> Vec<NodeRef>;
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    // ... full API from spec lines 122-164
}

pub type NodeRef = Arc<RefCell<Box<dyn Node>>>;
```

**Phase 2: Element Implementation**

```rust
pub struct Element {
    node_data: NodeData,
    tag_name: String,
    namespace: Option<String>,
    attributes: HashMap<String, String>,
    class_list: Vec<String>,
}

impl Element {
    pub fn new(tag_name: &str) -> Self;
    pub fn get_attribute(&self, name: &str) -> Option<&str>;
    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), DomException>;
    pub fn has_attribute(&self, name: &str) -> bool;
    // ... full API from spec lines 203-238
}

impl Node for Element {
    // Implement all Node trait methods
}
```

**Phase 3: Document Implementation**

```rust
pub struct Document {
    node_data: NodeData,
    document_element: Option<ElementRef>,
    node_registry: HashMap<NodeId, WeakNodeRef>,
    id_map: HashMap<String, Vec<ElementRef>>,
}

impl Document {
    pub fn new() -> Self;
    pub fn create_element(&mut self, tag_name: &str) -> Result<ElementRef, DomException>;
    pub fn create_text_node(&mut self, data: &str) -> TextRef;
    pub fn get_element_by_id(&self, id: &str) -> Option<ElementRef>;
    // ... full API from spec lines 261-292
}

impl Node for Document {
    // Implement Node trait
}
```

### 3. File Structure
```
src/
├── lib.rs
├── node/
│   ├── mod.rs              # Node trait
│   ├── node_data.rs        # Common node data
│   └── node_ref.rs         # NodeRef type
├── element.rs              # Element implementation
├── document.rs             # Document implementation
├── text.rs                 # Text node
├── comment.rs              # Comment node
├── document_fragment.rs    # DocumentFragment
└── validation.rs           # Name validation
```

### 4. Critical Implementation Details

#### Tree Manipulation
```rust
impl Node for Element {
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException> {
        // 1. Validate hierarchy (no circular references)
        if self.contains(&*child.borrow()) {
            return Err(DomException::HierarchyRequestError);
        }

        // 2. Remove from old parent if exists
        if let Some(old_parent) = child.borrow().parent_node() {
            old_parent.borrow_mut().remove_child(child.clone())?;
        }

        // 3. Add to children
        self.node_data.children.push(child.clone());

        // 4. Set parent
        child.borrow_mut().set_parent(Arc::downgrade(&self_ref));

        Ok(child)
    }
}
```

### 5. Testing Requirements

**Coverage Target**: ≥80%

Test categories:
1. **Node operations** (`tests/unit/test_node.rs`)
   - Creation, cloning, normalization
   - Parent-child relationships
2. **Tree manipulation** (`tests/unit/test_tree.rs`)
   - append_child, remove_child, insert_before
   - Edge cases (circular refs, wrong document)
3. **Element attributes** (`tests/unit/test_attributes.rs`)
   - get/set/remove attribute
   - Namespace handling
4. **Document operations** (`tests/unit/test_document.rs`)
   - Element creation
   - ID-based lookup
5. **Integration tests** (`tests/integration/`)
   - Complete tree operations
   - Cross-node-type operations

### 6. Quality Gates
- ✅ All tests pass (100%)
- ✅ Coverage ≥ 80%
- ✅ No circular reference bugs
- ✅ Memory management correct (no leaks)
- ✅ `cargo clippy` passes
- ✅ Proper error handling

### 7. Performance Requirements
- Element creation: <1μs
- append_child: <1μs
- Deep tree (1000 nodes): <50ms

## Contract API

### Exports to Other Components
```rust
// Core trait
pub trait Node: Send + Sync {
    fn node_type(&self) -> NodeType;
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn child_nodes(&self) -> Vec<NodeRef>;
    // ... full interface
}

// Node implementations
pub struct Element { ... }
pub struct Document { ... }
pub struct Text { ... }
pub struct Comment { ... }

// Reference types
pub type NodeRef = Arc<RefCell<Box<dyn Node>>>;
pub type ElementRef = Arc<RefCell<Element>>;
pub type DocumentRef = Arc<RefCell<Document>>;
```

### Required by Consumers
- dom_collections needs: Node trait, child_nodes(), parent_node()
- dom_events needs: Node trait for EventTarget implementation
- dom_selectors needs: Element, attributes, tree traversal

## Success Criteria
1. Complete Node trait implementation
2. Element with attributes working
3. Document with element creation
4. Tree operations correct (no circular refs, no memory leaks)
5. All tests pass (100%)
6. Coverage ≥ 80%
7. Performance requirements met

## Estimated Effort
- **LOC**: ~8,000-10,000
- **Time**: 20-24 hours

## Critical Notes
- This is the FOUNDATION component - everything depends on this
- Thread safety is critical (use Arc<RefCell<>>)
- Prevent circular references in tree operations
- Validate hierarchy on every tree modification
