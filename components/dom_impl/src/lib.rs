//! DOM Implementation for CortenBrowser.
//!
//! This is the main integration library that re-exports all DOM functionality
//! and provides the message bus interface for browser component communication.
//!
//! # Overview
//!
//! The `browser_dom_impl` crate serves as the unified entry point for the
//! CortenBrowser DOM implementation. It provides:
//!
//! - **Unified API**: Re-exports all public APIs from component crates
//! - **Message Bus**: Communication interface with other browser components
//! - **Configuration**: Runtime configuration for DOM behavior
//! - **Integration Points**: Interfaces for HTML parser, CSS engine, JS runtime
//!
//! # Architecture
//!
//! The DOM implementation follows a modular architecture:
//!
//! ```text
//! +------------------+
//! | browser_dom_impl |  <-- This crate (integration layer)
//! +------------------+
//!          |
//!          +-- dom-types      (Type definitions)
//!          +-- dom-storage    (Memory management)
//!          +-- dom-core       (Document, Element, Text, etc.)
//!          +-- dom-collections (NodeList, HTMLCollection, etc.)
//!          +-- dom-events     (Event dispatch and handling)
//!          +-- dom-selectors  (CSS selector engine)
//!          +-- dom-advanced   (Shadow DOM, MutationObserver)
//! ```
//!
//! # Quick Start
//!
//! ## Creating the DOM Component
//!
//! ```rust
//! use browser_dom_impl::{DomComponent, DomConfig};
//!
//! // Create with default configuration
//! let mut dom = DomComponent::new(DomConfig::default());
//!
//! // Or with custom configuration
//! let config = DomConfig {
//!     max_tree_depth: 1024,
//!     enable_mutation_observers: true,
//!     ..Default::default()
//! };
//! let mut dom = DomComponent::new(config);
//! ```
//!
//! ## Handling Messages
//!
//! ```rust
//! use browser_dom_impl::{DomComponent, DomConfig, DomComponentMessage, ParsedNode, ParsedNodeType};
//! use std::collections::HashMap;
//!
//! let mut dom = DomComponent::new(DomConfig::default());
//!
//! // Handle a parsed document from the HTML parser
//! let parsed_html = ParsedNode {
//!     node_type: ParsedNodeType::Element,
//!     tag_name: Some("html".to_string()),
//!     attributes: HashMap::new(),
//!     text_content: None,
//!     children: vec![],
//! };
//!
//! let msg = DomComponentMessage::ParsedDocument {
//!     request_id: 1,
//!     root: parsed_html,
//!     doctype: None,
//! };
//!
//! let response = dom.handle_message(msg);
//! ```
//!
//! # Message Types
//!
//! ## Incoming Messages (`DomComponentMessage`)
//!
//! | Message | Source | Description |
//! |---------|--------|-------------|
//! | `ParsedDocument` | HTML Parser | Parsed HTML tree |
//! | `ScriptManipulation` | JS Runtime | DOM operations from scripts |
//! | `StyleInvalidation` | CSS Engine | Style recalculation needed |
//! | `UserInteraction` | Browser Shell | User events (click, key, etc.) |
//!
//! ## Outgoing Responses (`DomComponentResponse`)
//!
//! | Response | Destination | Description |
//! |----------|-------------|-------------|
//! | `DomMutated` | JS Runtime | Mutation records |
//! | `DomTreeChanged` | CSS Engine | Tree structure changed |
//! | `LayoutInvalidated` | Rendering | Layout needs recalculation |
//! | `QueryResult` | Various | Query response data |
//!
//! # Configuration
//!
//! ```rust
//! use browser_dom_impl::DomConfig;
//!
//! let config = DomConfig {
//!     max_tree_depth: 512,           // Maximum DOM tree depth
//!     max_children: 10000,           // Maximum children per node
//!     enable_mutation_observers: true,
//!     enable_shadow_dom: true,
//!     gc_threshold: 100000,          // GC trigger threshold
//!     arena_capacity: 50000,         // Initial arena capacity
//! };
//! ```
//!
//! # Integration Points
//!
//! ## HTML Parser Integration
//!
//! ```rust
//! use browser_dom_impl::HtmlParserIntegration;
//!
//! // Implement this trait to integrate with HTML parser
//! // trait HtmlParserIntegration {
//! //     fn on_document_start(&mut self);
//! //     fn on_element(&mut self, tag: &str, attrs: &[(String, String)]);
//! //     fn on_text(&mut self, text: &str);
//! //     fn on_document_end(&mut self);
//! // }
//! ```
//!
//! ## CSS Engine Integration
//!
//! ```rust
//! use browser_dom_impl::CssEngineIntegration;
//!
//! // Implement this trait to integrate with CSS engine
//! // trait CssEngineIntegration {
//! //     fn get_computed_style(&self, element_id: NodeId) -> ComputedStyleMap;
//! //     fn invalidate_style(&mut self, node_id: NodeId);
//! // }
//! ```
//!
//! ## JavaScript Bindings
//!
//! ```rust
//! use browser_dom_impl::JsBindings;
//!
//! // Register JavaScript bindings for DOM methods
//! // let bindings = JsBindings::new();
//! // bindings.register_method("getElementById", ...);
//! ```
//!
//! # Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `dom-advanced` | Enable Shadow DOM and MutationObserver |
//!
//! # Re-exported Modules
//!
//! All public APIs from component crates are re-exported for convenience:
//!
//! - From `dom_types`: `NodeType`, `DomException`, `DOMRect`, etc.
//! - From `dom_core`: `Document`, `Element`, `Node`, etc.
//! - From `dom_collections`: `NodeList`, `HTMLCollection`, `TreeWalker`, etc.
//! - From `dom_events`: `Event`, `EventDispatcher`, `MouseEvent`, etc.
//! - From `dom_selectors`: `Selectable`, `SelectorQuery`
//! - From `dom_storage`: `NodeArena`, `GarbageCollector`
//! - From `dom_advanced`: `MutationObserver`, `ShadowRoot`, `Range`, etc.

#![warn(missing_docs)]

// Re-export all public APIs from component crates
pub use dom_collections::*;
pub use dom_core::*;
pub use dom_events::*;
pub use dom_selectors::*;
pub use dom_storage::*;
pub use dom_types::*;

#[cfg(feature = "dom-advanced")]
pub use dom_advanced::*;

pub mod atoms;
pub mod component;
pub mod config;
pub mod integration;
pub mod message_handler;
pub mod messages;
pub mod validation;

// Re-exports for convenience
pub use component::DomComponent;
pub use config::DomConfig;
pub use messages::{
    DomComponentMessage, DomComponentResponse, DomOperation, EventData, InvalidationReason,
    LayoutInvalidationType, MutationRecord, MutationType, OperationParams, ParsedNode,
    ParsedNodeType, QueryResultType, QueryType, TreeChangeType,
};

// Integration traits and types
pub use integration::{
    ComputedStyleMap, CssEngineIntegration, DomTestHarness, HtmlParserIntegration,
    JsBindingRegistry, JsBindings, JsMethodBinding, JsPropertyBinding, JsValue, StyleNode,
    TestAssertion, TestHarness, TestResult,
};

// Atom string interning
pub use atoms::{all_atoms, atom_count, Atom};
/// Re-export predefined atoms module
pub mod predefined_atoms {
    pub use super::atoms::atoms::*;
}

// Message handler for direct DOM operations
pub use message_handler::{DirectDomMessage, DirectDomResponse, DomErrorCode, DomMessageHandler};
