//! Geometry types for DOM bounding rectangles.
//!
//! This module defines the [`DOMRect`] and [`DOMRectList`] types used for
//! representing element bounding boxes and geometry information.

/// A rectangle representing element geometry with x, y, width, and height.
///
/// `DOMRect` provides the geometry of an element's bounding box with
/// computed properties for the edges (top, right, bottom, left).
///
/// # Examples
///
/// ```
/// use dom_types::DOMRect;
///
/// let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
/// assert_eq!(rect.x, 10.0);
/// assert_eq!(rect.y, 20.0);
/// assert_eq!(rect.top(), 20.0);
/// assert_eq!(rect.right(), 110.0);
/// assert_eq!(rect.bottom(), 70.0);
/// assert_eq!(rect.left(), 10.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DOMRect {
    /// The x coordinate of the rectangle's origin.
    pub x: f64,
    /// The y coordinate of the rectangle's origin.
    pub y: f64,
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
}

impl DOMRect {
    /// Creates a new `DOMRect` with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the rectangle's origin
    /// * `y` - The y coordinate of the rectangle's origin
    /// * `width` - The width of the rectangle
    /// * `height` - The height of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(0.0, 0.0, 100.0, 200.0);
    /// assert_eq!(rect.width, 100.0);
    /// assert_eq!(rect.height, 200.0);
    /// ```
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    /// Creates an empty `DOMRect` at the origin with zero dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::empty();
    /// assert_eq!(rect.x, 0.0);
    /// assert_eq!(rect.width, 0.0);
    /// ```
    pub fn empty() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Returns the top edge coordinate (same as y).
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
    /// assert_eq!(rect.top(), 20.0);
    /// ```
    #[inline]
    pub fn top(&self) -> f64 {
        self.y
    }

    /// Returns the right edge coordinate (x + width).
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
    /// assert_eq!(rect.right(), 110.0);
    /// ```
    #[inline]
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Returns the bottom edge coordinate (y + height).
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
    /// assert_eq!(rect.bottom(), 70.0);
    /// ```
    #[inline]
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    /// Returns the left edge coordinate (same as x).
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
    /// assert_eq!(rect.left(), 10.0);
    /// ```
    #[inline]
    pub fn left(&self) -> f64 {
        self.x
    }

    /// Checks if this rectangle is empty (has zero area).
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// assert!(DOMRect::empty().is_empty());
    /// assert!(!DOMRect::new(0.0, 0.0, 10.0, 10.0).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.width == 0.0 || self.height == 0.0
    }

    /// Returns the area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(0.0, 0.0, 10.0, 20.0);
    /// assert_eq!(rect.area(), 200.0);
    /// ```
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Checks if a point is contained within the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRect;
    ///
    /// let rect = DOMRect::new(0.0, 0.0, 100.0, 100.0);
    /// assert!(rect.contains_point(50.0, 50.0));
    /// assert!(!rect.contains_point(150.0, 50.0));
    /// ```
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.left() && x <= self.right() && y >= self.top() && y <= self.bottom()
    }
}

impl Default for DOMRect {
    fn default() -> Self {
        Self::empty()
    }
}

/// A list of `DOMRect` objects.
///
/// `DOMRectList` is used to represent multiple bounding rectangles,
/// typically returned by methods like `getClientRects()`.
///
/// # Examples
///
/// ```
/// use dom_types::{DOMRect, DOMRectList};
///
/// let rects = vec![
///     DOMRect::new(0.0, 0.0, 100.0, 50.0),
///     DOMRect::new(0.0, 50.0, 100.0, 50.0),
/// ];
/// let list = DOMRectList::new(rects);
/// assert_eq!(list.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct DOMRectList(Vec<DOMRect>);

