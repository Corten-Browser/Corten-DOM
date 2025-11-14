//! Live collections and tree traversal for DOM
//!
//! This crate provides HTMLCollection, NodeList, and tree walking
//! functionality.

#![warn(missing_docs)]

pub mod html_collection;
pub mod node_list;
pub mod dom_token_list;
pub mod node_iterator;
pub mod tree_walker;

// Re-exports
pub use html_collection::HTMLCollection;
pub use node_list::NodeList;
pub use dom_token_list::DOMTokenList;
pub use node_iterator::NodeIterator;
pub use tree_walker::TreeWalker;
