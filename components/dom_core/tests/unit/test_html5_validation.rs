//! Tests for HTML5 validation rules
//!
//! These tests verify that HTML5 tag names, attribute names, and values
//! are validated according to the HTML5 specification.

use dom_core::html5_validation::{
    is_raw_text_element, is_void_element, validate_html5_attribute_name,
    validate_html5_attribute_value, validate_html5_tag_name,
};
use dom_types::DomException;

// ============================================================================
// Tag Name Validation Tests
// ============================================================================

#[test]
fn test_valid_standard_tag_names() {
    assert!(validate_html5_tag_name("div").is_ok());
    assert!(validate_html5_tag_name("span").is_ok());
    assert!(validate_html5_tag_name("p").is_ok());
    assert!(validate_html5_tag_name("a").is_ok());
    assert!(validate_html5_tag_name("table").is_ok());
    assert!(validate_html5_tag_name("section").is_ok());
    assert!(validate_html5_tag_name("article").is_ok());
    assert!(validate_html5_tag_name("nav").is_ok());
    assert!(validate_html5_tag_name("header").is_ok());
    assert!(validate_html5_tag_name("footer").is_ok());
}

#[test]
fn test_valid_custom_element_names() {
    // Custom elements must contain a hyphen
    assert!(validate_html5_tag_name("my-element").is_ok());
    assert!(validate_html5_tag_name("custom-button").is_ok());
    assert!(validate_html5_tag_name("app-header").is_ok());
    assert!(validate_html5_tag_name("x-foo").is_ok());
    assert!(validate_html5_tag_name("my-custom-web-component").is_ok());
}

