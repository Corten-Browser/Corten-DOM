//! Shared type definitions for DOM implementation
//!
//! This crate provides common types, enums, and error definitions
//! used across all DOM components.
//!
//! # Modules
//!
//! - [`error`] - DOM exception types
//! - [`geometry`] - DOMRect and DOMRectList for bounding boxes
//! - [`ids`] - NodeId and DocumentId type aliases
//! - [`mutation`] - MutationType enum for MutationObserver
//! - [`node_type`] - NodeType enum for DOM node types
//! - [`shadow_root`] - ShadowRootMode enum for shadow DOM
//! - [`string_map`] - DOMStringMap for element.dataset access

#![warn(missing_docs)]

pub mod error;
pub mod geometry;
pub mod ids;
pub mod mutation;
pub mod node_type;
pub mod shadow_root;
pub mod string_map;

// Re-exports
pub use error::DomException;
pub use geometry::{DOMRect, DOMRectList};
pub use ids::{DocumentId, NodeId};
pub use mutation::MutationType;
pub use node_type::NodeType;
pub use shadow_root::ShadowRootMode;
pub use string_map::DOMStringMap;
