//! UIEvent implementation - base class for user interface events

use crate::event::{Event, EventInit, EventRef};
use parking_lot::RwLock;
use std::sync::Arc;

/// UIEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct UIEventInit {
    /// Base event initialization
    pub event_init: EventInit,
    /// Window object (currently just a placeholder)
    pub view: Option<()>,
    /// Detail value for event-specific information
    pub detail: i32,
}

/// UIEvent reference type
pub type UIEventRef = Arc<RwLock<UIEvent>>;

/// UIEvent - base for all UI-related events
#[derive(Debug)]
pub struct UIEvent {
    /// Base event data
    event: Event,
    /// Window object (placeholder for now)
    view: Option<()>,
    /// Event-specific detail information
    detail: i32,
}

impl UIEvent {
    /// Create a new UIEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: UIEventInit) -> Self {
        Self {
            event: Event::new(event_type, init.event_init),
            view: init.view,
            detail: init.detail,
        }
    }

    /// Get the base event
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Get the base event (mutable)
    pub fn event_mut(&mut self) -> &mut Event {
        &mut self.event
    }

    /// Get the view (window) associated with this event
    pub fn view(&self) -> Option<()> {
        self.view
    }

    /// Get the detail value
    pub fn detail(&self) -> i32 {
        self.detail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_event_creation() {
        let init = UIEventInit {
            event_init: EventInit {
                bubbles: true,
                cancelable: true,
                composed: false,
            },
            view: None,
            detail: 5,
        };

        let ui_event = UIEvent::new("click", init);

        assert_eq!(ui_event.event().event_type(), "click");
        assert_eq!(ui_event.event().bubbles(), true);
        assert_eq!(ui_event.event().cancelable(), true);
        assert_eq!(ui_event.detail(), 5);
    }

    #[test]
    fn test_ui_event_default() {
        let init = UIEventInit::default();
        let ui_event = UIEvent::new("test", init);

        assert_eq!(ui_event.detail(), 0);
        assert_eq!(ui_event.event().bubbles(), false);
    }

    #[test]
    fn test_ui_event_detail() {
        let init = UIEventInit {
            detail: 42,
            ..Default::default()
        };
        let ui_event = UIEvent::new("test", init);

        assert_eq!(ui_event.detail(), 42);
    }
}
