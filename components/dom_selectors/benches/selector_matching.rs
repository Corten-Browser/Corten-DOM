//! Benchmarks for CSS selector matching

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dom_core::{Document, Element};
use dom_selectors::Selectable;

fn bench_simple_selector(c: &mut Criterion) {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    // Add some elements
    for i in 0..100 {
        let elem = doc.create_element("span").unwrap();
        elem.write()
            .set_attribute("class", &format!("item-{}", i))
            .unwrap();
    }

    c.bench_function("simple selector", |b| {
        b.iter(|| {
            let _result = root.read().query_selector(black_box("span"));
        });
    });
}

fn bench_class_selector(c: &mut Criterion) {
    let mut doc = Document::new();
    let root = doc.create_element("div").unwrap();

    for i in 0..100 {
        let elem = doc.create_element("span").unwrap();
        elem.write()
            .set_attribute("class", &format!("item-{}", i))
            .unwrap();
    }

    c.bench_function("class selector", |b| {
        b.iter(|| {
            let _result = root.read().query_selector(black_box(".item-50"));
        });
    });
}

criterion_group!(benches, bench_simple_selector, bench_class_selector);
criterion_main!(benches);
