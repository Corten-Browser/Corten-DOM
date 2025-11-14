//! Event implementation with all properties and methods

use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Event phase enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    /// No event is being processed
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

/// Event reference type
pub type EventRef = Arc<RwLock<Event>>;

/// EventTargetRef type (re-exported from event_target module)
pub type EventTargetRef = crate::event_target::EventTargetRef;

/// Event struct with all DOM Level 4 properties
#[derive(Debug)]
pub struct Event {
    /// Type of event (e.g., "click", "load")
    event_type: String,
    /// The target to which the event was originally dispatched
    target: Option<EventTargetRef>,
    /// The target whose event listener is currently being processed
    current_target: Option<EventTargetRef>,
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
    /// Internal flag: event dispatch is in progress
    pub(crate) dispatch_flag: bool,
    /// Internal flag: stop propagation was called
    pub(crate) stop_propagation_flag: bool,
    /// Internal flag: stop immediate propagation was called
    pub(crate) stop_immediate_propagation_flag: bool,
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
            target: None,
            current_target: None,
            event_phase: EventPhase::None,
            bubbles: init.bubbles,
            cancelable: init.cancelable,
            default_prevented: false,
            composed: init.composed,
            is_trusted: false,
            time_stamp,
            dispatch_flag: false,
            stop_propagation_flag: false,
            stop_immediate_propagation_flag: false,
        }
    }

    /// Get the event type
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// Get the target
    pub fn target(&self) -> Option<EventTargetRef> {
        self.target.clone()
    }

    /// Get the current target
    pub fn current_target(&self) -> Option<EventTargetRef> {
        self.current_target.clone()
    }

    /// Get the current event phase
    pub fn event_phase(&self) -> EventPhase {
        self.event_phase
    }

    /// Check if event bubbles
    pub fn bubbles(&self) -> bool {
        self.bubbles
    }

    /// Check if event is cancelable
    pub fn cancelable(&self) -> bool {
        self.cancelable
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

    /// Set the target (internal use only)
    pub(crate) fn set_target(&mut self, target: EventTargetRef) {
        self.target = Some(target);
    }

    /// Set the current target (internal use only)
    pub(crate) fn set_current_target(&mut self, target: Option<EventTargetRef>) {
        self.current_target = target;
    }

    /// Set the event phase (internal use only)
    pub(crate) fn set_event_phase(&mut self, phase: EventPhase) {
        self.event_phase = phase;
    }

    /// Set trusted flag (internal use only)
    pub(crate) fn set_is_trusted(&mut self, trusted: bool) {
        self.is_trusted = trusted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation_default() {
        let event = Event::new("click", EventInit::default());
        assert_eq!(event.event_type(), "click");
        assert!(!event.bubbles());
        assert!(!event.cancelable());
        assert!(!event.composed());
        assert_eq!(event.event_phase(), EventPhase::None);
        assert!(!event.default_prevented());
        assert!(!event.is_trusted());
    }

    #[test]
    fn test_event_creation_with_options() {
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
        assert!(!event.composed());
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
    fn test_stop_propagation() {
        let mut event = Event::new("click", EventInit::default());
        assert!(!event.stop_propagation_flag);
        event.stop_propagation();
        assert!(event.stop_propagation_flag);
    }

    #[test]
    fn test_stop_immediate_propagation() {
        let mut event = Event::new("click", EventInit::default());
        assert!(!event.stop_propagation_flag);
        assert!(!event.stop_immediate_propagation_flag);
        event.stop_immediate_propagation();
        assert!(event.stop_propagation_flag);
        assert!(event.stop_immediate_propagation_flag);
    }

    #[test]
    fn test_event_phase_values() {
        assert_eq!(EventPhase::None as u8, 0);
        assert_eq!(EventPhase::Capturing as u8, 1);
        assert_eq!(EventPhase::AtTarget as u8, 2);
        assert_eq!(EventPhase::Bubbling as u8, 3);
    }

    #[test]
    fn test_timestamp_is_positive() {
        let event = Event::new("click", EventInit::default());
        assert!(event.time_stamp() > 0.0);
    }
}
