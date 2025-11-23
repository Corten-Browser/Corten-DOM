//! Integration traits for external component communication
//!
//! These traits define how the DOM component integrates with:
//! - HTML Parser: Converting parsed HTML to DOM nodes
//! - JavaScript Runtime: Exposing DOM API to JS
//! - CSS Engine: Providing DOM structure for style calculation
//! - WPT Test Harness: Running Web Platform Tests
//!
//! # Architecture
//!
//! The DOM component is designed to be integration-agnostic. These traits
//! define the interfaces that external components must implement or that
//! the DOM component exposes for external use.
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │ HTML Parser │     │ JS Runtime  │     │ CSS Engine  │
//! └──────┬──────┘     └──────┬──────┘     └──────┬──────┘
//!        │                   │                   │
//!        ▼                   ▼                   ▼
//!   HtmlParserIntegration  JsBindings    CssEngineIntegration
//!        │                   │                   │
//!        └───────────────────┼───────────────────┘
//!                            ▼
//!                    ┌───────────────┐
//!                    │  DOM Component │
//!                    └───────────────┘
//! ```
//!
//! # Examples
//!
//! ## HTML Parser Integration
//!
//! ```rust
//! use browser_dom_impl::integration::{HtmlParserIntegration, ParsedNode, ParsedNodeType};
//! use browser_dom_impl::{DomException, NodeId};
//! use std::collections::HashMap;
//!
//! struct MyDomBuilder {
//!     next_id: NodeId,
//! }
//!
//! impl HtmlParserIntegration for MyDomBuilder {
//!     fn handle_parsed_document(&mut self, parsed: ParsedNode) -> Result<NodeId, DomException> {
//!         let id = self.next_id;
//!         self.next_id += 1;
//!         Ok(id)
//!     }
//!
//!     fn handle_parsed_fragment(&mut self, _parent: NodeId, parsed: ParsedNode) -> Result<NodeId, DomException> {
//!         let id = self.next_id;
//!         self.next_id += 1;
//!         Ok(id)
//!     }
//! }
//! ```

use crate::messages::{ParsedNode, ParsedNodeType};
use dom_types::{DomException, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ========== HTML Parser Integration ==========

/// Trait for handling parsed HTML documents
///
/// This trait defines how the DOM component receives parsed HTML from the HTML parser.
/// Implementations should convert parsed nodes into the internal DOM representation.
///
/// # Implementation Notes
///
/// - `handle_parsed_document` is called for complete documents
/// - `handle_parsed_fragment` is called for incremental/streaming parsing
/// - Both methods should validate the parsed structure before conversion
pub trait HtmlParserIntegration {
    /// Convert parsed document into DOM tree
    ///
    /// Takes the root of a parsed document tree and converts it into DOM nodes.
    /// Returns the NodeId of the created document root.
    ///
    /// # Arguments
    ///
    /// * `parsed` - The root parsed node representing the entire document
    ///
    /// # Errors
    ///
    /// Returns `DomException` if the parsed structure is invalid or cannot be converted.
    fn handle_parsed_document(&mut self, parsed: ParsedNode) -> Result<NodeId, DomException>;

    /// Handle incremental parsing (streaming)
    ///
    /// Appends a parsed fragment to an existing parent node. Used for streaming
    /// HTML parsing where the document is built incrementally.
    ///
    /// # Arguments
    ///
    /// * `parent` - The NodeId of the parent node to append to
    /// * `parsed` - The parsed fragment to append
    ///
    /// # Errors
    ///
    /// Returns `DomException` if:
    /// - The parent node doesn't exist (`NotFoundError`)
    /// - The fragment cannot be appended at this location (`HierarchyRequestError`)
    fn handle_parsed_fragment(
        &mut self,
        parent: NodeId,
        parsed: ParsedNode,
    ) -> Result<NodeId, DomException>;
}

// ========== JavaScript Runtime Integration ==========

/// JavaScript value representation for DOM-JS interop
///
/// This enum represents JavaScript values that can be passed to and from
/// DOM methods when called from JavaScript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JsValue {
    /// JavaScript `undefined`
    Undefined,
    /// JavaScript `null`
    Null,
    /// JavaScript boolean
    Boolean(bool),
    /// JavaScript number (all JS numbers are f64)
    Number(f64),
    /// JavaScript string
    String(String),
    /// Reference to a DOM node
    NodeRef(NodeId),
    /// JavaScript array
    Array(Vec<JsValue>),
    /// JavaScript object (simple key-value representation)
    Object(HashMap<String, JsValue>),
}

