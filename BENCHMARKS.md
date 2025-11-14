# Performance Benchmarks

This document describes the performance benchmark suite for the Corten-DOM project.

## Overview

The benchmark suite measures performance of critical DOM operations to ensure our implementation meets performance targets (within 2x of Chrome's DOM implementation).

## Benchmark Components

### dom_core Benchmarks

Located in `components/dom_core/benches/`:

#### 1. node_operations.rs
Measures basic node creation and manipulation:
- `createElement` - Creating elements with different tag names
- `createTextNode` - Creating text nodes with varying text sizes
- `createComment` - Creating comment nodes
- `cloneNode` (shallow) - Cloning elements without children
- `cloneNode` (deep) - Cloning elements with varying tree depths
- `nodeType` - Node type checking
- `contains` - Tree containment checks with varying depths

**Key Metrics**:
- createElement: < 1μs per element
- cloneNode (shallow): < 500ns
- cloneNode (deep, 10 levels): < 50μs

#### 2. tree_manipulation.rs
Measures DOM tree modification operations:
- `appendChild` - Adding children (1, 10, 50, 100 children)
- `removeChild` - Removing children from various positions
- `insertBefore` - Inserting at beginning, middle, end
- `childNodes` - Traversing children (10-500 children)
- `parentNode` - Traversing up the tree (various depths)
- `build_realistic_tree` - Building a complex DOM structure

**Key Metrics**:
- appendChild (single): < 2μs
- appendChild (100 children): < 200μs
- removeChild: < 1μs per removal
- insertBefore: < 5μs
- childNodes traversal: O(1) access time

#### 3. attribute_operations.rs
Measures attribute manipulation:
- `setAttribute` - Setting attributes with varying value sizes
- `getAttribute` - Reading attribute values
- `hasAttribute` - Checking attribute existence
- `removeAttribute` - Removing attributes
- `attributes` enumeration - Iterating over all attributes
- `classList` - Class list manipulation
- `id` operations - ID get/set
- `getAttributeNode` / `setAttributeNode` - Attr node operations

**Key Metrics**:
- setAttribute: < 500ns
- getAttribute: < 100ns
- hasAttribute: < 100ns
- classList update: < 1μs

#### 4. query_selectors.rs
Measures basic DOM querying:
- `getElementById` - Finding elements by ID (various document sizes)
- `getElementById` (miss) - Searching for non-existent IDs
- `getElementsByTagName` - Finding elements by tag
- `getElementsByClassName` - Finding elements by class

**Key Metrics**:
- getElementById (1000 elements): < 10μs
- getElementsByTagName: O(n) where n = number of elements
- getElementsByClassName: O(n) where n = number of elements

### dom_selectors Benchmarks

Located in `components/dom_selectors/benches/`:

#### selector_matching.rs
Measures CSS selector matching and querying:
- `querySelector` - Finding first match (ID, class, tag, combinators, complex)
- `querySelectorAll` - Finding all matches
- `matches` - Testing if element matches selector
- `closest` - Finding nearest ancestor matching selector
- `selector_parsing` - Parsing and compiling selectors
- `combinators` - Descendant, child, sibling selectors
- `pseudo_classes` - Pseudo-class matching (if implemented)

**Key Metrics**:
- querySelector (simple): < 10μs
- querySelector (complex): < 50μs
- querySelectorAll: O(n) where n = number of elements
- matches: < 1μs
- closest: < 5μs per level

## Running Benchmarks

### Run All Benchmarks

```bash
# Run all benchmarks in dom_core
cd components/dom_core
cargo bench

# Run all benchmarks in dom_selectors
cd components/dom_selectors
cargo bench
```

### Run Specific Benchmark

```bash
# Run only node_operations benchmark
cd components/dom_core
cargo bench --bench node_operations

# Run only a specific test within a benchmark
cargo bench --bench node_operations -- createElement
```

### Generate Detailed Reports

```bash
# Run with verbose output
cargo bench --bench node_operations -- --verbose

# Save results to file
cargo bench --bench node_operations > benchmark_results.txt
```

## Interpreting Results

Criterion produces output like:

```
createElement/div       time:   [892.45 ns 897.23 ns 902.51 ns]
                        change: [-2.3421% +0.1234% +2.1234%] (p = 0.45 > 0.05)
                        No change in performance detected.
```

This means:
- **Median time**: 897.23 ns
- **95% confidence interval**: [892.45 ns, 902.51 ns]
- **Change from previous run**: Between -2.34% and +2.12% (not statistically significant)

### Performance Targets

Our targets are based on Chrome's DOM performance:

| Operation | Target | Current | Status |
|-----------|--------|---------|--------|
| createElement | < 1μs | TBD | TBD |
| appendChild | < 2μs | TBD | TBD |
| setAttribute | < 500ns | TBD | TBD |
| getAttribute | < 100ns | TBD | TBD |
| querySelector (simple) | < 10μs | TBD | TBD |
| querySelector (complex) | < 50μs | TBD | TBD |

**Target**: Within 2x of Chrome performance

## Benchmark History

Results are stored in `target/criterion/` directory:

```
target/criterion/
├── createElement/
│   ├── base/
│   │   ├── benchmark.json
│   │   └── estimates.json
│   └── new/
│       ├── benchmark.json
│       └── estimates.json
└── report/
    └── index.html  # HTML report with charts
```

View the HTML report:
```bash
open target/criterion/report/index.html  # macOS
xdg-open target/criterion/report/index.html  # Linux
```

## Continuous Performance Monitoring

### Regression Detection

Criterion automatically detects performance regressions:
- Green: No significant change
- Yellow: Possible regression (borderline)
- Red: Significant regression detected

### Comparing Against Baseline

```bash
# Save current results as baseline
cargo bench --bench node_operations -- --save-baseline my-baseline

# Compare against baseline
cargo bench --bench node_operations -- --baseline my-baseline
```

## Performance Optimization Tips

Based on benchmark results:

1. **Hot paths identified**:
   - createElement
   - appendChild
   - getAttribute/setAttribute
   - querySelector

2. **Optimization opportunities**:
   - Intern commonly-used tag names and attribute names
   - Cache selector parsing results
   - Use IndexMap for attribute storage (insertion-order preservation)
   - Optimize parent pointer traversal

3. **Memory vs. Speed tradeoffs**:
   - String interning reduces memory but adds lookup overhead
   - Arc/RwLock adds thread-safety overhead
   - Trade-offs documented in ARCHITECTURE.md

## Adding New Benchmarks

To add a new benchmark:

1. Create a new file in `benches/`:
   ```bash
   touch components/dom_core/benches/my_benchmark.rs
   ```

2. Add to `Cargo.toml`:
   ```toml
   [[bench]]
   name = "my_benchmark"
   harness = false
   ```

3. Write benchmark using criterion:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   
   fn bench_my_feature(c: &mut Criterion) {
       c.bench_function("my_feature", |b| {
           b.iter(|| {
               // Code to benchmark
               black_box(my_function())
           });
       });
   }
   
   criterion_group!(benches, bench_my_feature);
   criterion_main!(benches);
   ```

4. Run the new benchmark:
   ```bash
   cargo bench --bench my_benchmark
   ```

## Chrome Comparison

To compare against Chrome:

1. Use Chrome DevTools Performance API:
   ```javascript
   performance.mark('start');
   document.createElement('div');
   performance.mark('end');
   performance.measure('createElement', 'start', 'end');
   console.log(performance.getEntriesByName('createElement')[0].duration);
   ```

2. Run multiple iterations and calculate median

3. Compare with our benchmark results

4. Document findings in this file

## Future Benchmarks

Planned benchmarks for future phases:

- Event dispatch performance
- Mutation observer notifications
- DOM serialization/parsing
- Large-scale tree operations (10k+ nodes)
- Memory usage benchmarks
- Multi-threaded DOM access patterns

## References

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [DOM Living Standard - Performance](https://dom.spec.whatwg.org/#performance)

---

Last Updated: 2025-11-14
Version: 0.9.0
