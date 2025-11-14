//! Weak reference management for arena-allocated nodes.
//!
//! Weak references allow nodes to be referenced without preventing their
//! deallocation during garbage collection.

use crate::arena::{NodeArena, NodeId};
use std::any::Any;
use std::sync::Arc;

/// A weak reference to a node in the arena.
///
/// Unlike strong references (Arc), weak references do not prevent the node
/// from being deallocated. When you try to upgrade a weak reference to a
/// strong reference, it may fail if the node has been deallocated.
///
/// # Examples
///
/// ```
/// use dom_storage::arena::NodeArena;
/// use dom_storage::weak_refs::WeakNodeRef;
///
/// let arena = NodeArena::new();
/// let id = arena.allocate("test".to_string());
///
/// let weak = WeakNodeRef::new(&arena, id);
/// assert!(weak.upgrade().is_some());
///
/// arena.deallocate(id);
/// assert!(weak.upgrade().is_none());
/// ```
#[derive(Clone)]
pub struct WeakNodeRef {
    /// The arena containing the node
    arena: NodeArena,
    /// The node's ID
    node_id: NodeId,
}

impl WeakNodeRef {
    /// Creates a new weak reference to a node.
    ///
    /// # Arguments
    ///
    /// * `arena` - The arena containing the node
    /// * `node_id` - The ID of the node to reference
    pub fn new(arena: &NodeArena, node_id: NodeId) -> Self {
        // Increment weak reference count
        arena.increment_weak(node_id);

        Self {
            arena: arena.clone(),
            node_id,
        }
    }

    /// Attempts to upgrade the weak reference to a strong reference.
    ///
    /// # Returns
    ///
    /// `Some(Arc<dyn Any>)` if the node still exists, `None` if it has been
    /// deallocated or the generation doesn't match.
    pub fn upgrade(&self) -> Option<Arc<dyn Any + Send + Sync>> {
        self.arena.get(self.node_id)
    }

    /// Returns the node ID this weak reference points to.
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns true if the node still exists in the arena.
    pub fn is_valid(&self) -> bool {
        self.upgrade().is_some()
    }
}

impl Drop for WeakNodeRef {
    fn drop(&mut self) {
        // Decrement weak reference count
        self.arena.decrement_weak(self.node_id);
    }
}

impl PartialEq for WeakNodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Eq for WeakNodeRef {}

impl std::fmt::Debug for WeakNodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakNodeRef")
            .field("node_id", &self.node_id)
            .field("valid", &self.is_valid())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_ref_size() {
        // WeakNodeRef should be relatively small
        println!(
            "WeakNodeRef size: {} bytes",
            std::mem::size_of::<WeakNodeRef>()
        );
    }

    #[test]
    fn test_weak_ref_debug() {
        let arena = NodeArena::new();
        let id = arena.allocate("test".to_string());
        let weak = WeakNodeRef::new(&arena, id);

        let debug_str = format!("{:?}", weak);
        assert!(debug_str.contains("WeakNodeRef"));
        assert!(debug_str.contains("valid"));
    }
}