impl Default for JsValue {
    fn default() -> Self {
        JsValue::Undefined
    }
}

impl JsValue {
    /// Creates a new undefined value
    pub fn undefined() -> Self {
        JsValue::Undefined
    }

    /// Creates a new null value
    pub fn null() -> Self {
        JsValue::Null
    }

    /// Creates a new boolean value
    pub fn boolean(value: bool) -> Self {
        JsValue::Boolean(value)
    }

    /// Creates a new number value
    pub fn number(value: f64) -> Self {
        JsValue::Number(value)
    }

    /// Creates a new string value
    pub fn string(value: impl Into<String>) -> Self {
        JsValue::String(value.into())
    }

    /// Creates a new node reference
    pub fn node_ref(id: NodeId) -> Self {
        JsValue::NodeRef(id)
    }

    /// Returns true if this value is undefined
    pub fn is_undefined(&self) -> bool {
        matches!(self, JsValue::Undefined)
    }

    /// Returns true if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, JsValue::Null)
    }

    /// Returns true if this value is null or undefined
    pub fn is_nullish(&self) -> bool {
        matches!(self, JsValue::Undefined | JsValue::Null)
    }

    /// Attempts to extract a boolean value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to extract a number value
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Attempts to extract a string value
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to extract a node reference
    pub fn as_node_ref(&self) -> Option<NodeId> {
        match self {
            JsValue::NodeRef(id) => Some(*id),
            _ => None,
        }
    }
}

/// Binding information for a JavaScript-accessible DOM method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsMethodBinding {
    /// Method name as exposed to JavaScript
    pub name: String,
    /// Number of arguments the method accepts
    pub arg_count: usize,
    /// Whether the method returns a value
    pub returns_value: bool,
    /// Description for documentation/debugging
    pub description: Option<String>,
}

impl JsMethodBinding {
    /// Creates a new method binding
    pub fn new(name: impl Into<String>, arg_count: usize) -> Self {
        Self {
            name: name.into(),
            arg_count,
            returns_value: true,
            description: None,
        }
    }

    /// Creates a void method binding (no return value)
    pub fn void(name: impl Into<String>, arg_count: usize) -> Self {
        Self {
            name: name.into(),
            arg_count,
            returns_value: false,
            description: None,
        }
    }

    /// Adds a description to the binding
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Binding information for a JavaScript-accessible DOM property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsPropertyBinding {
    /// Property name as exposed to JavaScript
    pub name: String,
    /// Whether the property can be read
    pub readable: bool,
    /// Whether the property can be written
    pub writable: bool,
    /// Description for documentation/debugging
    pub description: Option<String>,
}

impl JsPropertyBinding {
    /// Creates a new read-write property binding
    pub fn read_write(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            readable: true,
            writable: true,
            description: None,
        }
    }

    /// Creates a new read-only property binding
    pub fn read_only(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            readable: true,
            writable: false,
            description: None,
        }
    }

    /// Adds a description to the binding
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Registry of JavaScript-accessible DOM bindings
///
/// This registry collects all methods and properties that should be
/// exposed to the JavaScript runtime for DOM manipulation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsBindingRegistry {
    /// Method bindings
    pub methods: Vec<JsMethodBinding>,
    /// Property bindings
    pub properties: Vec<JsPropertyBinding>,
}

impl JsBindingRegistry {
    /// Creates a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a method binding to the registry
    pub fn add_method(&mut self, binding: JsMethodBinding) -> &mut Self {
        self.methods.push(binding);
        self
    }

    /// Adds a property binding to the registry
    pub fn add_property(&mut self, binding: JsPropertyBinding) -> &mut Self {
        self.properties.push(binding);
        self
    }

    /// Returns the number of registered methods
    pub fn method_count(&self) -> usize {
        self.methods.len()
    }

    /// Returns the number of registered properties
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }
}

/// Trait for exposing DOM API to JavaScript runtime
///
/// This trait defines how the DOM component exposes its API to JavaScript.
/// The JavaScript runtime should call `register_bindings` during initialization
/// to discover available methods and properties, then use `handle_js_call`
/// and related methods to interact with the DOM.
pub trait JsBindings {
    /// Register DOM bindings with the JS runtime
    ///
    /// Returns a registry of all methods and properties that JavaScript
    /// can call on the DOM.
    fn register_bindings(&self) -> JsBindingRegistry;

