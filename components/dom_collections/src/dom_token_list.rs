//! DOMTokenList implementation (for class lists, etc.)

use dom_core::ElementRef;
use dom_types::DomException;
use std::collections::HashSet;
use std::sync::Weak;

/// DOMTokenList manages space-separated tokens (like CSS classes)
///
/// This is typically used for class names but can be used for any
/// space-separated attribute values.
pub struct DOMTokenList {
    /// Weak reference to the element
    element: Weak<parking_lot::RwLock<dom_core::Element>>,

    /// Attribute name (e.g., "class")
    attribute_name: String,
}

impl DOMTokenList {
    /// Creates a new DOMTokenList
    pub fn new(element: ElementRef, attribute_name: impl Into<String>) -> Self {
        DOMTokenList {
            element: std::sync::Arc::downgrade(&element),
            attribute_name: attribute_name.into(),
        }
    }

    /// Gets the current token list as a HashSet
    fn get_tokens(&self) -> HashSet<String> {
        if let Some(element) = self.element.upgrade() {
            if let Some(value) = element.read().get_attribute(&self.attribute_name) {
                return value.split_whitespace().map(|s| s.to_string()).collect();
            }
        }
        HashSet::new()
    }

    /// Sets the token list from a HashSet
    fn set_tokens(&mut self, tokens: &HashSet<String>) -> Result<(), DomException> {
        if let Some(element) = self.element.upgrade() {
            let mut tokens_vec: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
            tokens_vec.sort(); // Maintain consistent order
            let value = tokens_vec.join(" ");
            element.write().set_attribute(&self.attribute_name, value)?;
        }
        Ok(())
    }

    /// Returns the number of tokens
    pub fn length(&self) -> usize {
        if let Some(element) = self.element.upgrade() {
            if let Some(value) = element.read().get_attribute(&self.attribute_name) {
                return value.split_whitespace().count();
            }
        }
        0
    }

    /// Returns the token at the given index
    pub fn item(&self, index: usize) -> Option<String> {
        if let Some(element) = self.element.upgrade() {
            if let Some(value) = element.read().get_attribute(&self.attribute_name) {
                return value.split_whitespace().nth(index).map(|s| s.to_string());
            }
        }
        None
    }

    /// Checks if the token exists
    pub fn contains(&self, token: &str) -> bool {
        if let Some(element) = self.element.upgrade() {
            if let Some(value) = element.read().get_attribute(&self.attribute_name) {
                return value.split_whitespace().any(|t| t == token);
            }
        }
        false
    }

    /// Validates a token (no whitespace allowed)
    fn validate_token(token: &str) -> Result<(), DomException> {
        if token.is_empty() {
            return Err(DomException::syntax_error("Token cannot be empty"));
        }
        if token.contains(char::is_whitespace) {
            return Err(DomException::InvalidCharacterError);
        }
        Ok(())
    }

    /// Adds tokens
    pub fn add(&mut self, tokens: &[&str]) -> Result<(), DomException> {
        // Validate all tokens first
        for token in tokens {
            Self::validate_token(token)?;
        }

        let mut current_tokens = self.get_tokens();

        for token in tokens {
            current_tokens.insert(token.to_string());
        }

        self.set_tokens(&current_tokens)?;
        Ok(())
    }

    /// Removes tokens
    pub fn remove(&mut self, tokens: &[&str]) -> Result<(), DomException> {
        // Validate all tokens first
        for token in tokens {
            Self::validate_token(token)?;
        }

        let mut current_tokens = self.get_tokens();

        for token in tokens {
            current_tokens.remove(*token);
        }

        self.set_tokens(&current_tokens)?;
        Ok(())
    }

    /// Toggles a token
    pub fn toggle(&mut self, token: &str, force: Option<bool>) -> Result<bool, DomException> {
        Self::validate_token(token)?;

        let mut current_tokens = self.get_tokens();

        let result = if let Some(forced) = force {
            if forced {
                current_tokens.insert(token.to_string());
                true
            } else {
                current_tokens.remove(token);
                false
            }
        } else if current_tokens.contains(token) {
            current_tokens.remove(token);
            false
        } else {
            current_tokens.insert(token.to_string());
            true
        };

        self.set_tokens(&current_tokens)?;
        Ok(result)
    }

    /// Replaces a token
    pub fn replace(&mut self, old_token: &str, new_token: &str) -> Result<bool, DomException> {
        Self::validate_token(old_token)?;
        Self::validate_token(new_token)?;

        let mut current_tokens = self.get_tokens();

        if current_tokens.contains(old_token) {
            current_tokens.remove(old_token);
            current_tokens.insert(new_token.to_string());
            self.set_tokens(&current_tokens)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
