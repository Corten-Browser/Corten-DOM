//! Core DOM nodes and tree operations
//!
//! This crate provides the fundamental DOM node types and tree
//! manipulation operations.

#![warn(missing_docs)]

pub mod node;
pub mod element;
pub mod document;
pub mod text;
pub mod comment;

// Re-exports
pub use node::{Node, NodeData, NodeRef};
pub use element::{Element, ElementRef};
pub use document::{Document, DocumentRef};
pub use text::Text;
pub use comment::Comment;
