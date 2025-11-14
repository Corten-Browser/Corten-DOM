//! Event listener types and implementations

use crate::event::Event;
use std::sync::Arc;

/// Event listener callback function type
pub type EventListenerFn = Arc<dyn Fn(&Event) + Send + Sync>;

/// Event listener object that implements handle_event
pub trait EventListenerObject: Send + Sync {
    /// Handle an event
    fn handle_event(&self, event: &Event);
}

/// Event listener enumeration supporting both function and object listeners
#[derive(Clone)]
pub enum EventListener {
    /// Function-based event listener
    Function(EventListenerFn),
    /// Object-based event listener
    Object(Arc<dyn EventListenerObject>),
}

impl EventListener {
    /// Create a new function-based event listener
    pub fn from_fn<F>(f: F) -> Self
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        Self::Function(Arc::new(f))
    }

    /// Create a new object-based event listener
    pub fn from_object<O>(obj: O) -> Self
    where
        O: EventListenerObject + 'static,
    {
        Self::Object(Arc::new(obj))
    }

    /// Invoke the event listener
    pub fn invoke(&self, event: &Event) {
        match self {
            EventListener::Function(f) => f(event),
            EventListener::Object(obj) => obj.handle_event(event),
        }
    }
}

impl std::fmt::Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventListener::Function(_) => write!(f, "EventListener::Function"),
            EventListener::Object(_) => write!(f, "EventListener::Object"),
        }
    }
}

/// Options for addEventListener
#[derive(Debug, Clone, Default)]
pub struct AddEventListenerOptions {
    /// Use capture phase instead of bubble phase
    pub capture: bool,
    /// Listener should be invoked at most once
    pub once: bool,
    /// Listener will never call preventDefault()
    pub passive: bool,
}

/// Options for removeEventListener
#[derive(Debug, Clone, Default)]
pub struct EventListenerOptions {
    /// Match listener in capture phase
    pub capture: bool,
}

/// Internal storage for registered event listeners
#[derive(Debug, Clone)]
pub(crate) struct RegisteredEventListener {
    /// The event listener
    pub listener: EventListener,
    /// Whether this is a capture listener
    pub capture: bool,
    /// Whether this listener should be invoked once
    pub once: bool,
    /// Whether this listener is passive
    pub passive: bool,
    /// Whether this listener has been removed
    pub removed: bool,
}

impl RegisteredEventListener {
    /// Create a new registered event listener
    pub fn new(listener: EventListener, options: AddEventListenerOptions) -> Self {
        Self {
            listener,
            capture: options.capture,
            once: options.once,
            passive: options.passive,
            removed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Event, EventInit};
    use std::sync::{Arc, Mutex};

    struct CounterListener {
        count: Arc<Mutex<i32>>,
    }

    impl EventListenerObject for CounterListener {
        fn handle_event(&self, _event: &Event) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
        }
    }

    #[test]
    fn test_function_listener() {
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        let listener = EventListener::from_fn(move |_event| {
            let mut c = called_clone.lock().unwrap();
            *c = true;
        });

        let event = Event::new("click", EventInit::default());
        listener.invoke(&event);

        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_object_listener() {
        let count = Arc::new(Mutex::new(0));
        let listener = EventListener::from_object(CounterListener {
            count: count.clone(),
        });

        let event = Event::new("click", EventInit::default());
        listener.invoke(&event);
        listener.invoke(&event);

        assert_eq!(*count.lock().unwrap(), 2);
    }

    #[test]
    fn test_add_event_listener_options_default() {
        let options = AddEventListenerOptions::default();
        assert!(!options.capture);
        assert!(!options.once);
        assert!(!options.passive);
    }

    #[test]
    fn test_add_event_listener_options_custom() {
        let options = AddEventListenerOptions {
            capture: true,
            once: true,
            passive: false,
        };
        assert!(options.capture);
        assert!(options.once);
        assert!(!options.passive);
    }

    #[test]
    fn test_registered_event_listener() {
        let listener = EventListener::from_fn(|_| {});
        let options = AddEventListenerOptions {
            capture: true,
            once: false,
            passive: true,
        };

        let registered = RegisteredEventListener::new(listener, options);
        assert!(registered.capture);
        assert!(!registered.once);
        assert!(registered.passive);
        assert!(!registered.removed);
    }
}
