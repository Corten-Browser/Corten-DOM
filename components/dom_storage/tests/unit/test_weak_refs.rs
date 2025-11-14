use dom_storage::arena::NodeArena;
use dom_storage::weak_refs::WeakNodeRef;

#[test]
fn test_weak_ref_upgrade_succeeds() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak_ref = WeakNodeRef::new(&arena, node_id);

    // Should be able to upgrade
    let strong = weak_ref.upgrade();
    assert!(strong.is_some());
}

#[test]
fn test_weak_ref_upgrade_fails_after_deallocation() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak_ref = WeakNodeRef::new(&arena, node_id);

    // Deallocate the node
    arena.deallocate(node_id);

    // Upgrade should fail
    let strong = weak_ref.upgrade();
    assert!(strong.is_none());
}

#[test]
fn test_weak_ref_does_not_prevent_deallocation() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak_ref = WeakNodeRef::new(&arena, node_id);

    // Holding weak ref should not prevent deallocation
    arena.deallocate(node_id);

    // Node should be gone
    assert!(arena.get(node_id).is_none());
    assert!(weak_ref.upgrade().is_none());
}

#[test]
fn test_multiple_weak_refs() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak1 = WeakNodeRef::new(&arena, node_id);
    let weak2 = WeakNodeRef::new(&arena, node_id);
    let weak3 = WeakNodeRef::new(&arena, node_id);

    // All should upgrade successfully
    assert!(weak1.upgrade().is_some());
    assert!(weak2.upgrade().is_some());
    assert!(weak3.upgrade().is_some());

    // After deallocation, none should upgrade
    arena.deallocate(node_id);

    assert!(weak1.upgrade().is_none());
    assert!(weak2.upgrade().is_none());
    assert!(weak3.upgrade().is_none());
}

#[test]
fn test_weak_ref_clone() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak1 = WeakNodeRef::new(&arena, node_id);
    let weak2 = weak1.clone();

    // Both should work
    assert!(weak1.upgrade().is_some());
    assert!(weak2.upgrade().is_some());
}

#[test]
fn test_weak_ref_with_different_generations() {
    let arena = NodeArena::new();

    // Allocate, deallocate, and reallocate in same slot
    let id1 = arena.allocate("first".to_string());
    let weak1 = WeakNodeRef::new(&arena, id1);

    arena.deallocate(id1);

    let id2 = arena.allocate("second".to_string());

    // weak1 should not upgrade (old generation)
    assert!(weak1.upgrade().is_none());

    // New weak ref should work
    let weak2 = WeakNodeRef::new(&arena, id2);
    assert!(weak2.upgrade().is_some());
}

#[test]
fn test_weak_ref_equality() {
    let arena = NodeArena::new();
    let node_id = arena.allocate("test".to_string());

    let weak1 = WeakNodeRef::new(&arena, node_id);
    let weak2 = WeakNodeRef::new(&arena, node_id);

    // Should be equal (point to same node)
    assert_eq!(weak1, weak2);
}

#[test]
fn test_weak_ref_inequality() {
    let arena = NodeArena::new();
    let id1 = arena.allocate("test1".to_string());
    let id2 = arena.allocate("test2".to_string());

    let weak1 = WeakNodeRef::new(&arena, id1);
    let weak2 = WeakNodeRef::new(&arena, id2);

    // Should not be equal (different nodes)
    assert_ne!(weak1, weak2);
}

#[test]
fn test_weak_ref_upgrade_returns_correct_data() {
    let arena = NodeArena::new();
    let test_data = "test_data".to_string();
    let node_id = arena.allocate(test_data.clone());

    let weak_ref = WeakNodeRef::new(&arena, node_id);

    let strong = weak_ref.upgrade().unwrap();
    let data = strong.downcast_ref::<String>().unwrap();
    assert_eq!(data, &test_data);
}

#[test]
fn test_concurrent_weak_refs() {
    use std::sync::Arc;
    use std::thread;

    let arena = Arc::new(NodeArena::new());
    let node_id = arena.allocate("test".to_string());

    let weak_ref = WeakNodeRef::new(&arena, node_id);

    let mut handles = Vec::new();

    for _ in 0..10 {
        let weak_clone = weak_ref.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _strong = weak_clone.upgrade();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Weak ref should still work
    assert!(weak_ref.upgrade().is_some());
}
