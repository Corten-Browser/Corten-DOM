# DOM Core Component

Core DOM implementation for CortenBrowser following DOM Level 4 specification.

## Quick Start

```rust
use dom_core::{Document, Element, NodeRef};
use dom_types::NodeType;

// Create a document
let mut doc = Document::new();

// Create an element
let div = doc.create_element("div")?;

// Set attributes
div.write().set_attribute("id", "main")?;
div.write().set_attribute("class", "container")?;

// Create text content
let text = doc.create_text_node("Hello, World!");

// Append text to element
div.write().append_child_element_to_text(text)?;

// Set document element
doc.set_document_element(div.clone());

// Query elements
let found = doc.get_element_by_id("main");
assert!(found.is_some());
```

## Node Types

The following DOM node types are implemented:

| Type | Description | Example |
|------|-------------|---------|
| `Document` | Root document node | `Document::new()` |
| `Element` | HTML/XML elements | `Element::new("div")` |
| `Text` | Text content nodes | `Text::new("content")` |
| `Comment` | Comment nodes | `Comment::new("comment text")` |
| `DocumentFragment` | Lightweight document fragments | `DocumentFragment::new()` |
| `DocumentType` | DOCTYPE declarations | `DocumentType::new("html", "", "")` |
| `CDATASection` | CDATA sections (XML) | `CDATASection::new("data")` |
| `ProcessingInstruction` | Processing instructions | `ProcessingInstruction::new("xml", "version")` |
| `Attr` | Element attributes | `Attr::new("class", "value")` |

## Key APIs

### Node Trait

All DOM nodes implement the `Node` trait:

```rust
pub trait Node: Send + Sync + Debug {
    // Type information
    fn node_type(&self) -> NodeType;
    fn node_name(&self) -> &str;
    fn node_value(&self) -> Option<&str>;
    fn set_node_value(&mut self, value: Option<String>);

    // Text content
    fn text_content(&self) -> Option<String>;
    fn set_text_content(&mut self, text: String);

    // Tree navigation
    fn parent_node(&self) -> Option<NodeRef>;
    fn parent_element(&self) -> Option<NodeRef>;
    fn child_nodes(&self) -> Vec<NodeRef>;
    fn first_child(&self) -> Option<NodeRef>;
    fn last_child(&self) -> Option<NodeRef>;
    fn previous_sibling(&self) -> Option<NodeRef>;
    fn next_sibling(&self) -> Option<NodeRef>;

    // Tree manipulation
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn replace_child(&mut self, new: NodeRef, old: NodeRef) -> Result<NodeRef, DomException>;
    fn insert_before(&mut self, new: NodeRef, ref_node: Option<NodeRef>) -> Result<NodeRef, DomException>;

    // Cloning
    fn clone_node(&self, deep: bool) -> NodeRef;

    // Comparison
    fn is_same_node(&self, other: Option<&NodeRef>) -> bool;
    fn compare_document_position(&self, other: &NodeRef) -> DocumentPosition;
}
```

### Element Methods

```rust
impl Element {
    // Creation
    pub fn new(tag_name: impl Into<String>) -> Self;
    pub fn new_with_namespace(tag_name: impl Into<String>, namespace: impl Into<String>) -> Self;

    // Properties
    pub fn tag_name(&self) -> &str;
    pub fn namespace_uri(&self) -> Option<&str>;
    pub fn id(&self) -> Option<&str>;

    // Attribute operations
    pub fn get_attribute(&self, name: &str) -> Option<&str>;
    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), DomException>;
    pub fn remove_attribute(&mut self, name: &str);
    pub fn has_attribute(&self, name: &str) -> bool;
    pub fn toggle_attribute(&mut self, name: &str) -> Result<bool, DomException>;

    // Namespaced attributes
    pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&str>;
    pub fn set_attribute_ns(&mut self, ns: Option<&str>, name: &str, value: &str) -> Result<(), DomException>;
    pub fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool;
    pub fn remove_attribute_ns(&mut self, namespace: Option<&str>, local_name: &str);

    // CSS classes
    pub fn class_list(&self) -> &[String];
    pub fn class_name(&self) -> String;
    pub fn set_class_name(&mut self, class_name: &str);

    // Content
    pub fn inner_html(&self) -> String;
    pub fn outer_html(&self) -> String;
}
```

