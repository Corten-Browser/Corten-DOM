//! Shared type definitions for DOM implementation
//!
//! This crate provides common types, enums, and error definitions
//! used across all DOM components.

#![warn(missing_docs)]

pub mod error;
pub mod ids;
pub mod node_type;

// Re-exports
pub use error::DomException;
pub use ids::{DocumentId, NodeId};
pub use node_type::NodeType;
