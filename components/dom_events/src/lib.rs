//! Event system with dispatch and bubbling for DOM
//!
//! This crate provides the complete DOM event system including
//! event dispatch, bubbling, and capturing phases.

#![warn(missing_docs)]
#![allow(dead_code)] // Allow during development

pub mod event;
pub mod event_dispatcher;
pub mod event_listener;
pub mod event_target;

// Re-exports
pub use event::{Event, EventInit, EventPhase, EventRef, EventTargetRef as EventTarget};
pub use event_dispatcher::EventDispatcher;
pub use event_listener::{
    AddEventListenerOptions, EventListener, EventListenerObject, EventListenerOptions,
};
pub use event_target::{EventListenerRegistry, EventTargetData, EventTargetRef};