impl DOMRectList {
    /// Creates a new `DOMRectList` from a vector of `DOMRect`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::{DOMRect, DOMRectList};
    ///
    /// let list = DOMRectList::new(vec![DOMRect::empty()]);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn new(rects: Vec<DOMRect>) -> Self {
        Self(rects)
    }

    /// Creates an empty `DOMRectList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRectList;
    ///
    /// let list = DOMRectList::empty();
    /// assert!(list.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Returns the number of rectangles in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRectList;
    ///
    /// let list = DOMRectList::empty();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the list contains no rectangles.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::DOMRectList;
    ///
    /// let list = DOMRectList::empty();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the rectangle at the given index, or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::{DOMRect, DOMRectList};
    ///
    /// let list = DOMRectList::new(vec![DOMRect::new(10.0, 20.0, 30.0, 40.0)]);
    /// assert!(list.item(0).is_some());
    /// assert!(list.item(1).is_none());
    /// ```
    pub fn item(&self, index: usize) -> Option<&DOMRect> {
        self.0.get(index)
    }

    /// Returns an iterator over the rectangles.
    pub fn iter(&self) -> impl Iterator<Item = &DOMRect> {
        self.0.iter()
    }
}

impl From<Vec<DOMRect>> for DOMRectList {
    fn from(rects: Vec<DOMRect>) -> Self {
        Self::new(rects)
    }
}

impl IntoIterator for DOMRectList {
    type Item = DOMRect;
    type IntoIter = std::vec::IntoIter<DOMRect>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a DOMRectList {
    type Item = &'a DOMRect;
    type IntoIter = std::slice::Iter<'a, DOMRect>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_rect_new() {
        let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
    }

    #[test]
    fn test_dom_rect_edges() {
        let rect = DOMRect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.bottom(), 70.0);
        assert_eq!(rect.left(), 10.0);
    }

    #[test]
    fn test_dom_rect_empty() {
        let rect = DOMRect::empty();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 0.0);
        assert_eq!(rect.height, 0.0);
        assert!(rect.is_empty());
    }

    #[test]
    fn test_dom_rect_area() {
        let rect = DOMRect::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
    }

    #[test]
    fn test_dom_rect_contains_point() {
        let rect = DOMRect::new(0.0, 0.0, 100.0, 100.0);
        assert!(rect.contains_point(50.0, 50.0));
        assert!(rect.contains_point(0.0, 0.0));
        assert!(rect.contains_point(100.0, 100.0));
        assert!(!rect.contains_point(-1.0, 50.0));
        assert!(!rect.contains_point(101.0, 50.0));
    }

    #[test]
    fn test_dom_rect_default() {
        let rect: DOMRect = Default::default();
        assert!(rect.is_empty());
    }

    #[test]
    fn test_dom_rect_clone_copy() {
        let rect = DOMRect::new(1.0, 2.0, 3.0, 4.0);
        let copied = rect;
        let cloned = rect.clone();
        assert_eq!(rect, copied);
        assert_eq!(rect, cloned);
    }

    #[test]
    fn test_dom_rect_list_new() {
        let rects = vec![
            DOMRect::new(0.0, 0.0, 100.0, 50.0),
            DOMRect::new(0.0, 50.0, 100.0, 50.0),
        ];
        let list = DOMRectList::new(rects);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_dom_rect_list_empty() {
        let list = DOMRectList::empty();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_dom_rect_list_item() {
        let list = DOMRectList::new(vec![DOMRect::new(10.0, 20.0, 30.0, 40.0)]);
        let item = list.item(0).unwrap();
        assert_eq!(item.x, 10.0);
        assert!(list.item(1).is_none());
    }

    #[test]
    fn test_dom_rect_list_iter() {
        let rects = vec![
            DOMRect::new(0.0, 0.0, 10.0, 10.0),
            DOMRect::new(10.0, 10.0, 20.0, 20.0),
        ];
        let list = DOMRectList::new(rects);
        let collected: Vec<_> = list.iter().collect();
        assert_eq!(collected.len(), 2);
    }

    #[test]
    fn test_dom_rect_list_from_vec() {
        let rects = vec![DOMRect::empty()];
        let list: DOMRectList = rects.into();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_dom_rect_serialization() {
        let rect = DOMRect::new(1.0, 2.0, 3.0, 4.0);
        let json = serde_json::to_string(&rect).unwrap();
        let deserialized: DOMRect = serde_json::from_str(&json).unwrap();
        assert_eq!(rect, deserialized);
    }
}
