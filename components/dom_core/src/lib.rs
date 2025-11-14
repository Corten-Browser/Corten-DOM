//! Core DOM nodes and tree operations
//!
//! This crate provides the fundamental DOM node types and tree
//! manipulation operations.

#![warn(missing_docs)]
#![allow(dead_code)] // Allow during development

pub mod comment;
pub mod document;
pub mod document_fragment;
pub mod element;
pub mod node;
pub mod text;

// Re-exports
pub use comment::Comment;
pub use document::{Document, DocumentRef};
pub use document_fragment::DocumentFragment;
pub use element::{Element, ElementRef};
pub use node::{Node, NodeData, NodeRef, WeakNodeRef};
pub use text::Text;
