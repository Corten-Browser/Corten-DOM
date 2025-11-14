# CortenBrowser DOM Implementation - Architecture Plan

## Project Overview

**Goal**: Implement complete DOM Level 4 specification for CortenBrowser
**Total Estimated LOC**: 100,000-125,000 lines (including tests)
**Language**: Rust
**Architecture**: Multi-component with clear dependency hierarchy

## Component Architecture

### Design Principles

1. **Token Budget Compliance**: Each component stays under 70k tokens (optimal)
2. **Clear Boundaries**: Architectural boundaries follow specification modules
3. **Dependency Hierarchy**: Base → Core → Feature → Application (no circular deps)
4. **Parallel Development**: Independent components enable 7 concurrent agents
5. **Single Responsibility**: Each component has focused, well-defined purpose

### Component Breakdown (8 Components)

#### Level 0: Base Components

**1. dom_types** (Base, no dependencies)
- **Purpose**: Shared type definitions, enums, error types
- **Estimated LOC**: 3,000
- **Estimated Tokens**: 3,000 + 10,400 (CLAUDE.md) = 13,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - NodeType enum
  - DomException error types
  - Common type aliases
  - Constants and atom strings
- **Public Exports**:
  ```rust
  pub enum NodeType { Element, Text, Comment, Document, ... }
  pub enum DomException { HierarchyRequestError, ... }
  pub type NodeId = u64;
  pub type DocumentId = u64;
  ```

#### Level 1: Core Components

**2. dom_storage** (Core, deps: dom_types)
- **Purpose**: Thread-safe node storage, memory management
- **Estimated LOC**: 5,000
- **Estimated Tokens**: 5,000 + 10,400 = 15,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - NodeArena (arena allocator)
  - Weak reference management
  - Garbage collection
  - Memory optimization
- **Public Exports**:
  ```rust
  pub struct NodeArena { ... }
  pub struct WeakNodeRef { ... }
  pub trait GarbageCollector { ... }
  ```

**3. dom_core** (Core, deps: dom_types, dom_storage)
- **Purpose**: Core DOM nodes and tree operations
- **Estimated LOC**: 25,000 (Node, Element, Document, Text, basic ops)
- **Estimated Tokens**: 25,000 + 10,400 = 35,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - Node trait and base implementation
  - Element struct with attributes
  - Document struct with creation methods
  - Text, Comment, DocumentFragment nodes
  - Tree manipulation (append, remove, insert)
  - Node cloning and normalization
- **Public Exports**:
  ```rust
  pub trait Node { ... }
  pub struct Element { ... }
  pub struct Document { ... }
  pub struct Text { ... }
  pub type NodeRef = Arc<RefCell<Box<dyn Node>>>;
  ```

#### Level 2: Feature Components

**4. dom_collections** (Feature, deps: dom_core)
- **Purpose**: Live collections and tree traversal
- **Estimated LOC**: 14,000 (collections + traversal combined)
- **Estimated Tokens**: 14,000 + 10,400 = 24,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - HTMLCollection (live)
  - NodeList (live and static)
  - DOMTokenList
  - NamedNodeMap
  - NodeIterator
  - TreeWalker
  - NodeFilter
- **Public Exports**:
  ```rust
  pub struct HTMLCollection { ... }
  pub struct NodeList { ... }
  pub struct DOMTokenList { ... }
  pub struct NodeIterator { ... }
  pub struct TreeWalker { ... }
  ```

**5. dom_events** (Feature, deps: dom_core)
- **Purpose**: Complete event system with dispatch and bubbling
- **Estimated LOC**: 15,000
- **Estimated Tokens**: 15,000 + 10,400 = 25,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - Event interface
  - EventTarget trait
  - EventListener (function/object)
  - Event dispatcher with capture/bubble phases
  - Event types (UIEvent, MouseEvent, KeyboardEvent, etc.)
  - Event listener management
- **Public Exports**:
  ```rust
  pub struct Event { ... }
  pub trait EventTarget { ... }
  pub enum EventListener { ... }
  pub struct EventDispatcher { ... }
  ```

**6. dom_selectors** (Feature, deps: dom_core, dom_collections)
- **Purpose**: CSS selector matching and querying
- **Estimated LOC**: 8,000
- **Estimated Tokens**: 8,000 + 10,400 = 18,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - querySelector/querySelectorAll
  - matches() method
  - closest() method
  - Selector parsing (using cssparser crate)
  - Selector matching engine
- **Public Exports**:
  ```rust
  pub trait Selectable {
      fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>>;
      fn query_selector_all(&self, selector: &str) -> Result<NodeList>;
      fn matches(&self, selector: &str) -> Result<bool>;
      fn closest(&self, selector: &str) -> Result<Option<ElementRef>>;
  }
  ```

**7. dom_advanced** (Feature, deps: dom_core, dom_events)
- **Purpose**: Advanced features (MutationObserver, Range, Shadow DOM)
- **Estimated LOC**: 25,000 (mutation + ranges + shadow combined)
- **Estimated Tokens**: 25,000 + 10,400 = 35,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - MutationObserver implementation
  - MutationRecord and mutation queue
  - Range API (Range, boundary points)
  - Selection API
  - ShadowRoot implementation
  - Slot distribution
  - Event retargeting for shadow DOM
