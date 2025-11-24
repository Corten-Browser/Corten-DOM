//! Shared type definitions for CortenBrowser DOM implementation.
//!
//! This crate provides common types, enums, and error definitions
//! used across all DOM components. It serves as the foundational
//! type system for the entire DOM implementation.
//!
//! # Overview
//!
//! `dom_types` is a base-level crate with no internal dependencies,
//! providing type definitions that ensure consistency across all
//! DOM components.
//!
//! # Core Types
//!
//! ## Node Types
//!
//! The [`NodeType`] enum represents all DOM node types:
//!
//! ```rust
//! use dom_types::NodeType;
//!
//! let node_type = NodeType::Element;
//! assert_eq!(node_type as u16, 1);
//! ```
//!
//! ## Error Handling
//!
//! The [`DomException`] enum provides all DOM exception types:
//!
//! ```rust
//! use dom_types::DomException;
//!
//! fn validate_name(name: &str) -> Result<(), DomException> {
//!     if name.is_empty() {
//!         return Err(DomException::InvalidCharacterError);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Geometry
//!
//! [`DOMRect`] and [`DOMRectList`] for bounding box calculations:
//!
//! ```rust
//! use dom_types::{DOMRect, DOMRectList};
//!
//! let rect = DOMRect::new(0.0, 0.0, 100.0, 50.0);
//! assert_eq!(rect.width, 100.0);
//! assert_eq!(rect.height, 50.0);
//! ```
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`error`] | DOM exception types (`DomException`) |
//! | [`geometry`] | `DOMRect` and `DOMRectList` for bounding boxes |
//! | [`ids`] | `NodeId` and `DocumentId` type aliases |
//! | [`mutation`] | `MutationType` enum for MutationObserver |
//! | [`node_type`] | `NodeType` enum for DOM node types |
//! | [`shadow_root`] | `ShadowRootMode` enum for shadow DOM |
//! | [`string_map`] | `DOMStringMap` for element.dataset access |
//!
//! # Feature Flags
//!
//! All types derive `serde::Serialize` and `serde::Deserialize` for
//! serialization support, enabling message bus communication between
//! browser components.

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
