//! Event system with dispatch and bubbling for DOM
//!
//! This crate provides the complete DOM event system including
//! event dispatch, bubbling, and capturing phases.

#![warn(missing_docs)]

pub mod event;
pub mod event_target;
pub mod event_dispatcher;
pub mod event_listener;

// Re-exports
pub use event::Event;
pub use event_target::EventTarget;
pub use event_dispatcher::EventDispatcher;
pub use event_listener::{EventListener, EventListenerObject};
