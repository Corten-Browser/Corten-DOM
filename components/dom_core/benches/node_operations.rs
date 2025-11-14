//! Benchmarks for basic DOM node operations
//!
//! Measures performance of node creation, cloning, and basic operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dom_core::{Document, Node};
use parking_lot::RwLock;
use std::sync::Arc;

/// Benchmark createElement with different tag names
fn bench_create_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("createElement");

    let tag_names = vec!["div", "span", "p", "section", "article", "header"];

    for tag in &tag_names {
        group.bench_with_input(BenchmarkId::from_parameter(tag), tag, |b, &tag| {
            b.iter(|| {
                let mut doc = Document::new();
                black_box(doc.create_element(tag))
            });
        });
    }

    group.finish();
}

/// Benchmark createTextNode with different text sizes
fn bench_create_text_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("createTextNode");

    let long_text = "a".repeat(1000);
    let texts = vec![
        ("short", "Hello"),
        ("medium", "Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
        ("long", long_text.as_str()),
    ];

    for (name, text) in &texts {
        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, &text| {
            b.iter(|| {
                let mut doc = Document::new();
                black_box(doc.create_text_node(text))
            });
        });
    }

    group.finish();
}

/// Benchmark createComment
fn bench_create_comment(c: &mut Criterion) {
    c.bench_function("createComment", |b| {
        b.iter(|| {
            let mut doc = Document::new();
            black_box(doc.create_comment("test comment"))
        });
    });
}

/// Benchmark cloneNode (shallow)
fn bench_clone_node_shallow(c: &mut Criterion) {
    c.bench_function("cloneNode_shallow", |b| {
        let mut doc = Document::new();
        let elem = doc.create_element("div").unwrap();
        {
            let mut elem_write = elem.write();
            elem_write.set_attribute("class", "test").unwrap();
            elem_write.set_attribute("id", "element-123").unwrap();
        }

        b.iter(|| {
            let cloned = elem.read().clone_node(black_box(false));
            black_box(cloned)
        });
    });
}

/// Benchmark cloneNode (deep) with various tree depths
fn bench_clone_node_deep(c: &mut Criterion) {
    let mut group = c.benchmark_group("cloneNode_deep");

    for depth in [1, 3, 5, 10] {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            let mut doc = Document::new();
            let root = doc.create_element("div").unwrap();

            // Build tree of specified depth
            let mut current = root.clone();
            for _ in 0..depth {
                let child = doc.create_element("div").unwrap();
                current.write().append_child(
                    Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn dom_core::Node>))
                ).unwrap();
                current = child;
            }

            b.iter(|| {
                let cloned = root.read().clone_node(black_box(true));
                black_box(cloned)
            });
        });
    }

    group.finish();
}

/// Benchmark node type checking
fn bench_node_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("nodeType");

    let mut doc = Document::new();
    let element = doc.create_element("div").unwrap();
    let text = doc.create_text_node("test");
    let comment = doc.create_comment("test");

    group.bench_function("element", |b| {
        b.iter(|| {
            black_box(element.read().node_type())
        });
    });

    group.bench_function("text", |b| {
        b.iter(|| {
            black_box(text.read().node_type())
        });
    });

    group.bench_function("comment", |b| {
        b.iter(|| {
            black_box(comment.read().node_type())
        });
    });

    group.finish();
}

/// Benchmark contains() with different tree structures
fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");

    for depth in [1, 5, 10] {
        group.bench_with_input(BenchmarkId::from_parameter(depth), &depth, |b, &depth| {
            let mut doc = Document::new();
            let root = doc.create_element("div").unwrap();

            // Build chain
            let mut current = root.clone();
            let mut target = root.clone();
            for i in 0..depth {
                let child = doc.create_element("div").unwrap();
                current.write().append_child(
                    Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn dom_core::Node>))
                ).unwrap();
                if i == depth - 1 {
                    target = child.clone();
                }
                current = child;
            }

            b.iter(|| {
                black_box(root.read().contains(&*target.read()))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_create_element,
    bench_create_text_node,
    bench_create_comment,
    bench_clone_node_shallow,
    bench_clone_node_deep,
    bench_node_type,
    bench_contains,
);

criterion_main!(benches);
