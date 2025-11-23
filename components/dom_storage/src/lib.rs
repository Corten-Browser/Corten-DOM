//! Thread-safe Node Storage and Memory Management for CortenBrowser DOM.
//!
//! This crate provides arena-based allocation and garbage collection
//! for DOM nodes, ensuring efficient memory usage and thread safety.
//!
//! # Overview
//!
//! The storage system provides:
//!
//! - **Thread-safe**: All operations are safe for concurrent access
//! - **Generational indices**: Prevents use-after-free bugs
//! - **Garbage collection**: Mark-and-sweep with automatic compaction
//! - **Weak references**: Non-owning references that don't prevent GC
//! - **High performance**: < 100ns allocation/deallocation, < 10ms GC for 10k nodes
//!
//! # Architecture
//!
//! ```text
//! +-------------------+
//! |    NodeArena      |  Arena-based allocator with generational indices
//! +-------------------+
//!          |
//!          v
//! +-------------------+
//! | GarbageCollector  |  Mark-and-sweep GC with compaction
//! +-------------------+
//!          |
//!          v
//! +-------------------+
//! |   WeakNodeRef     |  Non-owning references for cycle prevention
//! +-------------------+
//! ```
//!
//! # Quick Start
//!
//! ## Arena Allocation
//!
//! ```rust
//! use dom_storage::arena::NodeArena;
//!
//! // Create arena with default capacity
//! let arena = NodeArena::new();
//!
//! // Or with specific capacity
//! let arena = NodeArena::with_capacity(10000);
//!
//! // Allocate nodes
//! let id = arena.allocate("my_node".to_string());
//!
//! // Retrieve nodes
//! if let Some(node) = arena.get(id) {
//!     // Use node...
//! }
//!
//! // Deallocate
//! arena.deallocate(id);
//! ```
//!
//! ## Garbage Collection
//!
//! ```rust
//! use dom_storage::{NodeArena, GarbageCollector};
//!
//! let mut arena = NodeArena::new();
//!
//! // Allocate some nodes
//! let root = arena.allocate("root".to_string());
//! let orphan = arena.allocate("orphan".to_string());
//!
//! // Collect unreachable nodes (keeping root as GC root)
//! arena.collect(&[root]);
//!
//! // Only root survives
//! assert!(arena.get(root).is_some());
//! assert!(arena.get(orphan).is_none());
//! ```
//!
//! ## Weak References
//!
//! ```rust
//! use dom_storage::WeakNodeRef;
//!
//! // Weak references don't prevent garbage collection
//! // Use for parent pointers to avoid reference cycles
//! ```
//!
//! # Performance Characteristics
//!
//! | Operation | Complexity | Typical Time |
//! |-----------|------------|--------------|
//! | Allocate | O(1) | < 100ns |
//! | Deallocate | O(1) | < 100ns |
//! | Get | O(1) | < 50ns |
//! | GC (10k nodes) | O(n) | < 10ms |
//! | Compact | O(n) | < 5ms |
//!
//! # Generational Indices
//!
//! Each node ID contains a generation counter that prevents use-after-free:
//!
//! ```rust
//! use dom_storage::NodeArena;
//!
//! let arena = NodeArena::new();
//! let id = arena.allocate("test".to_string());
//!
//! // Deallocate the node
//! arena.deallocate(id);
//!
//! // The ID is now invalid due to generation mismatch
//! assert!(arena.get(id).is_none());
//!
//! // Even if the slot is reused, old IDs won't work
//! let new_id = arena.allocate("new".to_string());
//! assert!(arena.get(id).is_none()); // Old ID still invalid
//! ```
//!
//! # Thread Safety
//!
//! All arena operations are thread-safe:
//!
//! ```rust
//! use dom_storage::NodeArena;
//! use std::sync::Arc;
//!
//! let arena = Arc::new(NodeArena::new());
//!
//! // Safe to share across threads
//! let arena_clone = arena.clone();
//! std::thread::spawn(move || {
//!     let id = arena_clone.allocate("from_thread".to_string());
//! });
//! ```
//!
//! # Related Crates
//!
//! - [`dom_types`](../dom_types/index.html) - Type definitions
//! - [`dom_core`](../dom_core/index.html) - Uses storage for node management

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod arena;
pub mod gc;
pub mod weak_refs;

// Re-exports
pub use arena::{NodeArena, NodeId};
pub use gc::{GarbageCollector, GcStats, GcWithStats};
pub use weak_refs::WeakNodeRef;
