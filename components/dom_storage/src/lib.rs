//! Thread-safe node storage and memory management for DOM
//!
//! This crate provides arena-based allocation and garbage collection
//! for DOM nodes with the following features:
//!
//! - **Thread-safe**: All operations are safe for concurrent access
//! - **Generational indices**: Prevents use-after-free bugs
//! - **Garbage collection**: Mark-and-sweep with automatic compaction
//! - **Weak references**: Non-owning references that don't prevent GC
//! - **High performance**: < 100ns allocation/deallocation, < 10ms GC for 10k nodes
//!
//! # Examples
//!
//! ```
//! use dom_storage::arena::NodeArena;
//! use dom_storage::gc::GarbageCollector;
//!
//! // Create arena
//! let mut arena = NodeArena::new();
//!
//! // Allocate nodes
//! let id1 = arena.allocate("node1".to_string());
//! let id2 = arena.allocate("node2".to_string());
//! let id3 = arena.allocate("node3".to_string());
//!
//! // Garbage collect (keeping id1 as root)
//! arena.collect(&[id1]);
//!
//! // Only id1 remains
//! assert!(arena.get(id1).is_some());
//! assert!(arena.get(id2).is_none());
//! assert!(arena.get(id3).is_none());
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod arena;
pub mod gc;
pub mod weak_refs;

// Re-exports
pub use arena::{NodeArena, NodeId};
pub use gc::{GarbageCollector, GcStats, GcWithStats};
pub use weak_refs::WeakNodeRef;