- **Public Exports**:
  ```rust
  pub struct MutationObserver { ... }
  pub struct MutationRecord { ... }
  pub struct Range { ... }
  pub struct Selection { ... }
  pub struct ShadowRoot { ... }
  ```

#### Level 3: Application Component

**8. dom_impl** (Application, deps: ALL)
- **Purpose**: Main library, message bus integration, public API
- **Estimated LOC**: 6,000
- **Estimated Tokens**: 6,000 + 10,400 = 16,400
- **Status**: ✅ Under 70k optimal
- **Contents**:
  - Public API re-exports
  - Message bus integration
  - Component configuration
  - Browser shell integration
  - DomComponent message handler
- **Public Exports**:
  ```rust
  // Re-exports from all components
  pub use dom_core::*;
  pub use dom_events::*;
  pub use dom_collections::*;
  // ... etc

  // Integration layer
  pub struct DomComponent { ... }
  pub enum DomComponentMessage { ... }
  pub enum DomComponentResponse { ... }
  ```

### Total Budget Analysis

| Component | Level | LOC | Tokens (w/ CLAUDE.md) | Status |
|-----------|-------|-----|----------------------|--------|
| dom_types | Base | 3k | 13.4k | ✅ Well under limit |
| dom_storage | Core | 5k | 15.4k | ✅ Well under limit |
| dom_core | Core | 25k | 35.4k | ✅ Under optimal (70k) |
| dom_collections | Feature | 14k | 24.4k | ✅ Well under limit |
| dom_events | Feature | 15k | 25.4k | ✅ Well under limit |
| dom_selectors | Feature | 8k | 18.4k | ✅ Well under limit |
| dom_advanced | Feature | 25k | 35.4k | ✅ Under optimal (70k) |
| dom_impl | Application | 6k | 16.4k | ✅ Well under limit |
| **TOTAL** | | **101k** | **184k** | ✅ All compliant |

**Notes**:
- Each component is under 70k optimal limit
- Largest components (dom_core, dom_advanced) are 35.4k tokens (~50% of optimal)
- Enables parallel development (7 agents available, 8 components)
- Total implementation: ~101k LOC (spec target: 100k-125k including tests)

## Dependency Graph

```
Level 0 (Base):
  dom_types

Level 1 (Core):
  dom_storage ──→ dom_types
  dom_core ──────→ dom_types, dom_storage

Level 2 (Feature):
  dom_collections ──→ dom_core
  dom_events ───────→ dom_core
  dom_selectors ────→ dom_core, dom_collections
  dom_advanced ─────→ dom_core, dom_events

Level 3 (Application):
  dom_impl ────────→ ALL
```

**Build Order** (for parallel development):
1. **Phase 1**: dom_types, dom_storage (2 parallel agents)
2. **Phase 2**: dom_core (1 agent, waits for Phase 1)
3. **Phase 3**: dom_collections, dom_events (2 parallel agents, wait for Phase 2)
4. **Phase 4**: dom_selectors, dom_advanced (2 parallel agents, wait for Phase 3)
5. **Phase 5**: dom_impl (1 agent, waits for all)

**Maximum Parallelism**: 2 agents in most phases, 4 components can be developed simultaneously in Phase 3-4.

## Technology Stack (Per Specification)

**Language**: Rust 2021 Edition

**Key Dependencies**:
- `tokio` - Async runtime
- `dashmap` - Concurrent HashMap
- `typed-arena` - Arena allocation
- `string_cache` - Atom strings
- `cssparser`, `selectors` - CSS selector parsing
- `html5ever`, `markup5ever` - HTML namespace/validation
- `parking_lot` - Fast mutexes
- `crossbeam`, `rayon` - Concurrency primitives
- `serde`, `serde_json` - Serialization
- `thiserror`, `anyhow` - Error handling
- `tracing` - Logging

**Test Dependencies**:
- `criterion` - Benchmarking
- `proptest` - Property testing
- `insta` - Snapshot testing
- `mockall` - Mocking

## Testing Strategy

### Test Organization
- **Unit Tests**: In each component's `tests/unit/` directory
- **Integration Tests**: In `dom_impl/tests/integration/`
- **Contract Tests**: Validate component interfaces work together
- **WPT Tests**: Web Platform Tests for standards compliance
- **Benchmarks**: Performance benchmarking suite

### Quality Gates (Per Component)
1. ✅ Unit tests: 100% pass rate
2. ✅ Test coverage: ≥80%
3. ✅ No memory leaks (valgrind)
4. ✅ Thread safety (TSAN)
5. ✅ Linting (clippy)
6. ✅ Formatting (rustfmt)

