//! DOM Event System for CortenBrowser.
//!
//! This crate provides the complete DOM event system including event dispatch,
//! bubbling, capturing phases, and event listener management.
//!
//! # Overview
//!
//! The event system implements the DOM Level 4 event model with:
//!
//! - **Event Dispatch**: Three-phase event propagation (capture, target, bubble)
//! - **Event Types**: UI, Mouse, Keyboard, Focus, Input, Wheel, Custom events
//! - **Listener Management**: Add/remove event listeners with options
//! - **Event Creation**: Factory methods for creating typed events
//!
//! # Quick Start
//!
//! ```rust
//! use dom_events::{Event, EventInit, EventDispatcher};
//!
//! // Create an event
//! let event = Event::new("click", EventInit {
//!     bubbles: true,
//!     cancelable: true,
//!     ..Default::default()
//! });
//!
//! // Events can be dispatched to targets
//! // target.dispatch_event(event)?;
//! ```
//!
//! # Event Types
//!
//! | Type | Description | Common Events |
//! |------|-------------|---------------|
//! | [`Event`] | Base event type | `load`, `error` |
//! | [`UIEvent`] | User interface events | `resize`, `scroll` |
//! | [`MouseEvent`] | Mouse interactions | `click`, `mouseover` |
//! | [`KeyboardEvent`] | Keyboard input | `keydown`, `keyup` |
//! | [`FocusEvent`] | Focus changes | `focus`, `blur` |
//! | [`InputEvent`] | Input changes | `input`, `beforeinput` |
//! | [`WheelEvent`] | Scroll wheel | `wheel` |
//! | [`CustomEvent`] | Custom data events | user-defined |
//!
//! # Event Phases
//!
//! Events propagate through three phases:
//!
//! 1. **Capture Phase**: Event travels from window to target
//! 2. **Target Phase**: Event reaches the target element
//! 3. **Bubble Phase**: Event travels from target back to window
//!
//! ```text
//! Window -> Document -> Body -> Target -> Body -> Document -> Window
//!          [------Capture------]  |  [--------Bubble--------]
//!                               Target
//! ```
//!
//! # Event Listeners
//!
//! ```rust
//! use dom_events::{EventListener, AddEventListenerOptions};
//!
//! // Create listener options
//! let options = AddEventListenerOptions {
//!     capture: false,  // Listen during bubble phase
//!     once: true,      // Remove after first invocation
//!     passive: false,  // Allow preventDefault()
//! };
//!
//! // Listeners can be functions or objects implementing EventListenerObject
//! ```
//!
//! # Document Event Creation
//!
//! Use the `DocumentEventExt` trait for creating events:
//!
//! ```rust
//! use dom_events::{create_mouse_event, MouseEventInit};
//!
//! let event = create_mouse_event("click", MouseEventInit {
//!     screen_x: 100,
//!     screen_y: 200,
//!     ..Default::default()
//! });
//! ```
//!
//! # Related Crates
//!
//! - [`dom_core`](../dom_core/index.html) - Core DOM nodes
//! - [`dom_types`](../dom_types/index.html) - Type definitions

#![warn(missing_docs)]
#![allow(dead_code)] // Allow during development

pub mod event;
pub mod event_dispatcher;
pub mod event_listener;
pub mod event_target;
pub mod event_types;
pub mod document_ext;

// Re-exports
pub use event::{Event, EventInit, EventPhase, EventRef, EventTargetRef as EventTarget};
pub use event_dispatcher::EventDispatcher;
pub use event_listener::{
    AddEventListenerOptions, EventListener, EventListenerObject, EventListenerOptions,
};
pub use event_target::{EventListenerRegistry, EventTargetData, EventTargetRef};
pub use event_types::{
    UIEvent, UIEventInit, UIEventRef,
    MouseEvent, MouseEventInit, MouseEventRef, MouseButton,
    KeyboardEvent, KeyboardEventInit, KeyboardEventRef,
    FocusEvent, FocusEventInit, FocusEventRef,
    InputEvent, InputEventInit, InputEventRef,
    WheelEvent, WheelEventInit, WheelEventRef, DeltaMode,
    CustomEvent, CustomEventInit, CustomEventRef,
    CompositionEvent, CompositionEventInit, CompositionEventRef,
};
pub use document_ext::{
    DocumentEventExt, create_event_by_interface,
    create_ui_event, create_mouse_event, create_keyboard_event,
    create_focus_event, create_input_event, create_wheel_event,
    create_custom_event, create_composition_event,
};