    /// Handle JS call to DOM method
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to call
    /// * `args` - Arguments passed from JavaScript
    ///
    /// # Errors
    ///
    /// Returns `DomException` if:
    /// - The method doesn't exist (`NotSupportedError`)
    /// - Invalid arguments (`SyntaxError`)
    /// - DOM operation fails (various exceptions)
    fn handle_js_call(
        &mut self,
        method: &str,
        args: Vec<JsValue>,
    ) -> Result<JsValue, DomException>;

    /// Get a property value
    ///
    /// # Arguments
    ///
    /// * `property` - The name of the property to get
    /// * `node_id` - The node to get the property from
    ///
    /// # Errors
    ///
    /// Returns `DomException` if the property doesn't exist or can't be read.
    fn get_property(&self, property: &str, node_id: NodeId) -> Result<JsValue, DomException>;

    /// Set a property value
    ///
    /// # Arguments
    ///
    /// * `property` - The name of the property to set
    /// * `node_id` - The node to set the property on
    /// * `value` - The value to set
    ///
    /// # Errors
    ///
    /// Returns `DomException` if the property doesn't exist, is read-only,
    /// or the value is invalid.
    fn set_property(
        &mut self,
        property: &str,
        node_id: NodeId,
        value: JsValue,
    ) -> Result<(), DomException>;
}

// ========== CSS Engine Integration ==========

/// Style node representation for CSS engine
///
/// This is a simplified view of a DOM node containing only the information
/// needed for CSS selector matching and style calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleNode {
    /// The DOM node ID this style node represents
    pub node_id: NodeId,
    /// Element tag name (lowercase)
    pub tag_name: String,
    /// Element ID attribute (if present)
    pub id: Option<String>,
    /// Element class list
    pub classes: Vec<String>,
    /// All element attributes (for attribute selectors)
    pub attributes: HashMap<String, String>,
    /// Child style nodes
    pub children: Vec<StyleNode>,
    /// Parent node ID (None for root)
    pub parent_id: Option<NodeId>,
    /// Pseudo-element type (if this represents a pseudo-element)
    pub pseudo_element: Option<String>,
}

impl StyleNode {
    /// Creates a new style node
    pub fn new(node_id: NodeId, tag_name: impl Into<String>) -> Self {
        Self {
            node_id,
            tag_name: tag_name.into().to_lowercase(),
            id: None,
            classes: Vec::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
            parent_id: None,
            pseudo_element: None,
        }
    }

    /// Sets the element ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a class to the class list
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Adds multiple classes
    pub fn with_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Sets an attribute
    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }

    /// Adds a child node
    pub fn with_child(mut self, child: StyleNode) -> Self {
        self.children.push(child);
        self
    }

    /// Checks if this node matches a simple selector
    ///
    /// This is a helper method for basic selector matching.
    /// Full selector matching should be done by the CSS engine.
    pub fn matches_tag(&self, tag: &str) -> bool {
        self.tag_name.eq_ignore_ascii_case(tag)
    }

    /// Checks if this node has a specific class
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.iter().any(|c| c == class)
    }

    /// Checks if this node has a specific ID
    pub fn has_id(&self, id: &str) -> bool {
        self.id.as_ref().is_some_and(|i| i == id)
    }
}

/// Computed style map type alias
pub type ComputedStyleMap = HashMap<String, String>;

/// Trait for CSS engine integration
///
/// This trait defines how the CSS engine interacts with the DOM to:
/// - Build a style tree for selector matching
/// - Retrieve computed styles for nodes
/// - Handle style-related queries
pub trait CssEngineIntegration {
    /// Build style tree from DOM for CSS calculation
    ///
    /// Creates a `StyleNode` tree representing the DOM structure with
    /// all information needed for CSS selector matching.
    ///
    /// # Arguments
    ///
    /// * `root` - The NodeId of the root node to start from
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the root node doesn't exist.
    fn get_style_tree(&self, root: NodeId) -> Result<StyleNode, DomException>;

    /// Get computed style for a node
    ///
    /// Returns the final computed CSS property values for a node,
    /// after cascade, inheritance, and default value resolution.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The NodeId of the element
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the node doesn't exist.
    fn get_computed_style(&self, node_id: NodeId) -> Result<ComputedStyleMap, DomException>;

