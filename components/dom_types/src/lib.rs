//! Shared type definitions for DOM implementation
//!
//! This crate provides common types, enums, and error definitions
//! used across all DOM components.

#![warn(missing_docs)]

pub mod error;
pub mod node_type;
pub mod ids;

// Re-exports
pub use error::DomException;
pub use node_type::NodeType;
pub use ids::{NodeId, DocumentId};
