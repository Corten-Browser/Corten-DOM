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
