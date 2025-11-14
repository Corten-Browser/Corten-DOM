//! Document extensions for event creation
//!
//! This module provides extension methods for Document to create events.
//! These are separate from dom_core to avoid circular dependencies.

use crate::event::{Event, EventInit};
use crate::event_types::{
    CompositionEvent, CompositionEventInit, CustomEvent, CustomEventInit, FocusEvent,
    FocusEventInit, InputEvent, InputEventInit, KeyboardEvent, KeyboardEventInit, MouseEvent,
    MouseEventInit, UIEvent, UIEventInit, WheelEvent, WheelEventInit,
};
use dom_types::DomException;

/// Event creation methods for Document
///
/// This trait extends Document with event creation capabilities.
/// Import this trait to use create_event() on Document instances.
pub trait DocumentEventExt {
    /// Create a new event of the specified type
    ///
    /// # Arguments
    /// * `event_type` - The type of event to create (e.g., "UIEvents", "MouseEvents")
    ///
    /// # Returns
    /// * `Ok(Event)` - A new event of the specified type
    /// * `Err(DomException::NotSupportedError)` - If the event type is not recognized
    ///
    /// # Example
    /// ```ignore
    /// use dom_core::Document;
    /// use dom_events::DocumentEventExt;
    ///
    /// let mut doc = Document::new();
    /// let event = doc.create_event("UIEvents").unwrap();
    /// ```
    fn create_event(&mut self, event_type: &str) -> Result<Event, DomException>;
}

// Note: The actual implementation would be in an impl block for Document
// when both dom_core and dom_events are available. For now, we provide
// helper functions that can be used to create events.

/// Create a UI event
pub fn create_ui_event(event_type: &str) -> UIEvent {
    UIEvent::new(event_type, UIEventInit::default())
}

/// Create a mouse event
pub fn create_mouse_event(event_type: &str) -> MouseEvent {
    MouseEvent::new(event_type, MouseEventInit::default())
}

/// Create a keyboard event
pub fn create_keyboard_event(event_type: &str) -> KeyboardEvent {
    KeyboardEvent::new(event_type, KeyboardEventInit::default())
}

/// Create a focus event
pub fn create_focus_event(event_type: &str) -> FocusEvent {
    FocusEvent::new(event_type, FocusEventInit::default())
}

/// Create an input event
pub fn create_input_event(event_type: &str) -> InputEvent {
    InputEvent::new(event_type, InputEventInit::default())
}

/// Create a wheel event
pub fn create_wheel_event(event_type: &str) -> WheelEvent {
    WheelEvent::new(event_type, WheelEventInit::default())
}

/// Create a custom event
pub fn create_custom_event(event_type: &str) -> CustomEvent {
    CustomEvent::new(event_type, CustomEventInit::default())
}

/// Create a composition event
pub fn create_composition_event(event_type: &str) -> CompositionEvent {
    CompositionEvent::new(event_type, CompositionEventInit::default())
}

/// Create an event based on the legacy event type string
///
/// This function mimics the DOM Level 2 createEvent() behavior.
///
/// # Arguments
/// * `event_interface` - The event interface name (e.g., "UIEvents", "MouseEvents")
///
/// # Returns
/// * `Ok(Event)` - A new event instance
/// * `Err(DomException::NotSupportedError)` - If the interface is not recognized
pub fn create_event_by_interface(event_interface: &str) -> Result<Event, DomException> {
    match event_interface {
        "Event" | "Events" | "HTMLEvents" => {
            Ok(Event::new("", EventInit::default()))
        }
        "UIEvent" | "UIEvents" => {
            let ui_event = UIEvent::new("", UIEventInit::default());
            Ok(ui_event.event().clone())
        }
        "MouseEvent" | "MouseEvents" => {
            let mouse_event = MouseEvent::new("", MouseEventInit::default());
            Ok(mouse_event.ui_event().event().clone())
        }
        "KeyboardEvent" => {
            let keyboard_event = KeyboardEvent::new("", KeyboardEventInit::default());
            Ok(keyboard_event.ui_event().event().clone())
        }
        "FocusEvent" => {
            let focus_event = FocusEvent::new("", FocusEventInit::default());
            Ok(focus_event.ui_event().event().clone())
        }
        "InputEvent" => {
            let input_event = InputEvent::new("", InputEventInit::default());
            Ok(input_event.ui_event().event().clone())
        }
        "WheelEvent" => {
            let wheel_event = WheelEvent::new("", WheelEventInit::default());
            Ok(wheel_event.mouse_event().ui_event().event().clone())
        }
        "CustomEvent" => {
            let custom_event = CustomEvent::new("", CustomEventInit::default());
            Ok(custom_event.event().clone())
        }
        "CompositionEvent" => {
            let composition_event = CompositionEvent::new("", CompositionEventInit::default());
            Ok(composition_event.ui_event().event().clone())
        }
        _ => Err(DomException::NotSupportedError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event_by_interface_events() {
        let event = create_event_by_interface("Events").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_by_interface_ui_events() {
        let event = create_event_by_interface("UIEvents").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_by_interface_mouse_events() {
        let event = create_event_by_interface("MouseEvents").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_by_interface_keyboard_event() {
        let event = create_event_by_interface("KeyboardEvent").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_by_interface_custom_event() {
        let event = create_event_by_interface("CustomEvent").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_by_interface_unsupported() {
        let result = create_event_by_interface("UnknownEvent");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::NotSupportedError);
    }

    #[test]
    fn test_create_ui_event() {
        let event = create_ui_event("load");
        assert_eq!(event.event().event_type(), "load");
    }

    #[test]
    fn test_create_mouse_event() {
        let event = create_mouse_event("click");
        assert_eq!(event.ui_event().event().event_type(), "click");
    }

    #[test]
    fn test_create_keyboard_event() {
        let event = create_keyboard_event("keydown");
        assert_eq!(event.ui_event().event().event_type(), "keydown");
    }

    #[test]
    fn test_create_custom_event() {
        let event = create_custom_event("my-event");
        assert_eq!(event.event().event_type(), "my-event");
    }
}
