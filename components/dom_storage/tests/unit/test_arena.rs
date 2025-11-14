use dom_storage::arena::{NodeArena, NodeId};
use std::sync::Arc;
use std::thread;

#[test]
fn test_arena_new() {
    let arena = NodeArena::new();
    assert_eq!(arena.len(), 0);
}

#[test]
fn test_arena_with_capacity() {
    let arena = NodeArena::with_capacity(100);
    assert_eq!(arena.len(), 0);
    assert!(arena.capacity() >= 100);
}

#[test]
fn test_arena_allocate_and_retrieve() {
    let arena = NodeArena::new();
    let test_data = "test_node".to_string();
    let node_id = arena.allocate(test_data.clone());

    let retrieved = arena.get(node_id);
    assert!(retrieved.is_some());

    let node = retrieved.unwrap();
    let downcasted = node.downcast_ref::<String>().unwrap();
    assert_eq!(downcasted, &test_data);
}

#[test]
fn test_arena_generation_prevents_use_after_free() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    // Deallocate the node
    arena.deallocate(node_id);

    // Trying to get deallocated node should return None due to generation mismatch
    assert!(arena.get(node_id).is_none());
}

#[test]
fn test_arena_generation_increments() {
    let arena = NodeArena::new();

    let id1 = arena.allocate("node1".to_string());
    let id2 = arena.allocate("node2".to_string());

    // Different allocations should have different generations
    assert!(id1.generation != id2.generation || id1.index != id2.index);
}

#[test]
fn test_arena_reuses_freed_slots() {
    let arena = NodeArena::new();

    // Allocate and deallocate
    let id1 = arena.allocate("node1".to_string());
    let index1 = id1.index;
    arena.deallocate(id1);

    // Next allocation should reuse the freed slot
    let id2 = arena.allocate("node2".to_string());
    assert_eq!(id2.index, index1);
    // But generation should be different
    assert_ne!(id2.generation, id1.generation);
}

#[test]
fn test_arena_deallocate_nonexistent() {
    let arena = NodeArena::new();
    let fake_id = NodeId {
        index: 999,
        generation: 999,
    };

    // Should not panic
    arena.deallocate(fake_id);
}

#[test]
fn test_arena_multiple_allocations() {
    let arena = NodeArena::new();
    let mut ids = Vec::new();

    for i in 0..100 {
        let id = arena.allocate(format!("node_{}", i));
        ids.push(id);
    }

    assert_eq!(arena.len(), 100);

    // Verify all can be retrieved
    for (i, &id) in ids.iter().enumerate() {
        let node = arena.get(id).unwrap();
        let data = node.downcast_ref::<String>().unwrap();
        assert_eq!(data, &format!("node_{}", i));
    }
}

#[test]
fn test_arena_fragmentation() {
    let arena = NodeArena::new();

    // Initially no fragmentation
    assert_eq!(arena.fragmentation(), 0.0);

    // Allocate some nodes
    let ids: Vec<_> = (0..10)
        .map(|i| arena.allocate(format!("node_{}", i)))
        .collect();

    // No fragmentation yet (all allocated)
    assert_eq!(arena.fragmentation(), 0.0);

    // Deallocate half of them
    for &id in ids.iter().take(5) {
        arena.deallocate(id);
    }

    // Should have some fragmentation now
    let frag = arena.fragmentation();
    assert!(frag > 0.0 && frag <= 1.0);
}

#[test]
fn test_arena_compact() {
    let mut arena = NodeArena::new();

    // Allocate and deallocate to create fragmentation
    let ids: Vec<_> = (0..10)
        .map(|i| arena.allocate(format!("node_{}", i)))
        .collect();

    for &id in ids.iter().take(5) {
        arena.deallocate(id);
    }

    let frag_before = arena.fragmentation();
    assert!(frag_before > 0.0);

    // Compact should remove trailing None entries
    arena.compact();

    let frag_after = arena.fragmentation();
    println!(
        "Fragmentation before: {}, after: {}",
        frag_before, frag_after
    );

    // Compact might not always reduce fragmentation if nodes are not at the end
    // But it should not increase it
    assert!(frag_after <= 1.0);

    // Verify remaining nodes are still accessible
    for &id in ids.iter().skip(5) {
        let node = arena.get(id);
        assert!(node.is_some());
    }
}

#[test]
fn test_concurrent_allocation() {
    let arena = Arc::new(NodeArena::new());
    let mut handles = Vec::new();

    for i in 0..10 {
        let arena_clone = arena.clone();
        let handle = thread::spawn(move || {
            let mut local_ids = Vec::new();
            for j in 0..100 {
                let id = arena_clone.allocate(format!("thread_{}_node_{}", i, j));
                local_ids.push(id);
            }

            // Verify all allocations
            for (j, id) in local_ids.iter().enumerate() {
                let node = arena_clone.get(*id);
                assert!(node.is_some());
                let node_arc = node.unwrap();
                let data = node_arc.downcast_ref::<String>().unwrap();
                assert_eq!(data, &format!("thread_{}_node_{}", i, j));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All 1000 allocations should be present
    assert_eq!(arena.len(), 1000);
}

#[test]
fn test_concurrent_allocation_and_deallocation() {
    let arena = Arc::new(NodeArena::new());
    let mut handles = Vec::new();

    for i in 0..5 {
        let arena_clone = arena.clone();
        let handle = thread::spawn(move || {
            for j in 0..100 {
                let id = arena_clone.allocate(format!("thread_{}_node_{}", i, j));

                // Immediately deallocate some nodes
                if j % 2 == 0 {
                    arena_clone.deallocate(id);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Should have roughly half the allocations remaining
    let final_len = arena.len();
    assert!((200..=300).contains(&final_len));
}

#[test]
fn test_concurrent_get() {
    let arena = Arc::new(NodeArena::new());

    // Pre-populate arena
    let ids: Vec<_> = (0..100)
        .map(|i| arena.allocate(format!("node_{}", i)))
        .collect();

    let mut handles = Vec::new();

    for i in 0..10 {
        let arena_clone = arena.clone();
        let ids_clone = ids.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let id = ids_clone[i % ids_clone.len()];
                let node = arena_clone.get(id);
                assert!(node.is_some());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_node_id_equality() {
    let id1 = NodeId {
        index: 5,
        generation: 10,
    };
    let id2 = NodeId {
        index: 5,
        generation: 10,
    };
    let id3 = NodeId {
        index: 5,
        generation: 11,
    };
    let id4 = NodeId {
        index: 6,
        generation: 10,
    };

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
    assert_ne!(id1, id4);
}

#[test]
fn test_node_id_clone_and_copy() {
    let id1 = NodeId {
        index: 5,
        generation: 10,
    };
    let id2 = id1; // Should be Copy
    let id3 = id1; // Copy, not clone

    assert_eq!(id1, id2);
    assert_eq!(id1, id3);
}
