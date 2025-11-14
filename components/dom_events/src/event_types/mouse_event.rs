//! MouseEvent implementation - for mouse-related events

use super::ui_event::{UIEvent, UIEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Main button (usually left)
    Main = 0,
    /// Auxiliary button (usually middle/wheel)
    Auxiliary = 1,
    /// Secondary button (usually right)
    Secondary = 2,
    /// Fourth button (typically browser back)
    Fourth = 3,
    /// Fifth button (typically browser forward)
    Fifth = 4,
}

/// MouseEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct MouseEventInit {
    /// Base UIEvent initialization
    pub ui_event_init: UIEventInit,
    /// X coordinate relative to viewport
    pub client_x: i32,
    /// Y coordinate relative to viewport
    pub client_y: i32,
    /// X coordinate relative to screen
    pub screen_x: i32,
    /// Y coordinate relative to screen
    pub screen_y: i32,
    /// Mouse button that triggered the event
    pub button: i16,
    /// Buttons being pressed when event was triggered
    pub buttons: u16,
    /// Whether Ctrl key was pressed
    pub ctrl_key: bool,
    /// Whether Shift key was pressed
    pub shift_key: bool,
    /// Whether Alt key was pressed
    pub alt_key: bool,
    /// Whether Meta key was pressed
    pub meta_key: bool,
}

/// MouseEvent reference type
pub type MouseEventRef = Arc<RwLock<MouseEvent>>;

/// MouseEvent - for mouse-related user interaction events
#[derive(Debug)]
pub struct MouseEvent {
    /// Base UIEvent data
    ui_event: UIEvent,
    /// X coordinate relative to viewport
    client_x: i32,
    /// Y coordinate relative to viewport
    client_y: i32,
    /// X coordinate relative to screen
    screen_x: i32,
    /// Y coordinate relative to screen
    screen_y: i32,
    /// Mouse button
    button: i16,
    /// Buttons being pressed
    buttons: u16,
    /// Ctrl key modifier
    ctrl_key: bool,
    /// Shift key modifier
    shift_key: bool,
    /// Alt key modifier
    alt_key: bool,
    /// Meta key modifier
    meta_key: bool,
}

impl MouseEvent {
    /// Create a new MouseEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: MouseEventInit) -> Self {
        Self {
            ui_event: UIEvent::new(event_type, init.ui_event_init),
            client_x: init.client_x,
            client_y: init.client_y,
            screen_x: init.screen_x,
            screen_y: init.screen_y,
            button: init.button,
            buttons: init.buttons,
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

    /// Get client X coordinate
    pub fn client_x(&self) -> i32 {
        self.client_x
    }

    /// Get client Y coordinate
    pub fn client_y(&self) -> i32 {
        self.client_y
    }

    /// Get screen X coordinate
    pub fn screen_x(&self) -> i32 {
        self.screen_x
    }

    /// Get screen Y coordinate
    pub fn screen_y(&self) -> i32 {
        self.screen_y
    }

    /// Get mouse button
    pub fn button(&self) -> i16 {
        self.button
    }

    /// Get buttons being pressed
    pub fn buttons(&self) -> u16 {
        self.buttons
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
    fn test_mouse_event_creation() {
        let init = MouseEventInit {
            ui_event_init: UIEventInit {
                event_init: EventInit {
                    bubbles: true,
                    cancelable: true,
                    composed: false,
                },
                view: None,
                detail: 1,
            },
            client_x: 100,
            client_y: 200,
            screen_x: 300,
            screen_y: 400,
            button: 0, // Main button
            buttons: 1,
            ctrl_key: false,
            shift_key: false,
            alt_key: false,
            meta_key: false,
        };

        let mouse_event = MouseEvent::new("click", init);

        assert_eq!(mouse_event.ui_event().event().event_type(), "click");
        assert_eq!(mouse_event.client_x(), 100);
        assert_eq!(mouse_event.client_y(), 200);
        assert_eq!(mouse_event.screen_x(), 300);
        assert_eq!(mouse_event.screen_y(), 400);
        assert_eq!(mouse_event.button(), 0);
    }

    #[test]
    fn test_mouse_event_modifiers() {
        let init = MouseEventInit {
            ctrl_key: true,
            shift_key: true,
            alt_key: false,
            meta_key: false,
            ..Default::default()
        };

        let mouse_event = MouseEvent::new("click", init);

        assert_eq!(mouse_event.ctrl_key(), true);
        assert_eq!(mouse_event.shift_key(), true);
        assert_eq!(mouse_event.alt_key(), false);
        assert_eq!(mouse_event.meta_key(), false);
    }

    #[test]
    fn test_mouse_event_buttons() {
        let init = MouseEventInit {
            button: 2, // Secondary (right) button
            buttons: 2, // Right button pressed
            ..Default::default()
        };

        let mouse_event = MouseEvent::new("contextmenu", init);

        assert_eq!(mouse_event.button(), 2);
        assert_eq!(mouse_event.buttons(), 2);
    }

    #[test]
    fn test_mouse_event_default() {
        let init = MouseEventInit::default();
        let mouse_event = MouseEvent::new("mousemove", init);

        assert_eq!(mouse_event.client_x(), 0);
        assert_eq!(mouse_event.client_y(), 0);
        assert_eq!(mouse_event.ctrl_key(), false);
    }
}
