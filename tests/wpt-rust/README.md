# WPT-Inspired Rust Tests

This directory contains Rust tests translated from the Web Platform Tests (WPT) test suite.

## Purpose

The official WPT test suite is written in JavaScript for browser environments. These tests translate critical WPT scenarios to native Rust tests while preserving test intent and assertions.

## Structure

```
wpt-rust/
├── nodes/           # Node interface tests (dom/nodes/)
├── events/          # Event tests (dom/events/)
├── collections/     # Collection tests (dom/collections/)
├── mutation_observer/  # MutationObserver tests
└── README.md        # This file
```

## Test Naming Convention

Each test file documents its WPT source:

```rust
/// WPT: dom/nodes/Node-appendChild.html
/// Test: appendChild should set parent-child relationships
#[test]
fn test_append_child_sets_relationships() {
    // ...
}
```

## Running Tests

```bash
# Run all WPT-inspired tests
cargo test --test wpt_*

# Run specific category
cargo test --test wpt_nodes

# Run with output
cargo test --test wpt_nodes -- --nocapture
```

## Coverage Tracking

See `docs/wpt/WPT_TEST_MAPPING.md` for complete mapping of:
- WPT test → Rust test
- Test status (passing/failing)
- Coverage percentages

## Contributing New Tests

When adding new WPT-inspired tests:

1. Review original WPT test source
2. Understand test assertions and intent
3. Translate to idiomatic Rust
4. Document WPT source in test comments
5. Update WPT_TEST_MAPPING.md

## Status

**Phase**: v0.8.0 (in progress)
**Test Count**: 0 (starting)
**Target**: 50-100 translated scenarios
**Pass Rate Goal**: ≥ 90%

---

Last Updated: 2025-11-14
