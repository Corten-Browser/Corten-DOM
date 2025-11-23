//! Core Node trait and base implementation

use crate::tree_order::{compare_document_position, DocumentPosition};
use dom_types::{DomException, NodeType};
use parking_lot::RwLock;
use std::fmt;
use std::sync::{Arc, Weak};

/// Thread-safe reference to a DOM node
pub type NodeRef = Arc<RwLock<Box<dyn Node>>>;

/// Weak reference to a DOM node (non-owning)
pub type WeakNodeRef = Weak<RwLock<Box<dyn Node>>>;

/// Core node trait that all DOM nodes must implement
pub trait Node: Send + Sync + std::fmt::Debug {
    /// Returns the type of this node
    fn node_type(&self) -> NodeType;

    /// Returns the name of this node (uppercase for elements)
    fn node_name(&self) -> &str;

    /// Returns the node value (text content for text nodes, None for elements)
    fn node_value(&self) -> Option<&str> {
        None
    }

    /// Sets the node value
    fn set_node_value(&mut self, _value: Option<String>) {
        // Default implementation does nothing
    }

    /// Returns the text content of node and descendants
    fn text_content(&self) -> Option<String> {
        None
    }

    /// Sets the text content of node
    fn set_text_content(&mut self, _text: String) {
        // Default implementation
    }

    /// Returns the parent node
    fn parent_node(&self) -> Option<NodeRef>;

    /// Returns the parent element (skips non-element parents)
    fn parent_element(&self) -> Option<NodeRef> {
        let parent = self.parent_node()?;
        if parent.read().node_type() == NodeType::Element {
            Some(parent)
        } else {
            parent.read().parent_element()
        }
    }

    /// Returns all child nodes
    fn child_nodes(&self) -> Vec<NodeRef>;

    /// Returns the first child node
    fn first_child(&self) -> Option<NodeRef> {
        self.child_nodes().first().cloned()
    }

    /// Returns the last child node
    fn last_child(&self) -> Option<NodeRef> {
        self.child_nodes().last().cloned()
    }

    /// Returns the previous sibling
    fn previous_sibling(&self) -> Option<NodeRef> {
        None // Must be implemented by concrete types
    }

    /// Returns the next sibling
    fn next_sibling(&self) -> Option<NodeRef> {
        None // Must be implemented by concrete types
    }

    /// Appends a child to this node
    fn append_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;

    /// Removes a child from this node
    fn remove_child(&mut self, child: NodeRef) -> Result<NodeRef, DomException>;

    /// Replaces an old child with a new child
    fn replace_child(
        &mut self,
        new_child: NodeRef,
        old_child: NodeRef,
    ) -> Result<NodeRef, DomException> {
        // Default implementation using insert_before and remove_child
        self.insert_before(new_child, Some(old_child.clone()))?;
        self.remove_child(old_child)
    }

    /// Inserts a new child before a reference child
    fn insert_before(
        &mut self,
        new_child: NodeRef,
        ref_child: Option<NodeRef>,
    ) -> Result<NodeRef, DomException>;

    /// Clones this node (optionally deep)
    fn clone_node(&self, deep: bool) -> NodeRef;

    /// Normalizes the node tree (combines adjacent text nodes)
    fn normalize(&mut self) {
        // Default implementation
    }

    /// Checks if two nodes are equal
    fn is_equal_node(&self, other: &dyn Node) -> bool {
        self.node_type() == other.node_type() && self.node_name() == other.node_name()
    }

    /// Checks if two nodes are the same (same reference)
    fn is_same_node(&self, other: &dyn Node) -> bool
    where
        Self: Sized,
    {
        let self_ptr = self as *const Self as *const ();
        let other_ptr = other as *const dyn Node as *const ();
        self_ptr == other_ptr
    }

    /// Checks if this node contains another node
    fn contains(&self, other: &dyn Node) -> bool; // Must be implemented by concrete types

    /// Compares the document position of this node with another node
    ///
    /// Returns a bitmask of `DocumentPosition` flags indicating the relationship.
    /// Common return values:
    /// - 0: Nodes are the same
    /// - PRECEDING (2): Other node precedes this node
    /// - FOLLOWING (4): Other node follows this node
    /// - CONTAINS (8): Other node contains this node
    /// - CONTAINED_BY (16): Other node is contained by this node
    /// - DISCONNECTED (1): Nodes are not in the same tree
    fn compare_document_position(&self, other: &NodeRef, self_ref: &NodeRef) -> u16 {
        compare_document_position(self_ref, other)
    }

    /// Access to internal node data
    fn node_data(&self) -> &NodeData;

    /// Mutable access to internal node data
    fn node_data_mut(&mut self) -> &mut NodeData;

    /// Downcast to concrete type (for type checking)
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Common data shared by all node types
#[derive(Clone, Debug)]
pub struct NodeData {
    /// Node type
    pub node_type: NodeType,

    /// Node name
    pub node_name: String,

    /// Parent node (weak reference to avoid circular ownership)
    pub parent: Option<WeakNodeRef>,

    /// Child nodes
    pub children: Vec<NodeRef>,
}

impl NodeData {
    /// Creates new node data
    pub fn new(node_type: NodeType, node_name: impl Into<String>) -> Self {
        Self {
            node_type,
            node_name: node_name.into(),
            parent: None,
            children: Vec::new(),
        }
    }

    /// Sets the parent node
    pub fn set_parent(&mut self, parent: Option<WeakNodeRef>) {
        self.parent = parent;
    }

    /// Gets the parent node
    pub fn get_parent(&self) -> Option<NodeRef> {
        self.parent.as_ref().and_then(|weak| weak.upgrade())
    }

    /// Adds a child node
    pub fn add_child(&mut self, child: NodeRef) {
        self.children.push(child);
    }

    /// Removes a child node
    pub fn remove_child(&mut self, child: &NodeRef) -> Result<NodeRef, DomException> {
        let child_ptr = &**child.read() as *const dyn Node;

        if let Some(pos) = self.children.iter().position(|c| {
            let c_ptr = &**c.read() as *const dyn Node;
            c_ptr == child_ptr
        }) {
            Ok(self.children.remove(pos))
        } else {
            Err(DomException::NotFoundError)
        }
    }

    /// Inserts a child before a reference child
    pub fn insert_child_before(
        &mut self,
        new_child: NodeRef,
        ref_child: Option<&NodeRef>,
    ) -> Result<(), DomException> {
        if let Some(ref_child) = ref_child {
            let ref_ptr = &**ref_child.read() as *const dyn Node;

            if let Some(pos) = self.children.iter().position(|c| {
                let c_ptr = &**c.read() as *const dyn Node;
                c_ptr == ref_ptr
            }) {
                self.children.insert(pos, new_child);
                Ok(())
            } else {
                Err(DomException::NotFoundError)
            }
        } else {
            // No reference child means append
            self.children.push(new_child);
            Ok(())
        }
    }

    /// Checks if this node contains another node
    pub fn contains(&self, other_ptr: *const dyn Node) -> bool {
        for child in &self.children {
            let child_ptr = &**child.read() as *const dyn Node;
            if child_ptr == other_ptr {
                return true;
            }
            if child.read().node_data().contains(other_ptr) {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.node_name, self.node_type as u16)
    }
}
