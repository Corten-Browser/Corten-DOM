# DOM Specification Gap Analysis
**Generated**: 2025-11-14
**Current Version**: v0.2.0
**Target**: Full DOM Level 4 Specification (dom-component-specification.md)

## Executive Summary

Current implementation: **~35-40% complete** relative to full specification
- Tests passing: 202/202 (100% of implemented features)
- Components functional: 8/8
- Missing: ~60-65% of specified APIs and features

---

## 1. Missing Node Types (CRITICAL)

### Specification Requires (lines 168-178):
```rust
pub enum NodeType {
    Element = 1,
    Attribute = 2,           // ❌ MISSING
    Text = 3,
    CDataSection = 4,        // ❌ MISSING
    ProcessingInstruction = 7,  // ❌ MISSING
    Comment = 8,
    Document = 9,
    DocumentType = 10,       // ❌ MISSING
    DocumentFragment = 11,
}
```

### Currently Implemented:
- ✅ Element
- ✅ Text
- ✅ Comment
- ✅ Document
- ✅ DocumentFragment

### MISSING Node Types (4):
1. **Attr** (Attribute nodes)
   - Location: Should be in `components/dom_core/src/attr.rs`
   - Dependencies: NodeData, NamedNodeMap
   - Tests needed: 15-20 unit tests

2. **ProcessingInstruction**
   - Location: Should be in `components/dom_core/src/processing_instruction.rs`
   - Dependencies: NodeData
   - Tests needed: 10-15 unit tests

3. **CDATASection**
   - Location: Should be in `components/dom_core/src/cdata_section.rs`
   - Dependencies: Text node (inherits from Text)
   - Tests needed: 10-15 unit tests

4. **DocumentType**
   - Location: Should be in `components/dom_core/src/document_type.rs`
   - Dependencies: NodeData
   - Tests needed: 10-15 unit tests

---

## 2. Missing Collections (CRITICAL)

### Specification Requires:
- HTMLCollection ✅ (implemented)
- NodeList ✅ (implemented)
- DOMTokenList ✅ (implemented)
- **NamedNodeMap** ❌ (MISSING)

### MISSING:
**NamedNodeMap** - For managing element attributes
- Location: Should be in `components/dom_collections/src/named_node_map.rs`
- Required methods:
  - `length()`, `item(index)`, `get_named_item(name)`
  - `set_named_item(attr)`, `remove_named_item(name)`
  - `get_named_item_ns(namespace, local_name)`
- Dependencies: Attr node type
- Tests needed: 20-25 unit tests

---

## 3. Missing Document Methods (HIGH PRIORITY)

### Specification Lines 270-291:

Currently Missing from `components/dom_core/src/document.rs`:

```rust
// Node adoption and importing (lines 274-275)
pub fn import_node(&mut self, node: NodeRef, deep: bool)
    -> Result<NodeRef, DomException>;  // ❌ MISSING
pub fn adopt_node(&mut self, node: NodeRef)
    -> Result<NodeRef, DomException>;  // ❌ MISSING

// Attribute creation (lines 270-271)
pub fn create_attribute(&mut self, name: &str)
    -> Result<AttrRef, DomException>;  // ❌ MISSING
pub fn create_attribute_ns(&mut self, namespace: Option<&str>, qualified_name: &str)
    -> Result<AttrRef, DomException>;  // ❌ MISSING

// Traversal (lines 290-291)
pub fn create_node_iterator(&self, root: NodeRef, what_to_show: u32,
    filter: Option<NodeFilter>) -> NodeIterator;  // ❌ MISSING (stub exists)
pub fn create_tree_walker(&self, root: NodeRef, what_to_show: u32,
    filter: Option<NodeFilter>) -> TreeWalker;  // ❌ MISSING (stub exists)

// Events (line 284)
pub fn create_event(&self, interface: &str)
    -> Result<EventRef, DomException>;  // ❌ MISSING

// Ranges (line 287)
pub fn create_range(&mut self) -> RangeRef;  // ❌ MISSING (stub exists in dom_advanced)
```

**Tests needed**: 30-40 unit tests for these methods

---

## 4. Missing Element Methods (HIGH PRIORITY)

### Specification Lines 212-238:

Currently Missing from `components/dom_core/src/element.rs`:

