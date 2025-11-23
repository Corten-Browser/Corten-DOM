//! DOM Implementation for CortenBrowser
//!
//! This is the main library that re-exports all DOM functionality
//! and provides integration with the browser's message bus.
//!
//! # Overview
//!
//! This crate serves as the main integration point for the CortenBrowser DOM implementation.
//! It re-exports all public APIs from the individual DOM component crates and provides
//! a message bus interface for communication with other browser components.
//!
//! # Architecture
//!
//! The DOM implementation is split into several focused crates:
//!
//! - `dom-types`: Core types and traits
//! - `dom-storage`: Arena-based memory management
//! - `dom-core`: Core DOM nodes (Document, Element, Text, etc.)
//! - `dom-collections`: DOM collections (NodeList, HTMLCollection, etc.)
//! - `dom-events`: Event system
//! - `dom-selectors`: CSS selector engine
//! - `dom-advanced`: Advanced features (Shadow DOM, MutationObserver)
//!
//! # Example
//!
//! ```rust
//! use browser_dom_impl::{DomComponent, DomConfig, DomComponentMessage, ParsedNode, ParsedNodeType};
//! use std::collections::HashMap;
//!
//! // Create the DOM component
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

pub mod component;
pub mod config;
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
