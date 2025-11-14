# Component: dom_types

## Component Identification
- **Name**: dom_types
- **Type**: Base (Level 0)
- **Version**: 0.1.0
- **Language**: Rust
- **Project Root**: /home/user/Corten-DOM
- **Component Root**: /home/user/Corten-DOM/components/dom_types

## Responsibility
Provide shared type definitions, enums, and error types for all DOM components.

## Dependencies
- **Internal**: None (Base component)
- **External**: `serde`, `thiserror`

## Development Workflow

### 1. Read Specification
Read the main specification:
```bash
cat /home/user/Corten-DOM/dom-component-specification.md
```

Focus on sections related to types, especially:
- NodeType enum (lines 166-178)
- DomException types (lines 1262-1298)
- Type aliases (lines 180-184)

### 2. Implement Core Types (TDD)

**Step 1: Write Tests First (RED)**

Create test file `tests/unit/test_node_type.rs`:
```rust
use dom_types::NodeType;

#[test]
fn test_node_type_values() {
    assert_eq!(NodeType::Element as u16, 1);
    assert_eq!(NodeType::Attribute as u16, 2);
    assert_eq!(NodeType::Text as u16, 3);
    // ... test all values match spec
}

#[test]
fn test_node_type_debug() {
    assert_eq!(format!("{:?}", NodeType::Element), "Element");
}
```

Run tests (should FAIL):
```bash
cargo test
```

**Step 2: Implement Types (GREEN)**

Create `src/node_type.rs`:
```rust
/// Node type enumeration per DOM Level 4 spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u16)]
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
```

Run tests (should PASS):
```bash
cargo test
```

**Step 3: Implement DomException (TDD)**

Tests first, then implementation following same RED-GREEN-REFACTOR pattern.

### 3. File Structure

Create these files:
```
src/
├── lib.rs              # Public exports
├── error.rs            # DomException error enum
├── node_type.rs        # NodeType enum
└── ids.rs              # NodeId, DocumentId type aliases
```

### 4. Implementation Requirements

#### src/error.rs
Implement all DOM exception types from spec lines 1262-1298:
```rust
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomException {
    #[error("Hierarchy request error")]
    HierarchyRequestError,

    #[error("Wrong document error")]
    WrongDocumentError,

    #[error("Invalid character error")]
    InvalidCharacterError,

    // ... implement all exception types from spec
}
```

#### src/ids.rs
Type aliases for node and document IDs:
```rust
/// Unique identifier for a DOM node
pub type NodeId = u64;

/// Unique identifier for a document
pub type DocumentId = u64;
```

#### src/lib.rs
Re-export all types:
```rust
pub mod error;
pub mod node_type;
pub mod ids;

pub use error::DomException;
pub use node_type::NodeType;
pub use ids::{NodeId, DocumentId};
```

### 5. Testing Requirements

**Coverage Target**: ≥80%

Test files required:
- `tests/unit/test_node_type.rs` - Test all NodeType values and methods
- `tests/unit/test_error.rs` - Test DomException types and display
- `tests/unit/test_ids.rs` - Test ID type behavior

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_element() {
        assert_eq!(NodeType::Element as u16, 1);
    }

    #[test]
    fn test_error_display() {
        let err = DomException::HierarchyRequestError;
        assert_eq!(err.to_string(), "Hierarchy request error");
    }

    #[test]
    fn test_serialization() {
        let node_type = NodeType::Element;
        let json = serde_json::to_string(&node_type).unwrap();
        assert!(json.contains("Element"));
    }
}
```

### 6. Quality Gates

Before marking complete, verify:
- ✅ All tests pass (100%)
- ✅ Coverage ≥ 80%
- ✅ `cargo clippy` passes (zero warnings)
- ✅ `cargo fmt --check` passes
- ✅ All types from spec implemented
- ✅ Proper documentation (rustdoc comments)

### 7. Git Workflow

```bash
# After implementation is complete and all tests pass
git add components/dom_types/
git commit -m "[dom_types] Initial implementation with NodeType, DomException, and ID types"
```

## Contract API (What This Component Exports)

### Public Types
```rust
// NodeType enum with all DOM node types
pub enum NodeType {
    Element = 1,
    Attribute = 2,
    Text = 3,
    // ... all types from spec
}

// DomException error types
pub enum DomException {
    HierarchyRequestError,
    WrongDocumentError,
    InvalidCharacterError,
    // ... all exceptions from spec
}

// ID type aliases
pub type NodeId = u64;
pub type DocumentId = u64;
```

### Required Traits
- `NodeType`: Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize
- `DomException`: Debug, Error (from thiserror), Clone, PartialEq

## Success Criteria

✅ Component is complete when:
1. All types from specification implemented
2. All unit tests pass (100%)
3. Test coverage ≥ 80%
4. Clippy and rustfmt pass
5. Documentation complete (rustdoc)
6. No compiler warnings
7. Can be imported by other components

## Estimated Effort
- **LOC**: ~500-1000 (small base component)
- **Time**: 2-4 hours

## Notes
- This is a foundational component with NO dependencies on other DOM components
- Keep it simple and focused on types only
- No business logic, just type definitions
- Make all types serializable (required for message bus integration later)