### Document Methods

```rust
impl Document {
    // Creation
    pub fn new() -> Self;

    // Properties
    pub fn document_element(&self) -> Option<ElementRef>;
    pub fn set_document_element(&mut self, element: ElementRef);
    pub fn url(&self) -> &str;
    pub fn charset(&self) -> &str;

    // Node creation
    pub fn create_element(&mut self, tag_name: impl Into<String>) -> Result<ElementRef, DomException>;
    pub fn create_element_ns(&mut self, ns: impl Into<String>, name: impl Into<String>) -> Result<ElementRef, DomException>;
    pub fn create_text_node(&self, data: impl Into<String>) -> Arc<RwLock<Text>>;
    pub fn create_comment(&self, data: impl Into<String>) -> Arc<RwLock<Comment>>;
    pub fn create_document_fragment(&self) -> Arc<RwLock<DocumentFragment>>;
    pub fn create_attribute(&self, name: impl Into<String>) -> Result<AttrRef, DomException>;
    pub fn create_processing_instruction(&self, target: &str, data: &str) -> Result<ProcessingInstructionRef, DomException>;
    pub fn create_cdata_section(&self, data: &str) -> Result<CDATASectionRef, DomException>;

    // Queries
    pub fn get_element_by_id(&self, id: &str) -> Option<ElementRef>;

    // Node adoption/import
    pub fn adopt_node(&mut self, node: NodeRef) -> Result<NodeRef, DomException>;
    pub fn import_node(&self, node: &NodeRef, deep: bool) -> Result<NodeRef, DomException>;

    // Events
    pub fn create_event(&self, interface: &str) -> Result<EventRef, DomException>;

    // Range
    pub fn create_range(&self) -> Arc<RwLock<Range>>;
}
```

## Thread Safety

All node references use `Arc<RwLock<>>` for thread-safe access:

```rust
use dom_core::{Document, NodeRef, ElementRef};

let mut doc = Document::new();
let element: ElementRef = doc.create_element("div")?;

// Read access (shared)
{
    let elem = element.read();
    let name = elem.node_name();
    println!("Tag: {}", name);
}

// Write access (exclusive)
{
    let mut elem = element.write();
    elem.set_attribute("id", "my-id")?;
}
```

### Reference Types

| Type | Description |
|------|-------------|
| `NodeRef` | `Arc<RwLock<Box<dyn Node>>>` - Generic node reference |
| `ElementRef` | `Arc<RwLock<Element>>` - Element reference |
| `DocumentRef` | `Arc<RwLock<Document>>` - Document reference |
| `WeakNodeRef` | `Weak<RwLock<Box<dyn Node>>>` - Non-owning reference |

## Tree Manipulation Examples

### Building a DOM Tree

```rust
use dom_core::{Document, Element};
use dom_types::NodeType;

let mut doc = Document::new();

// Create structure: <div><ul><li>Item 1</li><li>Item 2</li></ul></div>
let div = doc.create_element("div")?;
let ul = doc.create_element("ul")?;
let li1 = doc.create_element("li")?;
let li2 = doc.create_element("li")?;

let text1 = doc.create_text_node("Item 1");
let text2 = doc.create_text_node("Item 2");

// Build tree
li1.write().append_child_element_to_text(text1)?;
li2.write().append_child_element_to_text(text2)?;
ul.write().append_child_element(li1)?;
ul.write().append_child_element(li2)?;
div.write().append_child_element(ul)?;

doc.set_document_element(div);
```

### Cloning Nodes

```rust
// Shallow clone (no children)
let shallow = element.read().clone_node(false);

// Deep clone (includes all descendants)
let deep = element.read().clone_node(true);
```

### Inserting and Removing Nodes

```rust
// Insert before a reference node
parent.write().insert_before(new_child, Some(reference_child))?;

// Remove a child
let removed = parent.write().remove_child(child)?;

// Replace a child
let replaced = parent.write().replace_child(new_child, old_child)?;
```

