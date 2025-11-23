# DOM Implementation Component Specification
## CortenBrowser DOM Component v1.0

### Executive Summary
The DOM Implementation (dom-impl) component provides the Document Object Model for the CortenBrowser, implementing the full DOM Level 4 specification with extensions for modern web standards. This component serves as the central data structure and API for representing and manipulating HTML/XML documents, handling events, and providing the bridge between the HTML parser, CSS engine, JavaScript runtime, and rendering pipeline.

**Component ID**: `dom-impl`  
**Version**: 1.0.0  
**Architecture**: Multi-threaded with thread-safe DOM operations  
**Language**: Rust  
**Estimated LOC**: 100,000-125,000  
**Development Time**: 8-10 weeks  

## Component Architecture

### High-Level Architecture
```
┌─────────────────────────────────────────────────┐
│                DOM Implementation                │
├─────────────────────────────────────────────────┤
│  ┌────────────┐  ┌────────────┐  ┌────────────┐│
│  │    Core    │  │   Events   │  │  Traversal ││
│  │    Nodes   │  │   System   │  │  & Ranges  ││
│  └────────────┘  └────────────┘  └────────────┘│
│  ┌────────────┐  ┌────────────┐  ┌────────────┐│
│  │ Mutations  │  │ Collections│  │  Shadow    ││
│  │  Observer  │  │  & Lists   │  │    DOM     ││
│  └────────────┘  └────────────┘  └────────────┘│
│  ┌────────────────────────────────────────────┐│
│  │          Thread-Safe Node Storage           ││
│  └────────────────────────────────────────────┘│
└─────────────────────────────────────────────────┘
```

### Module Structure
```
dom-impl/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Component public interface
│   ├── node/
│   │   ├── mod.rs               # Node trait and base implementation
│   │   ├── element.rs           # Element node implementation
│   │   ├── text.rs              # Text node implementation
│   │   ├── document.rs          # Document node implementation
│   │   ├── document_fragment.rs # DocumentFragment implementation
│   │   ├── comment.rs           # Comment node implementation
│   │   ├── processing_instruction.rs
│   │   └── attr.rs              # Attribute nodes
│   ├── events/
│   │   ├── mod.rs               # Event system core
│   │   ├── event.rs             # Event interface
│   │   ├── event_target.rs      # EventTarget trait
│   │   ├── event_dispatcher.rs  # Event dispatch algorithm
│   │   ├── event_types/         # Specific event types
│   │   └── listeners.rs         # Event listener management
│   ├── traversal/
│   │   ├── mod.rs
│   │   ├── node_iterator.rs    # NodeIterator implementation
│   │   ├── tree_walker.rs      # TreeWalker implementation
│   │   └── node_filter.rs      # NodeFilter interface
│   ├── ranges/
│   │   ├── mod.rs
│   │   ├── range.rs             # Range API implementation
│   │   ├── selection.rs        # Selection API
│   │   └── boundary_points.rs  # Range boundary logic
│   ├── collections/
│   │   ├── mod.rs
│   │   ├── html_collection.rs  # Live HTMLCollection
│   │   ├── node_list.rs        # NodeList (live and static)
│   │   ├── named_node_map.rs   # NamedNodeMap for attributes
│   │   └── dom_token_list.rs   # DOMTokenList for classes
│   ├── mutation/
│   │   ├── mod.rs
│   │   ├── observer.rs         # MutationObserver implementation
│   │   ├── record.rs           # MutationRecord
│   │   └── queue.rs            # Mutation queue management
│   ├── shadow/
│   │   ├── mod.rs
│   │   ├── shadow_root.rs      # ShadowRoot implementation
│   │   ├── slot.rs             # Slot element handling
│   │   └── distribution.rs     # Slot distribution algorithm
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── arena.rs           # Thread-safe node arena
│   │   ├── weak_refs.rs       # Weak reference management
│   │   └── gc.rs              # Garbage collection for nodes
│   ├── validation/
│   │   ├── mod.rs
│   │   ├── namespace.rs       # Namespace validation
│   │   ├── qualified_name.rs  # Qualified name validation
│   │   └── html5.rs           # HTML5-specific validation
│   ├── interfaces/
│   │   ├── mod.rs             # Public API definitions
│   │   ├── dom_core.rs        # Core DOM interfaces
│   │   ├── dom_html.rs        # HTML-specific interfaces
│   │   └── dom_svg.rs         # SVG-specific interfaces
│   └── utils/
│       ├── mod.rs
│       ├── strings.rs         # Atom string interning
│       ├── namespace.rs       # Namespace utilities
│       └── ordering.rs        # Tree order comparisons
├── tests/
│   ├── unit/                  # Unit tests for each module
│   ├── integration/           # Integration tests
│   ├── wpt/                   # Web Platform Test harness
│   └── benchmarks/            # Performance benchmarks
└── benches/
    └── dom_operations.rs      # Benchmark suite
```

## Public API Specification

