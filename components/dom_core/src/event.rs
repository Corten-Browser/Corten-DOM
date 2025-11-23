//! Basic Event implementation for Document.createEvent()
//!
//! This provides a minimal Event type for dom_core. The full Event implementation
//! with all DOM4 features is in the dom_events crate.

use dom_types::DomException;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event phase enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventPhase {
    /// No event is being processed
    #[default]
    None = 0,
    /// Capture phase - event is being dispatched to ancestors
    Capturing = 1,
    /// Target phase - event has reached the target
    AtTarget = 2,
    /// Bubble phase - event is bubbling up to ancestors
    Bubbling = 3,
}

/// Event initialization options
#[derive(Debug, Clone, Default)]
pub struct EventInit {
    /// Whether the event bubbles up through the DOM
    pub bubbles: bool,
    /// Whether the event can be canceled
    pub cancelable: bool,
    /// Whether the event is composed (crosses shadow DOM boundaries)
    pub composed: bool,
}

/// Thread-safe reference to an Event
pub type EventRef = Arc<RwLock<Event>>;

/// Basic Event struct for Document.createEvent()
///
/// This is a minimal implementation suitable for use with createEvent().
/// For full event functionality including dispatch, see the dom_events crate.
#[derive(Debug, Clone)]
pub struct Event {
    /// Type of event (e.g., "click", "load")
    event_type: String,
    /// The current phase of event flow
    event_phase: EventPhase,
    /// Whether the event bubbles
    bubbles: bool,
    /// Whether the event can be canceled
    cancelable: bool,
    /// Whether preventDefault() has been called
    default_prevented: bool,
    /// Whether the event is composed (crosses shadow boundaries)
    composed: bool,
    /// Whether the event was dispatched by browser (vs script)
    is_trusted: bool,
    /// Timestamp when the event was created
    time_stamp: f64,
    /// Internal flag: stop propagation was called
    stop_propagation_flag: bool,
    /// Internal flag: stop immediate propagation was called
    stop_immediate_propagation_flag: bool,
}

impl Event {
    /// Create a new event with the specified type and initialization options
    pub fn new(event_type: &str, init: EventInit) -> Self {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0; // Convert to milliseconds

        Self {
            event_type: event_type.to_string(),
            event_phase: EventPhase::None,
            bubbles: init.bubbles,
            cancelable: init.cancelable,
            default_prevented: false,
            composed: init.composed,
            is_trusted: false,
            time_stamp,
            stop_propagation_flag: false,
            stop_immediate_propagation_flag: false,
        }
    }

    /// Get the event type
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Set the event type (used by initEvent)
    pub fn set_event_type(&mut self, event_type: &str) {
        self.event_type = event_type.to_string();
    }

    /// Get the current event phase
    pub fn event_phase(&self) -> EventPhase {
        self.event_phase
    }

    /// Check if event bubbles
    pub fn bubbles(&self) -> bool {
        self.bubbles
    }

    /// Set bubbles flag
    pub fn set_bubbles(&mut self, bubbles: bool) {
        self.bubbles = bubbles;
    }

    /// Check if event is cancelable
    pub fn cancelable(&self) -> bool {
        self.cancelable
    }

    /// Set cancelable flag
    pub fn set_cancelable(&mut self, cancelable: bool) {
        self.cancelable = cancelable;
    }

    /// Check if default action has been prevented
    pub fn default_prevented(&self) -> bool {
        self.default_prevented
    }

    /// Check if event is composed
    pub fn composed(&self) -> bool {
        self.composed
    }

    /// Check if event is trusted
    pub fn is_trusted(&self) -> bool {
        self.is_trusted
    }

    /// Get the event timestamp
    pub fn time_stamp(&self) -> f64 {
        self.time_stamp
    }

    /// Prevent the default action associated with this event
    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }

    /// Stop the event from propagating further
    pub fn stop_propagation(&mut self) {
        self.stop_propagation_flag = true;
    }

    /// Stop the event from propagating and prevent other listeners on the same target
    pub fn stop_immediate_propagation(&mut self) {
        self.stop_propagation_flag = true;
        self.stop_immediate_propagation_flag = true;
    }

    /// Initialize the event (legacy DOM Level 2 method)
    ///
    /// This is used after createEvent() to set up the event properties.
    pub fn init_event(&mut self, event_type: &str, bubbles: bool, cancelable: bool) {
        self.event_type = event_type.to_string();
        self.bubbles = bubbles;
        self.cancelable = cancelable;
        self.default_prevented = false;
        self.stop_propagation_flag = false;
        self.stop_immediate_propagation_flag = false;
    }
}