    /// Get inline styles for a node
    ///
    /// Returns only the styles defined in the element's `style` attribute.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The NodeId of the element
    ///
    /// # Errors
    ///
    /// Returns `DomException::NotFoundError` if the node doesn't exist.
    fn get_inline_styles(&self, node_id: NodeId) -> Result<ComputedStyleMap, DomException>;

    /// Invalidate styles for a subtree
    ///
    /// Called when the DOM changes in a way that might affect styles
    /// (e.g., class added, attribute changed).
    ///
    /// # Arguments
    ///
    /// * `root` - The root of the subtree to invalidate
    fn invalidate_styles(&mut self, root: NodeId);
}

// ========== WPT Test Harness ==========

/// Test result from Web Platform Tests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestResult {
    /// Test passed successfully
    Pass,
    /// Test failed with a message
    Fail(String),
    /// Test timed out
    Timeout,
    /// Test encountered an error
    Error(String),
    /// Test was skipped
    Skip(String),
}

impl TestResult {
    /// Returns true if the test passed
    pub fn is_pass(&self) -> bool {
        matches!(self, TestResult::Pass)
    }

    /// Returns true if the test failed (not error or timeout)
    pub fn is_fail(&self) -> bool {
        matches!(self, TestResult::Fail(_))
    }

    /// Returns true if the test succeeded (not fail, error, or timeout)
    pub fn is_success(&self) -> bool {
        matches!(self, TestResult::Pass | TestResult::Skip(_))
    }

    /// Creates a failure result
    pub fn fail(message: impl Into<String>) -> Self {
        TestResult::Fail(message.into())
    }

    /// Creates an error result
    pub fn error(message: impl Into<String>) -> Self {
        TestResult::Error(message.into())
    }

    /// Creates a skip result
    pub fn skip(reason: impl Into<String>) -> Self {
        TestResult::Skip(reason.into())
    }
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestResult::Pass => write!(f, "PASS"),
            TestResult::Fail(msg) => write!(f, "FAIL: {}", msg),
            TestResult::Timeout => write!(f, "TIMEOUT"),
            TestResult::Error(msg) => write!(f, "ERROR: {}", msg),
            TestResult::Skip(reason) => write!(f, "SKIP: {}", reason),
        }
    }
}

/// Test assertion for WPT compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAssertion {
    /// Assertion description
    pub description: String,
    /// Expected value (as string representation)
    pub expected: String,
    /// Actual value (as string representation)
    pub actual: String,
    /// Whether the assertion passed
    pub passed: bool,
}

impl TestAssertion {
    /// Creates a passing assertion
    pub fn pass(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            expected: String::new(),
            actual: String::new(),
            passed: true,
        }
    }

    /// Creates a failing assertion
    pub fn fail(
        description: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self {
            description: description.into(),
            expected: expected.into(),
            actual: actual.into(),
            passed: false,
        }
    }
}

/// Trait for Web Platform Test harness integration
///
/// This trait defines the interface for running Web Platform Tests
/// against the DOM implementation. It provides setup, teardown,
/// and test execution capabilities.
pub trait TestHarness {
    /// Run a single test
    ///
    /// # Arguments
    ///
    /// * `test_name` - Name/identifier of the test to run
    ///
    /// # Returns
    ///
    /// The result of running the test.
    fn run_test(&mut self, test_name: &str) -> TestResult;

    /// Run multiple tests
    ///
    /// # Arguments
    ///
    /// * `test_names` - Names of tests to run
    ///
    /// # Returns
    ///
    /// A vector of (test_name, result) pairs.
    fn run_tests(&mut self, test_names: &[&str]) -> Vec<(String, TestResult)> {
        test_names
            .iter()
            .map(|name| {
                self.setup();
                let result = self.run_test(name);
                self.teardown();
                (name.to_string(), result)
            })
            .collect()
    }

    /// Set up test environment
    ///
    /// Called before each test to initialize a clean test environment.
    fn setup(&mut self);

    /// Tear down test environment
    ///
    /// Called after each test to clean up resources.
    fn teardown(&mut self);

    /// Get DOM document for testing
    ///
    /// Returns the NodeId of the document created for testing.
    fn get_test_document(&self) -> NodeId;