### Core Node Interface
```rust
use std::sync::Arc;
use std::cell::RefCell;
use browser_messages::{ComponentMessage, ComponentResponse};
use browser_types::{NodeId, DocumentId};

/// Core node trait that all DOM nodes must implement
pub trait Node: Send + Sync {
    /// Node type enumeration
    fn node_type(&self) -> NodeType;
    
    /// Node name (uppercase for elements)
    fn node_name(&self) -> &str;
    
    /// Node value (text content for text nodes)
    fn node_value(&self) -> Option<&str>;
    fn set_node_value(&mut self, value: Option<String>);
    
    /// Text content of node and descendants
    fn text_content(&self) -> Option<String>;
    fn set_text_content(&mut self, text: String);
    
    /// Parent-child relationships
    fn parent_node(&self) -> Option<NodeRef>;
    fn parent_element(&self) -> Option<ElementRef>;
    fn child_nodes(&self) -> NodeList;
    fn first_child(&self) -> Option<NodeRef>;
    fn last_child(&self) -> Option<NodeRef>;
    fn previous_sibling(&self) -> Option<NodeRef>;
    fn next_sibling(&self) -> Option<NodeRef>;
    
    /// Tree modification
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;
    fn replace_child(&mut self, new_child: NodeRef, old_child: NodeRef) -> Result<NodeRef, DomException>;
    fn insert_before(&mut self, new_child: NodeRef, ref_child: Option<NodeRef>) -> Result<NodeRef, DomException>;
    
    /// Node operations
    fn clone_node(&self, deep: bool) -> NodeRef;
    fn normalize(&mut self);
    fn is_equal_node(&self, other: &dyn Node) -> bool;
    fn is_same_node(&self, other: &dyn Node) -> bool;
    fn contains(&self, other: &dyn Node) -> bool;
    
    /// Document association
    fn owner_document(&self) -> Option<DocumentRef>;
    
    /// Comparison
    fn compare_document_position(&self, other: &dyn Node) -> DocumentPosition;
}

/// Node type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    CDataSection = 4,
    ProcessingInstruction = 7,
    Comment = 8,
    Document = 9,
    DocumentType = 10,
    DocumentFragment = 11,
}

/// Thread-safe reference to a DOM node
pub type NodeRef = Arc<RefCell<Box<dyn Node>>>;
pub type ElementRef = Arc<RefCell<Element>>;
pub type DocumentRef = Arc<RefCell<Document>>;
```

### Element Interface
```rust
/// Element node implementation
pub struct Element {
    // Base node data
    node_data: NodeData,
    
    // Element-specific data
    tag_name: String,
    namespace: Option<String>,
    prefix: Option<String>,
    attributes: NamedNodeMap,
    class_list: DOMTokenList,
    dataset: DOMStringMap,
    shadow_root: Option<ShadowRootRef>,
}

impl Element {
    /// Element creation
    pub fn new(tag_name: &str, namespace: Option<&str>) -> Self;
    
    /// Attribute operations
    pub fn get_attribute(&self, name: &str) -> Option<String>;
    pub fn set_attribute(&mut self, name: &str, value: &str) -> Result<(), DomException>;
    pub fn remove_attribute(&mut self, name: &str) -> Result<(), DomException>;
    pub fn has_attribute(&self, name: &str) -> bool;
    pub fn get_attribute_node(&self, name: &str) -> Option<AttrRef>;
    pub fn set_attribute_node(&mut self, attr: AttrRef) -> Result<Option<AttrRef>, DomException>;
    
    /// Namespaced attributes
    pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<String>;
    pub fn set_attribute_ns(&mut self, namespace: Option<&str>, qualified_name: &str, value: &str) -> Result<(), DomException>;
    
    /// Element traversal
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> HTMLCollection;
    pub fn get_elements_by_class_name(&self, class_names: &str) -> HTMLCollection;
    pub fn get_elements_by_tag_name_ns(&self, namespace: Option<&str>, local_name: &str) -> HTMLCollection;
    
    /// Selectors API
    pub fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
    pub fn query_selector_all(&self, selector: &str) -> Result<NodeList, DomException>;
    pub fn matches(&self, selector: &str) -> Result<bool, DomException>;
    pub fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
    
    /// Shadow DOM
    pub fn attach_shadow(&mut self, mode: ShadowRootMode) -> Result<ShadowRootRef, DomException>;
    pub fn shadow_root(&self) -> Option<ShadowRootRef>;
    
    /// Geometry
    pub fn get_bounding_client_rect(&self) -> DOMRect;
    pub fn get_client_rects(&self) -> DOMRectList;
    pub fn scroll_into_view(&self, options: ScrollIntoViewOptions);
}
```

### Document Interface
```rust
pub struct Document {
    node_data: NodeData,
    doctype: Option<DocumentTypeRef>,
    document_element: Option<ElementRef>,
    implementation: DOMImplementation,
    url: String,
    document_uri: String,
    compatibility_mode: CompatibilityMode,
    character_set: String,
    content_type: String,
    
    // Node registry
    node_registry: HashMap<NodeId, WeakNodeRef>,
    id_map: HashMap<String, Vec<ElementRef>>,
    name_map: HashMap<String, Vec<ElementRef>>,
}

impl Document {
    /// Document creation
    pub fn new() -> Self;
    
    /// Element creation
    pub fn create_element(&mut self, tag_name: &str) -> Result<ElementRef, DomException>;
    pub fn create_element_ns(&mut self, namespace: Option<&str>, qualified_name: &str) -> Result<ElementRef, DomException>;
    pub fn create_text_node(&mut self, data: &str) -> TextRef;
    pub fn create_comment(&mut self, data: &str) -> CommentRef;
    pub fn create_document_fragment(&mut self) -> DocumentFragmentRef;
    pub fn create_attribute(&mut self, name: &str) -> Result<AttrRef, DomException>;
    pub fn create_attribute_ns(&mut self, namespace: Option<&str>, qualified_name: &str) -> Result<AttrRef, DomException>;
    
    /// Node adoption and importing
    pub fn import_node(&mut self, node: NodeRef, deep: bool) -> Result<NodeRef, DomException>;
    pub fn adopt_node(&mut self, node: NodeRef) -> Result<NodeRef, DomException>;
    
    /// Element access
    pub fn get_element_by_id(&self, id: &str) -> Option<ElementRef>;
    pub fn get_elements_by_tag_name(&self, tag_name: &str) -> HTMLCollection;
    pub fn get_elements_by_class_name(&self, class_names: &str) -> HTMLCollection;
    pub fn get_elements_by_name(&self, name: &str) -> NodeList;
    
    /// Events
    pub fn create_event(&self, interface: &str) -> Result<EventRef, DomException>;
    
    /// Ranges
    pub fn create_range(&mut self) -> RangeRef;
    
    /// Tree walking
    pub fn create_node_iterator(&self, root: NodeRef, what_to_show: u32, filter: Option<NodeFilter>) -> NodeIterator;
    pub fn create_tree_walker(&self, root: NodeRef, what_to_show: u32, filter: Option<NodeFilter>) -> TreeWalker;
}
```