```rust
// Attribute node access (lines 212-213)
pub fn get_attribute_node(&self, name: &str)
    -> Option<AttrRef>;  // ❌ MISSING
pub fn set_attribute_node(&mut self, attr: AttrRef)
    -> Result<Option<AttrRef>, DomException>;  // ❌ MISSING

// Geometry (lines 235-237)
pub fn get_bounding_client_rect(&self) -> DOMRect;  // ❌ MISSING
pub fn get_client_rects(&self) -> DOMRectList;  // ❌ MISSING
pub fn scroll_into_view(&self, options: ScrollIntoViewOptions);  // ❌ MISSING
```

**Tests needed**: 25-30 unit tests

---

## 5. Stub Implementations Needing Completion (CRITICAL)

### NodeIterator (`components/dom_collections/src/node_iterator.rs`)
**Status**: Skeleton exists, all methods return `None` or have TODO comments
**Missing Implementation**:
```rust
pub fn next_node(&mut self) -> Option<NodeRef>;  // TODO - line 25
pub fn previous_node(&mut self) -> Option<NodeRef>;  // TODO - line 31
pub fn detach(&mut self);  // Missing entirely
```
**Tests needed**: 20-25 unit tests
**Specification**: Lines 58-61

### TreeWalker (`components/dom_collections/src/tree_walker.rs`)
**Status**: Skeleton exists, all navigation methods return `None`
**Missing Implementation**:
```rust
pub fn parent_node(&mut self) -> Option<NodeRef>;  // TODO - line 35
pub fn first_child(&mut self) -> Option<NodeRef>;  // TODO - line 41
pub fn last_child(&mut self) -> Option<NodeRef>;  // TODO - line 47
pub fn previous_sibling(&mut self) -> Option<NodeRef>;  // TODO - line 53
pub fn next_sibling(&mut self) -> Option<NodeRef>;  // TODO - line 59
pub fn previous_node(&mut self) -> Option<NodeRef>;  // TODO - line 65
pub fn next_node(&mut self) -> Option<NodeRef>;  // TODO - line 71
```
**Tests needed**: 30-35 unit tests
**Specification**: Lines 58-61

### Range API (`components/dom_advanced/src/range.rs`)
**Status**: Skeleton exists, minimal implementation
**Need to verify**: Full Range API implementation per specification

### Selection API (`components/dom_advanced/src/selection.rs`)
**Status**: Basic structure exists
**Need to verify**: Full Selection API implementation

---

## 6. Missing Event Types (MEDIUM PRIORITY)

### Specification Lines 55-56, 295-336:

Currently Implemented:
- ✅ Base `Event` structure

Missing Event Types:
```rust
pub struct UIEvent { /* extends Event */ }  // ❌ MISSING
pub struct MouseEvent { /* extends UIEvent */ }  // ❌ MISSING
pub struct KeyboardEvent { /* extends UIEvent */ }  // ❌ MISSING
pub struct WheelEvent { /* extends MouseEvent */ }  // ❌ MISSING
pub struct FocusEvent { /* extends UIEvent */ }  // ❌ MISSING
pub struct InputEvent { /* extends UIEvent */ }  // ❌ MISSING
pub struct CustomEvent { /* extends Event */ }  // ❌ MISSING
pub struct CompositionEvent { /* extends UIEvent */ }  // ❌ MISSING
```

**Location**: Should be in `components/dom_events/src/event_types/` directory
**Tests needed**: 50-60 unit tests (across all event types)
**Specification**: DOM Level 4 event types

---

## 7. Missing CSS Selector Features (MEDIUM PRIORITY)

### Currently Implemented (v0.2.0):
- ✅ Tag selectors (`div`)
- ✅ Class selectors (`.classname`)
- ✅ ID selectors (`#id`)
- ✅ Attribute selectors (`[type='text']`, `[disabled]`)
- ✅ Multiple class matching (`.btn.primary`)
- ✅ Combined selectors (`button.primary`)
- ✅ Universal selector (`*`)

### MISSING Selectors:
**CSS Combinators** (marked as `#[ignore]` in tests):
```rust
// Descendant combinator (space) - test_complex_selectors.rs:10
"div li"  // ❌ NOT IMPLEMENTED

// Child combinator (>) - test_complex_selectors.rs:32
"div > ul"  // ❌ NOT IMPLEMENTED

// Adjacent sibling combinator (+)
"h1 + p"  // ❌ NOT IMPLEMENTED

// General sibling combinator (~)
"h1 ~ p"  // ❌ NOT IMPLEMENTED

// Complex chains - test_complex_selectors.rs:127
"div > ul > li.item"  // ❌ NOT IMPLEMENTED
```

