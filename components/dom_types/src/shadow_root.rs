//! Shadow DOM mode enumeration.
//!
//! This module defines the [`ShadowRootMode`] enum which specifies whether
//! a shadow root is open (accessible from JavaScript) or closed (encapsulated).

/// Specifies the encapsulation mode for a shadow root.
///
/// When attaching a shadow root to an element using `attachShadow()`,
/// you must specify whether the shadow root should be "open" or "closed".
///
/// - **Open**: The shadow root is accessible from JavaScript outside the shadow tree
///   via `element.shadowRoot`.
/// - **Closed**: The shadow root is not accessible from JavaScript outside the shadow tree.
///   `element.shadowRoot` returns `null`.
///
/// # Examples
///
/// ```
/// use dom_types::ShadowRootMode;
///
/// let mode = ShadowRootMode::Open;
/// assert!(mode.is_open());
/// assert!(!mode.is_closed());
///
/// let closed_mode = ShadowRootMode::Closed;
/// assert!(closed_mode.is_closed());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ShadowRootMode {
    /// The shadow root is accessible via `element.shadowRoot`.
    ///
    /// This mode allows external JavaScript to access and manipulate
    /// the shadow DOM content.
    Open,

    /// The shadow root is not accessible via `element.shadowRoot`.
    ///
    /// This mode provides encapsulation by hiding the shadow DOM
    /// content from external JavaScript. The shadow root can only
    /// be accessed from within the shadow tree itself.
    Closed,
}

impl ShadowRootMode {
    /// Returns `true` if this is an open shadow root mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::ShadowRootMode;
    ///
    /// assert!(ShadowRootMode::Open.is_open());
    /// assert!(!ShadowRootMode::Closed.is_open());
    /// ```
    #[inline]
    pub fn is_open(self) -> bool {
        matches!(self, ShadowRootMode::Open)
    }

    /// Returns `true` if this is a closed shadow root mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::ShadowRootMode;
    ///
    /// assert!(ShadowRootMode::Closed.is_closed());
    /// assert!(!ShadowRootMode::Open.is_closed());
    /// ```
    #[inline]
    pub fn is_closed(self) -> bool {
        matches!(self, ShadowRootMode::Closed)
    }

    /// Returns the string representation of the mode.
    ///
    /// This matches the JavaScript API where `shadowRoot.mode` returns
    /// "open" or "closed".
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::ShadowRootMode;
    ///
    /// assert_eq!(ShadowRootMode::Open.as_str(), "open");
    /// assert_eq!(ShadowRootMode::Closed.as_str(), "closed");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self {
            ShadowRootMode::Open => "open",
            ShadowRootMode::Closed => "closed",
        }
    }
}

impl std::fmt::Display for ShadowRootMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ShadowRootMode {
    type Err = String;

    /// Parses a string into a `ShadowRootMode`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::ShadowRootMode;
    ///
    /// let open: ShadowRootMode = "open".parse().unwrap();
    /// assert_eq!(open, ShadowRootMode::Open);
    ///
    /// let closed: ShadowRootMode = "closed".parse().unwrap();
    /// assert_eq!(closed, ShadowRootMode::Closed);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(ShadowRootMode::Open),
            "closed" => Ok(ShadowRootMode::Closed),
            _ => Err(format!("Invalid shadow root mode: '{}'. Expected 'open' or 'closed'.", s)),
        }
    }
}

impl Default for ShadowRootMode {
    /// Returns the default shadow root mode, which is `Open`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::ShadowRootMode;
    ///
    /// let mode: ShadowRootMode = Default::default();
    /// assert_eq!(mode, ShadowRootMode::Open);
    /// ```
    fn default() -> Self {
        ShadowRootMode::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open() {
        assert!(ShadowRootMode::Open.is_open());
        assert!(!ShadowRootMode::Closed.is_open());
    }

    #[test]
    fn test_is_closed() {
        assert!(ShadowRootMode::Closed.is_closed());
        assert!(!ShadowRootMode::Open.is_closed());
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ShadowRootMode::Open.as_str(), "open");
        assert_eq!(ShadowRootMode::Closed.as_str(), "closed");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", ShadowRootMode::Open), "open");
        assert_eq!(format!("{}", ShadowRootMode::Closed), "closed");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("open".parse::<ShadowRootMode>().unwrap(), ShadowRootMode::Open);
        assert_eq!("closed".parse::<ShadowRootMode>().unwrap(), ShadowRootMode::Closed);
        assert_eq!("OPEN".parse::<ShadowRootMode>().unwrap(), ShadowRootMode::Open);
        assert_eq!("Closed".parse::<ShadowRootMode>().unwrap(), ShadowRootMode::Closed);
        assert!("invalid".parse::<ShadowRootMode>().is_err());
    }

    #[test]
    fn test_default() {
        let mode: ShadowRootMode = Default::default();
        assert_eq!(mode, ShadowRootMode::Open);
    }

    #[test]
    fn test_clone_copy() {
        let mode = ShadowRootMode::Open;
        let copied = mode;
        let cloned = mode.clone();
        assert_eq!(mode, copied);
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ShadowRootMode::Open);
        set.insert(ShadowRootMode::Closed);
        set.insert(ShadowRootMode::Open); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&ShadowRootMode::Open));
        assert!(set.contains(&ShadowRootMode::Closed));
    }

    #[test]
    fn test_serialization() {
        let mode = ShadowRootMode::Open;
        let json = serde_json::to_string(&mode).unwrap();
        let deserialized: ShadowRootMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, deserialized);

        let closed = ShadowRootMode::Closed;
        let json = serde_json::to_string(&closed).unwrap();
        let deserialized: ShadowRootMode = serde_json::from_str(&json).unwrap();
        assert_eq!(closed, deserialized);
    }
}
