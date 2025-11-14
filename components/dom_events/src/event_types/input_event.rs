//! InputEvent implementation - for text input events

use super::ui_event::{UIEvent, UIEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// InputEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct InputEventInit {
    /// Base UIEvent initialization
    pub ui_event_init: UIEventInit,
    /// Input data (text being inserted)
    pub data: Option<String>,
    /// Input type (e.g., "insertText", "deleteContentBackward")
    pub input_type: String,
    /// Is composing (IME composition in progress)
    pub is_composing: bool,
}

/// InputEvent reference type
pub type InputEventRef = Arc<RwLock<InputEvent>>;

/// InputEvent - for text input events
#[derive(Debug)]
pub struct InputEvent {
    /// Base UIEvent data
    ui_event: UIEvent,
    /// Input data
    data: Option<String>,
    /// Input type
    input_type: String,
    /// Is composing
    is_composing: bool,
}

impl InputEvent {
    /// Create a new InputEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: InputEventInit) -> Self {
        Self {
            ui_event: UIEvent::new(event_type, init.ui_event_init),
            data: init.data,
            input_type: init.input_type,
            is_composing: init.is_composing,
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

    /// Get the input data
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }

    /// Get the input type
    pub fn input_type(&self) -> &str {
        &self.input_type
    }

    /// Check if composing
    pub fn is_composing(&self) -> bool {
        self.is_composing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventInit;

    #[test]
    fn test_input_event_creation() {
        let init = InputEventInit {
            ui_event_init: UIEventInit {
                event_init: EventInit {
                    bubbles: true,
                    cancelable: true,
                    composed: true,
                },
                view: None,
                detail: 0,
            },
            data: Some("Hello".to_string()),
            input_type: "insertText".to_string(),
            is_composing: false,
        };

        let input_event = InputEvent::new("input", init);

        assert_eq!(input_event.ui_event().event().event_type(), "input");
        assert_eq!(input_event.data(), Some("Hello"));
        assert_eq!(input_event.input_type(), "insertText");
        assert_eq!(input_event.is_composing(), false);
    }

    #[test]
    fn test_input_event_delete() {
        let init = InputEventInit {
            input_type: "deleteContentBackward".to_string(),
            data: None,
            ..Default::default()
        };

        let input_event = InputEvent::new("beforeinput", init);

        assert_eq!(input_event.ui_event().event().event_type(), "beforeinput");
        assert_eq!(input_event.input_type(), "deleteContentBackward");
        assert_eq!(input_event.data(), None);
    }

    #[test]
    fn test_input_event_composing() {
        let init = InputEventInit {
            data: Some("あ".to_string()),
            input_type: "insertCompositionText".to_string(),
            is_composing: true,
            ..Default::default()
        };

        let input_event = InputEvent::new("input", init);

        assert_eq!(input_event.is_composing(), true);
        assert_eq!(input_event.data(), Some("あ"));
    }

    #[test]
    fn test_input_event_default() {
        let init = InputEventInit::default();
        let input_event = InputEvent::new("input", init);

        assert_eq!(input_event.data(), None);
        assert_eq!(input_event.input_type(), "");
        assert_eq!(input_event.is_composing(), false);
    }
}
