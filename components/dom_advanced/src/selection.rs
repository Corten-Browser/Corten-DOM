//! Selection API implementation
//!
//! Represents the user's text selection or cursor position.

use crate::range::Range;
use dom_core::NodeRef;
use dom_types::DomException;

/// Direction of selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDirection {
    /// Selection made from left to right
    Forward,
    /// Selection made from right to left
    Backward,
    /// Direction is not specified
    None,
}

/// Represents a user selection
///
/// A Selection object represents the range(s) selected by the user or the current
/// position of the caret.
#[derive(Debug, Clone)]
pub struct Selection {
    ranges: Vec<Range>,
    direction: SelectionDirection,
}

impl Selection {
    /// Create a new empty Selection
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
            direction: SelectionDirection::None,
        }
    }

    /// Get the anchor node (start of selection)
    pub fn anchor_node(&self) -> Option<NodeRef> {
        self.ranges.first().map(|r| match self.direction {
            SelectionDirection::Forward => r.start_container().clone(),
            SelectionDirection::Backward => r.end_container().clone(),
            SelectionDirection::None => r.start_container().clone(),
        })
    }

    /// Get the anchor offset
    pub fn anchor_offset(&self) -> usize {
        self.ranges
            .first()
            .map(|r| match self.direction {
                SelectionDirection::Forward => r.start_offset(),
                SelectionDirection::Backward => r.end_offset(),
                SelectionDirection::None => r.start_offset(),
            })
            .unwrap_or(0)
    }

    /// Get the focus node (end of selection)
    pub fn focus_node(&self) -> Option<NodeRef> {
        self.ranges.first().map(|r| match self.direction {
            SelectionDirection::Forward => r.end_container().clone(),
            SelectionDirection::Backward => r.start_container().clone(),
            SelectionDirection::None => r.end_container().clone(),
        })
    }

    /// Get the focus offset
    pub fn focus_offset(&self) -> usize {
        self.ranges
            .first()
            .map(|r| match self.direction {
                SelectionDirection::Forward => r.end_offset(),
                SelectionDirection::Backward => r.start_offset(),
                SelectionDirection::None => r.end_offset(),
            })
            .unwrap_or(0)
    }

    /// Check if the selection is collapsed (empty)
    pub fn is_collapsed(&self) -> bool {
        self.ranges.is_empty() || self.ranges.iter().all(|r| r.collapsed())
    }

    /// Get the number of ranges in the selection
    pub fn range_count(&self) -> usize {
        self.ranges.len()
    }

    /// Get a range at the specified index
    pub fn get_range_at(&self, index: usize) -> Option<&Range> {
        self.ranges.get(index)
    }

    /// Add a range to the selection
    ///
    /// # Errors
    ///
    /// Returns an error if the range is invalid.
    pub fn add_range(&mut self, range: Range) -> Result<(), DomException> {
        // Most browsers only support a single range
        // We'll replace any existing range
        self.ranges.clear();
        self.ranges.push(range);
        self.direction = SelectionDirection::Forward;
        Ok(())
    }

    /// Remove a range from the selection
    pub fn remove_range(&mut self, range: &Range) -> Result<(), DomException> {
        // Find and remove the range
        // Compare by boundary points
        self.ranges.retain(|r| {
            !(r.start_container().ptr_eq(range.start_container())
                && r.start_offset() == range.start_offset()
                && r.end_container().ptr_eq(range.end_container())
                && r.end_offset() == range.end_offset())
        });
        Ok(())
    }

    /// Remove all ranges from the selection
    pub fn remove_all_ranges(&mut self) {
        self.ranges.clear();
        self.direction = SelectionDirection::None;
    }

    /// Collapse the selection to a single point
    ///
    /// # Errors
    ///
    /// Returns an error if the node is invalid.
    pub fn collapse(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        // For now, create a document inline since owner_document doesn't exist on Node trait
        let doc = dom_core::Document::new();

        let mut range = Range::new(&doc);
        range.set_start(node.clone(), offset)?;
        range.set_end(node, offset)?;

        self.ranges.clear();
        self.ranges.push(range);
        self.direction = SelectionDirection::None;

        Ok(())
    }

    /// Extend the selection to a new position
    ///
    /// # Errors
    ///
    /// Returns an error if there is no selection or the node is invalid.
    pub fn extend(&mut self, node: NodeRef, offset: usize) -> Result<(), DomException> {
        if self.ranges.is_empty() {
            return Err(DomException::InvalidStateError);
        }

        let range = &mut self.ranges[0];
        range.set_end(node, offset)?;
        self.direction = SelectionDirection::Forward;

        Ok(())
    }

    /// Select all contents of the specified node
    ///
    /// # Errors
    ///
    /// Returns an error if the node is invalid.
    pub fn select_all_children(&mut self, node: NodeRef) -> Result<(), DomException> {
        // For now, create a document inline since owner_document doesn't exist on Node trait
        let doc = dom_core::Document::new();

        let mut range = Range::new(&doc);
        range.select_node_contents(node)?;

        self.ranges.clear();
        self.ranges.push(range);
        self.direction = SelectionDirection::Forward;

        Ok(())
    }

    /// Delete the selected content from the document
    pub fn delete_from_document(&mut self) -> Result<(), DomException> {
        for range in &mut self.ranges {
            range.delete_contents()?;
        }
        Ok(())
    }

    /// Get the string representation of the selection
    pub fn to_string(&self) -> String {
        self.ranges
            .iter()
            .filter_map(|r| {
                if r.collapsed() {
                    None
                } else {
                    // Get text content from range
                    // Simplified implementation
                    r.start_container().read().text_content()
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_core::{Document, Node, Text};
    use std::sync::Arc;
    use parking_lot::RwLock;

    fn create_text_node_ref(content: &str) -> NodeRef {
        let text = Text::new(content);
        Arc::new(RwLock::new(Box::new(text) as Box<dyn Node>))
    }

    #[test]
    fn test_selection_creation() {
        let selection = Selection::new();
        assert!(selection.is_collapsed());
        assert_eq!(selection.range_count(), 0);
    }

    #[test]
    fn test_selection_add_range() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");
        let mut range = Range::new(&doc);
        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref, 5).unwrap();

        let mut selection = Selection::new();
        selection.add_range(range).unwrap();

        assert_eq!(selection.range_count(), 1);
        assert!(!selection.is_collapsed());
    }

    #[test]
    fn test_selection_collapse() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");

        let mut selection = Selection::new();
        selection.collapse(text_ref, 5).unwrap();

        assert_eq!(selection.range_count(), 1);
        assert!(selection.is_collapsed());
        assert_eq!(selection.anchor_offset(), 5);
        assert_eq!(selection.focus_offset(), 5);
    }

    #[test]
    fn test_selection_remove_all_ranges() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello");
        let mut range = Range::new(&doc);
        range.set_start(text_ref.clone(), 0).unwrap();
        range.set_end(text_ref, 5).unwrap();

        let mut selection = Selection::new();
        selection.add_range(range).unwrap();

        assert_eq!(selection.range_count(), 1);

        selection.remove_all_ranges();

        assert_eq!(selection.range_count(), 0);
        assert!(selection.is_collapsed());
    }

    #[test]
    fn test_selection_extend() {
        let doc = Document::new();
        let text_ref = create_text_node_ref("Hello World");

        let mut selection = Selection::new();
        selection.collapse(text_ref.clone(), 0).unwrap();

        selection.extend(text_ref, 5).unwrap();

        assert!(!selection.is_collapsed());
        assert_eq!(selection.anchor_offset(), 0);
        assert_eq!(selection.focus_offset(), 5);
    }
}
