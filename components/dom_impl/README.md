# browser-dom-impl

**Type**: Application Component
**Dependencies**: ALL DOM components
**Version**: 0.1.0

## Purpose
Main library for DOM implementation with browser integration.

## Key Features
- Public API re-exports
- Message bus integration
- Configuration
- Browser shell integration

## Usage
```rust
use browser_dom_impl::*;

let doc = Document::new();
let element = doc.create_element("div")?;
```

See `CLAUDE.md` for detailed development instructions.
