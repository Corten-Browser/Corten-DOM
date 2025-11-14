//! CompositionEvent implementation - for IME composition events

use super::ui_event::{UIEvent, UIEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// CompositionEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct CompositionEventInit {
    /// Base UIEvent initialization
    pub ui_event_init: UIEventInit,
    /// Composition data (text being composed)
    pub data: String,
}

/// CompositionEvent reference type
pub type CompositionEventRef = Arc<RwLock<CompositionEvent>>;

/// CompositionEvent - for IME composition events
#[derive(Debug)]
pub struct CompositionEvent {
    /// Base UIEvent data
    ui_event: UIEvent,
    /// Composition data
    data: String,
}

impl CompositionEvent {
    /// Create a new CompositionEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: CompositionEventInit) -> Self {
        Self {
            ui_event: UIEvent::new(event_type, init.ui_event_init),
            data: init.data,
        }
    }

    /// Get the base UIEvent
    pub fn ui_event(&self) -> &UIEvent {
        &self.ui_event
    }

    /// Get the base UIEvent (mutable)
    pub fn ui_event_mut(&mut self) -> &mut UIEvent {
        &mut self.ui_event
    }

    /// Get the composition data
    pub fn data(&self) -> &str {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventInit;

    #[test]
    fn test_composition_event_creation() {
        let init = CompositionEventInit {
            ui_event_init: UIEventInit {
                event_init: EventInit {
                    bubbles: true,
                    cancelable: true,
                    composed: true,
                },
                view: None,
                detail: 0,
            },
            data: "あ".to_string(),
        };

        let comp_event = CompositionEvent::new("compositionstart", init);

        assert_eq!(comp_event.ui_event().event().event_type(), "compositionstart");
        assert_eq!(comp_event.data(), "あ");
    }

    #[test]
    fn test_composition_event_update() {
        let init = CompositionEventInit {
            data: "あい".to_string(),
            ..Default::default()
        };

        let comp_event = CompositionEvent::new("compositionupdate", init);

        assert_eq!(comp_event.ui_event().event().event_type(), "compositionupdate");
        assert_eq!(comp_event.data(), "あい");
    }

    #[test]
    fn test_composition_event_end() {
        let init = CompositionEventInit {
            data: "愛".to_string(),
            ..Default::default()
        };

        let comp_event = CompositionEvent::new("compositionend", init);

        assert_eq!(comp_event.ui_event().event().event_type(), "compositionend");
        assert_eq!(comp_event.data(), "愛");
    }

    #[test]
    fn test_composition_event_empty() {
        let init = CompositionEventInit::default();
        let comp_event = CompositionEvent::new("compositionstart", init);

        assert_eq!(comp_event.data(), "");
    }
}
