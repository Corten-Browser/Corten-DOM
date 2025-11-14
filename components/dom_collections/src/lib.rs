//! Live collections and tree traversal for DOM
//!
//! This crate provides HTMLCollection, NodeList, and tree walking
//! functionality.

#![warn(missing_docs)]

pub mod dom_token_list;
pub mod html_collection;
pub mod named_node_map;
pub mod node_iterator;
pub mod node_list;
pub mod tree_walker;

// Re-exports
pub use dom_token_list::DOMTokenList;
pub use html_collection::HTMLCollection;
pub use named_node_map::NamedNodeMap;
pub use node_iterator::{
    FilterResult, NodeFilter, NodeIterator, SHOW_ALL, SHOW_COMMENT, SHOW_DOCUMENT, SHOW_ELEMENT,
    SHOW_TEXT,
};
pub use node_list::NodeList;
pub use tree_walker::TreeWalker;
