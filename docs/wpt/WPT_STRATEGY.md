# Web Platform Tests (WPT) Strategy for Corten-DOM

**Version**: 0.8.0
**Date**: 2025-11-14
**Status**: Planning Phase

## Overview

Web Platform Tests (WPT) are the W3C's official conformance test suite for web platform specifications. This document outlines the strategy for integrating WPT-style testing into Corten-DOM, a Rust-based DOM implementation.

## Challenge: JavaScript Tests vs Rust Implementation

The official WPT test suite is written in JavaScript and designed to run in browser environments. Corten-DOM is implemented in Rust, creating a mismatch:

**WPT Tests**: JavaScript (`.html`, `.js` files)
**Corten-DOM**: Rust implementation

## Integration Approaches

### Approach 1: WASM Bindings + Browser Testing (Long-term)

**Description**: Compile Corten-DOM to WebAssembly and run official WPT tests in a browser environment.

**Pros**:
- Tests actual WPT test suite
- Highest confidence in spec compliance
- Tests real browser integration scenarios

**Cons**:
- Requires complete WASM bindings (significant work)
- Requires browser automation (Selenium/WebDriver)
- Complex infrastructure
- Estimated effort: 3-4 weeks

**Status**: DEFERRED (post-v1.0.0)

---

### Approach 2: Test Scenario Translation (Recommended for v0.8.0)

**Description**: Manually translate critical WPT test scenarios to Rust test cases, preserving test intent and assertions.

**Pros**:
- Works with pure Rust implementation
- No WASM/browser dependency
- Can start immediately
- Validates core behavior against spec

**Cons**:
- Manual translation effort
- Not testing exact WPT suite
- May miss edge cases

**Status**: RECOMMENDED for Phase 6

**Implementation**:
1. Identify high-priority WPT test categories
2. Review WPT test source
3. Translate test scenarios to Rust
4. Validate behavior matches expected results
5. Document mapping: WPT test → Rust test

---

### Approach 3: Rust Test Harness with WPT Structure (Hybrid)

**Description**: Create Rust tests that mirror WPT test structure and naming, making future WASM integration easier.

**Pros**:
- Prepares for future WASM integration
- Tests in Rust natively
- Clear mapping to WPT tests

**Cons**:
- Still requires translation
- More complex test structure

**Status**: HYBRID approach with Approach 2

---

## Phase 6 Implementation Plan (v0.8.0)

### Goals

1. ✅ Set up WPT-inspired test infrastructure
2. 📋 Identify critical WPT test categories
3. 📋 Translate 50-100 WPT scenarios to Rust tests
4. 📋 Achieve 90%+ pass rate on translated tests
5. 📋 Document test coverage mapping

### Test Categories (Priority Order)

Based on DOM Level 4 specification and WPT test suite:

| Priority | Category | WPT Path | Estimated Tests | Status |
|----------|----------|----------|-----------------|--------|
| 🔴 HIGH | Node Interface | `dom/nodes/` | 20-30 | ⏳ Not started |
| 🔴 HIGH | Element Interface | `dom/nodes/Element-*` | 15-25 | ⏳ Not started |
| 🔴 HIGH | Document Interface | `dom/nodes/Document-*` | 15-20 | ⏳ Not started |
| 🟡 MEDIUM | ParentNode | `dom/nodes/ParentNode-*` | 10-15 | ⏳ Not started |
| 🟡 MEDIUM | ChildNode | `dom/nodes/ChildNode-*` | 8-12 | ⏳ Not started |
| 🟡 MEDIUM | Events | `dom/events/` | 15-20 | ⏳ Not started |
| 🟢 LOW | Collections | `dom/collections/` | 10-15 | ⏳ Not started |
| 🟢 LOW | MutationObserver | `dom/mutation-observer/` | 8-12 | ⏳ Not started |

**Total Tests**: ~100-150 translated scenarios

### Test Translation Process

For each WPT test category:

1. **Review Official WPT Tests**
   - Clone WPT repository: `https://github.com/web-platform-tests/wpt.git`
   - Navigate to relevant category (e.g., `dom/nodes/`)
   - Review `.html` test files

2. **Identify Test Scenarios**
   - Extract test assertions
   - Note edge cases
   - Understand spec requirements

3. **Translate to Rust**
   - Create equivalent Rust test in `tests/wpt-rust/<category>/`
   - Preserve test intent
   - Use Rust idioms (avoid JavaScript patterns)
   - Document WPT source in test comments

4. **Validate Behavior**
   - Run Rust test
   - Compare behavior to WPT expected results
   - Fix any discrepancies in implementation

5. **Document Mapping**
   - Maintain mapping: `WPT_TEST_MAPPING.md`
   - Link Rust test → Original WPT test
   - Note any intentional differences

### Example Translation

**Original WPT Test** (`dom/nodes/Node-appendChild.html`):

