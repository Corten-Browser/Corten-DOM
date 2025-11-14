# dom_collections

**Type**: Feature Component
**Dependencies**: dom_core
**Version**: 0.1.0

## Purpose
Live collections and tree traversal.

## Key Features
- HTMLCollection (live)
- NodeList (live and static)
- DOMTokenList
- NodeIterator, TreeWalker

## Public API
```rust
pub struct HTMLCollection { ... }
pub struct NodeList { ... }
pub struct NodeIterator { ... }
pub struct TreeWalker { ... }
```

See `CLAUDE.md` for detailed development instructions.
