//! MutationRecord and related types

use dom_core::NodeRef;

/// Type of mutation that occurred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    /// Child nodes were added or removed
    ChildList,
    /// Attributes were modified
    Attributes,
    /// Character data was modified
    CharacterData,
}

/// Configuration for what mutations to observe
#[derive(Debug, Clone, Default)]
pub struct MutationObserverInit {
    /// Observe changes to child nodes
    pub child_list: bool,
    /// Observe changes to attributes
    pub attributes: bool,
    /// Observe changes to character data
    pub character_data: bool,
    /// Observe changes to descendants (subtree)
    pub subtree: bool,
    /// Record old attribute values
    pub attribute_old_value: bool,
    /// Record old character data values
    pub character_data_old_value: bool,
    /// Filter specific attributes (None = all attributes)
    pub attribute_filter: Option<Vec<String>>,
}

/// Record of a single mutation
#[derive(Debug, Clone)]
pub struct MutationRecord {
    /// Type of mutation
    pub record_type: MutationType,
    /// Target node of the mutation
    pub target: NodeRef,
    /// Nodes that were added
    pub added_nodes: Vec<NodeRef>,
    /// Nodes that were removed
    pub removed_nodes: Vec<NodeRef>,
    /// Previous sibling of added/removed nodes
    pub previous_sibling: Option<NodeRef>,
    /// Next sibling of added/removed nodes
    pub next_sibling: Option<NodeRef>,
    /// Name of changed attribute
    pub attribute_name: Option<String>,
    /// Namespace of changed attribute
    pub attribute_namespace: Option<String>,
    /// Previous value (for attributes or character data)
    pub old_value: Option<String>,
}

impl MutationRecord {
    /// Create a new child list mutation record
    pub fn child_list(
        target: NodeRef,
        added: Vec<NodeRef>,
        removed: Vec<NodeRef>,
        previous_sibling: Option<NodeRef>,
        next_sibling: Option<NodeRef>,
    ) -> Self {
        Self {
            record_type: MutationType::ChildList,
            target,
            added_nodes: added,
            removed_nodes: removed,
            previous_sibling,
            next_sibling,
            attribute_name: None,
            attribute_namespace: None,
            old_value: None,
        }
    }

    /// Create a new attributes mutation record
    pub fn attributes(
        target: NodeRef,
        attribute_name: String,
        attribute_namespace: Option<String>,
        old_value: Option<String>,
    ) -> Self {
        Self {
            record_type: MutationType::Attributes,
            target,
            added_nodes: Vec::new(),
            removed_nodes: Vec::new(),
            previous_sibling: None,
            next_sibling: None,
            attribute_name: Some(attribute_name),
            attribute_namespace,
            old_value,
        }
    }

    /// Create a new character data mutation record
    pub fn character_data(target: NodeRef, old_value: Option<String>) -> Self {
        Self {
            record_type: MutationType::CharacterData,
            target,
            added_nodes: Vec::new(),
            removed_nodes: Vec::new(),
            previous_sibling: None,
            next_sibling: None,
            attribute_name: None,
            attribute_namespace: None,
            old_value,
        }
    }
}
