//! DOM Geometry APIs - DOMRect, DOMRectReadOnly, DOMRectList
//!
//! Implements the CSSOM View Module geometry interfaces for
//! measuring element positions and sizes.

use parking_lot::RwLock;
use std::sync::Arc;

/// DOMRectReadOnly represents an immutable rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DOMRectReadOnly {
    /// X coordinate of the rectangle's origin
    pub x: f64,
    /// Y coordinate of the rectangle's origin
    pub y: f64,
    /// Width of the rectangle
    pub width: f64,
    /// Height of the rectangle
    pub height: f64,
}

impl DOMRectReadOnly {
    /// Create a new DOMRectReadOnly
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the top coordinate (y)
    pub fn top(&self) -> f64 {
        self.y.min(self.y + self.height)
    }

    /// Get the right coordinate (x + width)
    pub fn right(&self) -> f64 {
        self.x.max(self.x + self.width)
    }

    /// Get the bottom coordinate (y + height)
    pub fn bottom(&self) -> f64 {
        self.y.max(self.y + self.height)
    }

    /// Get the left coordinate (x)
    pub fn left(&self) -> f64 {
        self.x.min(self.x + self.width)
    }

    /// Convert to a DOMRect (mutable version)
    pub fn to_dom_rect(&self) -> DOMRect {
        DOMRect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl Default for DOMRectReadOnly {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

/// DOMRect represents a mutable rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DOMRect {
    /// X coordinate of the rectangle's origin
    pub x: f64,
    /// Y coordinate of the rectangle's origin
    pub y: f64,
    /// Width of the rectangle
    pub width: f64,
    /// Height of the rectangle
    pub height: f64,
}

impl DOMRect {
    /// Create a new DOMRect
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Get the top coordinate (y)
    pub fn top(&self) -> f64 {
        self.y.min(self.y + self.height)
    }

    /// Get the right coordinate (x + width)
    pub fn right(&self) -> f64 {
        self.x.max(self.x + self.width)
    }

    /// Get the bottom coordinate (y + height)
    pub fn bottom(&self) -> f64 {
        self.y.max(self.y + self.height)
    }

    /// Get the left coordinate (x)
    pub fn left(&self) -> f64 {
        self.x.min(self.x + self.width)
    }

    /// Convert to a DOMRectReadOnly (immutable version)
    pub fn to_dom_rect_read_only(&self) -> DOMRectReadOnly {
        DOMRectReadOnly {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    /// Create a DOMRect from LTRB (left, top, right, bottom) coordinates
    pub fn from_rect(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }
    }
}

impl Default for DOMRect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<DOMRectReadOnly> for DOMRect {
    fn from(rect: DOMRectReadOnly) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl From<DOMRect> for DOMRectReadOnly {
    fn from(rect: DOMRect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

/// DOMRectList represents a list of DOMRect objects
pub type DOMRectListRef = Arc<RwLock<DOMRectList>>;

/// DOMRectList is a collection of DOMRect objects
#[derive(Debug, Clone, Default)]
pub struct DOMRectList {
    rects: Vec<DOMRect>,
}

impl DOMRectList {
    /// Create a new empty DOMRectList
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    /// Create a DOMRectList from a vector of DOMRect
    pub fn from_vec(rects: Vec<DOMRect>) -> Self {
        Self { rects }
    }

    /// Get the number of rectangles in the list
    pub fn length(&self) -> usize {
        self.rects.len()
    }

    /// Get a rectangle by index
    pub fn item(&self, index: usize) -> Option<&DOMRect> {
        self.rects.get(index)
    }

    /// Add a rectangle to the list
    pub fn push(&mut self, rect: DOMRect) {
        self.rects.push(rect);
    }

    /// Get an iterator over the rectangles
    pub fn iter(&self) -> impl Iterator<Item = &DOMRect> {
        self.rects.iter()
    }
}

/// ScrollIntoViewOptions for configuring scroll behavior
#[derive(Debug, Clone, Default)]
pub struct ScrollIntoViewOptions {
    /// Scroll behavior: "auto" or "smooth"
    pub behavior: ScrollBehavior,
    /// Vertical alignment: "start", "center", "end", or "nearest"
    pub block: ScrollLogicalPosition,
    /// Horizontal alignment: "start", "center", "end", or "nearest"
    pub inline: ScrollLogicalPosition,
}

/// Scroll behavior enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBehavior {
    /// Scroll immediately
    #[default]
    Auto,
    /// Scroll smoothly with animation
    Smooth,
}

/// Scroll logical position enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollLogicalPosition {
    /// Align to start edge
    Start,
    /// Align to center
    Center,
    /// Align to end edge
    End,
    /// Use nearest edge
    #[default]
    Nearest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_rect_readonly_creation() {
        let rect = DOMRectReadOnly::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_dom_rect_readonly_coordinates() {
        let rect = DOMRectReadOnly::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.bottom(), 70.0);
    }

    #[test]
    fn test_dom_rect_readonly_negative_dimensions() {
        let rect = DOMRectReadOnly::new(10.0, 20.0, -100.0, -50.0);
        // With negative width/height, left/top should be min values
        assert_eq!(rect.left(), -90.0);
        assert_eq!(rect.top(), -30.0);
        assert_eq!(rect.right(), 10.0);
        assert_eq!(rect.bottom(), 20.0);
    }

    #[test]
    fn test_dom_rect_creation() {
        let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_dom_rect_coordinates() {
        let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.left(), 10.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.bottom(), 70.0);
    }

    #[test]
    fn test_dom_rect_from_ltrb() {
        let rect = DOMRect::from_rect(10.0, 20.0, 110.0, 70.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_dom_rect_default() {
        let rect = DOMRect::default();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
    }

    #[test]
    fn test_dom_rect_conversion() {
        let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
        let readonly = rect.to_dom_rect_read_only();
        assert_eq!(readonly.x, 10.0);
        assert_eq!(readonly.y, 20.0);
        assert_eq!(readonly.width, 100.0);
        assert_eq!(readonly.height, 50.0);

        let rect2 = readonly.to_dom_rect();
        assert_eq!(rect, rect2);
    }

    #[test]
    fn test_dom_rect_list_creation() {
        let list = DOMRectList::new();
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_dom_rect_list_push_and_item() {
        let mut list = DOMRectList::new();
        list.push(DOMRect::new(10.0, 20.0, 100.0, 50.0));
        list.push(DOMRect::new(5.0, 15.0, 80.0, 40.0));

        assert_eq!(list.length(), 2);
        assert_eq!(list.item(0).unwrap().x, 10.0);
        assert_eq!(list.item(1).unwrap().x, 5.0);
        assert!(list.item(2).is_none());
    }

    #[test]
    fn test_dom_rect_list_from_vec() {
        let rects = vec![
            DOMRect::new(10.0, 20.0, 100.0, 50.0),
            DOMRect::new(5.0, 15.0, 80.0, 40.0),
        ];
        let list = DOMRectList::from_vec(rects);
        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_dom_rect_list_iter() {
        let mut list = DOMRectList::new();
        list.push(DOMRect::new(10.0, 20.0, 100.0, 50.0));
        list.push(DOMRect::new(5.0, 15.0, 80.0, 40.0));

        let count = list.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_scroll_into_view_options_default() {
        let options = ScrollIntoViewOptions::default();
        assert_eq!(options.behavior, ScrollBehavior::Auto);
        assert_eq!(options.block, ScrollLogicalPosition::Nearest);
        assert_eq!(options.inline, ScrollLogicalPosition::Nearest);
    }

    #[test]
    fn test_scroll_behavior() {
        assert_ne!(ScrollBehavior::Auto, ScrollBehavior::Smooth);
    }

    #[test]
    fn test_scroll_logical_position() {
        assert_ne!(ScrollLogicalPosition::Start, ScrollLogicalPosition::End);
        assert_ne!(ScrollLogicalPosition::Center, ScrollLogicalPosition::Nearest);
    }
}
