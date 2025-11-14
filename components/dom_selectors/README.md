# dom_selectors

**Type**: Feature Component
**Dependencies**: dom_core, dom_collections
**Version**: 0.1.0

## Purpose
CSS selector matching and querying.

## Key Features
- querySelector/querySelectorAll
- matches() method
- closest() method
- Selector parsing and matching

## Public API
```rust
pub trait Selectable {
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>>;
    fn query_selector_all(&self, selector: &str) -> Result<NodeList>;
}
```

See `CLAUDE.md` for detailed development instructions.
