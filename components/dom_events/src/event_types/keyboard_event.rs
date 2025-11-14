//! KeyboardEvent implementation - for keyboard input events

use super::ui_event::{UIEvent, UIEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// KeyboardEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct KeyboardEventInit {
    /// Base UIEvent initialization
    pub ui_event_init: UIEventInit,
    /// Key value (e.g., "a", "Enter", "ArrowUp")
    pub key: String,
    /// Physical key code (e.g., "KeyA", "Enter", "ArrowUp")
    pub code: String,
    /// Location of the key on the keyboard
    pub location: u32,
    /// Whether key press is repeating
    pub repeat: bool,
    /// Whether Ctrl key was pressed
    pub ctrl_key: bool,
    /// Whether Shift key was pressed
    pub shift_key: bool,
    /// Whether Alt key was pressed
    pub alt_key: bool,
    /// Whether Meta key was pressed
    pub meta_key: bool,
}

/// KeyboardEvent reference type
pub type KeyboardEventRef = Arc<RwLock<KeyboardEvent>>;

/// KeyboardEvent - for keyboard input events
#[derive(Debug)]
pub struct KeyboardEvent {
    /// Base UIEvent data
    ui_event: UIEvent,
    /// Key value
    key: String,
    /// Physical key code
    code: String,
    /// Key location
    location: u32,
    /// Is repeating
    repeat: bool,
    /// Ctrl key modifier
    ctrl_key: bool,
    /// Shift key modifier
    shift_key: bool,
    /// Alt key modifier
    alt_key: bool,
    /// Meta key modifier
    meta_key: bool,
}

impl KeyboardEvent {
    /// Create a new KeyboardEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: KeyboardEventInit) -> Self {
        Self {
            ui_event: UIEvent::new(event_type, init.ui_event_init),
            key: init.key,
            code: init.code,
            location: init.location,
            repeat: init.repeat,
            ctrl_key: init.ctrl_key,
            shift_key: init.shift_key,
            alt_key: init.alt_key,
            meta_key: init.meta_key,
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

    /// Get the key value
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the physical key code
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the key location
    pub fn location(&self) -> u32 {
        self.location
    }

    /// Check if key is repeating
    pub fn repeat(&self) -> bool {
        self.repeat
    }

    /// Check if Ctrl key was pressed
    pub fn ctrl_key(&self) -> bool {
        self.ctrl_key
    }

    /// Check if Shift key was pressed
    pub fn shift_key(&self) -> bool {
        self.shift_key
    }

    /// Check if Alt key was pressed
    pub fn alt_key(&self) -> bool {
        self.alt_key
    }

    /// Check if Meta key was pressed
    pub fn meta_key(&self) -> bool {
        self.meta_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventInit;

    #[test]
    fn test_keyboard_event_creation() {
        let init = KeyboardEventInit {
            ui_event_init: UIEventInit {
                event_init: EventInit {
                    bubbles: true,
                    cancelable: true,
                    composed: false,
                },
                view: None,
                detail: 0,
            },
            key: "a".to_string(),
            code: "KeyA".to_string(),
            location: 0,
            repeat: false,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        };

        let keyboard_event = KeyboardEvent::new("keydown", init);

        assert_eq!(keyboard_event.ui_event().event().event_type(), "keydown");
        assert_eq!(keyboard_event.key(), "a");
        assert_eq!(keyboard_event.code(), "KeyA");
        assert_eq!(keyboard_event.location(), 0);
        assert_eq!(keyboard_event.repeat(), false);
    }

    #[test]
    fn test_keyboard_event_modifiers() {
        let init = KeyboardEventInit {
            key: "Enter".to_string(),
            code: "Enter".to_string(),
            ctrl_key: true,
            shift_key: false,
            ..Default::default()
        };

        let keyboard_event = KeyboardEvent::new("keypress", init);

        assert_eq!(keyboard_event.ctrl_key(), true);
        assert_eq!(keyboard_event.shift_key(), false);
    }

    #[test]
    fn test_keyboard_event_repeat() {
        let init = KeyboardEventInit {
            key: "ArrowRight".to_string(),
            code: "ArrowRight".to_string(),
            repeat: true,
            ..Default::default()
        };

        let keyboard_event = KeyboardEvent::new("keydown", init);

        assert_eq!(keyboard_event.key(), "ArrowRight");
        assert_eq!(keyboard_event.repeat(), true);
    }

    #[test]
    fn test_keyboard_event_default() {
        let init = KeyboardEventInit::default();
        let keyboard_event = KeyboardEvent::new("keyup", init);

        assert_eq!(keyboard_event.key(), "");
        assert_eq!(keyboard_event.repeat(), false);
    }
}
