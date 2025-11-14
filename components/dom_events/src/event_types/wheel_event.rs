//! WheelEvent implementation - for mouse wheel scroll events

use super::mouse_event::{MouseEvent, MouseEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// Delta mode constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DeltaMode {
    /// Delta in pixels
    Pixel = 0x00,
    /// Delta in lines
    Line = 0x01,
    /// Delta in pages
    Page = 0x02,
}

/// WheelEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct WheelEventInit {
    /// Base MouseEvent initialization
    pub mouse_event_init: MouseEventInit,
    /// Horizontal scroll amount
    pub delta_x: f64,
    /// Vertical scroll amount
    pub delta_y: f64,
    /// Z-axis scroll amount
    pub delta_z: f64,
    /// Delta mode (pixel, line, or page)
    pub delta_mode: u32,
}

/// WheelEvent reference type
pub type WheelEventRef = Arc<RwLock<WheelEvent>>;

/// WheelEvent - for mouse wheel scroll events
#[derive(Debug)]
pub struct WheelEvent {
    /// Base MouseEvent data
    mouse_event: MouseEvent,
    /// Horizontal scroll amount
    delta_x: f64,
    /// Vertical scroll amount
    delta_y: f64,
    /// Z-axis scroll amount
    delta_z: f64,
    /// Delta mode
    delta_mode: u32,
}

impl WheelEvent {
    /// DOM_DELTA_PIXEL constant
    pub const DOM_DELTA_PIXEL: u32 = DeltaMode::Pixel as u32;

    /// DOM_DELTA_LINE constant
    pub const DOM_DELTA_LINE: u32 = DeltaMode::Line as u32;

    /// DOM_DELTA_PAGE constant
    pub const DOM_DELTA_PAGE: u32 = DeltaMode::Page as u32;

    /// Create a new WheelEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: WheelEventInit) -> Self {
        Self {
            mouse_event: MouseEvent::new(event_type, init.mouse_event_init),
            delta_x: init.delta_x,
            delta_y: init.delta_y,
            delta_z: init.delta_z,
            delta_mode: init.delta_mode,
        }
    }

    /// Get the base MouseEvent
    pub fn mouse_event(&self) -> &MouseEvent {
        &self.mouse_event
    }

    /// Get the base MouseEvent (mutable)
    pub fn mouse_event_mut(&mut self) -> &mut MouseEvent {
        &mut self.mouse_event
    }

    /// Get horizontal scroll amount
    pub fn delta_x(&self) -> f64 {
        self.delta_x
    }

    /// Get vertical scroll amount
    pub fn delta_y(&self) -> f64 {
        self.delta_y
    }

    /// Get Z-axis scroll amount
    pub fn delta_z(&self) -> f64 {
        self.delta_z
    }

    /// Get delta mode
    pub fn delta_mode(&self) -> u32 {
        self.delta_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventInit;
    use crate::event_types::ui_event::UIEventInit;

    #[test]
    fn test_wheel_event_creation() {
        let init = WheelEventInit {
            mouse_event_init: MouseEventInit {
                ui_event_init: UIEventInit {
                    event_init: EventInit {
                        bubbles: true,
                        cancelable: true,
                        composed: true,
                    },
                    view: None,
                    detail: 0,
                },
                client_x: 100,
                client_y: 200,
                ..Default::default()
            },
            delta_x: 0.0,
            delta_y: 120.0,
            delta_z: 0.0,
            delta_mode: WheelEvent::DOM_DELTA_PIXEL,
        };

        let wheel_event = WheelEvent::new("wheel", init);

        assert_eq!(wheel_event.mouse_event().ui_event().event().event_type(), "wheel");
        assert_eq!(wheel_event.delta_x(), 0.0);
        assert_eq!(wheel_event.delta_y(), 120.0);
        assert_eq!(wheel_event.delta_z(), 0.0);
        assert_eq!(wheel_event.delta_mode(), WheelEvent::DOM_DELTA_PIXEL);
    }

    #[test]
    fn test_wheel_event_horizontal_scroll() {
        let init = WheelEventInit {
            delta_x: -50.0,
            delta_y: 0.0,
            delta_mode: WheelEvent::DOM_DELTA_LINE,
            ..Default::default()
        };

        let wheel_event = WheelEvent::new("wheel", init);

        assert_eq!(wheel_event.delta_x(), -50.0);
        assert_eq!(wheel_event.delta_mode(), WheelEvent::DOM_DELTA_LINE);
    }

    #[test]
    fn test_wheel_event_page_scroll() {
        let init = WheelEventInit {
            delta_y: 1.0,
            delta_mode: WheelEvent::DOM_DELTA_PAGE,
            ..Default::default()
        };

        let wheel_event = WheelEvent::new("wheel", init);

        assert_eq!(wheel_event.delta_y(), 1.0);
        assert_eq!(wheel_event.delta_mode(), WheelEvent::DOM_DELTA_PAGE);
    }

    #[test]
    fn test_wheel_event_default() {
        let init = WheelEventInit::default();
        let wheel_event = WheelEvent::new("wheel", init);

        assert_eq!(wheel_event.delta_x(), 0.0);
        assert_eq!(wheel_event.delta_y(), 0.0);
        assert_eq!(wheel_event.delta_z(), 0.0);
        assert_eq!(wheel_event.delta_mode(), 0);
    }

    #[test]
    fn test_wheel_event_constants() {
        assert_eq!(WheelEvent::DOM_DELTA_PIXEL, 0);
        assert_eq!(WheelEvent::DOM_DELTA_LINE, 1);
        assert_eq!(WheelEvent::DOM_DELTA_PAGE, 2);
    }
}
