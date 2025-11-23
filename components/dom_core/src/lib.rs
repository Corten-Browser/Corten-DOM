//! Core DOM nodes and tree operations for CortenBrowser.
//!
//! This crate provides the fundamental DOM node types and tree manipulation
//! operations following the DOM Level 4 specification.
//!
//! # Overview
//!
//! The `dom_core` crate is the foundation of the CortenBrowser DOM implementation.
//! It provides:
//!
//! - **Core Node Types**: [`Document`], [`Element`], [`Text`], [`Comment`], etc.
//! - **Tree Operations**: `append_child`, `remove_child`, `insert_before`, `clone_node`
//! - **Attribute Management**: Get, set, remove attributes with namespace support
//! - **Thread Safety**: All types use `Arc<RwLock<>>` for safe concurrent access
//!
//! # Quick Start
//!
//! ```rust
//! use dom_core::{Document, Element};
//!
//! // Create a new document
//! let mut doc = Document::new();
//!
//! // Create elements
//! let div = doc.create_element("div").unwrap();
//! let span = doc.create_element("span").unwrap();
//!
//! // Set attributes
//! div.write().set_attribute("id", "main").unwrap();
//! div.write().set_attribute("class", "container").unwrap();
//!
//! // Build tree structure
//! div.write().append_child_element(span).unwrap();
//!
//! // Set as document root
//! doc.set_document_element(div);
//! ```
//!
//! # Thread Safety
//!
//! All node types are wrapped in `Arc<RwLock<>>` for thread-safe access:
//!
//! ```rust
//! use dom_core::ElementRef;
//!
//! fn example(element: ElementRef) {
//!     // Read access (can be held by multiple threads)
//!     let tag = element.read().tag_name().to_string();
//!
//!     // Write access (exclusive)
//!     element.write().set_attribute("key", "value").unwrap();
//! }
//! ```
//!
//! # Node Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Document`] | Root document node |
//! | [`Element`] | HTML/XML elements |
//! | [`Text`] | Text content |
//! | [`Comment`] | Comment nodes |
//! | [`DocumentFragment`] | Lightweight document fragments |
//! | [`DocumentType`] | DOCTYPE declarations |
//! | [`CDATASection`] | CDATA sections (XML) |
//! | [`ProcessingInstruction`] | Processing instructions |
//! | [`Attr`] | Element attributes |
//!
//! # Reference Types
//!
//! - [`NodeRef`] - `Arc<RwLock<Box<dyn Node>>>` for generic nodes
//! - [`ElementRef`] - `Arc<RwLock<Element>>` for elements
//! - [`DocumentRef`] - `Arc<RwLock<Document>>` for documents
//! - [`WeakNodeRef`] - Non-owning reference to prevent cycles
//!
//! # Related Crates
//!
//! - [`dom_types`](../dom_types/index.html) - Type definitions
//! - [`dom_storage`](../dom_storage/index.html) - Memory management
//! - [`dom_events`](../dom_events/index.html) - Event system
//! - [`dom_collections`](../dom_collections/index.html) - Collections
//! - [`dom_selectors`](../dom_selectors/index.html) - CSS selectors

#![warn(missing_docs)]
#![allow(dead_code)] // Allow during development

pub mod attr;
pub mod cdata_section;
pub mod comment;
pub mod document;
pub mod document_fragment;
pub mod document_type;
pub mod dom_implementation;
pub mod element;
pub mod event;
pub mod namespaces;
pub mod node;
pub mod processing_instruction;
pub mod range;
pub mod text;
pub mod tree_order;

// Re-exports
pub use attr::{Attr, AttrRef};
pub use cdata_section::{CDATASection, CDATASectionRef};
pub use comment::Comment;
pub use document::{Document, DocumentRef};
pub use document_fragment::DocumentFragment;
pub use document_type::{DocumentType, DocumentTypeRef};
pub use dom_implementation::DOMImplementation;
pub use element::{Element, ElementRef};
pub use event::{Event, EventInit, EventPhase, EventRef};
pub use namespaces::*;
pub use node::{Node, NodeData, NodeRef, WeakNodeRef};
pub use processing_instruction::{ProcessingInstruction, ProcessingInstructionRef};
pub use range::{Range, RangeRef};
pub use text::Text;
pub use tree_order::*;
