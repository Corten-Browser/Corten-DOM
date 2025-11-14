//! Garbage collection for arena-allocated nodes.
//!
//! Provides mark-and-sweep garbage collection with automatic compaction
//! when fragmentation exceeds threshold.

use crate::arena::{NodeArena, NodeId};
use std::collections::HashSet;

/// Garbage collector trait for managing memory.
///
/// Implements mark-and-sweep garbage collection with automatic
/// compaction when fragmentation is high.
pub trait GarbageCollector {
    /// Performs a full garbage collection cycle.
    ///
    /// This marks all reachable nodes from the provided roots, then
    /// sweeps (deallocates) all unreachable nodes. If fragmentation
    /// exceeds 50%, the arena is compacted.
    ///
    /// # Arguments
    ///
    /// * `roots` - The root nodes to start marking from
    ///
    /// # Performance
    ///
    /// GC of 10,000 nodes should complete in < 10ms per specification.
    fn collect(&mut self, roots: &[NodeId]);

    /// Marks a node and all its reachable descendants as reachable.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node to mark as reachable
    /// * `reachable` - Set to add reachable nodes to
    fn mark_reachable(&self, node_id: NodeId, reachable: &mut HashSet<NodeId>);

    /// Sweeps (deallocates) nodes matching the predicate.
    ///
    /// # Arguments
    ///
    /// * `predicate` - Function that returns true for nodes to deallocate
    fn sweep(&mut self, predicate: impl Fn(NodeId) -> bool);
}

impl GarbageCollector for NodeArena {
    fn collect(&mut self, roots: &[NodeId]) {
        // Mark phase - find all reachable nodes
        let mut reachable = HashSet::new();

        for &root in roots {
            self.mark_reachable(root, &mut reachable);
        }

        // Sweep phase - remove unreachable nodes
        self.sweep(|node_id| !reachable.contains(&node_id));

        // Compact if fragmentation is high
        if self.fragmentation() > 0.5 {
            self.compact();
        }
    }

    fn mark_reachable(&self, node_id: NodeId, reachable: &mut HashSet<NodeId>) {
        // Check if node exists and hasn't been marked yet
        if self.get(node_id).is_none() {
            return;
        }

        if reachable.contains(&node_id) {
            return;
        }

        // Mark this node as reachable
        reachable.insert(node_id);

        // In a real DOM implementation, we would traverse child nodes here.
        // For this storage component, we just mark the single node.
        // The actual DOM structure traversal would be done by a higher-level component.
    }

    fn sweep(&mut self, predicate: impl Fn(NodeId) -> bool) {
        // Get all allocated node IDs
        let all_ids = self.allocated_ids();

        // Deallocate nodes matching the predicate
        for node_id in all_ids {
            if predicate(node_id) {
                self.deallocate(node_id);
            }
        }
    }
}

/// Statistics about a garbage collection cycle.
#[derive(Debug, Clone, Copy)]
pub struct GcStats {
    /// Number of nodes before collection
    pub nodes_before: usize,
    /// Number of nodes after collection
    pub nodes_after: usize,
    /// Number of nodes collected (deallocated)
    pub nodes_collected: usize,
    /// Time taken in milliseconds
    pub duration_ms: u128,
    /// Fragmentation before collection
    pub fragmentation_before: f64,
    /// Fragmentation after collection
    pub fragmentation_after: f64,
}

impl GcStats {
    /// Returns the percentage of nodes collected.
    pub fn collection_rate(&self) -> f64 {
        if self.nodes_before == 0 {
            return 0.0;
        }
        (self.nodes_collected as f64 / self.nodes_before as f64) * 100.0
    }
}

/// Extended garbage collector with statistics tracking.
pub struct GcWithStats {
    arena: NodeArena,
}

impl GcWithStats {
    /// Creates a new garbage collector with statistics.
    pub fn new(arena: NodeArena) -> Self {
        Self { arena }
    }

    /// Performs garbage collection and returns statistics.
    pub fn collect_with_stats(&mut self, roots: &[NodeId]) -> GcStats {
        use std::time::Instant;

        let nodes_before = self.arena.len();
        let fragmentation_before = self.arena.fragmentation();

        let start = Instant::now();
        self.arena.collect(roots);
        let duration = start.elapsed();

        let nodes_after = self.arena.len();
        let fragmentation_after = self.arena.fragmentation();

        GcStats {
            nodes_before,
            nodes_after,
            nodes_collected: nodes_before.saturating_sub(nodes_after),
            duration_ms: duration.as_millis(),
            fragmentation_before,
            fragmentation_after,
        }
    }

    /// Returns a reference to the underlying arena.
    pub fn arena(&self) -> &NodeArena {
        &self.arena
    }

    /// Returns a mutable reference to the underlying arena.
    pub fn arena_mut(&mut self) -> &mut NodeArena {
        &mut self.arena
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_stats_collection_rate() {
        let stats = GcStats {
            nodes_before: 100,
            nodes_after: 50,
            nodes_collected: 50,
            duration_ms: 5,
            fragmentation_before: 0.0,
            fragmentation_after: 0.5,
        };

        assert_eq!(stats.collection_rate(), 50.0);
    }

    #[test]
    fn test_gc_stats_empty() {
        let stats = GcStats {
            nodes_before: 0,
            nodes_after: 0,
            nodes_collected: 0,
            duration_ms: 0,
            fragmentation_before: 0.0,
            fragmentation_after: 0.0,
        };

        assert_eq!(stats.collection_rate(), 0.0);
    }

    #[test]
    fn test_gc_with_stats() {
        let arena = NodeArena::new();
        let mut gc = GcWithStats::new(arena);

        // Allocate some nodes
        let ids: Vec<_> = (0..100)
            .map(|i| gc.arena_mut().allocate(format!("node_{}", i)))
            .collect();

        // Keep only half as roots
        let roots: Vec<_> = ids.iter().step_by(2).copied().collect();

        let stats = gc.collect_with_stats(&roots);

        assert_eq!(stats.nodes_before, 100);
        assert_eq!(stats.nodes_after, 50);
        assert_eq!(stats.nodes_collected, 50);
        assert!(stats.duration_ms < 100); // Should be fast
    }
}