### Event System Interface
```rust
/// Event interface
pub struct Event {
    event_type: String,
    target: Option<EventTargetRef>,
    current_target: Option<EventTargetRef>,
    event_phase: EventPhase,
    bubbles: bool,
    cancelable: bool,
    default_prevented: bool,
    composed: bool,
    is_trusted: bool,
    time_stamp: f64,
    stop_propagation_flag: bool,
    stop_immediate_propagation_flag: bool,
}

impl Event {
    pub fn new(event_type: &str, init: EventInit) -> Self;
    pub fn prevent_default(&mut self);
    pub fn stop_propagation(&mut self);
    pub fn stop_immediate_propagation(&mut self);
}

/// EventTarget trait for nodes that can receive events
pub trait EventTarget: Node {
    fn add_event_listener(&mut self, event_type: &str, listener: EventListener, options: AddEventListenerOptions);
    fn remove_event_listener(&mut self, event_type: &str, listener: EventListener, options: EventListenerOptions);
    fn dispatch_event(&mut self, event: EventRef) -> Result<bool, DomException>;
}

/// Event listener types
pub enum EventListener {
    Function(Box<dyn Fn(&Event) + Send + Sync>),
    Object(Box<dyn EventListenerObject + Send + Sync>),
}

pub trait EventListenerObject: Send + Sync {
    fn handle_event(&self, event: &Event);
}
```

### Collections Interface
```rust
/// Live HTMLCollection
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

/// NodeList (can be live or static)
pub enum NodeList {
    Live {
        root: WeakNodeRef,
        filter: Box<dyn Fn(&dyn Node) -> bool + Send + Sync>,
    },
    Static {
        nodes: Vec<NodeRef>,
    },
}

impl NodeList {
    pub fn length(&self) -> usize;
    pub fn item(&self, index: usize) -> Option<NodeRef>;
}

/// DOMTokenList for managing space-separated tokens
pub struct DOMTokenList {
    element: WeakElementRef,
    attribute_name: String,
}

impl DOMTokenList {
    pub fn length(&self) -> usize;
    pub fn item(&self, index: usize) -> Option<String>;
    pub fn contains(&self, token: &str) -> bool;
    pub fn add(&mut self, tokens: &[&str]) -> Result<(), DomException>;
    pub fn remove(&mut self, tokens: &[&str]) -> Result<(), DomException>;
    pub fn toggle(&mut self, token: &str, force: Option<bool>) -> Result<bool, DomException>;
    pub fn replace(&mut self, old_token: &str, new_token: &str) -> Result<bool, DomException>;
}
```

### MutationObserver Interface
```rust
pub struct MutationObserver {
    callback: Box<dyn Fn(&[MutationRecord], &MutationObserver) + Send + Sync>,
    observed_nodes: Vec<(WeakNodeRef, MutationObserverInit)>,
    record_queue: Vec<MutationRecord>,
}

impl MutationObserver {
    pub fn new(callback: Box<dyn Fn(&[MutationRecord], &MutationObserver) + Send + Sync>) -> Self;
    pub fn observe(&mut self, target: NodeRef, options: MutationObserverInit) -> Result<(), DomException>;
    pub fn disconnect(&mut self);
    pub fn take_records(&mut self) -> Vec<MutationRecord>;
}

pub struct MutationRecord {
    pub record_type: MutationType,
    pub target: NodeRef,
    pub added_nodes: NodeList,
    pub removed_nodes: NodeList,
    pub previous_sibling: Option<NodeRef>,
    pub next_sibling: Option<NodeRef>,
    pub attribute_name: Option<String>,
    pub attribute_namespace: Option<String>,
    pub old_value: Option<String>,
}
```

## Component Message Bus Integration

### Incoming Messages
```rust
pub enum DomComponentMessage {
    // From HTML Parser
    ParsedDocument {
        request_id: u64,
        root: ParsedNode,
        doctype: Option<DocumentType>,
    },
    
    // From JavaScript Runtime
    ScriptManipulation {
        operation: DomOperation,
        node_id: NodeId,
        params: OperationParams,
    },
    
    // From CSS Engine
    StyleInvalidation {
        node_id: NodeId,
        reason: InvalidationReason,
    },
    
    // From Browser Shell
    UserInteraction {
        event_type: String,
        target_id: NodeId,
        event_data: EventData,
    },
    
    // From Network Stack
    ResourceLoaded {
        element_id: NodeId,
        resource_type: ResourceType,
        data: Vec<u8>,
    },
}

pub enum DomOperation {
    AppendChild { parent_id: NodeId, child_id: NodeId },
    RemoveChild { parent_id: NodeId, child_id: NodeId },
    SetAttribute { element_id: NodeId, name: String, value: String },
    RemoveAttribute { element_id: NodeId, name: String },
    SetTextContent { node_id: NodeId, text: String },
    CreateElement { tag_name: String, namespace: Option<String> },
    QuerySelector { selector: String, root_id: Option<NodeId> },
}
```

