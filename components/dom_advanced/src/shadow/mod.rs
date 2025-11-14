//! Shadow DOM implementation
//!
//! Provides encapsulation for DOM subtrees with style and markup isolation.

mod shadow_root;
mod slot;

pub use shadow_root::{ShadowRoot, ShadowRootMode};
pub use slot::{SlotAssignmentMode, SlotElement};
