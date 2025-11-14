//! Element extensions for geometry and scrolling
//!
//! This module provides extension methods for Element to add geometry
//! measurement and scrolling capabilities.

use crate::geometry::{DOMRect, DOMRectList, ScrollIntoViewOptions};

/// Geometry and scrolling methods for Element
///
/// This trait extends Element with CSSOM View Module methods for
/// measuring element geometry and controlling scrolling.
pub trait ElementGeometryExt {
    /// Get the bounding box of the element relative to the viewport
    ///
    /// Returns a DOMRect representing the size and position of the element
    /// relative to the viewport. This includes the element's padding and borders
    /// but not margins.
    ///
    /// # Returns
    /// A DOMRect with the element's position and dimensions
    ///
    /// # Example
    /// ```ignore
    /// use dom_core::Element;
    /// use dom_advanced::ElementGeometryExt;
    ///
    /// let element = /* get element */;
    /// let rect = element.get_bounding_client_rect();
    /// println!("Element is at ({}, {}) with size {}x{}",
    ///          rect.x, rect.y, rect.width, rect.height);
    /// ```
    fn get_bounding_client_rect(&self) -> DOMRect;

    /// Get the list of CSS border boxes for the element
    ///
    /// For most elements, this returns a single DOMRect. For inline elements
    /// that span multiple lines, this returns multiple DOMRect objects, one
    /// for each line box.
    ///
    /// # Returns
    /// A DOMRectList containing one or more DOMRect objects
    ///
    /// # Example
    /// ```ignore
    /// use dom_core::Element;
    /// use dom_advanced::ElementGeometryExt;
    ///
    /// let element = /* get element */;
    /// let rects = element.get_client_rects();
    /// for i in 0..rects.length() {
    ///     if let Some(rect) = rects.item(i) {
    ///         println!("Box {}: ({}, {})", i, rect.x, rect.y);
    ///     }
    /// }
    /// ```
    fn get_client_rects(&self) -> DOMRectList;

    /// Scroll the element into the visible area of the viewport
    ///
    /// # Arguments
    /// * `options` - Configuration for the scroll behavior
    ///
    /// # Example
    /// ```ignore
    /// use dom_core::Element;
    /// use dom_advanced::{ElementGeometryExt, ScrollIntoViewOptions};
    /// use dom_advanced::{ScrollBehavior, ScrollLogicalPosition};
    ///
    /// let element = /* get element */;
    /// let options = ScrollIntoViewOptions {
    ///     behavior: ScrollBehavior::Smooth,
    ///     block: ScrollLogicalPosition::Center,
    ///     inline: ScrollLogicalPosition::Nearest,
    /// };
    /// element.scroll_into_view(options);
    /// ```
    fn scroll_into_view(&self, options: ScrollIntoViewOptions);
}

// Note: The actual implementation would be provided when both dom_core
// and dom_advanced are available. For now, we provide helper functions
// that can be used to implement these methods.

/// Helper: Create a default bounding rect (stub implementation)
pub fn get_default_bounding_rect() -> DOMRect {
    DOMRect::new(0.0, 0.0, 0.0, 0.0)
}

/// Helper: Create a default rect list (stub implementation)
pub fn get_default_client_rects() -> DOMRectList {
    DOMRectList::new()
}

/// Helper: Perform scroll into view operation (stub implementation)
pub fn perform_scroll_into_view(_options: &ScrollIntoViewOptions) {
    // Stub: In a full implementation, this would:
    // 1. Calculate the element's position relative to the viewport
    // 2. Determine the scroll offset needed
    // 3. Apply scrolling to ancestor scrollable containers
    // 4. Optionally animate the scroll if behavior is Smooth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{ScrollBehavior, ScrollLogicalPosition};

    #[test]
    fn test_get_default_bounding_rect() {
        let rect = get_default_bounding_rect();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    #[test]
    fn test_get_default_client_rects() {
        let rects = get_default_client_rects();
        assert_eq!(rects.length(), 0);
    }

    #[test]
    fn test_perform_scroll_into_view() {
        let options = ScrollIntoViewOptions {
            behavior: ScrollBehavior::Smooth,
            block: ScrollLogicalPosition::Center,
            inline: ScrollLogicalPosition::Nearest,
        };
        // Should not panic
        perform_scroll_into_view(&options);
    }

    #[test]
    fn test_perform_scroll_into_view_default() {
        let options = ScrollIntoViewOptions::default();
        // Should not panic with default options
        perform_scroll_into_view(&options);
    }
}