### Outgoing Messages
```rust
pub enum DomComponentResponse {
    // To JavaScript Runtime
    DomMutated {
        mutations: Vec<MutationRecord>,
        affected_nodes: Vec<NodeId>,
    },
    
    // To CSS Engine
    DomTreeChanged {
        change_type: TreeChangeType,
        affected_subtree: NodeId,
        needs_style_recalc: bool,
    },
    
    // To Rendering Engine
    LayoutInvalidated {
        root_id: NodeId,
        invalidation_type: LayoutInvalidationType,
    },
    
    // To Browser Shell
    EventDispatched {
        event_id: u64,
        prevented_default: bool,
        propagation_stopped: bool,
    },
    
    // Query responses
    QueryResult {
        request_id: u64,
        result: QueryResultType,
    },
}

pub enum QueryResultType {
    Element(Option<NodeId>),
    NodeList(Vec<NodeId>),
    Boolean(bool),
    Text(String),
}
```

## Internal Implementation Details

### Thread-Safe Node Storage
```rust
/// Arena-based node storage with generational indices
pub struct NodeArena {
    nodes: Arc<RwLock<Vec<Option<NodeEntry>>>>,
    free_list: Arc<Mutex<Vec<usize>>>,
    generation: AtomicU64,
}

struct NodeEntry {
    node: Box<dyn Node>,
    generation: u64,
    strong_refs: AtomicUsize,
    weak_refs: AtomicUsize,
}

impl NodeArena {
    pub fn allocate(&self, node: Box<dyn Node>) -> NodeId {
        let mut nodes = self.nodes.write().unwrap();
        let mut free_list = self.free_list.lock().unwrap();
        
        let index = if let Some(index) = free_list.pop() {
            index
        } else {
            let index = nodes.len();
            nodes.push(None);
            index
        };
        
        let generation = self.generation.fetch_add(1, Ordering::SeqCst);
        nodes[index] = Some(NodeEntry {
            node,
            generation,
            strong_refs: AtomicUsize::new(1),
            weak_refs: AtomicUsize::new(0),
        });
        
        NodeId { index, generation }
    }
    
    pub fn get(&self, id: NodeId) -> Option<NodeRef> {
        let nodes = self.nodes.read().unwrap();
        if let Some(Some(entry)) = nodes.get(id.index) {
            if entry.generation == id.generation {
                entry.strong_refs.fetch_add(1, Ordering::SeqCst);
                return Some(Arc::new(RefCell::new(entry.node.clone())));
            }
        }
        None
    }
}
```

### Event Dispatch Algorithm
```rust
impl EventDispatcher {
    pub fn dispatch(&self, event: EventRef, target: EventTargetRef) -> Result<bool, DomException> {
        // 1. Validate event state
        if event.borrow().dispatch_flag {
            return Err(DomException::InvalidStateError);
        }
        
        // 2. Initialize event
        event.borrow_mut().dispatch_flag = true;
        event.borrow_mut().target = Some(target.clone());
        
        // 3. Determine propagation path
        let path = self.calculate_event_path(target.clone(), event.borrow().composed);
        
        // 4. Capture phase
        event.borrow_mut().event_phase = EventPhase::Capturing;
        for ancestor in path.iter().rev().skip(1) {
            if event.borrow().stop_propagation_flag {
                break;
            }
            self.invoke_listeners(ancestor, &event, EventPhase::Capturing);
        }
        
        // 5. Target phase
        if !event.borrow().stop_propagation_flag {
            event.borrow_mut().event_phase = EventPhase::AtTarget;
            self.invoke_listeners(&target, &event, EventPhase::AtTarget);
        }
        
        // 6. Bubble phase (if event bubbles)
        if event.borrow().bubbles && !event.borrow().stop_propagation_flag {
            event.borrow_mut().event_phase = EventPhase::Bubbling;
            for ancestor in path.iter().skip(1) {
                if event.borrow().stop_propagation_flag {
                    break;
                }
                self.invoke_listeners(ancestor, &event, EventPhase::Bubbling);
            }
        }
        
        // 7. Cleanup
        event.borrow_mut().event_phase = EventPhase::None;
        event.borrow_mut().current_target = None;
        event.borrow_mut().dispatch_flag = false;
        
        Ok(!event.borrow().default_prevented)
    }
    
    fn invoke_listeners(&self, target: &EventTargetRef, event: &EventRef, phase: EventPhase) {
        let listeners = target.borrow().get_listeners(&event.borrow().event_type, phase);
        
        event.borrow_mut().current_target = Some(target.clone());
        
        for listener in listeners {
            if event.borrow().stop_immediate_propagation_flag {
                break;
            }
            
            match listener {
                EventListener::Function(f) => f(&event.borrow()),
                EventListener::Object(obj) => obj.handle_event(&event.borrow()),
            }
        }
    }
}
```