```javascript
test(() => {
  const parent = document.createElement("div");
  const child = document.createElement("span");
  parent.appendChild(child);
  assert_equals(parent.firstChild, child);
  assert_equals(child.parentNode, parent);
}, "appendChild should set parent-child relationships");
```

**Translated Rust Test** (`tests/wpt-rust/nodes/test_node_append_child.rs`):

```rust
/// WPT: dom/nodes/Node-appendChild.html
/// Test: appendChild should set parent-child relationships
#[test]
fn test_append_child_sets_relationships() {
    let mut doc = Document::new();
    let parent = doc.create_element("div").unwrap();
    let child = doc.create_element("span").unwrap();

    parent.write().append_child(child.clone()).unwrap();

    // Verify parent-child relationships
    let first_child = parent.read().first_child().unwrap();
    assert!(Arc::ptr_eq(&first_child, &child));

    let child_parent = child.read().parent_node().unwrap();
    assert!(Arc::ptr_eq(&child_parent, &parent));
}
```

### Directory Structure

```
tests/wpt-rust/
├── nodes/
│   ├── test_node_append_child.rs
│   ├── test_node_insert_before.rs
│   ├── test_node_remove_child.rs
│   ├── test_element_attributes.rs
│   └── test_document_methods.rs
├── events/
│   ├── test_event_dispatch.rs
│   ├── test_event_bubbling.rs
│   └── test_event_capturing.rs
├── collections/
│   ├── test_html_collection.rs
│   └── test_node_list.rs
├── mutation_observer/
│   └── test_mutation_observer.rs
└── README.md

docs/wpt/
├── WPT_STRATEGY.md (this file)
├── WPT_TEST_MAPPING.md (test coverage mapping)
└── WPT_INTEGRATION.md (future WASM integration plan)
```

### Success Criteria

**Phase 6 Completion Requirements**:

- ✅ WPT test infrastructure created
- ✅ 50-100 WPT scenarios translated to Rust
- ✅ 90%+ pass rate on translated tests
- ✅ Test mapping documented
- ✅ Remaining work identified for future phases

**Metrics**:
- **Test Coverage**: 50-100 WPT scenarios
- **Pass Rate**: ≥ 90%
- **Categories Covered**: Node, Element, Document, Events (high priority)
- **Documentation**: Complete mapping and strategy docs

---

## Implementation Timeline

### Week 1: Node Interface Tests
- **Days 1-2**: Review WPT `dom/nodes/` tests
- **Days 3-5**: Translate Node interface tests (20-30 tests)
- **Day 6-7**: Fix failures, document coverage

### Week 2: Element & Document Tests
- **Days 1-3**: Translate Element interface tests (15-25 tests)
- **Days 4-6**: Translate Document interface tests (15-20 tests)
- **Day 7**: Integration testing, documentation

### Week 3: Events & Collections
- **Days 1-3**: Translate Event tests (15-20 tests)
- **Days 4-5**: Translate Collection tests (10-15 tests)
- **Day 6-7**: Final testing, achieve 90%+ pass rate, complete documentation

**Total Estimated Effort**: 2-3 weeks for Phase 6 completion

---

## Future Work (Post-v1.0.0)

### WASM Bindings + Full WPT Integration

Once core specification compliance is validated with translated tests:

1. **WASM Bindings**: Create `wasm-bindgen` bindings for all DOM APIs
2. **Browser Testing**: Set up WebDriver/Selenium automation
3. **Full WPT Run**: Execute official WPT test suite against WASM build
4. **Continuous Integration**: Integrate WPT into CI/CD pipeline

**Estimated Effort**: 4-6 weeks
**Target Version**: v2.0.0 or later

---

## Resources

### WPT Repository
- **GitHub**: https://github.com/web-platform-tests/wpt
- **Documentation**: https://web-platform-tests.org/
- **DOM Tests**: `wpt/dom/`

### Specification References
- **DOM Level 4**: https://dom.spec.whatwg.org/
- **CSSOM View**: https://drafts.csswg.org/cssom-view/
- **UI Events**: https://w3c.github.io/uievents/

### Testing Tools
- **Rust Testing**: Built-in `cargo test`
- **Future WASM**: `wasm-bindgen-test`
- **Future Browser**: Selenium WebDriver

---

## Conclusion

Phase 6 (v0.8.0) takes a pragmatic approach to WPT integration by:
1. Translating critical WPT scenarios to native Rust tests
2. Validating core behavior against spec requirements
3. Building foundation for future WASM+WPT integration

This approach provides high confidence in spec compliance while working within the constraints of a Rust-native implementation.

**Next Steps**:
1. Begin Node interface test translation
2. Set up test harness in `tests/wpt-rust/`
3. Document test coverage mapping
4. Achieve 90%+ pass rate

---

**Status**: Strategy Complete - Ready for Implementation
**Target**: v0.8.0
**Date**: 2025-11-14
