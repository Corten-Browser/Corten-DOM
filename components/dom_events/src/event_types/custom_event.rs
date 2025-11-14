//! CustomEvent implementation - for custom application-specific events

use crate::event::{Event, EventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// CustomEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct CustomEventInit {
    /// Base Event initialization
    pub event_init: EventInit,
    /// Custom detail data (would be Any in full implementation)
    pub detail: Option<String>, // Simplified - would be generic data in full impl
}

/// CustomEvent reference type
pub type CustomEventRef = Arc<RwLock<CustomEvent>>;

/// CustomEvent - for custom application-specific events
#[derive(Debug)]
pub struct CustomEvent {
    /// Base Event data
    event: Event,
    /// Custom detail data
    detail: Option<String>,
}

impl CustomEvent {
    /// Create a new CustomEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: CustomEventInit) -> Self {
        Self {
            event: Event::new(event_type, init.event_init),
            detail: init.detail,
        }
    }

    /// Get the base Event
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Get the base Event (mutable)
    pub fn event_mut(&mut self) -> &mut Event {
        &mut self.event
    }

    /// Get the custom detail data
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_event_creation() {
        let init = CustomEventInit {
            event_init: EventInit {
                bubbles: true,
                cancelable: false,
                composed: true,
            },
            detail: Some("Custom data payload".to_string()),
        };

        let custom_event = CustomEvent::new("my-custom-event", init);

        assert_eq!(custom_event.event().event_type(), "my-custom-event");
        assert_eq!(custom_event.event().bubbles(), true);
        assert_eq!(custom_event.detail(), Some("Custom data payload"));
    }

    #[test]
    fn test_custom_event_no_detail() {
        let init = CustomEventInit {
            event_init: EventInit {
                bubbles: false,
                cancelable: true,
                composed: false,
            },
            detail: None,
        };

        let custom_event = CustomEvent::new("app:notification", init);

        assert_eq!(custom_event.event().event_type(), "app:notification");
        assert_eq!(custom_event.detail(), None);
    }

    #[test]
    fn test_custom_event_json_detail() {
        let init = CustomEventInit {
            detail: Some(r#"{"user": "john", "action": "login"}"#.to_string()),
            ..Default::default()
        };

        let custom_event = CustomEvent::new("user:action", init);

        assert_eq!(custom_event.detail(), Some(r#"{"user": "john", "action": "login"}"#));
    }

    #[test]
    fn test_custom_event_default() {
        let init = CustomEventInit::default();
        let custom_event = CustomEvent::new("custom", init);

        assert_eq!(custom_event.event().bubbles(), false);
        assert_eq!(custom_event.detail(), None);
    }
}