### Live Collection Implementation
```rust
impl HTMLCollection {
    pub fn new<F>(root: NodeRef, filter: F) -> Self 
    where
        F: Fn(&Element) -> bool + Send + Sync + 'static,
    {
        HTMLCollection {
            root: Arc::downgrade(&root),
            filter: Box::new(filter),
            cached_items: RefCell::new(Vec::new()),
            version: 0,
        }
    }
    
    fn update_cache(&self) {
        let mut items = Vec::new();
        
        if let Some(root) = self.root.upgrade() {
            self.collect_matching_elements(&root, &mut items);
        }
        
        *self.cached_items.borrow_mut() = items;
    }
    
    fn collect_matching_elements(&self, node: &NodeRef, items: &mut Vec<ElementRef>) {
        if let Some(element) = node.borrow().as_element() {
            if (self.filter)(&element) {
                items.push(Arc::new(RefCell::new(element.clone())));
            }
        }
        
        for child in node.borrow().child_nodes().iter() {
            self.collect_matching_elements(&child, items);
        }
    }
    
    pub fn length(&self) -> usize {
        self.update_cache();
        self.cached_items.borrow().len()
    }
    
    pub fn item(&self, index: usize) -> Option<ElementRef> {
        self.update_cache();
        self.cached_items.borrow().get(index).cloned()
    }
}
```

### Mutation Observation
```rust
impl MutationQueue {
    pub fn queue_mutation(&mut self, record: MutationRecord) {
        // Find or create queue for observer
        for observer in &self.active_observers {
            if observer.is_interested_in(&record) {
                observer.queue_record(record.clone());
            }
        }
        
        // Schedule microtask to deliver mutations
        if !self.delivery_scheduled {
            self.delivery_scheduled = true;
            self.schedule_mutation_delivery();
        }
    }
    
    fn schedule_mutation_delivery(&self) {
        // Queue a microtask to deliver all pending mutations
        microtask::queue(|| {
            let observers = self.active_observers.clone();
            for observer in observers {
                let records = observer.take_records();
                if !records.is_empty() {
                    (observer.callback)(&records, &observer);
                }
            }
            self.delivery_scheduled = false;
        });
    }
}
```

## Build Configuration

### Cargo.toml
```toml
[package]
name = "browser-dom-impl"
version = "0.1.0"
edition = "2021"
authors = ["CortenBrowser Team"]
description = "DOM Implementation for CortenBrowser"

[dependencies]
# Core browser dependencies
browser-interfaces = { path = "../../shared/interfaces" }
browser-messages = { path = "../../shared/messages" }
browser-types = { path = "../../shared/types" }

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Data structures
dashmap = "5.5"           # Concurrent HashMap
indexmap = "2.1"          # Order-preserving HashMap
smallvec = "1.11"         # Small vector optimization
string_cache = "0.8"      # Atom strings

# Memory management
typed-arena = "2.0"       # Arena allocation
weak-table = "0.3"        # Weak reference tables

# Parsing and validation
cssparser = "0.31"        # For selector parsing
selectors = "0.25"        # CSS selector matching
html5ever = "0.26"        # HTML namespace/validation
markup5ever = "0.11"      # Markup utilities

# Serialization
serde = { version = "1.0", features = ["derive", "rc"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Thread safety
parking_lot = "0.12"      # Faster mutexes
crossbeam = "0.8"         # Concurrent utilities
rayon = "1.8"             # Parallel iteration

# Testing utilities
[dev-dependencies]
criterion = "0.5"         # Benchmarking
proptest = "1.4"         # Property testing
insta = "1.34"           # Snapshot testing
mockall = "0.12"         # Mocking framework

# Optional features
[features]
default = ["parallel", "validation"]
parallel = ["rayon"]
validation = []           # Extra validation in debug builds
profiling = ["pprof", "flame"]
experimental-shadow-dom = []
experimental-custom-elements = []

# Profile settings
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
debug = false

[profile.bench]
inherits = "release"

[profile.test]
opt-level = 2
```

### Build Script (build.rs)
```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // Generate atom string cache
    println!("cargo:rerun-if-changed=src/atoms.txt");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let atoms_file = "src/atoms.txt";
    
    // Read common DOM strings to intern
    let atoms = std::fs::read_to_string(atoms_file)
        .expect("Failed to read atoms file");
    
    // Generate static atom cache
    let mut atom_code = String::from("
        pub mod atoms {
            use string_cache::Atom;
            
    ");
    
    for atom in atoms.lines() {
        if !atom.is_empty() && !atom.starts_with('#') {
            let const_name = atom.to_uppercase().replace('-', "_");
            atom_code.push_str(&format!(
                "    pub static {}: Atom = Atom::from(\"{}\");\n",
                const_name, atom
            ));
        }
    }
    
    atom_code.push_str("}\n");
    
    std::fs::write(out_path.join("atoms.rs"), atom_code)
        .expect("Failed to write atoms file");
    
    // Platform-specific configuration
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "linux" => {
            println!("cargo:rustc-cfg=use_jemalloc");
        },
        "windows" => {
            println!("cargo:rustc-cfg=use_mimalloc");
        },
        _ => {}
    }
}
```

## Testing Strategy

