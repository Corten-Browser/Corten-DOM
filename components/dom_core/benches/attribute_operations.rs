//! Benchmarks for DOM attribute operations
//!
//! Measures performance of getAttribute, setAttribute, removeAttribute, and attribute enumeration.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dom_core::Document;

/// Benchmark setAttribute with different value sizes
fn bench_set_attribute(c: &mut Criterion) {
    let mut group = c.benchmark_group("setAttribute");

    let values = vec![
        ("short", "test"),
        ("medium", "Lorem ipsum dolor sit amet"),
        ("long", &"a".repeat(1000)),
    ];

    for (name, value) in &values {
        group.bench_with_input(BenchmarkId::from_parameter(name), value, |b, &value| {
            let mut doc = Document::new();
            let elem = doc.create_element("div").unwrap();

            b.iter(|| {
                elem.write().set_attribute("data-value", value).unwrap();
                black_box(&elem)
            });
        });
    }

    group.finish();
}

/// Benchmark getAttribute
fn bench_get_attribute(c: &mut Criterion) {
    let mut doc = Document::new();
    let elem = doc.create_element("div").unwrap();
    elem.write().set_attribute("class", "container active").unwrap();
    elem.write().set_attribute("id", "main-container").unwrap();
    elem.write().set_attribute("data-value", "test").unwrap();

    c.bench_function("getAttribute", |b| {
        b.iter(|| {
            black_box(elem.read().get_attribute("class"))
        });
    });
}

/// Benchmark hasAttribute
fn bench_has_attribute(c: &mut Criterion) {
    let mut doc = Document::new();
    let elem = doc.create_element("div").unwrap();
    elem.write().set_attribute("class", "container").unwrap();
    elem.write().set_attribute("id", "main").unwrap();

    let mut group = c.benchmark_group("hasAttribute");

    group.bench_function("exists", |b| {
        b.iter(|| {
            black_box(elem.read().has_attribute("class"))
        });
    });

    group.bench_function("missing", |b| {
        b.iter(|| {
            black_box(elem.read().has_attribute("data-missing"))
        });
    });

    group.finish();
}

/// Benchmark removeAttribute
fn bench_remove_attribute(c: &mut Criterion) {
    c.bench_function("removeAttribute", |b| {
        b.iter_batched(
            || {
                let mut doc = Document::new();
                let elem = doc.create_element("div").unwrap();
                elem.write().set_attribute("class", "test").unwrap();
                elem.write().set_attribute("id", "element").unwrap();
                elem.write().set_attribute("data-value", "123").unwrap();
                elem
            },
            |elem| {
                elem.write().remove_attribute("class").unwrap();
                black_box(elem)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

/// Benchmark attributes() enumeration
fn bench_attributes_enumeration(c: &mut Criterion) {
    let mut group = c.benchmark_group("attributes_enumeration");

    for num_attrs in [1, 5, 10, 20] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_attrs),
            &num_attrs,
            |b, &num_attrs| {
                let mut doc = Document::new();
                let elem = doc.create_element("div").unwrap();

                for i in 0..num_attrs {
                    elem.write().set_attribute(&format!("data-attr-{}", i), &format!("value-{}", i)).unwrap();
                }

                b.iter(|| {
                    let attrs = elem.read().attributes();
                    black_box(attrs.len())
                });
            },
        );
    }

    group.finish();
}

/// Benchmark class manipulation
fn bench_class_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("classList");

    let class_strings = vec![
        ("single", "container"),
        ("multiple", "container active large primary"),
        ("many", &vec!["class"; 20].join(" ")),
    ];

    for (name, classes) in &class_strings {
        group.bench_with_input(BenchmarkId::from_parameter(name), classes, |b, &classes| {
            let mut doc = Document::new();
            let elem = doc.create_element("div").unwrap();

            b.iter(|| {
                elem.write().set_attribute("class", classes).unwrap();
                let class_list = elem.read().class_list();
                black_box(class_list.len())
            });
        });
    }

    group.finish();
}

/// Benchmark ID operations
fn bench_id_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("id_operations");

    group.bench_function("set_id", |b| {
        let mut doc = Document::new();
        let elem = doc.create_element("div").unwrap();

        b.iter(|| {
            elem.write().set_attribute("id", "element-123").unwrap();
            black_box(&elem)
        });
    });

    group.bench_function("get_id", |b| {
        let mut doc = Document::new();
        let elem = doc.create_element("div").unwrap();
        elem.write().set_attribute("id", "element-123").unwrap();

        b.iter(|| {
            black_box(elem.read().id())
        });
    });

    group.finish();
}

/// Benchmark getAttributeNode
fn bench_get_attribute_node(c: &mut Criterion) {
    let mut doc = Document::new();
    let elem = doc.create_element("div").unwrap();
    elem.write().set_attribute("class", "container").unwrap();
    elem.write().set_attribute("id", "main").unwrap();

    c.bench_function("getAttributeNode", |b| {
        b.iter(|| {
            black_box(elem.read().get_attribute_node("class"))
        });
    });
}

/// Benchmark setAttributeNode
fn bench_set_attribute_node(c: &mut Criterion) {
    c.bench_function("setAttributeNode", |b| {
        b.iter_batched(
            || {
                let mut doc = Document::new();
                let elem = doc.create_element("div").unwrap();
                let attr = doc.create_attribute("class").unwrap();
                attr.write().set_value("container");
                (elem, attr)
            },
            |(elem, attr)| {
                elem.write().set_attribute_node(attr).unwrap();
                black_box(elem)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_set_attribute,
    bench_get_attribute,
    bench_has_attribute,
    bench_remove_attribute,
    bench_attributes_enumeration,
    bench_class_list,
    bench_id_operations,
    bench_get_attribute_node,
    bench_set_attribute_node,
);

criterion_main!(benches);
