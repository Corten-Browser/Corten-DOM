# dom_types

**Type**: Base Component
**Dependencies**: None
**Version**: 0.1.0

## Purpose

Shared type definitions, enums, and error types for the DOM implementation.

## Responsibility

This component provides the common types used across all DOM components:
- `NodeType` enum
- `DomException` error types
- `NodeId` and `DocumentId` type aliases
- Common constants

## Structure

```
dom_types/
├── src/
│   ├── lib.rs          # Public exports
│   ├── error.rs        # DomException types
│   ├── node_type.rs    # NodeType enum
│   └── ids.rs          # ID type aliases
├── tests/
│   └── unit/
└── Cargo.toml
```

## Public API

```rust
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

pub enum DomException {
    HierarchyRequestError,
    WrongDocumentError,
    InvalidCharacterError,
    NotFoundError,
    // ... etc
}

pub type NodeId = u64;
pub type DocumentId = u64;
```

## Testing

```bash
cargo test
```

## Development

See `CLAUDE.md` for detailed development instructions.
