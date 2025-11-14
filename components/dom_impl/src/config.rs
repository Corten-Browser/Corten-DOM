//! Configuration for the DOM component

use serde::{Deserialize, Serialize};

/// Configuration for the DOM component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomConfig {
    /// Maximum depth of the DOM tree
    pub max_tree_depth: usize,

    /// Maximum number of children per node
    pub max_children: usize,

    /// Enable mutation observers
    pub enable_mutation_observers: bool,

    /// Enable shadow DOM support
    pub enable_shadow_dom: bool,

    /// Garbage collection threshold (number of nodes)
    pub gc_threshold: usize,

    /// Initial arena capacity for node storage
    pub arena_capacity: usize,
}

impl Default for DomConfig {
    fn default() -> Self {
        DomConfig {
            max_tree_depth: 512,
            max_children: 10000,
            enable_mutation_observers: true,
            enable_shadow_dom: true,
            gc_threshold: 100000,
            arena_capacity: 50000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DomConfig::default();
        assert_eq!(config.max_tree_depth, 512);
        assert_eq!(config.max_children, 10000);
        assert!(config.enable_mutation_observers);
        assert!(config.enable_shadow_dom);
        assert_eq!(config.gc_threshold, 100000);
        assert_eq!(config.arena_capacity, 50000);
    }

    #[test]
    fn test_config_serialization() {
        let config = DomConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DomConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.max_tree_depth, deserialized.max_tree_depth);
        assert_eq!(config.max_children, deserialized.max_children);
    }

    #[test]
    fn test_custom_config() {
        let config = DomConfig {
            max_tree_depth: 256,
            max_children: 5000,
            enable_mutation_observers: false,
            enable_shadow_dom: false,
            gc_threshold: 50000,
            arena_capacity: 25000,
        };

        assert_eq!(config.max_tree_depth, 256);
        assert!(!config.enable_mutation_observers);
    }
}
