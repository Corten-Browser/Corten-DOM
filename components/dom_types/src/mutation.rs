//! Mutation type enumeration for MutationObserver.
//!
//! This module defines the [`MutationType`] enum which represents the different
//! types of mutations that can be observed by a MutationObserver.

/// Types of mutations that can be observed by MutationObserver.
///
/// When creating a MutationObserver, you specify which types of mutations
/// to observe. The MutationRecord's `type` property indicates which kind
/// of mutation occurred.
///
/// # Mutation Types
///
/// - **Attributes**: Changes to element attributes (e.g., `class`, `id`, `style`)
/// - **CharacterData**: Changes to text content of Text or Comment nodes
/// - **ChildList**: Addition or removal of child nodes
///
/// # Examples
///
/// ```
/// use dom_types::MutationType;
///
/// let mutation_type = MutationType::Attributes;
/// assert_eq!(mutation_type.as_str(), "attributes");
///
/// let parsed: MutationType = "childList".parse().unwrap();
/// assert_eq!(parsed, MutationType::ChildList);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MutationType {
    /// An attribute on an element was modified, added, or removed.
    ///
    /// This includes any attribute change, such as:
    /// - `element.setAttribute("class", "new-class")`
    /// - `element.removeAttribute("id")`
    /// - `element.className = "different"`
    Attributes,

    /// The text content of a Text or Comment node was modified.
    ///
    /// This occurs when:
    /// - `textNode.data = "new text"`
    /// - `textNode.textContent = "new content"`
    CharacterData,

    /// Child nodes were added to or removed from an element.
    ///
    /// This includes:
    /// - `parent.appendChild(child)`
    /// - `parent.removeChild(child)`
    /// - `parent.insertBefore(newChild, existingChild)`
    /// - `parent.replaceChild(newChild, oldChild)`
    ChildList,
}

impl MutationType {
    /// Returns the string representation of the mutation type.
    ///
    /// This matches the `MutationRecord.type` property values in the DOM API.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// assert_eq!(MutationType::Attributes.as_str(), "attributes");
    /// assert_eq!(MutationType::CharacterData.as_str(), "characterData");
    /// assert_eq!(MutationType::ChildList.as_str(), "childList");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self {
            MutationType::Attributes => "attributes",
            MutationType::CharacterData => "characterData",
            MutationType::ChildList => "childList",
        }
    }

    /// Returns `true` if this is an attributes mutation.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// assert!(MutationType::Attributes.is_attributes());
    /// assert!(!MutationType::ChildList.is_attributes());
    /// ```
    #[inline]
    pub fn is_attributes(self) -> bool {
        matches!(self, MutationType::Attributes)
    }

    /// Returns `true` if this is a character data mutation.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// assert!(MutationType::CharacterData.is_character_data());
    /// assert!(!MutationType::Attributes.is_character_data());
    /// ```
    #[inline]
    pub fn is_character_data(self) -> bool {
        matches!(self, MutationType::CharacterData)
    }

    /// Returns `true` if this is a child list mutation.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// assert!(MutationType::ChildList.is_child_list());
    /// assert!(!MutationType::Attributes.is_child_list());
    /// ```
    #[inline]
    pub fn is_child_list(self) -> bool {
        matches!(self, MutationType::ChildList)
    }

    /// Returns all mutation types as an array.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// let all = MutationType::all();
    /// assert_eq!(all.len(), 3);
    /// ```
    pub fn all() -> [MutationType; 3] {
        [
            MutationType::Attributes,
            MutationType::CharacterData,
            MutationType::ChildList,
        ]
    }
}

impl std::fmt::Display for MutationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MutationType {
    type Err = String;

    /// Parses a string into a `MutationType`.
    ///
    /// The parsing is case-sensitive to match the DOM API.
    ///
    /// # Examples
    ///
    /// ```
    /// use dom_types::MutationType;
    ///
    /// let attr: MutationType = "attributes".parse().unwrap();
    /// assert_eq!(attr, MutationType::Attributes);
    ///
    /// let char_data: MutationType = "characterData".parse().unwrap();
    /// assert_eq!(char_data, MutationType::CharacterData);
    ///
    /// let child_list: MutationType = "childList".parse().unwrap();
    /// assert_eq!(child_list, MutationType::ChildList);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attributes" => Ok(MutationType::Attributes),
            "characterData" => Ok(MutationType::CharacterData),
            "childList" => Ok(MutationType::ChildList),
            _ => Err(format!(
                "Invalid mutation type: '{}'. Expected 'attributes', 'characterData', or 'childList'.",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(MutationType::Attributes.as_str(), "attributes");
        assert_eq!(MutationType::CharacterData.as_str(), "characterData");
        assert_eq!(MutationType::ChildList.as_str(), "childList");
    }

    #[test]
    fn test_is_attributes() {
        assert!(MutationType::Attributes.is_attributes());
        assert!(!MutationType::CharacterData.is_attributes());
        assert!(!MutationType::ChildList.is_attributes());
    }

    #[test]
    fn test_is_character_data() {
        assert!(!MutationType::Attributes.is_character_data());
        assert!(MutationType::CharacterData.is_character_data());
        assert!(!MutationType::ChildList.is_character_data());
    }

    #[test]
    fn test_is_child_list() {
        assert!(!MutationType::Attributes.is_child_list());
        assert!(!MutationType::CharacterData.is_child_list());
        assert!(MutationType::ChildList.is_child_list());
    }

    #[test]
    fn test_all() {
        let all = MutationType::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&MutationType::Attributes));
        assert!(all.contains(&MutationType::CharacterData));
        assert!(all.contains(&MutationType::ChildList));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", MutationType::Attributes), "attributes");
        assert_eq!(format!("{}", MutationType::CharacterData), "characterData");
        assert_eq!(format!("{}", MutationType::ChildList), "childList");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            "attributes".parse::<MutationType>().unwrap(),
            MutationType::Attributes
        );
        assert_eq!(
            "characterData".parse::<MutationType>().unwrap(),
            MutationType::CharacterData
        );
        assert_eq!(
            "childList".parse::<MutationType>().unwrap(),
            MutationType::ChildList
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("Attributes".parse::<MutationType>().is_err());
        assert!("ATTRIBUTES".parse::<MutationType>().is_err());
        assert!("invalid".parse::<MutationType>().is_err());
        assert!("child_list".parse::<MutationType>().is_err());
    }

    #[test]
    fn test_clone_copy() {
        let mutation_type = MutationType::Attributes;
        let copied = mutation_type;
        let cloned = mutation_type.clone();
        assert_eq!(mutation_type, copied);
        assert_eq!(mutation_type, cloned);
    }

    #[test]
    fn test_eq_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(MutationType::Attributes);
        set.insert(MutationType::CharacterData);
        set.insert(MutationType::ChildList);
        set.insert(MutationType::Attributes); // Duplicate

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_serialization() {
        for mutation_type in MutationType::all() {
            let json = serde_json::to_string(&mutation_type).unwrap();
            let deserialized: MutationType = serde_json::from_str(&json).unwrap();
            assert_eq!(mutation_type, deserialized);
        }
    }
}