## Document Position

Compare node positions in the document:

```rust
use dom_core::node::DocumentPosition;

let position = node1.read().compare_document_position(&node2);

if position.contains(DocumentPosition::PRECEDING) {
    println!("node2 precedes node1");
}

if position.contains(DocumentPosition::CONTAINS) {
    println!("node1 contains node2");
}
```

### DocumentPosition Flags

| Flag | Description |
|------|-------------|
| `DISCONNECTED` | Nodes are in different trees |
| `PRECEDING` | Other node precedes this node |
| `FOLLOWING` | Other node follows this node |
| `CONTAINS` | Other node is contained by this node |
| `CONTAINED_BY` | Other node contains this node |

## Namespaces

Work with XML namespaces:

```rust
use dom_core::namespaces::{HTML_NAMESPACE, SVG_NAMESPACE, MATHML_NAMESPACE};

// Create SVG element
let svg = doc.create_element_ns(SVG_NAMESPACE, "svg")?;

// Set namespaced attribute
svg.write().set_attribute_ns(
    Some("http://www.w3.org/1999/xlink"),
    "xlink:href",
    "#myId"
)?;
```

### Predefined Namespace Constants

- `HTML_NAMESPACE`: `http://www.w3.org/1999/xhtml`
- `SVG_NAMESPACE`: `http://www.w3.org/2000/svg`
- `MATHML_NAMESPACE`: `http://www.w3.org/1998/Math/MathML`
- `XLINK_NAMESPACE`: `http://www.w3.org/1999/xlink`
- `XML_NAMESPACE`: `http://www.w3.org/XML/1998/namespace`
- `XMLNS_NAMESPACE`: `http://www.w3.org/2000/xmlns/`

## Error Handling

All operations that can fail return `Result<T, DomException>`:

```rust
use dom_types::DomException;

match doc.create_element("123invalid") {
    Ok(elem) => println!("Created element"),
    Err(DomException::InvalidCharacterError) => {
        eprintln!("Invalid element name");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Common Exceptions

| Exception | Description |
|-----------|-------------|
| `HierarchyRequestError` | Invalid parent-child relationship |
| `WrongDocumentError` | Node belongs to different document |
| `InvalidCharacterError` | Invalid characters in name |
| `NotFoundError` | Referenced node not found |
| `NotSupportedError` | Operation not supported |

## Performance Characteristics

| Operation | Complexity | Typical Time |
|-----------|------------|--------------|
| Element creation | O(1) | < 1us |
| `append_child` | O(1) | < 1us |
| `remove_child` | O(n) | < 1us |
| `get_element_by_id` | O(1) | < 100ns |
| Deep clone (1000 nodes) | O(n) | < 50ms |

## Module Structure

```
dom_core/
  src/
    lib.rs                  # Public exports
    node/
      mod.rs               # Node trait and NodeData
    element.rs             # Element implementation
    document.rs            # Document implementation
    text.rs                # Text node
    comment.rs             # Comment node
    document_fragment.rs   # DocumentFragment
    document_type.rs       # DocumentType
    cdata_section.rs       # CDATASection
    processing_instruction.rs  # ProcessingInstruction
    attr.rs                # Attr node
    range.rs               # Range API
    event.rs               # Basic event types
    namespaces.rs          # Namespace constants
    tree_order.rs          # Document position
    dom_implementation.rs  # DOMImplementation
```

## Related Components

- **dom_types**: Type definitions (`NodeType`, `DomException`)
- **dom_storage**: Memory management and garbage collection
- **dom_collections**: `NodeList`, `HTMLCollection`, `TreeWalker`
- **dom_events**: Event dispatch and handling
- **dom_selectors**: CSS selector queries
- **dom_advanced**: Shadow DOM, MutationObserver, Selection

## See Also

- [DOM Living Standard](https://dom.spec.whatwg.org/)
- [MDN Web Docs - Node](https://developer.mozilla.org/en-US/docs/Web/API/Node)
- [MDN Web Docs - Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
- [MDN Web Docs - Document](https://developer.mozilla.org/en-US/docs/Web/API/Document)

---

For development instructions, see `CLAUDE.md`.
