//! NodeList implementation (live and static variants)

use dom_core::NodeRef;
use std::sync::Arc;

/// NodeList can be live or static
pub enum NodeList {
    /// Live list that updates with DOM changes
    Live {
        /// Root node for traversal
        root: Arc<parking_lot::RwLock<Box<dyn dom_core::Node>>>,
        /// Filter function to determine which nodes to include
        filter:
            Arc<dyn Fn(&Arc<parking_lot::RwLock<Box<dyn dom_core::Node>>>) -> bool + Send + Sync>,
    },
    /// Static snapshot of nodes
    Static {
        /// Static list of nodes
        nodes: Vec<NodeRef>
    },
}

impl NodeList {
    /// Creates a new static NodeList
    pub fn new_static(nodes: Vec<NodeRef>) -> Self {
        NodeList::Static { nodes }
    }

    /// Creates a new live NodeList
    pub fn new_live<F>(root: NodeRef, filter: F) -> Self
    where
        F: Fn(&NodeRef) -> bool + Send + Sync + 'static,
    {
        NodeList::Live {
            root,
            filter: Arc::new(filter),
        }
    }

    /// Returns the number of nodes
    pub fn length(&self) -> usize {
        match self {
            NodeList::Static { nodes } => nodes.len(),
            NodeList::Live { root, filter } => {
                // TODO: Implement live traversal
                let mut count = 0;
                if (filter)(root) {
                    count += 1;
                }
                count
            }
        }
    }

    /// Returns the node at the given index
    pub fn item(&self, index: usize) -> Option<NodeRef> {
        match self {
            NodeList::Static { nodes } => nodes.get(index).cloned(),
            NodeList::Live { .. } => {
                // TODO: Implement live traversal
                None
            }
        }
    }
}
