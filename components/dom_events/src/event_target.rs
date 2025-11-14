//! EventTarget trait and implementation

use crate::event::EventPhase;
use crate::event_listener::{
    AddEventListenerOptions, EventListener, EventListenerOptions, RegisteredEventListener,
};
use dom_core::NodeRef;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// Reference to an EventTarget
/// This is Arc<RwLock<EventTargetData>> which stores event listeners
pub type EventTargetRef = Arc<RwLock<EventTargetData>>;

/// EventTarget data that stores event listeners
///
/// This struct can be included in any node type that needs event handling
#[derive(Default)]
pub struct EventTargetData {
    /// Map of event type to registered listeners
    pub(crate) listeners: HashMap<String, Vec<RegisteredEventListener>>,
    /// Reference to the node (for tree traversal during event dispatch)
    pub(crate) node_ref: Option<NodeRef>,
}

impl std::fmt::Debug for EventTargetData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventTargetData")
            .field("listeners", &self.listeners)
            .field("node_ref", &self.node_ref.as_ref().map(|_| "<NodeRef>"))
            .finish()
    }
}

impl EventTargetData {
    /// Create a new empty event target
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            node_ref: None,
        }
    }

    /// Create an event target with a node reference
    pub fn with_node(node_ref: NodeRef) -> Self {
        Self {
            listeners: HashMap::new(),
            node_ref: Some(node_ref),
        }
    }

    /// Add an event listener
    pub fn add_event_listener(
        &mut self,
        event_type: &str,
        listener: EventListener,
        options: AddEventListenerOptions,
    ) {
        let registered = RegisteredEventListener::new(listener, options);
        self.listeners
            .entry(event_type.to_string())
            .or_default()
            .push(registered);
    }

    /// Remove an event listener
    pub fn remove_event_listener(
        &mut self,
        event_type: &str,
        _listener: EventListener,
        options: EventListenerOptions,
    ) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            // Mark matching listeners as removed
            // Note: In a real implementation, we'd need to compare listener equality
            // For now, we remove the first matching capture/bubble listener
            if let Some(pos) = listeners.iter().position(|l| l.capture == options.capture) {
                listeners[pos].removed = true;
            }
        }
    }

    /// Get listeners for a specific event type and phase
    pub fn get_listeners(&self, event_type: &str, phase: EventPhase) -> Vec<EventListener> {
        if let Some(listeners) = self.listeners.get(event_type) {
            listeners
                .iter()
                .filter(|l| !l.removed)
                .filter(|l| match phase {
                    EventPhase::Capturing => l.capture,
                    EventPhase::AtTarget => true, // Both capture and bubble listeners
                    EventPhase::Bubbling => !l.capture,
                    EventPhase::None => false,
                })
                .map(|l| l.listener.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get the parent node for event propagation
    pub fn get_parent(&self) -> Option<NodeRef> {
        self.node_ref
            .as_ref()
            .and_then(|node_ref| node_ref.read().parent_node())
    }

    /// Set the node reference
    pub fn set_node_ref(&mut self, node_ref: NodeRef) {
        self.node_ref = Some(node_ref);
    }
}

/// Helper struct to store event listeners on a target
#[derive(Default)]
pub struct EventListenerRegistry {
    /// Map of event type to registered listeners
    listeners: HashMap<String, Vec<RegisteredEventListener>>,
}

impl EventListenerRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Add an event listener
    pub fn add_listener(
        &mut self,
        event_type: &str,
        listener: EventListener,
        options: AddEventListenerOptions,
    ) {
        let registered = RegisteredEventListener::new(listener, options);
        self.listeners
            .entry(event_type.to_string())
            .or_default()
            .push(registered);
    }

    /// Remove an event listener
    pub fn remove_listener(
        &mut self,
        event_type: &str,
        _listener: EventListener,
        options: EventListenerOptions,
    ) {
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            // Mark matching listeners as removed
            // Note: In a real implementation, we'd need to compare listener equality
            // For now, we remove the first matching capture/bubble listener
            if let Some(pos) = listeners.iter().position(|l| l.capture == options.capture) {
                listeners[pos].removed = true;
            }
        }
    }

    /// Get listeners for a specific event type and phase
    pub fn get_listeners(&self, event_type: &str, phase: EventPhase) -> Vec<EventListener> {
        if let Some(listeners) = self.listeners.get(event_type) {
            listeners
                .iter()
                .filter(|l| !l.removed)
                .filter(|l| match phase {
                    EventPhase::Capturing => l.capture,
                    EventPhase::AtTarget => true, // Both capture and bubble listeners
                    EventPhase::Bubbling => !l.capture,
                    EventPhase::None => false,
                })
                .map(|l| l.listener.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all listeners (internal)
    pub(crate) fn get_all(&self) -> &HashMap<String, Vec<RegisteredEventListener>> {
        &self.listeners
    }

    /// Get mutable access to listeners (internal)
    pub(crate) fn get_all_mut(&mut self) -> &mut HashMap<String, Vec<RegisteredEventListener>> {
        &mut self.listeners
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventPhase;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_event_target_data_add_listener() {
        let mut target = EventTargetData::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        let listener = EventListener::from_fn(move |_| {
            *called_clone.lock().unwrap() = true;
        });

        target.add_event_listener(
            "click",
            listener,
            AddEventListenerOptions {
                capture: false,
                once: false,
                passive: false,
            },
        );

        let listeners = target.get_listeners("click", EventPhase::Bubbling);
        assert_eq!(listeners.len(), 1);
    }

    #[test]
    fn test_registry_add_listener() {
        let mut registry = EventListenerRegistry::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        let listener = EventListener::from_fn(move |_| {
            *called_clone.lock().unwrap() = true;
        });

        registry.add_listener(
            "click",
            listener,
            AddEventListenerOptions {
                capture: false,
                once: false,
                passive: false,
            },
        );

        let listeners = registry.get_listeners("click", EventPhase::Bubbling);
        assert_eq!(listeners.len(), 1);
    }

    #[test]
    fn test_registry_capture_vs_bubble() {
        let mut registry = EventListenerRegistry::new();

        let capture_listener = EventListener::from_fn(|_| {});
        let bubble_listener = EventListener::from_fn(|_| {});

        registry.add_listener(
            "click",
            capture_listener,
            AddEventListenerOptions {
                capture: true,
                once: false,
                passive: false,
            },
        );

        registry.add_listener(
            "click",
            bubble_listener,
            AddEventListenerOptions {
                capture: false,
                once: false,
                passive: false,
            },
        );

        // Capture phase should only get capture listener
        let capture_listeners = registry.get_listeners("click", EventPhase::Capturing);
        assert_eq!(capture_listeners.len(), 1);

        // Bubble phase should only get bubble listener
        let bubble_listeners = registry.get_listeners("click", EventPhase::Bubbling);
        assert_eq!(bubble_listeners.len(), 1);

        // At target phase should get both
        let target_listeners = registry.get_listeners("click", EventPhase::AtTarget);
        assert_eq!(target_listeners.len(), 2);
    }

    #[test]
    fn test_registry_remove_listener() {
        let mut registry = EventListenerRegistry::new();

        let listener = EventListener::from_fn(|_| {});
        registry.add_listener(
            "click",
            listener.clone(),
            AddEventListenerOptions {
                capture: false,
                once: false,
                passive: false,
            },
        );

        // Should have one listener
        assert_eq!(
            registry.get_listeners("click", EventPhase::Bubbling).len(),
            1
        );

        // Remove listener
        registry.remove_listener("click", listener, EventListenerOptions { capture: false });

        // Should have zero listeners
        assert_eq!(
            registry.get_listeners("click", EventPhase::Bubbling).len(),
            0
        );
    }

    #[test]
    fn test_registry_multiple_event_types() {
        let mut registry = EventListenerRegistry::new();

        registry.add_listener(
            "click",
            EventListener::from_fn(|_| {}),
            AddEventListenerOptions::default(),
        );

        registry.add_listener(
            "submit",
            EventListener::from_fn(|_| {}),
            AddEventListenerOptions::default(),
        );

        assert_eq!(
            registry.get_listeners("click", EventPhase::Bubbling).len(),
            1
        );
        assert_eq!(
            registry.get_listeners("submit", EventPhase::Bubbling).len(),
            1
        );
        assert_eq!(
            registry.get_listeners("load", EventPhase::Bubbling).len(),
            0
        );
    }
}
