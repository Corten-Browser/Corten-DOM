//! CSS Selector Engine for CortenBrowser DOM.
//!
//! This crate provides CSS selector parsing, matching, and querying
//! functionality following the Selectors Level 4 specification.
//!
//! # Overview
//!
//! The selector engine supports:
//!
//! - **Query Methods**: `querySelector`, `querySelectorAll`
//! - **Matching**: `matches()`, `closest()`
//! - **Complex Selectors**: Combinators, pseudo-classes, attribute selectors
//!
//! # Quick Start
//!
//! ```rust
//! use dom_selectors::{Selectable, SelectorQuery};
//!
//! // Query elements (on types implementing Selectable)
//! // let result = element.query_selector("div.container > p.intro");
//! // let all = element.query_selector_all("a[href^='https']");
//! ```
//!
//! # Selectable Trait
//!
//! Types implementing `Selectable` support CSS queries:
//!
//! ```rust
//! use dom_selectors::Selectable;
//!
//! // Available methods:
//! // fn query_selector(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
//! // fn query_selector_all(&self, selector: &str) -> Result<NodeList, DomException>;
//! // fn matches(&self, selector: &str) -> Result<bool, DomException>;
//! // fn closest(&self, selector: &str) -> Result<Option<ElementRef>, DomException>;
//! ```
//!
//! # Supported Selectors
//!
//! ## Simple Selectors
//!
//! | Selector | Example | Description |
//! |----------|---------|-------------|
//! | Type | `div`, `p` | Element tag name |
//! | Class | `.container` | Class attribute |
//! | ID | `#main` | ID attribute |
//! | Universal | `*` | Any element |
//!
//! ## Attribute Selectors
//!
//! | Selector | Example | Description |
//! |----------|---------|-------------|
//! | Exists | `[href]` | Attribute exists |
//! | Equals | `[type="text"]` | Exact match |
//! | Contains | `[class*="btn"]` | Substring match |
//! | Starts | `[href^="https"]` | Prefix match |
//! | Ends | `[src$=".png"]` | Suffix match |
//!
//! ## Combinators
//!
//! | Combinator | Example | Description |
//! |------------|---------|-------------|
//! | Descendant | `div p` | Any descendant |
//! | Child | `ul > li` | Direct child |
//! | Adjacent | `h1 + p` | Immediate sibling |
//! | General | `h1 ~ p` | Any following sibling |
//!
//! ## Pseudo-Classes
//!
//! | Pseudo | Example | Description |
//! |--------|---------|-------------|
//! | `:first-child` | `li:first-child` | First child element |
//! | `:last-child` | `li:last-child` | Last child element |
//! | `:nth-child()` | `tr:nth-child(2n)` | Nth child by formula |
//! | `:not()` | `p:not(.intro)` | Negation |
//! | `:empty` | `div:empty` | No children |
//!
//! # Performance
//!
//! - Selector parsing is cached for repeated queries
//! - Matching is optimized for common patterns
//! - Complex selectors: < 2ms typical query time
//!
//! # Related Crates
//!
//! - [`dom_core`](../dom_core/index.html) - Core DOM nodes
//! - [`dom_collections`](../dom_collections/index.html) - NodeList results

#![warn(missing_docs)]

pub mod matcher;
pub mod query;

// Re-exports
pub use query::{Selectable, SelectorQuery};