### Integration Quality Gates (dom_impl)
1. ✅ All contract tests pass: 100%
2. ✅ Integration tests pass: 100%
3. ✅ WPT dom/* tests: ≥90% pass rate
4. ✅ Benchmarks within 2x of Chrome baseline

## Contracts Between Components

### dom_types → (consumed by all)
```rust
// Provides shared types
pub enum NodeType { ... }
pub enum DomException { ... }
pub type NodeId = u64;
```

### dom_storage → dom_core
```rust
// Arena allocates nodes
pub struct NodeArena {
    pub fn allocate(&self, node: Box<dyn Node>) -> NodeId;
    pub fn get(&self, id: NodeId) -> Option<NodeRef>;
    pub fn deallocate(&self, id: NodeId);
}
```

### dom_core → dom_collections
```rust
// Provides base node operations for collections
pub trait Node {
    fn child_nodes(&self) -> Vec<NodeRef>;
    fn parent_node(&self) -> Option<NodeRef>;
    // ... etc
}

pub struct Element {
    pub fn get_elements_by_tag_name(&self, name: &str) -> Vec<ElementRef>;
    // ... etc
}
```

### dom_core → dom_events
```rust
// EventTarget trait implemented by Node types
pub trait EventTarget {
    fn add_event_listener(&mut self, type: &str, listener: EventListener);
    fn dispatch_event(&mut self, event: Event) -> Result<bool>;
}
```

### dom_collections + dom_core → dom_selectors
```rust
// Selector matching on elements and collections
pub trait Selectable {
    fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>>;
}
```

### dom_core + dom_events → dom_advanced
```rust
// MutationObserver observes Node changes
// Range operates on Node positions
// ShadowRoot extends Element
```

### ALL → dom_impl
```rust
// Integration layer re-exports all APIs
// Message bus handling
pub struct DomComponent {
    pub fn handle_message(&mut self, msg: DomComponentMessage) -> DomComponentResponse;
}
```

## Message Bus Integration (dom_impl only)

### Incoming Messages
- **From HTML Parser**: `ParsedDocument { root, doctype }`
- **From JavaScript**: `ScriptManipulation { operation, node_id, params }`
- **From CSS Engine**: `StyleInvalidation { node_id, reason }`
- **From Browser Shell**: `UserInteraction { event_type, target_id, data }`

### Outgoing Messages
- **To JavaScript**: `DomMutated { mutations, affected_nodes }`
- **To CSS Engine**: `DomTreeChanged { change_type, subtree, needs_style_recalc }`
- **To Rendering**: `LayoutInvalidated { root_id, invalidation_type }`
- **To Browser Shell**: `EventDispatched { event_id, prevented_default }`

## Development Phases

### Phase 1: Foundation (Weeks 1-2)
**Components**: dom_types, dom_storage, dom_core
**Deliverables**: Core DOM tree with basic node operations
**Agents**: 2 parallel (types+storage), then 1 (core)

### Phase 2: Collections & Events (Weeks 3-4)
**Components**: dom_collections, dom_events
**Deliverables**: Live collections, event system
**Agents**: 2 parallel

### Phase 3: Selectors & Advanced (Weeks 5-7)
**Components**: dom_selectors, dom_advanced
**Deliverables**: CSS selectors, MutationObserver, Range, Shadow DOM
**Agents**: 2 parallel

### Phase 4: Integration (Week 8)
**Components**: dom_impl
**Deliverables**: Message bus integration, complete API
**Agents**: 1

### Phase 5: Testing & Polish (Weeks 9-10)
**Activities**: WPT testing, benchmarking, optimization, documentation
**Agents**: All components for fixes

## Success Criteria

### Functional Requirements
- ✅ Full DOM Level 4 API implementation
- ✅ W3C standards compliance (WPT tests ≥90%)
- ✅ Thread-safe operations
- ✅ Event system with capture/bubble

### Quality Requirements
- ✅ Test coverage ≥80% per component
- ✅ Zero memory leaks
- ✅ Zero data races
- ✅ All integration tests pass (100%)

### Performance Requirements
- ✅ Element creation: <1μs
- ✅ Deep tree (1000 nodes): <50ms
- ✅ querySelector: <2ms for complex selectors
- ✅ Event dispatch: <10μs with 10 listeners

### Documentation Requirements
- ✅ Complete API documentation (rustdoc)
- ✅ Component README files
- ✅ Integration examples
- ✅ Architecture decision records (ADRs)

## Risk Mitigation

### Risk: Token Budget Overrun
- **Mitigation**: Component split already accounts for token limits
- **Contingency**: Further split large components (dom_core, dom_advanced) if needed

### Risk: Circular Dependencies
- **Mitigation**: Strict dependency hierarchy (Base→Core→Feature→App)
- **Contingency**: Extract shared types to dom_types if needed

### Risk: Integration Failures
- **Mitigation**: Contract-first development, contract tests before integration
- **Contingency**: Phase 4.5 contract validation catches issues early

### Risk: WPT Compliance
- **Mitigation**: Reference implementation (Servo) for comparison
- **Contingency**: Phase 5 dedicated to WPT fixes

---

**Architecture Version**: 1.0
**Date**: 2025-11-14
**Status**: Approved for implementation