**Location**: `components/dom_selectors/src/matcher.rs` and `query.rs`
**Tests**: 3 tests currently ignored (lines 10, 32, 127 of test_complex_selectors.rs)
**Tests needed**: ~15-20 additional unit tests for all combinator types
**Specification**: Lines 224-228, 1140-1148

---

## 8. Missing Geometry APIs (MEDIUM PRIORITY)

### Specification Lines 235-237:

```rust
pub struct DOMRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}  // ❌ MISSING

pub struct DOMRectList {
    rects: Vec<DOMRect>,
}  // ❌ MISSING

pub struct ScrollIntoViewOptions {
    pub behavior: ScrollBehavior,
    pub block: ScrollLogicalPosition,
    pub inline: ScrollLogicalPosition,
}  // ❌ MISSING
```

**Location**: Should be in `components/dom_types/src/geometry.rs`
**Tests needed**: 20-25 unit tests
**Note**: These require integration with rendering engine (may need message bus communication)

---

## 9. Missing Validation Module (LOW PRIORITY)

### Specification Lines 88-92:

Complete validation module missing:
```
components/dom_validation/  // ❌ ENTIRE COMPONENT MISSING
├── src/
│   ├── namespace.rs        // Namespace validation
│   ├── qualified_name.rs   // Qualified name validation
│   └── html5.rs            // HTML5-specific validation
```

**Current state**: Basic validation exists inline in Element/Document
**Should be**: Separate validation component with comprehensive rules
**Tests needed**: 40-50 unit tests
**LOC estimate**: 1,500-2,000 lines

---

## 10. Missing Utilities Module (LOW PRIORITY)

### Specification Lines 98-102:

```
components/dom_utils/  // ❌ MISSING or MINIMAL
├── src/
│   ├── strings.rs         // Atom string interning (MISSING)
│   ├── namespace.rs       // Namespace utilities (PARTIAL)
│   └── ordering.rs        // Tree order comparisons (MISSING)
```

**Tests needed**: 30-35 unit tests
**LOC estimate**: 800-1,000 lines

---

## 11. Missing Interfaces (LOW PRIORITY)

### DOMImplementation Interface
**Status**: ❌ MISSING
**Specification Lines**: Not explicitly detailed but referenced
**Methods needed**:
```rust
pub fn create_document_type(...) -> DocumentType;
pub fn create_document(...) -> Document;
pub fn create_html_document(...) -> Document;
pub fn has_feature(...) -> bool;  // deprecated but required
```
**Tests needed**: 15-20 unit tests

---

## 12. Missing Shadow DOM Methods (MEDIUM PRIORITY)

### Current State:
- ✅ ShadowRoot structure exists (`components/dom_advanced/src/shadow/shadow_root.rs`)
- ✅ Slot structure exists (`components/dom_advanced/src/shadow/slot.rs`)

### MISSING:
**Element shadow attachment** (Specification line 231-232):
```rust
pub fn attach_shadow(&mut self, mode: ShadowRootMode)
    -> Result<ShadowRootRef, DomException>;  // ❌ MISSING from Element
pub fn shadow_root(&self) -> Option<ShadowRootRef>;  // ❌ MISSING from Element
```

**Slot distribution algorithm** (Specification line 83):
- File exists but implementation needs verification
- Tests needed: 20-25 integration tests

---

## 13. Web Platform Tests Integration (CRITICAL FOR v1.0)

### Specification Lines 974-1031:

**Status**: ❌ NOT IMPLEMENTED
**Required**:
- WPT test harness in `tests/wpt/`
- Target: 90%+ pass rate on WPT `dom/*` tests
- Test categories:
  - Node interface tests
  - Element interface tests
  - Document interface tests
  - Event tests
  - Collections tests
  - Selectors tests
  - MutationObserver tests

**Estimated WPT tests**: 500-1,000 tests
**Current pass rate**: 0% (not run)

---

## 14. Performance Benchmarks (MEDIUM PRIORITY)

### Specification Lines 1033-1107:

**Status**: ❌ NOT IMPLEMENTED
**Required**:
- Benchmark suite in `benches/dom_operations.rs`
- Target performance: Within 2x of Chrome on standard benchmarks
- Benchmark categories:
  - Node creation/destruction
  - Tree manipulation
  - Query selectors
  - Event dispatch
  - Attribute access
  - Collection iteration

**Estimated benchmarks**: 30-40 benchmarks

---

## 15. Missing Namespace Support

