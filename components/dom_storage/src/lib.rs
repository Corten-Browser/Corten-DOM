//! Thread-safe node storage and memory management for DOM
//!
//! This crate provides arena-based allocation and garbage collection
//! for DOM nodes.

#![warn(missing_docs)]

pub mod arena;
pub mod weak_refs;
pub mod gc;

// Re-exports
pub use arena::NodeArena;
