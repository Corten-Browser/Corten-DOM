//! CSS selector matching and querying for DOM
//!
//! This crate provides querySelector, querySelectorAll, and related
//! selector matching functionality.

#![warn(missing_docs)]

pub mod query;
pub mod matcher;

// Re-exports
pub use query::{Selectable, SelectorQuery};
