//! Advanced DOM features
//!
//! This crate provides MutationObserver, Range, Selection, Shadow DOM,
//! and Geometry functionality.

#![warn(missing_docs)]

pub mod mutation;
pub mod range;
pub mod selection;
pub mod shadow;
pub mod geometry;
pub mod element_ext;

// Re-exports
pub use mutation::{MutationObserver, MutationRecord};
pub use range::Range;
pub use selection::Selection;
pub use shadow::{ShadowRoot, ShadowRootMode};
pub use geometry::{
    DOMRect, DOMRectReadOnly, DOMRectList, DOMRectListRef,
    ScrollIntoViewOptions, ScrollBehavior, ScrollLogicalPosition,
};
pub use element_ext::{
    ElementGeometryExt,
    get_default_bounding_rect, get_default_client_rects, perform_scroll_into_view,
};