#[test]
fn test_invalid_custom_element_names_starting_with_xml() {
    // Cannot start with "xml" (case-insensitive)
    assert_eq!(
        validate_html5_tag_name("xml-element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("XML-element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("Xml-element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("xMl-element"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_invalid_tag_names_with_invalid_characters() {
    // Must be ASCII alphanumeric + hyphens only
    assert_eq!(
        validate_html5_tag_name("my element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("my@element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("my$element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("my/element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("my>element"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_invalid_empty_tag_name() {
    assert_eq!(
        validate_html5_tag_name(""),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_invalid_tag_names_starting_with_digit() {
    // Tag names cannot start with digits
    assert_eq!(
        validate_html5_tag_name("123-element"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("9div"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_tag_name_case_sensitivity() {
    // HTML5 tag names are case-insensitive but should accept various cases
    assert!(validate_html5_tag_name("DIV").is_ok());
    assert!(validate_html5_tag_name("Div").is_ok());
    assert!(validate_html5_tag_name("dIv").is_ok());
    assert!(validate_html5_tag_name("MY-ELEMENT").is_ok());
}

// ============================================================================
// Attribute Name Validation Tests
// ============================================================================

#[test]
fn test_valid_standard_attribute_names() {
    assert!(validate_html5_attribute_name("id").is_ok());
    assert!(validate_html5_attribute_name("class").is_ok());
    assert!(validate_html5_attribute_name("title").is_ok());
    assert!(validate_html5_attribute_name("href").is_ok());
    assert!(validate_html5_attribute_name("src").is_ok());
    assert!(validate_html5_attribute_name("alt").is_ok());
    assert!(validate_html5_attribute_name("type").is_ok());
    assert!(validate_html5_attribute_name("name").is_ok());
}

#[test]
fn test_valid_data_attributes() {
    assert!(validate_html5_attribute_name("data-id").is_ok());
    assert!(validate_html5_attribute_name("data-value").is_ok());
    assert!(validate_html5_attribute_name("data-custom-prop").is_ok());
    assert!(validate_html5_attribute_name("data-123").is_ok());
}

#[test]
fn test_valid_aria_attributes() {
    assert!(validate_html5_attribute_name("aria-label").is_ok());
    assert!(validate_html5_attribute_name("aria-hidden").is_ok());
    assert!(validate_html5_attribute_name("aria-describedby").is_ok());
    assert!(validate_html5_attribute_name("aria-live").is_ok());
}

#[test]
fn test_invalid_attribute_names_with_forbidden_characters() {
    // Must not contain: space, ", ', >, /, =
    assert_eq!(
        validate_html5_attribute_name("my attr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my\"attr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my'attr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my>attr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my/attr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my=attr"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_invalid_empty_attribute_name() {
    assert_eq!(
        validate_html5_attribute_name(""),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_attribute_name_with_control_characters() {
    assert_eq!(
        validate_html5_attribute_name("my\nattr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my\tattr"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_attribute_name("my\rattr"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_attribute_name_case_preservation() {
    // Attribute names are case-insensitive but case should be preserved
    assert!(validate_html5_attribute_name("ID").is_ok());
    assert!(validate_html5_attribute_name("Class").is_ok());
    assert!(validate_html5_attribute_name("data-MyValue").is_ok());
}

// ============================================================================
// Attribute Value Validation Tests
// ============================================================================

#[test]
fn test_valid_attribute_values() {
    assert!(validate_html5_attribute_value("id", "main").is_ok());
    assert!(validate_html5_attribute_value("class", "button primary").is_ok());
    assert!(validate_html5_attribute_value("href", "https://example.com").is_ok());
    assert!(validate_html5_attribute_value("title", "Click me!").is_ok());
    assert!(validate_html5_attribute_value("data-value", "123").is_ok());
}

#[test]
fn test_valid_empty_attribute_value() {
    // Empty values are allowed for most attributes
    assert!(validate_html5_attribute_value("class", "").is_ok());
    assert!(validate_html5_attribute_value("title", "").is_ok());
}

#[test]
fn test_attribute_value_with_special_characters() {
    // Most special characters are allowed in values
    assert!(validate_html5_attribute_value("data-json", r#"{"key":"value"}"#).is_ok());
    assert!(validate_html5_attribute_value("title", "Line 1\nLine 2").is_ok());
}

#[test]
fn test_attribute_value_validation_is_lenient() {
    // HTML5 attribute values are generally very permissive
    // Only specific attributes have restrictions
    assert!(validate_html5_attribute_value("custom", "<script>").is_ok());
    assert!(validate_html5_attribute_value("custom", "ANY VALUE").is_ok());
}

// ============================================================================
// Void Elements Tests
// ============================================================================

#[test]
fn test_void_elements() {
    // All HTML5 void elements
    assert!(is_void_element("area"));
    assert!(is_void_element("base"));
    assert!(is_void_element("br"));
    assert!(is_void_element("col"));
    assert!(is_void_element("embed"));
    assert!(is_void_element("hr"));
    assert!(is_void_element("img"));
    assert!(is_void_element("input"));
    assert!(is_void_element("link"));
    assert!(is_void_element("meta"));
    assert!(is_void_element("source"));
    assert!(is_void_element("track"));
    assert!(is_void_element("wbr"));
}

#[test]
fn test_void_elements_case_insensitive() {
    assert!(is_void_element("BR"));
    assert!(is_void_element("Img"));
    assert!(is_void_element("INPUT"));
    assert!(is_void_element("Hr"));
}

#[test]
fn test_non_void_elements() {
    assert!(!is_void_element("div"));
    assert!(!is_void_element("span"));
    assert!(!is_void_element("p"));
    assert!(!is_void_element("script"));
    assert!(!is_void_element("style"));
    assert!(!is_void_element("body"));
    assert!(!is_void_element("html"));
}

// ============================================================================
// Raw Text Elements Tests
// ============================================================================

#[test]
fn test_raw_text_elements() {
    assert!(is_raw_text_element("script"));
    assert!(is_raw_text_element("style"));
}

#[test]
fn test_raw_text_elements_case_insensitive() {
    assert!(is_raw_text_element("SCRIPT"));
    assert!(is_raw_text_element("Script"));
    assert!(is_raw_text_element("STYLE"));
    assert!(is_raw_text_element("Style"));
}

#[test]
fn test_non_raw_text_elements() {
    assert!(!is_raw_text_element("div"));
    assert!(!is_raw_text_element("span"));
    assert!(!is_raw_text_element("p"));
    assert!(!is_raw_text_element("pre"));
    assert!(!is_raw_text_element("code"));
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_hyphenated_standard_tags_vs_custom_elements() {
    // Standard tags with hyphens (if any exist) vs custom elements
    // Most standard HTML5 tags don't have hyphens
    // Custom elements MUST have hyphens
    assert!(validate_html5_tag_name("my-custom").is_ok());
}

#[test]
fn test_very_long_tag_name() {
    let long_name = "a".repeat(1000);
    // Should be valid as long as it meets other criteria
    assert!(validate_html5_tag_name(&long_name).is_ok());

    let long_custom = format!("{}-element", "a".repeat(500));
    assert!(validate_html5_tag_name(&long_custom).is_ok());
}

#[test]
fn test_very_long_attribute_name() {
    let long_name = "a".repeat(1000);
    assert!(validate_html5_attribute_name(&long_name).is_ok());
}

#[test]
fn test_unicode_in_tag_names() {
    // HTML5 allows only ASCII alphanumeric + hyphens
    assert_eq!(
        validate_html5_tag_name("div™"),
        Err(DomException::InvalidCharacterError)
    );
    assert_eq!(
        validate_html5_tag_name("división"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_unicode_in_attribute_names() {
    // Attribute names should also be ASCII
    assert_eq!(
        validate_html5_attribute_name("título"),
        Err(DomException::InvalidCharacterError)
    );
}

#[test]
fn test_custom_element_with_multiple_hyphens() {
    assert!(validate_html5_tag_name("my-custom-web-component").is_ok());
    assert!(validate_html5_tag_name("a-b-c-d-e").is_ok());
}

#[test]
fn test_single_character_tag_names() {
    assert!(validate_html5_tag_name("a").is_ok());
    assert!(validate_html5_tag_name("b").is_ok());
    assert!(validate_html5_tag_name("i").is_ok());
    assert!(validate_html5_tag_name("p").is_ok());
    assert!(validate_html5_tag_name("s").is_ok());
}
