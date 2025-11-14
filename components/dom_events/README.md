# dom_events

**Type**: Feature Component
**Dependencies**: dom_core
**Version**: 0.1.0

## Purpose
Event system with dispatch and bubbling.

## Key Features
- Event interface
- EventTarget trait
- Event dispatch algorithm
- Capture/bubble phases

## Public API
```rust
pub struct Event { ... }
pub trait EventTarget { ... }
pub struct EventDispatcher { ... }
```

See `CLAUDE.md` for detailed development instructions.
