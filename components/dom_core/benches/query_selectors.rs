//! Benchmarks for basic DOM querying operations
//!
//! Measures performance of getElementById, getElementsByTagName, getElementsByClassName

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dom_core::{Document, Node};
use parking_lot::RwLock;
use std::sync::Arc;

/// Build a document with many elements for querying
fn build_large_document(num_elements: usize) -> Document {
    let mut doc = Document::new();

    for i in 0..num_elements {
        let elem = doc.create_element("div").unwrap();
        elem.write().set_attribute("id", &format!("element-{}", i)).unwrap();
        elem.write().set_attribute("class", &format!("item group-{}", i % 10)).unwrap();
    }

    doc
}

/// Benchmark getElementById with various document sizes
fn bench_get_element_by_id(c: &mut Criterion) {
    let mut group = c.benchmark_group("getElementById");

    for num_elements in [10, 100, 1000, 5000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_elements),
            &num_elements,
            |b, &num_elements| {
                let doc = build_large_document(num_elements);
                let target_id = format!("element-{}", num_elements / 2);

                b.iter(|| {
                    black_box(doc.get_element_by_id(&target_id))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark getElementById when element doesn't exist
fn bench_get_element_by_id_miss(c: &mut Criterion) {
    let doc = build_large_document(1000);

    c.bench_function("getElementById_miss", |b| {
        b.iter(|| {
            black_box(doc.get_element_by_id("nonexistent"))
        });
    });
}

/// Benchmark getElementsByTagName
fn bench_get_elements_by_tag_name(c: &mut Criterion) {
    let mut group = c.benchmark_group("getElementsByTagName");

    for num_elements in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_elements),
            &num_elements,
            |b, &num_elements| {
                let mut doc = Document::new();
                let root = doc.create_element("div").unwrap();

                for _ in 0..num_elements {
                    let child = doc.create_element("span").unwrap();
                    root.write().append_child(
                        Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>))
                    ).unwrap();
                }

                b.iter(|| {
                    black_box(root.read().get_elements_by_tag_name("span"))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark getElementsByClassName
fn bench_get_elements_by_class_name(c: &mut Criterion) {
    let mut group = c.benchmark_group("getElementsByClassName");

    for num_elements in [10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_elements),
            &num_elements,
            |b, &num_elements| {
                let mut doc = Document::new();
                let root = doc.create_element("div").unwrap();

                for i in 0..num_elements {
                    let child = doc.create_element("span").unwrap();
                    child.write().set_attribute("class", &format!("item group-{}", i % 10)).unwrap();
                    root.write().append_child(
                        Arc::new(RwLock::new(Box::new(child.read().clone()) as Box<dyn Node>))
                    ).unwrap();
                }

                b.iter(|| {
                    black_box(root.read().get_elements_by_class_name("item"))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_get_element_by_id,
    bench_get_element_by_id_miss,
    bench_get_elements_by_tag_name,
    bench_get_elements_by_class_name,
);

criterion_main!(benches);