    /// Assert equality (WPT-style)
    fn assert_equals<T: PartialEq + std::fmt::Debug>(
        &self,
        actual: T,
        expected: T,
        description: &str,
    ) -> TestAssertion {
        if actual == expected {
            TestAssertion::pass(description)
        } else {
            TestAssertion::fail(description, format!("{:?}", expected), format!("{:?}", actual))
        }
    }

    /// Assert truthy (WPT-style)
    fn assert_true(&self, value: bool, description: &str) -> TestAssertion {
        if value {
            TestAssertion::pass(description)
        } else {
            TestAssertion::fail(description, "true", "false")
        }
    }

    /// Assert falsy (WPT-style)
    fn assert_false(&self, value: bool, description: &str) -> TestAssertion {
        if !value {
            TestAssertion::pass(description)
        } else {
            TestAssertion::fail(description, "false", "true")
        }
    }
}

/// DOM-specific test harness implementation
///
/// A concrete implementation of `TestHarness` for DOM testing.
/// This provides a basic framework for running DOM-related tests.
#[derive(Debug)]
pub struct DomTestHarness {
    /// The document created for testing (None before setup)
    test_document: Option<NodeId>,
    /// Test assertions collected during the current test
    assertions: Vec<TestAssertion>,
    /// Counter for generating unique node IDs in tests
    next_node_id: NodeId,
}

impl DomTestHarness {
    /// Creates a new DOM test harness
    pub fn new() -> Self {
        Self {
            test_document: None,
            assertions: Vec::new(),
            next_node_id: 1,
        }
    }

    /// Generates a new unique node ID for testing
    pub fn generate_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    /// Adds an assertion to the current test
    pub fn add_assertion(&mut self, assertion: TestAssertion) {
        self.assertions.push(assertion);
    }

    /// Gets all assertions from the current test
    pub fn get_assertions(&self) -> &[TestAssertion] {
        &self.assertions
    }

    /// Clears all assertions
    pub fn clear_assertions(&mut self) {
        self.assertions.clear();
    }

    /// Returns whether all assertions in the current test passed
    pub fn all_assertions_passed(&self) -> bool {
        self.assertions.iter().all(|a| a.passed)
    }
}

impl Default for DomTestHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl TestHarness for DomTestHarness {
    fn run_test(&mut self, test_name: &str) -> TestResult {
        // Clear previous assertions
        self.clear_assertions();

        // Basic test name validation
        if test_name.is_empty() {
            return TestResult::error("Empty test name");
        }

        // Default implementation - subclasses should override with actual test logic
        // For now, return a placeholder indicating the test needs implementation
        TestResult::skip(format!("Test '{}' not implemented", test_name))
    }

    fn setup(&mut self) {
        // Create a fresh test document
        let doc_id = self.generate_node_id();
        self.test_document = Some(doc_id);
        self.assertions.clear();
    }

    fn teardown(&mut self) {
        // Clean up test document
        self.test_document = None;
        // Don't clear assertions - they're needed for result reporting
    }

