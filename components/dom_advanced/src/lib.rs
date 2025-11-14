//! Advanced DOM features
//!
//! This crate provides MutationObserver, Range, Selection, and Shadow DOM
//! functionality.

#![warn(missing_docs)]

pub mod mutation;
pub mod range;
pub mod selection;
pub mod shadow;

// Re-exports
pub use mutation::{MutationObserver, MutationRecord};
pub use range::Range;
pub use selection::Selection;
pub use shadow::{ShadowRoot, ShadowRootMode};
