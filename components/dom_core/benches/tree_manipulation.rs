//! Benchmarks for DOM tree manipulation operations
//!
//! Measures performance of appendChild, removeChild, insertBefore, and tree traversal.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dom_core::{Document, Node};
use parking_lot::RwLock;
use std::sync::Arc;

/// Benchmark appendChild with varying number of children
fn bench_append_child(c: &mut Criterion) {
    let mut group = c.benchmark_group("appendChild");

    for num_children in [1, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_children),
            &num_children,
            |b, &num_children| {
                b.iter(|| {
                    let mut doc = Document::new();
                    let parent = doc.create_element("div").unwrap();

                    for _ in 0..num_children {
                        let child = doc.create_element("span").unwrap();
                        parent.write().append_child(
                            Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>))
                        ).unwrap();
                    }
                    black_box(parent)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark removeChild with varying number of children
fn bench_remove_child(c: &mut Criterion) {
    let mut group = c.benchmark_group("removeChild");

    for num_children in [1, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_children),
            &num_children,
            |b, &num_children| {
                b.iter_batched(
                    || {
                        // Setup: create parent with children
                        let mut doc = Document::new();
                        let parent = doc.create_element("div").unwrap();
                        let mut children = Vec::new();

                        for _ in 0..num_children {
                            let child = doc.create_element("span").unwrap();
                            let child_ref = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));
                            parent.write().append_child(child_ref.clone()).unwrap();
                            children.push(child_ref);
                        }

                        (parent, children)
                    },
                    |(parent, children)| {
                        // Benchmark: remove all children
                        for child in children {
                            parent.write().remove_child(child).unwrap();
                        }
                        black_box(parent)
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark insertBefore at different positions
fn bench_insert_before(c: &mut Criterion) {
    let mut group = c.benchmark_group("insertBefore");

    let positions = vec![
        ("beginning", 0),
        ("middle", 50),
        ("end", 99),
    ];

    for (name, insert_pos) in positions {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &insert_pos,
            |b, &insert_pos| {
                b.iter_batched(
                    || {
                        // Setup: create parent with 100 children
                        let mut doc = Document::new();
                        let parent = doc.create_element("div").unwrap();
                        let mut children = Vec::new();

                        for _ in 0..100 {
                            let child = doc.create_element("span").unwrap();
                            let child_ref = Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>));
                            parent.write().append_child(child_ref.clone()).unwrap();
                            children.push(child_ref);
                        }

                        let new_child = doc.create_element("strong").unwrap();
                        let new_child_ref = Arc::new(RwLock::new(Box::new(new_child.read().clone()) as Box<dyn Node>));

                        (parent, children, new_child_ref, insert_pos)
                    },
                    |(parent, children, new_child, insert_pos)| {
                        let ref_child = if insert_pos < children.len() {
                            Some(children[insert_pos].clone())
                        } else {
                            None
                        };
                        parent.write().insert_before(new_child, ref_child).unwrap();
                        black_box(parent)
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark childNodes traversal
fn bench_child_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("childNodes");

    for num_children in [10, 50, 100, 500] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_children),
            &num_children,
            |b, &num_children| {
                // Setup: create parent with children
                let mut doc = Document::new();
                let parent = doc.create_element("div").unwrap();

                for _ in 0..num_children {
                    let child = doc.create_element("span").unwrap();
                    parent.write().append_child(
                        Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>))
                    ).unwrap();
                }

                b.iter(|| {
                    let children = parent.read().child_nodes();
                    black_box(children.len())
                });
            },
        );
    }

    group.finish();
}

/// Benchmark parentNode traversal up the tree
fn bench_parent_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("parentNode");

    for depth in [1, 5, 10, 20] {
        group.bench_with_input(
            BenchmarkId::from_parameter(depth),
            &depth,
            |b, &depth| {
                // Setup: create chain of elements
                let mut doc = Document::new();
                let root = doc.create_element("div").unwrap();
                let mut current = root.clone();
                let mut leaf = root.clone();

                for _ in 0..depth {
                    let child = doc.create_element("div").unwrap();
                    current.write().append_child(
                        Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>))
                    ).unwrap();
                    leaf = child.clone();
                    current = child;
                }

                b.iter(|| {
                    // Traverse up to root
                    let mut count = 0;
                    let mut node = Some(Arc::new(RwLock::new(Box::new(leaf.read().clone()) as Box<dyn Node>)));
                    while let Some(n) = node {
                        count += 1;
                        node = n.read().parent_node();
                    }
                    black_box(count)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark building a realistic DOM tree
fn bench_build_tree(c: &mut Criterion) {
    c.bench_function("build_realistic_tree", |b| {
        b.iter(|| {
            let mut doc = Document::new();

            // Build: <div><ul><li>Item 1</li><li>Item 2</li>...</ul></div>
            let container = doc.create_element("div").unwrap();
            let list = doc.create_element("ul").unwrap();

            for i in 0..20 {
                let item = doc.create_element("li").unwrap();
                let text = doc.create_text_node(&format!("Item {}", i));

                item.write().append_child(
                    Arc::new(RwLock::new(Box::new(text.read().clone()) as Box<dyn Node>))
                ).unwrap();

                list.write().append_child(
                    Arc::new(RwLock::new(Box::new(item.read().clone()) as Box<dyn Node>))
                ).unwrap();
            }

            container.write().append_child(
                Arc::new(RwLock::new(Box::new(list.read().clone()) as Box<dyn Node>))
            ).unwrap();

            black_box(container)
        });
    });
}

criterion_group!(
    benches,
    bench_append_child,
    bench_remove_child,
    bench_insert_before,
    bench_child_nodes,
    bench_parent_node,
    bench_build_tree,
);

criterion_main!(benches);
