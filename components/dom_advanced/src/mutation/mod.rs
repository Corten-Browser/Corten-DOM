//! MutationObserver implementation
//!
//! Provides asynchronous mutation observation for DOM trees.

mod observer;
mod record;

pub use observer::MutationObserver;
pub use record::{MutationRecord, MutationType, MutationObserverInit};
