//! DOM Implementation for CortenBrowser
//!
//! This is the main library that re-exports all DOM functionality
//! and provides integration with the browser's message bus.

#![warn(missing_docs)]

// Re-export all public APIs from component crates
pub use dom_types::*;
pub use dom_storage::*;
pub use dom_core::*;
pub use dom_collections::*;
pub use dom_events::*;
pub use dom_selectors::*;
pub use dom_advanced::*;

pub mod config;
pub mod messages;

// Re-exports
pub use config::DomConfig;
pub use messages::{DomComponentMessage, DomComponentResponse};
