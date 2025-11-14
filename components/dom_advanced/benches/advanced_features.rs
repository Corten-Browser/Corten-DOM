use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dom_advanced::*;

fn benchmark_mutation_observer(c: &mut Criterion) {
    c.bench_function("mutation_observer_creation", |b| {
        b.iter(|| {
            let _observer = MutationObserver::new(|_records| {
                // Callback
            });
        });
    });
}

fn benchmark_range(c: &mut Criterion) {
    use dom_core::Document;

    c.bench_function("range_creation", |b| {
        let doc = Document::new();
        b.iter(|| {
            let _range = Range::new(black_box(&doc));
        });
    });
}

fn benchmark_selection(c: &mut Criterion) {
    c.bench_function("selection_creation", |b| {
        b.iter(|| {
            let _selection = Selection::new();
        });
    });
}

criterion_group!(
    benches,
    benchmark_mutation_observer,
    benchmark_range,
    benchmark_selection
);
criterion_main!(benches);
