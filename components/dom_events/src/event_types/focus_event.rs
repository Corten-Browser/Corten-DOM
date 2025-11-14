//! FocusEvent implementation - for focus and blur events

use super::ui_event::{UIEvent, UIEventInit};
use parking_lot::RwLock;
use std::sync::Arc;

/// FocusEvent initialization options
#[derive(Debug, Clone, Default)]
pub struct FocusEventInit {
    /// Base UIEvent initialization
    pub ui_event_init: UIEventInit,
    /// Related event target (element losing/gaining focus)
    pub related_target: Option<()>, // TODO: Should be EventTarget when available
}

/// FocusEvent reference type
pub type FocusEventRef = Arc<RwLock<FocusEvent>>;

/// FocusEvent - for focus and blur events
#[derive(Debug)]
pub struct FocusEvent {
    /// Base UIEvent data
    ui_event: UIEvent,
    /// Related event target
    related_target: Option<()>,
}

impl FocusEvent {
    /// Create a new FocusEvent with the specified type and initialization options
    pub fn new(event_type: &str, init: FocusEventInit) -> Self {
        Self {
            ui_event: UIEvent::new(event_type, init.ui_event_init),
            related_target: init.related_target,
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

    /// Get the related target
    pub fn related_target(&self) -> Option<()> {
        self.related_target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventInit;

    #[test]
    fn test_focus_event_creation() {
        let init = FocusEventInit {
            ui_event_init: UIEventInit {
                event_init: EventInit {
                    bubbles: true,
                    cancelable: false,
                    composed: true,
                },
                view: None,
                detail: 0,
            },
            related_target: None,
        };

        let focus_event = FocusEvent::new("focus", init);

        assert_eq!(focus_event.ui_event().event().event_type(), "focus");
        assert_eq!(focus_event.ui_event().event().bubbles(), true);
        assert_eq!(focus_event.related_target(), None);
    }

    #[test]
    fn test_focus_event_blur() {
        let init = FocusEventInit::default();
        let focus_event = FocusEvent::new("blur", init);

        assert_eq!(focus_event.ui_event().event().event_type(), "blur");
        assert_eq!(focus_event.ui_event().event().bubbles(), false);
    }

    #[test]
    fn test_focus_event_default() {
        let init = FocusEventInit::default();
        let focus_event = FocusEvent::new("focusin", init);

        assert_eq!(focus_event.related_target(), None);
    }
}
