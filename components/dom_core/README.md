# dom_core

**Type**: Core Component
**Dependencies**: dom_types, dom_storage
**Version**: 0.1.0

## Purpose
Core DOM nodes and tree operations.

## Key Features
- Node, Element, Document, Text implementations
- Tree manipulation (append, remove, insert)
- Node cloning and normalization
- Attribute management

## Public API
```rust
pub trait Node { ... }
pub struct Element { ... }
pub struct Document { ... }
pub type NodeRef = Arc<RefCell<Box<dyn Node>>>;
```

See `CLAUDE.md` for detailed development instructions.
