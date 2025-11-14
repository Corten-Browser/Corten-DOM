use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dom_storage::arena::NodeArena;

fn benchmark_allocation(c: &mut Criterion) {
    c.bench_function("allocate_single_node", |b| {
        let arena = NodeArena::new();
        b.iter(|| {
            let id = arena.allocate(black_box("test_node".to_string()));
            black_box(id);
        });
    });
}

fn benchmark_get(c: &mut Criterion) {
    let arena = NodeArena::new();
    let id = arena.allocate("test_node".to_string());

    c.bench_function("get_node", |b| {
        b.iter(|| {
            let node = arena.get(black_box(id));
            black_box(node);
        });
    });
}

fn benchmark_deallocation(c: &mut Criterion) {
    c.bench_function("deallocate_node", |b| {
        b.iter_batched(
            || {
                let arena = NodeArena::new();
                let id = arena.allocate("test_node".to_string());
                (arena, id)
            },
            |(arena, id)| {
                arena.deallocate(black_box(id));
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn benchmark_gc(c: &mut Criterion) {
    use dom_storage::gc::GarbageCollector;

    c.bench_function("gc_10k_nodes", |b| {
        b.iter_batched(
            || {
                let arena = NodeArena::new();
                let ids: Vec<_> = (0..10_000)
                    .map(|i| arena.allocate(format!("node_{}", i)))
                    .collect();
                let roots: Vec<_> = ids
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 2 == 0)
                    .map(|(_, &id)| id)
                    .collect();
                (arena, roots)
            },
            |(mut arena, roots)| {
                arena.collect(&roots);
                black_box(arena);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    benchmark_allocation,
    benchmark_get,
    benchmark_deallocation,
    benchmark_gc
);
criterion_main!(benches);
