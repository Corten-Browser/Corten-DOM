//! Advanced DOM Features for CortenBrowser.
//!
//! This crate provides advanced DOM APIs including MutationObserver,
//! Range, Selection, Shadow DOM, and Element geometry functionality.
//!
//! # Overview
//!
//! The advanced features include:
//!
//! - **MutationObserver**: Watch for DOM changes asynchronously
//! - **Range**: Select and manipulate document regions
//! - **Selection**: Handle user text selections
//! - **Shadow DOM**: Encapsulated DOM subtrees
//! - **Geometry**: Element bounding boxes and scroll methods
//!
//! # MutationObserver
//!
//! Monitor DOM changes asynchronously:
//!
//! ```rust
//! use dom_advanced::{MutationObserver, MutationRecord};
//!
//! // Create observer with callback
//! // let observer = MutationObserver::new(|records, _observer| {
//! //     for record in records {
//! //         println!("Mutation: {:?}", record.mutation_type());
//! //     }
//! // });
//!
//! // Observe a target node
//! // observer.observe(target, MutationObserverInit {
//! //     child_list: true,
//! //     attributes: true,
//! //     subtree: true,
//! //     ..Default::default()
//! // });
//! ```
//!
//! ## Mutation Types
//!
//! | Type | Description |
//! |------|-------------|
//! | `childList` | Child nodes added/removed |
//! | `attributes` | Attribute value changed |
//! | `characterData` | Text content changed |
//!
//! # Range API
//!
//! Select and manipulate document regions:
//!
//! ```rust
//! use dom_advanced::Range;
//!
//! // Create a range
//! // let range = document.create_range();
//!
//! // Set boundaries
//! // range.set_start(start_node, 0);
//! // range.set_end(end_node, 5);
//!
//! // Extract or manipulate content
//! // let fragment = range.extract_contents();
//! // range.delete_contents();
//! ```
//!
//! ## Range Methods
//!
//! | Method | Description |
//! |--------|-------------|
//! | `set_start` / `set_end` | Set range boundaries |
//! | `collapse` | Collapse to start or end |
//! | `extract_contents` | Extract content to fragment |
//! | `delete_contents` | Remove content |
//! | `clone_contents` | Copy content to fragment |
//!
//! # Selection API
//!
//! Handle user text selections:
//!
//! ```rust
//! use dom_advanced::Selection;
//!
//! // Get window selection
//! // let selection = window.get_selection();
//!
//! // Work with ranges
//! // selection.add_range(range);
//! // selection.remove_all_ranges();
//! ```
//!
//! # Shadow DOM
//!
//! Create encapsulated DOM subtrees:
//!
//! ```rust
//! use dom_advanced::{ShadowRoot, ShadowRootMode};
//!
//! // Attach shadow root to element
//! // let shadow = element.attach_shadow(ShadowRootMode::Open);
//!
//! // Access shadow root
//! // if let Some(shadow) = element.shadow_root() {
//! //     // Add content to shadow DOM
//! // }
//! ```
//!
//! ## Shadow Root Modes
//!
//! | Mode | Description |
//! |------|-------------|
//! | `Open` | Shadow root accessible via `element.shadowRoot` |
//! | `Closed` | Shadow root not accessible externally |
//!
//! # Element Geometry
//!
//! Get element dimensions and positions:
//!
//! ```rust
//! use dom_advanced::{DOMRect, ElementGeometryExt, ScrollIntoViewOptions};
//!
//! // Get bounding rectangle
//! // let rect: DOMRect = element.get_bounding_client_rect();
//! // println!("Width: {}, Height: {}", rect.width(), rect.height());
//!
//! // Scroll element into view
//! // element.scroll_into_view(ScrollIntoViewOptions::default());
//! ```
//!
//! ## DOMRect Properties
//!
//! | Property | Description |
//! |----------|-------------|
//! | `x`, `y` | Position relative to viewport |
//! | `width`, `height` | Element dimensions |
//! | `top`, `right`, `bottom`, `left` | Edge positions |
//!
//! # Related Crates
//!
//! - [`dom_core`](../dom_core/index.html) - Core DOM nodes
//! - [`dom_events`](../dom_events/index.html) - Event system
//! - [`dom_types`](../dom_types/index.html) - Type definitions

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