/// Create an event based on the legacy event type string
///
/// This function mimics the DOM Level 2 createEvent() behavior.
///
/// # Arguments
/// * `event_interface` - The event interface name (e.g., "Event", "UIEvents", "MouseEvents")
///
/// # Returns
/// * `Ok(Event)` - A new event instance
/// * `Err(DomException::NotSupportedError)` - If the interface is not recognized
///
/// # Supported Interfaces
/// - "Event", "Events", "HTMLEvents" - Basic Event
/// - "UIEvent", "UIEvents" - UI Event (basic Event in this implementation)
/// - "MouseEvent", "MouseEvents" - Mouse Event (basic Event in this implementation)
/// - "KeyboardEvent" - Keyboard Event (basic Event in this implementation)
/// - "FocusEvent" - Focus Event (basic Event in this implementation)
/// - "CustomEvent" - Custom Event (basic Event in this implementation)
///
/// Note: For full event type support with specific properties, use the dom_events crate.
pub fn create_event(event_interface: &str) -> Result<Event, DomException> {
    match event_interface {
        "Event" | "Events" | "HTMLEvents" => Ok(Event::new("", EventInit::default())),
        "UIEvent" | "UIEvents" => Ok(Event::new("", EventInit::default())),
        "MouseEvent" | "MouseEvents" => Ok(Event::new("", EventInit::default())),
        "KeyboardEvent" => Ok(Event::new("", EventInit::default())),
        "FocusEvent" => Ok(Event::new("", EventInit::default())),
        "InputEvent" => Ok(Event::new("", EventInit::default())),
        "WheelEvent" => Ok(Event::new("", EventInit::default())),
        "CustomEvent" => Ok(Event::new("", EventInit::default())),
        "CompositionEvent" => Ok(Event::new("", EventInit::default())),
        _ => Err(DomException::NotSupportedError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new("click", EventInit::default());
        assert_eq!(event.event_type(), "click");
        assert!(!event.bubbles());
        assert!(!event.cancelable());
        assert!(!event.default_prevented());
    }

    #[test]
    fn test_event_with_options() {
        let event = Event::new(
            "submit",
            EventInit {
                bubbles: true,
                cancelable: true,
                composed: false,
            },
        );
        assert_eq!(event.event_type(), "submit");
        assert!(event.bubbles());
        assert!(event.cancelable());
    }

    #[test]
    fn test_prevent_default() {
        let mut event = Event::new(
            "click",
            EventInit {
                bubbles: true,
                cancelable: true,
                composed: false,
            },
        );
        assert!(!event.default_prevented());
        event.prevent_default();
        assert!(event.default_prevented());
    }

    #[test]
    fn test_prevent_default_non_cancelable() {
        let mut event = Event::new("click", EventInit::default());
        assert!(!event.cancelable());
        event.prevent_default();
        assert!(!event.default_prevented()); // Should not be set
    }

    #[test]
    fn test_init_event() {
        let mut event = Event::new("", EventInit::default());
        event.init_event("keydown", true, true);

        assert_eq!(event.event_type(), "keydown");
        assert!(event.bubbles());
        assert!(event.cancelable());
    }

    #[test]
    fn test_create_event_by_interface() {
        let event = create_event("Events").unwrap();
        assert_eq!(event.event_type(), "");

        let event = create_event("UIEvents").unwrap();
        assert_eq!(event.event_type(), "");

        let event = create_event("MouseEvents").unwrap();
        assert_eq!(event.event_type(), "");

        let event = create_event("CustomEvent").unwrap();
        assert_eq!(event.event_type(), "");
    }

    #[test]
    fn test_create_event_unsupported() {
        let result = create_event("UnknownEvent");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomException::NotSupportedError);
    }

    #[test]
    fn test_timestamp_positive() {
        let event = Event::new("test", EventInit::default());
        assert!(event.time_stamp() > 0.0);
    }
}
