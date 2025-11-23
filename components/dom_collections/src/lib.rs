//! Live collections and tree traversal for CortenBrowser DOM.
//!
//! This crate provides live and static collections, attribute maps,
//! and tree traversal iterators following the DOM specification.
//!
//! # Overview
//!
//! The collections system provides:
//!
//! - **Live Collections**: `HTMLCollection` and live `NodeList` that update automatically
//! - **Static Collections**: Snapshot `NodeList` for query results
//! - **Token Lists**: `DOMTokenList` for class and attribute management
//! - **Tree Traversal**: `TreeWalker` and `NodeIterator` for flexible navigation
//!
//! # Live vs Static Collections
//!
//! **Live collections** automatically reflect DOM changes:
//!
//! ```rust
//! use dom_collections::HTMLCollection;
//!
//! // HTMLCollection stays in sync with the DOM
//! // let divs = element.get_elements_by_tag_name("div");
//! // Adding a div to the DOM automatically updates the collection
//! ```
//!
//! **Static collections** are snapshots that don't change:
//!
//! ```rust
//! use dom_collections::NodeList;
//!
//! // querySelectorAll returns a static NodeList
//! // Adding elements won't affect this collection
//! ```
//!
//! # Collection Types
//!
//! | Type | Live | Description |
//! |------|------|-------------|
//! | [`HTMLCollection`] | Yes | Elements by tag name or class |
//! | [`NodeList`] | Both | Nodes from queries or childNodes |
//! | [`DOMTokenList`] | Yes | Space-separated token lists (classes) |
//! | [`NamedNodeMap`] | Yes | Element attributes |
//!
//! # Tree Traversal
//!
//! ## TreeWalker
//!
//! Navigate the tree with full control:
//!
//! ```rust
//! use dom_collections::{TreeWalker, SHOW_ELEMENT};
//!
//! // TreeWalker allows bidirectional traversal
//! // let walker = doc.create_tree_walker(root, SHOW_ELEMENT, None);
//! // walker.next_node();
//! // walker.previous_node();
//! // walker.parent_node();
//! ```
//!
//! ## NodeIterator
//!
//! Sequential iteration through nodes:
//!
//! ```rust
//! use dom_collections::{NodeIterator, SHOW_TEXT};
//!
//! // NodeIterator for sequential access
//! // let iter = doc.create_node_iterator(root, SHOW_TEXT, None);
//! // while let Some(node) = iter.next_node() { ... }
//! ```
//!
//! # Node Filter
//!
//! Filter nodes during traversal:
//!
//! ```rust
//! use dom_collections::{NodeFilter, FilterResult};
//!
//! // Custom filter implementation
//! struct VisibleElementFilter;
//!
//! impl NodeFilter for VisibleElementFilter {
//!     fn accept_node(&self, node: &dyn std::any::Any) -> FilterResult {
//!         FilterResult::Accept
//!     }
//! }
//! ```
//!
//! # Show Flags
//!
//! | Flag | Description |
//! |------|-------------|
//! | `SHOW_ALL` | Show all node types |
//! | `SHOW_ELEMENT` | Show only elements |
//! | `SHOW_TEXT` | Show only text nodes |
//! | `SHOW_COMMENT` | Show only comments |
//! | `SHOW_DOCUMENT` | Show document nodes |
//!
//! # Related Crates
//!
//! - [`dom_core`](../dom_core/index.html) - Core DOM nodes
//! - [`dom_selectors`](../dom_selectors/index.html) - CSS selector queries

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
