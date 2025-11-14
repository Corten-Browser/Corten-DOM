# Component: dom_storage

## Component Identification
- **Name**: dom_storage
- **Type**: Core (Level 1)
- **Version**: 0.1.0
- **Language**: Rust
- **Project Root**: /home/user/Corten-DOM
- **Component Root**: /home/user/Corten-DOM/components/dom_storage

## Responsibility
Provide thread-safe node storage, memory management, arena allocation, and garbage collection for DOM nodes.

## Dependencies
- **Internal**: dom-types
- **External**: `typed-arena`, `weak-table`, `parking_lot`, `crossbeam`, `tokio`

## Development Workflow

### 1. Read Specification
Read the main specification, focusing on:
- Thread-Safe Node Storage (lines 512-563)
- Memory Management (lines 1300-1339)
- NodeArena implementation details

### 2. Implement Arena Allocator (TDD)

**Key Components to Implement**:
1. **NodeArena** - Arena-based allocator with generational indices
2. **WeakNodeRef** - Weak reference management
3. **GarbageCollector** - Mark-and-sweep garbage collection

**Example Test (RED first)**:
```rust
#[test]
fn test_arena_allocate_and_retrieve() {
    let arena = NodeArena::new();
    let node_id = arena.allocate(create_test_node());
    let node = arena.get(node_id);
    assert!(node.is_some());
}

#[test]
fn test_arena_generation_prevents_use_after_free() {
    let arena = NodeArena::new();
    let node_id = arena.allocate(create_test_node());
    arena.deallocate(node_id);
    // Getting deallocated node should return None due to generation mismatch
    assert!(arena.get(node_id).is_none());
}
```

### 3. File Structure
```
src/
├── lib.rs              # Public exports
├── arena.rs            # NodeArena implementation
├── weak_refs.rs        # Weak reference management
└── gc.rs               # Garbage collector
```

### 4. Implementation Requirements

#### src/arena.rs
Implement arena from spec lines 512-563:
```rust
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use dom_types::NodeId;

pub struct NodeArena {
    nodes: Arc<RwLock<Vec<Option<NodeEntry>>>>,
    free_list: Arc<Mutex<Vec<usize>>>,
    generation: AtomicU64,
}

struct NodeEntry {
    node: Box<dyn Any + Send + Sync>,  // Will hold actual Node later
    generation: u64,
    strong_refs: AtomicUsize,
    weak_refs: AtomicUsize,
}

impl NodeArena {
    pub fn new() -> Self;
    pub fn with_capacity(capacity: usize) -> Self;
    pub fn allocate<T: Any + Send + Sync>(&self, node: T) -> NodeId;
    pub fn get(&self, id: NodeId) -> Option<Arc<dyn Any>>;
    pub fn deallocate(&self, id: NodeId);
    pub fn compact(&mut self);
    pub fn fragmentation(&self) -> f64;
}
```

#### src/gc.rs
Implement garbage collection from spec lines 1300-1339:
```rust
pub trait GarbageCollector {
    fn collect(&mut self);
    fn mark_reachable(&self, node_id: NodeId, reachable: &mut HashSet<NodeId>);
    fn sweep(&mut self, predicate: impl Fn(NodeId) -> bool);
}

impl GarbageCollector for NodeArena {
    fn collect(&mut self) {
        // Mark phase
        let mut reachable = HashSet::new();
        // Find all reachable nodes from roots

        // Sweep phase
        self.sweep(|node_id| !reachable.contains(&node_id));

        // Compact if needed
        if self.fragmentation() > 0.5 {
            self.compact();
        }
    }
}
```

### 5. Testing Requirements

**Coverage Target**: ≥80%

Required tests:
- `tests/unit/test_arena.rs` - Arena allocation/deallocation
- `tests/unit/test_weak_refs.rs` - Weak reference behavior
- `tests/unit/test_gc.rs` - Garbage collection
- `benches/arena_operations.rs` - Performance benchmarks

**Thread Safety Tests** (CRITICAL):
```rust
#[test]
fn test_concurrent_allocation() {
    let arena = Arc::new(NodeArena::new());
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let arena = arena.clone();
            std::thread::spawn(move || {
                for _ in 0..1000 {
                    let id = arena.allocate(format!("node_{}", i));
                    arena.get(id);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 6. Quality Gates
- ✅ All tests pass (100%)
- ✅ Coverage ≥ 80%
- ✅ Thread safety verified (no data races with TSAN)
- ✅ No memory leaks (valgrind or ASAN)
- ✅ `cargo clippy` passes
- ✅ `cargo fmt --check` passes
- ✅ Benchmarks run without panic

### 7. Performance Requirements
- Allocation: <100ns per node
- Deallocation: <100ns per node
- GC cycle (10k nodes): <10ms

### 8. Git Workflow
```bash
git add components/dom_storage/
git commit -m "[dom_storage] Implement thread-safe arena allocator and GC"
```

## Contract API

### Public Types
```rust
pub struct NodeArena {
    pub fn new() -> Self;
    pub fn allocate<T>(&self, node: T) -> NodeId;
    pub fn get(&self, id: NodeId) -> Option<Arc<dyn Any>>;
    pub fn deallocate(&self, id: NodeId);
}

pub trait GarbageCollector {
    fn collect(&mut self);
}

pub struct WeakNodeRef {
    pub fn upgrade(&self) -> Option<Arc<dyn Any>>;
}
```

## Success Criteria
1. Arena allocator works correctly
2. Thread-safe (concurrent allocation/deallocation)
3. GC correctly identifies and removes unreachable nodes
4. No memory leaks
5. Performance requirements met
6. All tests pass
7. Coverage ≥ 80%

## Estimated Effort
- **LOC**: ~1,500-2,000
- **Time**: 6-8 hours