### Unit Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    mod node_tests {
        use super::*;
        
        #[test]
        fn test_node_creation() {
            let doc = Document::new();
            let element = doc.create_element("div").unwrap();
            assert_eq!(element.borrow().node_type(), NodeType::Element);
            assert_eq!(element.borrow().node_name(), "DIV");
        }
        
        #[test]
        fn test_node_insertion() {
            let doc = Document::new();
            let parent = doc.create_element("div").unwrap();
            let child = doc.create_element("span").unwrap();
            
            parent.borrow_mut().append_child(child.clone()).unwrap();
            assert_eq!(parent.borrow().child_nodes().length(), 1);
            assert!(child.borrow().parent_node().is_some());
        }
        
        #[test]
        fn test_node_removal() {
            let doc = Document::new();
            let parent = doc.create_element("div").unwrap();
            let child = doc.create_element("span").unwrap();
            
            parent.borrow_mut().append_child(child.clone()).unwrap();
            parent.borrow_mut().remove_child(child.clone()).unwrap();
            
            assert_eq!(parent.borrow().child_nodes().length(), 0);
            assert!(child.borrow().parent_node().is_none());
        }
    }
    
    mod event_tests {
        use super::*;
        
        #[test]
        fn test_event_dispatch() {
            let doc = Document::new();
            let element = doc.create_element("button").unwrap();
            let event = Event::new("click", EventInit::default());
            
            let clicked = Arc::new(AtomicBool::new(false));
            let clicked_clone = clicked.clone();
            
            element.borrow_mut().add_event_listener(
                "click",
                EventListener::Function(Box::new(move |_| {
                    clicked_clone.store(true, Ordering::SeqCst);
                })),
                AddEventListenerOptions::default()
            );
            
            element.borrow_mut().dispatch_event(event).unwrap();
            assert!(clicked.load(Ordering::SeqCst));
        }
        
        #[test]
        fn test_event_bubbling() {
            let doc = Document::new();
            let parent = doc.create_element("div").unwrap();
            let child = doc.create_element("button").unwrap();
            parent.borrow_mut().append_child(child.clone()).unwrap();
            
            let mut capture_order = Vec::new();
            let capture_order_rc = Rc::new(RefCell::new(capture_order));
            
            // Add capturing listener on parent
            let order_clone = capture_order_rc.clone();
            parent.borrow_mut().add_event_listener(
                "click",
                EventListener::Function(Box::new(move |_| {
                    order_clone.borrow_mut().push("parent_capture");
                })),
                AddEventListenerOptions { capture: true, ..Default::default() }
            );
            
            // Add bubbling listener on parent
            let order_clone = capture_order_rc.clone();
            parent.borrow_mut().add_event_listener(
                "click",
                EventListener::Function(Box::new(move |_| {
                    order_clone.borrow_mut().push("parent_bubble");
                })),
                AddEventListenerOptions::default()
            );
            
            // Add target listener on child
            let order_clone = capture_order_rc.clone();
            child.borrow_mut().add_event_listener(
                "click",
                EventListener::Function(Box::new(move |_| {
                    order_clone.borrow_mut().push("child_target");
                })),
                AddEventListenerOptions::default()
            );
            
            // Dispatch event on child
            let event = Event::new("click", EventInit { bubbles: true, ..Default::default() });
            child.borrow_mut().dispatch_event(event).unwrap();
            
            // Check order
            assert_eq!(
                *capture_order_rc.borrow(),
                vec!["parent_capture", "child_target", "parent_bubble"]
            );
        }
    }
}
```

### Web Platform Test Integration
```rust
// tests/wpt/harness.rs
use wpt_runner::{TestHarness, TestResult};
use browser_dom_impl::*;

pub struct DomTestHarness {
    document: Document,
}

impl TestHarness for DomTestHarness {
    fn create_document(&mut self) -> serde_json::Value {
        self.document = Document::new();
        self.serialize_document()
    }
    
    fn create_element(&mut self, tag_name: &str) -> serde_json::Value {
        let element = self.document.create_element(tag_name).unwrap();
        self.serialize_node(element)
    }
    
    fn append_child(&mut self, parent_id: &str, child_id: &str) -> TestResult {
        let parent = self.get_node_by_id(parent_id)?;
        let child = self.get_node_by_id(child_id)?;
        parent.borrow_mut().append_child(child)
            .map(|_| TestResult::Pass)
            .unwrap_or(TestResult::Fail("Failed to append child".into()))
    }
    
