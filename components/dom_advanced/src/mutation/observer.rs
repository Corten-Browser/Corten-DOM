//! MutationObserver implementation

use super::{MutationObserverInit, MutationRecord, MutationType};
use dom_core::{NodeRef, WeakNodeRef};
use dom_types::DomException;
use parking_lot::Mutex;
use std::sync::Arc;

type ObserverCallback = Arc<dyn Fn(&[MutationRecord]) + Send + Sync>;

/// Observer for DOM mutations
///
/// MutationObserver provides a way to watch for changes being made to the DOM tree.
/// It will fire asynchronously via microtask queue when mutations occur.
#[derive(Clone)]
pub struct MutationObserver {
    inner: Arc<Mutex<MutationObserverInner>>,
}

struct MutationObserverInner {
    callback: ObserverCallback,
    observed_nodes: Vec<ObservedNode>,
    record_queue: Vec<MutationRecord>,
    is_active: bool,
}

struct ObservedNode {
    node: WeakNodeRef,
    options: MutationObserverInit,
}

impl MutationObserver {
    /// Create a new MutationObserver with the given callback
    ///
    /// The callback will be invoked asynchronously when mutations are detected.
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&[MutationRecord]) + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(Mutex::new(MutationObserverInner {
                callback: Arc::new(callback),
                observed_nodes: Vec::new(),
                record_queue: Vec::new(),
                is_active: true,
            })),
        }
    }

    /// Start observing a target node with the given options
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the target node is invalid.
    pub fn observe(
        &self,
        target: NodeRef,
        options: MutationObserverInit,
    ) -> Result<(), DomException> {
        // Validate options
        if !options.child_list
            && !options.attributes
            && !options.character_data
        {
            return Err(DomException::SyntaxError(
                "At least one of childList, attributes, or characterData must be true".to_string(),
            ));
        }

        let mut inner = self.inner.lock();

        // Check if already observing this node, update options if so
        if let Some(observed) = inner
            .observed_nodes
            .iter_mut()
            .find(|n| n.node.ptr_eq(&WeakNodeRef::from(target.clone())))
        {
            observed.options = options;
            return Ok(());
        }

        // Add new observation
        inner.observed_nodes.push(ObservedNode {
            node: WeakNodeRef::from(target),
            options,
        });

        Ok(())
    }

    /// Stop observing all nodes
    pub fn disconnect(&self) {
        let mut inner = self.inner.lock();
        inner.observed_nodes.clear();
        inner.record_queue.clear();
        inner.is_active = false;
    }

    /// Take all pending mutation records without waiting for callback
    ///
    /// This clears the record queue and returns all pending records.
    pub fn take_records(&self) -> Vec<MutationRecord> {
        let mut inner = self.inner.lock();
        std::mem::take(&mut inner.record_queue)
    }

    /// Queue a mutation record (internal API)
    ///
    /// This is called by the DOM implementation when mutations occur.
    pub(crate) fn queue_record(&self, record: MutationRecord) {
        let mut inner = self.inner.lock();

        if !inner.is_active {
            return;
        }

        // Check if this observer is interested in this mutation
        let is_interested = inner.observed_nodes.iter().any(|observed| {
            if let Some(target_node) = observed.node.upgrade() {
                // Check if this is the observed node or a descendant (if subtree)
                let is_target = std::sync::Arc::ptr_eq(&target_node, &record.target);
                let is_descendant = observed.options.subtree
                    && Self::is_descendant_of(&record.target, &target_node);

                if !is_target && !is_descendant {
                    return false;
                }

                // Check if mutation type matches observed options
                match record.record_type {
                    MutationType::ChildList => observed.options.child_list,
                    MutationType::Attributes => {
                        if !observed.options.attributes {
                            return false;
                        }
                        // Check attribute filter
                        if let Some(ref filter) = observed.options.attribute_filter {
                            if let Some(ref attr_name) = record.attribute_name {
                                filter.contains(attr_name)
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    }
                    MutationType::CharacterData => observed.options.character_data,
                }
            } else {
                false
            }
        });

        if is_interested {
            inner.record_queue.push(record);

            // Schedule delivery if we have records
            // In a real implementation, this would queue a microtask
            // For now, we'll deliver synchronously in tests
        }
    }

    /// Deliver pending mutations (internal API)
    ///
    /// This should be called from the microtask queue in a real implementation.
    pub(crate) fn deliver_mutations(&self) {
        let (callback, records) = {
            let mut inner = self.inner.lock();
            if inner.record_queue.is_empty() || !inner.is_active {
                return;
            }

            let callback = Arc::clone(&inner.callback);
            let records = std::mem::take(&mut inner.record_queue);
            (callback, records)
        };

        // Call the callback outside the lock
        callback(&records);
    }

    /// Check if the observer is interested in a node
    pub(crate) fn is_observing(&self, node: &NodeRef) -> bool {
        let inner = self.inner.lock();
        inner.observed_nodes.iter().any(|observed| {
            observed
                .node
                .upgrade()
                .map(|n| std::sync::Arc::ptr_eq(&n, node))
                .unwrap_or(false)
        })
    }

    // Helper method to check if a node is a descendant of another
    fn is_descendant_of(node: &NodeRef, ancestor: &NodeRef) -> bool {
        let mut current = node.read().parent_node();
        while let Some(parent) = current {
            if std::sync::Arc::ptr_eq(&parent, ancestor) {
                return true;
            }
            current = parent.read().parent_node();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_observer_creation() {
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let observer = MutationObserver::new(move |_records| {
            called_clone.store(true, Ordering::SeqCst);
        });

        assert!(!called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_observer_requires_options() {
        use dom_core::{Document, Node, Text};
        use std::sync::Arc;
        use parking_lot::RwLock;

        let mut document = Document::new();
        let text_node = document.create_text_node("test");

        let observer = MutationObserver::new(|_| {});

        // Should fail with no options enabled
        let result = observer.observe(
            text_node,
            MutationObserverInit {
                child_list: false,
                attributes: false,
                character_data: false,
                ..Default::default()
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_disconnect_clears_observations() {
        use dom_core::{Document, Node};

        let mut document = Document::new();
        let text_node = document.create_text_node("test");

        let observer = MutationObserver::new(|_| {});

        observer
            .observe(
                text_node.clone(),
                MutationObserverInit {
                    child_list: true,
                    ..Default::default()
                },
            )
            .unwrap();

        assert!(observer.is_observing(&text_node));

        observer.disconnect();

        assert!(!observer.is_observing(&text_node));
    }

    #[test]
    fn test_take_records() {
        use dom_core::Document;

        let mut document = Document::new();
        let node = document.create_text_node("test");

        let observer = MutationObserver::new(|_| {});

        observer
            .observe(
                node.clone(),
                MutationObserverInit {
                    child_list: true,
                    ..Default::default()
                },
            )
            .unwrap();

        // Queue a mutation
        let record = MutationRecord::child_list(node, vec![], vec![], None, None);
        observer.queue_record(record);

        let records = observer.take_records();
        assert_eq!(records.len(), 1);

        // Records should be cleared
        let records = observer.take_records();
        assert_eq!(records.len(), 0);
    }
}