    fn get_test_document(&self) -> NodeId {
        self.test_document.unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ParsedNode Tests (reusing from messages) ==========

    #[test]
    fn test_parsed_node_types() {
        assert_eq!(ParsedNodeType::Element, ParsedNodeType::Element);
        assert_ne!(ParsedNodeType::Element, ParsedNodeType::Text);
        assert_ne!(ParsedNodeType::Comment, ParsedNodeType::Document);
    }

    // ========== JsValue Tests ==========

    #[test]
    fn test_js_value_constructors() {
        assert!(JsValue::undefined().is_undefined());
        assert!(JsValue::null().is_null());
        assert_eq!(JsValue::boolean(true).as_bool(), Some(true));
        assert_eq!(JsValue::number(42.0).as_number(), Some(42.0));
        assert_eq!(JsValue::string("test").as_str(), Some("test"));
        assert_eq!(JsValue::node_ref(123).as_node_ref(), Some(123));
    }

    #[test]
    fn test_js_value_nullish() {
        assert!(JsValue::Undefined.is_nullish());
        assert!(JsValue::Null.is_nullish());
        assert!(!JsValue::Boolean(false).is_nullish());
        assert!(!JsValue::Number(0.0).is_nullish());
        assert!(!JsValue::String(String::new()).is_nullish());
    }

    #[test]
    fn test_js_value_default() {
        let value = JsValue::default();
        assert!(value.is_undefined());
    }

    #[test]
    fn test_js_value_array() {
        let arr = JsValue::Array(vec![
            JsValue::Number(1.0),
            JsValue::Number(2.0),
            JsValue::Number(3.0),
        ]);
        if let JsValue::Array(items) = arr {
            assert_eq!(items.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_js_value_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), JsValue::String("value".to_string()));
        let obj = JsValue::Object(map);
        if let JsValue::Object(m) = obj {
            assert_eq!(m.get("key").and_then(|v| v.as_str()), Some("value"));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_js_value_serialization() {
        let value = JsValue::String("test".to_string());
        let json = serde_json::to_string(&value).unwrap();
        let deserialized: JsValue = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, value);
    }

    // ========== JsBinding Tests ==========

    #[test]
    fn test_js_method_binding() {
        let binding = JsMethodBinding::new("appendChild", 1)
            .with_description("Appends a child node");
        assert_eq!(binding.name, "appendChild");
        assert_eq!(binding.arg_count, 1);
        assert!(binding.returns_value);
        assert_eq!(binding.description, Some("Appends a child node".to_string()));
    }

    #[test]
    fn test_js_method_binding_void() {
        let binding = JsMethodBinding::void("removeChild", 1);
        assert!(!binding.returns_value);
    }

    #[test]
    fn test_js_property_binding() {
        let binding = JsPropertyBinding::read_only("tagName")
            .with_description("Element tag name");
        assert!(binding.readable);
        assert!(!binding.writable);
    }

    #[test]
    fn test_js_property_binding_read_write() {
        let binding = JsPropertyBinding::read_write("innerHTML");
        assert!(binding.readable);
        assert!(binding.writable);
    }

    #[test]
    fn test_js_binding_registry() {
        let mut registry = JsBindingRegistry::new();
        registry
            .add_method(JsMethodBinding::new("appendChild", 1))
            .add_property(JsPropertyBinding::read_only("tagName"));

        assert_eq!(registry.method_count(), 1);
        assert_eq!(registry.property_count(), 1);
    }

    // ========== StyleNode Tests ==========

    #[test]
    fn test_style_node_creation() {
        let node = StyleNode::new(1, "DIV");
        assert_eq!(node.node_id, 1);
        assert_eq!(node.tag_name, "div"); // Should be lowercase
    }

    #[test]
    fn test_style_node_builder() {
        let node = StyleNode::new(1, "div")
            .with_id("main")
            .with_class("container")
            .with_classes(["active", "visible"])
            .with_attribute("data-id", "123");

        assert_eq!(node.id, Some("main".to_string()));
        assert!(node.has_class("container"));
        assert!(node.has_class("active"));
        assert!(node.has_class("visible"));
        assert_eq!(node.attributes.get("data-id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_style_node_with_children() {
        let child = StyleNode::new(2, "span");
        let parent = StyleNode::new(1, "div").with_child(child);
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].tag_name, "span");
    }

    #[test]
    fn test_style_node_matching() {
        let node = StyleNode::new(1, "div")
            .with_id("main")
            .with_class("container");

        assert!(node.matches_tag("div"));
        assert!(node.matches_tag("DIV")); // Case insensitive
        assert!(!node.matches_tag("span"));
        assert!(node.has_id("main"));
        assert!(!node.has_id("other"));
        assert!(node.has_class("container"));
        assert!(!node.has_class("other"));
    }

    #[test]
    fn test_style_node_serialization() {
        let node = StyleNode::new(1, "div").with_id("test");
        let json = serde_json::to_string(&node).unwrap();
        let deserialized: StyleNode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.node_id, 1);
        assert_eq!(deserialized.id, Some("test".to_string()));
    }

    // ========== TestResult Tests ==========

    #[test]
    fn test_test_result_pass() {
        let result = TestResult::Pass;
        assert!(result.is_pass());
        assert!(result.is_success());
        assert!(!result.is_fail());
        assert_eq!(result.to_string(), "PASS");
    }

    #[test]
    fn test_test_result_fail() {
        let result = TestResult::fail("assertion failed");
        assert!(result.is_fail());
        assert!(!result.is_pass());
        assert!(!result.is_success());
        assert!(result.to_string().contains("FAIL"));
    }

    #[test]
    fn test_test_result_timeout() {
        let result = TestResult::Timeout;
        assert!(!result.is_pass());
        assert!(!result.is_fail());
        assert!(!result.is_success());
        assert_eq!(result.to_string(), "TIMEOUT");
    }

    #[test]
    fn test_test_result_error() {
        let result = TestResult::error("unexpected error");
        assert!(!result.is_pass());
        assert!(!result.is_fail());
        assert!(!result.is_success());
        assert!(result.to_string().contains("ERROR"));
    }

    #[test]
    fn test_test_result_skip() {
        let result = TestResult::skip("not implemented");
        assert!(!result.is_pass());
        assert!(!result.is_fail());
        assert!(result.is_success()); // Skip is considered success
        assert!(result.to_string().contains("SKIP"));
    }

    // ========== TestAssertion Tests ==========

    #[test]
    fn test_assertion_pass() {
        let assertion = TestAssertion::pass("values match");
        assert!(assertion.passed);
        assert_eq!(assertion.description, "values match");
    }

    #[test]
    fn test_assertion_fail() {
        let assertion = TestAssertion::fail("values differ", "expected", "actual");
        assert!(!assertion.passed);
        assert_eq!(assertion.expected, "expected");
        assert_eq!(assertion.actual, "actual");
    }

    // ========== DomTestHarness Tests ==========

    #[test]
    fn test_dom_test_harness_creation() {
        let harness = DomTestHarness::new();
        assert!(harness.test_document.is_none());
        assert!(harness.assertions.is_empty());
    }

    #[test]
    fn test_dom_test_harness_default() {
        let harness = DomTestHarness::default();
        assert!(harness.test_document.is_none());
    }

    #[test]
    fn test_dom_test_harness_setup_teardown() {
        let mut harness = DomTestHarness::new();

        // Before setup, no document
        assert!(harness.test_document.is_none());

        // After setup, has document
        harness.setup();
        assert!(harness.test_document.is_some());
        let doc_id = harness.get_test_document();
        assert!(doc_id > 0);

        // After teardown, no document
        harness.teardown();
        assert!(harness.test_document.is_none());
    }

    #[test]
    fn test_dom_test_harness_generate_ids() {
        let mut harness = DomTestHarness::new();
        let id1 = harness.generate_node_id();
        let id2 = harness.generate_node_id();
        let id3 = harness.generate_node_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_dom_test_harness_assertions() {
        let mut harness = DomTestHarness::new();
        harness.setup();

        assert!(harness.all_assertions_passed()); // Empty = all passed

        harness.add_assertion(TestAssertion::pass("test 1"));
        assert!(harness.all_assertions_passed());

        harness.add_assertion(TestAssertion::fail("test 2", "a", "b"));
        assert!(!harness.all_assertions_passed());

        assert_eq!(harness.get_assertions().len(), 2);

        harness.clear_assertions();
        assert!(harness.get_assertions().is_empty());
    }

    #[test]
    fn test_dom_test_harness_run_test() {
        let mut harness = DomTestHarness::new();
        harness.setup();

        // Empty test name should error
        let result = harness.run_test("");
        assert!(matches!(result, TestResult::Error(_)));

        // Unimplemented test should skip
        let result = harness.run_test("some_test");
        assert!(matches!(result, TestResult::Skip(_)));
    }

    #[test]
    fn test_dom_test_harness_assert_helpers() {
        let harness = DomTestHarness::new();

        let pass = harness.assert_equals(1, 1, "one equals one");
        assert!(pass.passed);

        let fail = harness.assert_equals(1, 2, "one equals two");
        assert!(!fail.passed);

        let true_pass = harness.assert_true(true, "true is true");
        assert!(true_pass.passed);

        let true_fail = harness.assert_true(false, "false is true");
        assert!(!true_fail.passed);

        let false_pass = harness.assert_false(false, "false is false");
        assert!(false_pass.passed);

        let false_fail = harness.assert_false(true, "true is false");
        assert!(!false_fail.passed);
    }

    #[test]
    fn test_dom_test_harness_run_tests() {
        let mut harness = DomTestHarness::new();
        let results = harness.run_tests(&["test1", "test2", "test3"]);

        assert_eq!(results.len(), 3);
        for (name, result) in &results {
            assert!(!name.is_empty());
            // All should be skipped since not implemented
            assert!(matches!(result, TestResult::Skip(_)));
        }
    }
}