    fn query_selector(&mut self, selector: &str, root_id: Option<&str>) -> serde_json::Value {
        let root = root_id
            .and_then(|id| self.get_element_by_id(id))
            .unwrap_or_else(|| self.document.document_element());
        
        match root.borrow().query_selector(selector) {
            Ok(Some(element)) => self.serialize_node(element),
            Ok(None) => serde_json::Value::Null,
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
}

// Test runner configuration
fn run_wpt_tests() {
    let harness = DomTestHarness::new();
    let test_files = vec![
        "wpt/dom/nodes/Node-appendChild.html",
        "wpt/dom/nodes/Node-removeChild.html",
        "wpt/dom/nodes/Element-getElementsByTagName.html",
        "wpt/dom/events/Event-dispatch-*",
        "wpt/dom/traversal/TreeWalker-*",
    ];
    
    for test_file in test_files {
        wpt_runner::run_test_file(test_file, harness);
    }
}
```

### Performance Benchmarks
```rust
// benches/dom_operations.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use browser_dom_impl::*;

fn benchmark_element_creation(c: &mut Criterion) {
    c.bench_function("create_element", |b| {
        let doc = Document::new();
        b.iter(|| {
            black_box(doc.create_element("div").unwrap())
        });
    });
}

fn benchmark_deep_tree_creation(c: &mut Criterion) {
    c.bench_function("create_deep_tree_1000_nodes", |b| {
        b.iter(|| {
            let doc = Document::new();
            let root = doc.create_element("div").unwrap();
            
            for i in 0..10 {
                let child = doc.create_element("div").unwrap();
                for j in 0..10 {
                    let grandchild = doc.create_element("div").unwrap();
                    for k in 0..10 {
                        let leaf = doc.create_element("span").unwrap();
                        grandchild.borrow_mut().append_child(leaf).unwrap();
                    }
                    child.borrow_mut().append_child(grandchild).unwrap();
                }
                root.borrow_mut().append_child(child).unwrap();
            }
            black_box(root)
        });
    });
}

fn benchmark_query_selector(c: &mut Criterion) {
    let doc = Document::new();
    let root = create_test_tree(&doc, 1000);
    
    c.bench_function("query_selector_complex", |b| {
        b.iter(|| {
            black_box(
                root.borrow()
                    .query_selector("div > span:nth-child(3)[data-test='value']")
                    .unwrap()
            )
        });
    });
}

fn benchmark_event_dispatch(c: &mut Criterion) {
    let doc = Document::new();
    let tree = create_test_tree(&doc, 100);
    setup_event_listeners(&tree, 10);
    
    c.bench_function("event_dispatch_with_bubbling", |b| {
        b.iter(|| {
            let event = Event::new("click", EventInit { bubbles: true, ..Default::default() });
            black_box(get_deepest_element(&tree).borrow_mut().dispatch_event(event).unwrap())
        });
    });
}

criterion_group!(
    benches,
    benchmark_element_creation,
    benchmark_deep_tree_creation,
    benchmark_query_selector,
    benchmark_event_dispatch
);
criterion_main!(benches);
```

## Development Milestones

### Phase 1: Core DOM Structure (Weeks 1-2)
**Goal**: Basic DOM tree with node operations
**Deliverables**:
- Node, Element, Document, Text implementations
- Basic tree operations (append, remove, insert)
- Parent-child relationships
- Node cloning
- **Validation**: Internal unit tests pass 100%
- **LOC Target**: 15,000-20,000

### Phase 2: Collections and Traversal (Week 3)
**Goal**: Live collections and tree traversal
**Deliverables**:
- HTMLCollection, NodeList implementations
- NodeIterator, TreeWalker
- Element.getElementsBy* methods
- **Validation**: Collection tests pass 100%
- **LOC Target**: 10,000-12,000

### Phase 3: Event System (Weeks 4-5)
**Goal**: Complete event dispatch and handling
**Deliverables**:
- Event interface and types
- EventTarget implementation
- Event dispatch algorithm
- Capture/bubble propagation
- **Validation**: WPT events/* tests pass 85%+
- **LOC Target**: 15,000-18,000

### Phase 4: Selectors API (Week 6)
**Goal**: CSS selector matching
**Deliverables**:
- querySelector/querySelectorAll
- matches, closest methods
- Selector parsing and matching
- **Validation**: WPT selectors/* tests pass 90%+
- **LOC Target**: 8,000-10,000

### Phase 5: Advanced Features (Week 7)
**Goal**: MutationObserver and Ranges
**Deliverables**:
- MutationObserver implementation
- Range API
- Selection API
- **Validation**: Mutation observer tests pass 85%+
- **LOC Target**: 12,000-15,000

### Phase 6: Shadow DOM (Week 8)
**Goal**: Shadow DOM support
**Deliverables**:
- ShadowRoot implementation
- Slot distribution
- Event retargeting
- **Validation**: Shadow DOM tests pass 80%+
- **LOC Target**: 10,000-12,000

### Phase 7: Performance & Memory (Week 9)
**Goal**: Optimization and memory management
**Deliverables**:
- Arena allocator optimization
- Weak reference cleanup
- Collection caching
- **Validation**: Benchmarks within 2x of Chrome
- **LOC Target**: 5,000-7,000

### Phase 8: Integration & Polish (Week 10)
**Goal**: Full integration with other components
**Deliverables**:
- Message bus integration
- Cross-component testing
- Documentation
- **Validation**: WPT dom/* tests pass 90%+
- **Total LOC**: 100,000-125,000

## Integration Points

### HTML Parser Integration
```rust
// Receiving parsed tree from HTML parser
impl DomComponent {
    pub fn handle_parsed_document(&mut self, msg: ParsedDocumentMessage) {
        // Convert parsed nodes to DOM nodes
        let document = Document::new();
        let root = self.build_dom_tree(document.clone(), msg.root);
        
        // Set as document element
        document.set_document_element(root);
        
        // Store document
        self.documents.insert(msg.request_id, document);
        
        // Notify other components
        self.send_message(DomComponentResponse::DomTreeReady {
            request_id: msg.request_id,
            document_id: document.id(),
        });
    }
}
```

### JavaScript Runtime Integration
```rust
// Exposing DOM API to JavaScript
impl JsBindings for DomComponent {
    fn register_bindings(&self, runtime: &mut JsRuntime) {
        runtime.register_class::<Node>("Node");
        runtime.register_class::<Element>("Element");
        runtime.register_class::<Document>("Document");
        runtime.register_class::<Event>("Event");
        
        // Register global constructors
        runtime.register_constructor("Document", Document::new);
        runtime.register_constructor("Event", Event::new);
        
        // Register prototype methods
        runtime.register_method("Node", "appendChild", Node::append_child);
        runtime.register_method("Element", "querySelector", Element::query_selector);
        // ... etc
    }
}
```

### CSS Engine Integration
```rust
// Providing DOM tree for style calculation
impl DomComponent {
    pub fn get_style_tree(&self, document_id: DocumentId) -> StyleTree {
        let document = self.documents.get(&document_id).unwrap();
        let root = document.document_element().unwrap();
        
        self.build_style_tree(root)
    }
    
    fn build_style_tree(&self, node: NodeRef) -> StyleNode {
        StyleNode {
            node_id: node.id(),
            node_type: node.node_type(),
            tag_name: node.tag_name(),
            attributes: node.attributes(),
            children: node.child_nodes()
                .iter()
                .map(|child| self.build_style_tree(child))
                .collect(),
        }
    }
}
```

## Error Handling

### DomException Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum DomException {
    #[error("Hierarchy request error")]
    HierarchyRequestError,
    
    #[error("Wrong document error")]
    WrongDocumentError,
    
    #[error("Invalid character error")]
    InvalidCharacterError,
    
    #[error("No modification allowed error")]
    NoModificationAllowedError,
    
    #[error("Not found error")]
    NotFoundError,
    
    #[error("Not supported error")]
    NotSupportedError,
    
    #[error("Invalid state error")]
    InvalidStateError,
    
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    
    #[error("Invalid modification error")]
    InvalidModificationError,
    
    #[error("Namespace error")]
    NamespaceError,
    
    #[error("Security error")]
    SecurityError,
}
```

## Memory Management

### Garbage Collection Strategy
```rust
impl GarbageCollector for DomComponent {
    fn collect(&mut self) {
        // Mark phase - find all reachable nodes
        let mut reachable = HashSet::new();
        
        for document in self.documents.values() {
            self.mark_reachable(document.root(), &mut reachable);
        }
        
        // Sweep phase - remove unreachable nodes
        self.arena.sweep(|node_id| {
            !reachable.contains(&node_id)
        });
        
        // Compact arena if fragmentation is high
        if self.arena.fragmentation() > 0.5 {
            self.arena.compact();
        }
    }
    
    fn mark_reachable(&self, node: NodeRef, reachable: &mut HashSet<NodeId>) {
        reachable.insert(node.id());
        
        for child in node.child_nodes().iter() {
            self.mark_reachable(child, reachable);
        }
        
        // Mark event listeners
        for listener in node.event_listeners() {
            if let Some(target) = listener.get_target() {
                reachable.insert(target.id());
            }
        }
    }
}
```

## Security Considerations

### Input Validation
```rust
impl Validator {
    pub fn validate_tag_name(name: &str) -> Result<(), DomException> {
        // Check for valid characters
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(DomException::InvalidCharacterError);
        }
        
        // Check for reserved names
        if RESERVED_NAMES.contains(&name.to_lowercase().as_str()) {
            return Err(DomException::InvalidCharacterError);
        }
        
        Ok(())
    }
    
    pub fn validate_attribute_name(name: &str) -> Result<(), DomException> {
        // Similar validation for attributes
        // ...
    }
}
```

### XSS Prevention
```rust
impl SanitizationPolicy {
    pub fn sanitize_html(&self, html: &str) -> String {
        // Remove dangerous elements and attributes
        let mut sanitized = html.to_string();
        
        // Remove script tags
        sanitized = SCRIPT_TAG_REGEX.replace_all(&sanitized, "").to_string();
        
        // Remove event handler attributes
        sanitized = EVENT_HANDLER_REGEX.replace_all(&sanitized, "").to_string();
        
        // Remove javascript: URLs
        sanitized = JS_URL_REGEX.replace_all(&sanitized, "").to_string();
        
        sanitized
    }
}
```

## Configuration

### Component Configuration
```rust
#[derive(Debug, serde::Deserialize)]
pub struct DomConfig {
    /// Maximum tree depth
    pub max_tree_depth: usize,
    
    /// Maximum number of children per node
    pub max_children: usize,
    
    /// Enable mutation observers
    pub enable_mutation_observers: bool,
    
    /// Enable shadow DOM
    pub enable_shadow_dom: bool,
    
    /// GC threshold (number of nodes)
    pub gc_threshold: usize,
    
    /// Arena initial capacity
    pub arena_capacity: usize,
    
    /// Event listener limit per node
    pub max_listeners_per_node: usize,
}

impl Default for DomConfig {
    fn default() -> Self {
        DomConfig {
            max_tree_depth: 512,
            max_children: 10000,
            enable_mutation_observers: true,
            enable_shadow_dom: true,
            gc_threshold: 100000,
            arena_capacity: 50000,
            max_listeners_per_node: 100,
        }
    }
}
```

## Claude Code Development Instructions

### Initial Setup Commands
```bash
# Create component directory structure
mkdir -p dom-impl/{src/{node,events,traversal,ranges,collections,mutation,shadow,storage,validation,interfaces,utils},tests/{unit,integration,wpt},benches}

# Initialize Rust project
cd dom-impl
cargo init --lib

# Copy the provided Cargo.toml
# Create build.rs with the provided content

# Create atoms.txt for string interning
cat > src/atoms.txt << EOF
# Common DOM strings for interning
div
span
p
a
img
button
input
form
body
head
html
class
id
style
href
src
click
change
submit
load
DOMContentLoaded
EOF

# Set up git repository
git init
git add .
git commit -m "Initial DOM component structure"
```

### Development Workflow
```bash
# 1. Implement core modules in order
cargo build --lib

# 2. Run unit tests continuously
cargo watch -x "test --lib"

# 3. Run benchmarks after each phase
cargo bench --bench dom_operations

# 4. Integration test with WPT
./scripts/run-wpt-subset.sh dom/

# 5. Check memory usage
valgrind --leak-check=full cargo test

# 6. Profile performance
cargo build --release --features profiling
perf record ./target/release/dom-bench
perf report
```

### Validation Checkpoints

Each phase must pass these checks before proceeding:

1. **Unit Tests**: 100% pass rate
2. **Memory Leaks**: Zero leaks in valgrind
3. **Thread Safety**: No data races with `RUSTFLAGS="-Z sanitizer=thread"`
4. **Performance**: Benchmarks within target range
5. **WPT Tests**: Meet phase-specific pass rate

---

**End of DOM Component Specification v1.0**

This specification provides complete implementation details for autonomous development by Claude Code instances. The component can be built independently and integrated with other browser components through the defined message bus interface.