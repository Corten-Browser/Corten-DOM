//! Core DOM nodes and tree operations
//!
//! This crate provides the fundamental DOM node types and tree
//! manipulation operations.

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
pub mod node;
pub mod processing_instruction;
pub mod text;

// Re-exports
pub use attr::{Attr, AttrRef};
pub use cdata_section::{CDATASection, CDATASectionRef};
pub use comment::Comment;
pub use document::{Document, DocumentRef};
pub use document_fragment::DocumentFragment;
pub use document_type::{DocumentType, DocumentTypeRef};
pub use dom_implementation::DOMImplementation;
pub use element::{Element, ElementRef};
pub use node::{Node, NodeData, NodeRef, WeakNodeRef};
pub use processing_instruction::{ProcessingInstruction, ProcessingInstructionRef};
pub use text::Text;