### Current State:
- ✅ Basic namespace support in Element (namespace_uri field)
- ✅ create_element_ns() method exists

### MISSING:
- ❌ Full namespace validation
- ❌ Namespace prefix handling
- ❌ XML namespace constants
- ❌ SVG namespace support
- ❌ MathML namespace support
- ❌ Qualified name parsing and validation

**Tests needed**: 30-40 unit tests

---

## Summary: Work Required

### Phase 1: Core Missing Features (v0.3.0)
**Priority**: CRITICAL
**Estimated effort**: 2-3 weeks
1. Implement 4 missing node types (Attr, ProcessingInstruction, CDATASection, DocumentType)
2. Implement NamedNodeMap
3. Complete NodeIterator and TreeWalker (currently stubs)
4. Add missing Document methods (import_node, adopt_node, create_attribute, create_attribute_ns)
5. Add missing Element methods (get_attribute_node, set_attribute_node)
6. **Tests**: ~150-200 new unit tests

### Phase 2: Event Types (v0.4.0)
**Priority**: HIGH
**Estimated effort**: 1-2 weeks
1. Implement full event type hierarchy (UIEvent, MouseEvent, KeyboardEvent, etc.)
2. Add create_event() to Document
3. **Tests**: ~50-60 new unit tests

### Phase 3: CSS Selectors Enhancement (v0.5.0)
**Priority**: MEDIUM
**Estimated effort**: 1-2 weeks
1. Implement all CSS combinators (descendant, child, sibling)
2. Un-ignore 3 failing tests in test_complex_selectors.rs
3. **Tests**: ~15-20 new unit tests

### Phase 4: Geometry & Advanced Features (v0.6.0)
**Priority**: MEDIUM
**Estimated effort**: 2-3 weeks
1. Implement DOMRect, DOMRectList
2. Implement scroll_into_view with options
3. Complete Range and Selection APIs
4. Add Shadow DOM attachment to Element
5. **Tests**: ~60-70 new unit tests

### Phase 5: Validation & Utilities (v0.7.0)
**Priority**: LOW (but needed for spec compliance)
**Estimated effort**: 1-2 weeks
1. Create validation component (namespace, qualified names, HTML5)
2. String interning (atoms)
3. Tree order comparisons
4. DOMImplementation interface
5. **Tests**: ~70-80 new unit tests

### Phase 6: Web Platform Tests (v0.8.0)
**Priority**: CRITICAL FOR v1.0
**Estimated effort**: 2-3 weeks
1. Set up WPT test harness
2. Run WPT dom/* tests
3. Fix failing tests
4. Target: 90%+ pass rate
5. **Tests**: 500-1,000 WPT tests

### Phase 7: Performance Benchmarks (v0.9.0)
**Priority**: MEDIUM
**Estimated effort**: 1-2 weeks
1. Create benchmark suite
2. Profile and optimize
3. Target: Within 2x of Chrome
4. **Benchmarks**: 30-40 benchmarks

### Phase 8: Final Polish (v1.0.0-rc1)
**Priority**: HIGH
**Estimated effort**: 1 week
1. Code review and refactoring
2. Documentation completion
3. API consistency check
4. Integration testing
5. Prepare for 1.0.0 release

---

## Totals

**Missing Features**: ~40-50 major features/APIs
**Missing Tests**: ~500-600 unit tests + 500-1,000 WPT tests
**Missing Code**: Estimated 15,000-20,000 LOC
**Current Completion**: ~35-40% of specification
**Total Estimated Effort**: 12-16 weeks (3-4 months)

**Critical Path**:
Phase 1 (Core) → Phase 2 (Events) → Phase 3 (Selectors) → Phase 6 (WPT) → Phase 8 (v1.0.0)

---

## Immediate Next Steps (v0.3.0)

1. ✅ Complete this gap analysis
2. Begin Phase 1 implementation:
   - Create Attr node type (dom_core/src/attr.rs)
   - Create ProcessingInstruction node type
   - Create CDATASection node type
   - Create DocumentType node type
   - Implement NamedNodeMap (dom_collections/)
   - Complete NodeIterator implementation
   - Complete TreeWalker implementation
   - Add Document methods (import_node, adopt_node, etc.)
   - Add Element attribute node methods
3. Write comprehensive tests for all new features (150-200 tests)
4. Run verification: Target 100% test pass rate
5. Update version to v0.3.0

**Goal**: Achieve 60-65% specification completion by end of Phase 1

