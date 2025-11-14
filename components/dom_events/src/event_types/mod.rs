//! Event type hierarchy - UIEvent, MouseEvent, KeyboardEvent, etc.

pub mod ui_event;
pub mod mouse_event;
pub mod keyboard_event;
pub mod focus_event;
pub mod input_event;
pub mod wheel_event;
pub mod custom_event;
pub mod composition_event;

pub use ui_event::{UIEvent, UIEventInit, UIEventRef};
pub use mouse_event::{MouseEvent, MouseEventInit, MouseEventRef, MouseButton};
pub use keyboard_event::{KeyboardEvent, KeyboardEventInit, KeyboardEventRef};
pub use focus_event::{FocusEvent, FocusEventInit, FocusEventRef};
pub use input_event::{InputEvent, InputEventInit, InputEventRef};
pub use wheel_event::{WheelEvent, WheelEventInit, WheelEventRef, DeltaMode};
pub use custom_event::{CustomEvent, CustomEventInit, CustomEventRef};
pub use composition_event::{CompositionEvent, CompositionEventInit, CompositionEventRef};
