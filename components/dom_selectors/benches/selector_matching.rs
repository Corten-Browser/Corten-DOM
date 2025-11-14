//! Benchmarks for CSS selector matching and querying
//!
//! Measures performance of querySelector, querySelectorAll, matches, and closest.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use dom_core::{Document, Node};
use dom_selectors::Selectable;
use parking_lot::RwLock;
use std::sync::Arc;

/// Build a realistic DOM tree for benchmarking
fn build_test_tree(doc: &mut Document) -> Arc<RwLock<dom_core::Element>> {
    let root = doc.create_element("div").unwrap();
    root.write().set_attribute("id", "root").unwrap();
    root.write().set_attribute("class", "container").unwrap();

    // Build: <div id="root" class="container">
    //   <header class="site-header">
    //     <nav class="navbar">
    //       <ul class="nav-list">
    //         <li class="nav-item active"><a href="#">Home</a></li>
    //         <li class="nav-item"><a href="#">About</a></li>
    //         ...
    //       </ul>
    //     </nav>
    //   </header>
    //   <main class="content">
    //     <article class="post featured">...</article>
    //     ...
    //   </main>
    // </div>

    let header = doc.create_element("header").unwrap();
    header.write().set_attribute("class", "site-header").unwrap();

    let nav = doc.create_element("nav").unwrap();
    nav.write().set_attribute("class", "navbar").unwrap();

    let ul = doc.create_element("ul").unwrap();
    ul.write().set_attribute("class", "nav-list").unwrap();

    for i in 0..10 {
        let li = doc.create_element("li").unwrap();
        li.write().set_attribute("class", if i == 0 { "nav-item active" } else { "nav-item" }).unwrap();

        let a = doc.create_element("a").unwrap();
        a.write().set_attribute("href", "#").unwrap();
        let text = doc.create_text_node(&format!("Item {}", i)).unwrap();
        a.write().append_child(
            Arc::new(RwLock::new(Box::new(text.read().clone()) as Box<dyn Node>))
        ).unwrap();

        li.write().append_child(
            Arc::new(RwLock::new(Box::new(a.read().clone()) as Box<dyn Node>))
        ).unwrap();

        ul.write().append_child(
            Arc::new(RwLock::new(Box::new(li.read().clone()) as Box<dyn Node>))
        ).unwrap();
    }

    nav.write().append_child(
        Arc::new(RwLock::new(Box::new(ul.read().clone()) as Box<dyn Node>))
    ).unwrap();
    header.write().append_child(
        Arc::new(RwLock::new(Box::new(nav.read().clone()) as Box<dyn Node>))
    ).unwrap();

    let main_elem = doc.create_element("main").unwrap();
    main_elem.write().set_attribute("class", "content").unwrap();

    for i in 0..20 {
        let article = doc.create_element("article").unwrap();
        article.write().set_attribute("class", if i == 0 { "post featured" } else { "post" }).unwrap();
        article.write().set_attribute("id", &format!("post-{}", i)).unwrap();

        let title = doc.create_element("h2").unwrap();
        let title_text = doc.create_text_node(&format!("Post {}", i)).unwrap();
        title.write().append_child(
            Arc::new(RwLock::new(Box::new(title_text.read().clone()) as Box<dyn Node>))
        ).unwrap();

        article.write().append_child(
            Arc::new(RwLock::new(Box::new(title.read().clone()) as Box<dyn Node>))
        ).unwrap();

        main_elem.write().append_child(
            Arc::new(RwLock::new(Box::new(article.read().clone()) as Box<dyn Node>))
        ).unwrap();
    }

    root.write().append_child(
        Arc::new(RwLock::new(Box::new(header.read().clone()) as Box<dyn Node>))
    ).unwrap();
    root.write().append_child(
        Arc::new(RwLock::new(Box::new(main_elem.read().clone()) as Box<dyn Node>))
    ).unwrap();

    root
}

/// Benchmark querySelector with different selector types
fn bench_query_selector(c: &mut Criterion) {
    let mut group = c.benchmark_group("querySelector");

    let selectors = vec![
        ("id", "#post-5"),
        ("class", ".nav-item"),
        ("tag", "article"),
        ("descendant", "nav li"),
        ("child", "ul > li"),
        ("complex", "main article.featured"),
        ("attribute", "[href='#']"),
    ];

    for (name, selector) in &selectors {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let root = build_test_tree(&mut doc);

            b.iter(|| {
                black_box(root.read().query_selector(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark querySelectorAll with different selector types
fn bench_query_selector_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("querySelectorAll");

    let selectors = vec![
        ("class", ".nav-item"),
        ("tag", "li"),
        ("descendant", "nav li"),
        ("child", "ul > li"),
        ("complex", "article.post"),
        ("universal", "*"),
    ];

    for (name, selector) in &selectors {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let root = build_test_tree(&mut doc);

            b.iter(|| {
                black_box(root.read().query_selector_all(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark matches() method
fn bench_matches(c: &mut Criterion) {
    let mut group = c.benchmark_group("matches");

    let selectors = vec![
        ("tag", "div"),
        ("class", ".container"),
        ("id", "#root"),
        ("complex", "div.container"),
        ("attribute", "[class]"),
    ];

    for (name, selector) in &selectors {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let root = build_test_tree(&mut doc);

            b.iter(|| {
                black_box(root.read().matches(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark closest() method
fn bench_closest(c: &mut Criterion) {
    let mut group = c.benchmark_group("closest");

    let selectors = vec![
        ("self", ".nav-item"),
        ("parent", "ul"),
        ("grandparent", "nav"),
        ("ancestor", ".container"),
    ];

    for (name, selector) in &selectors {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let root = build_test_tree(&mut doc);

            // Get a deep nav-item element
            let nav_item = root.read().query_selector(".nav-item.active").unwrap().unwrap();

            b.iter(|| {
                black_box(nav_item.read().closest(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark selector parsing and compilation
fn bench_selector_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_parsing");

    let selectors = vec![
        ("simple", "div"),
        ("class", ".container"),
        ("id", "#root"),
        ("complex", "nav ul > li.active"),
        ("very_complex", "div.container > main article.post.featured[id]"),
    ];

    for (name, selector) in &selectors {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let elem = doc.create_element("div").unwrap();

            b.iter(|| {
                // Parsing happens inside matches()
                black_box(elem.read().matches(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark combinators
fn bench_combinators(c: &mut Criterion) {
    let mut group = c.benchmark_group("combinators");

    let combinators = vec![
        ("descendant", "nav li"),
        ("child", "ul > li"),
        ("adjacent_sibling", "li + li"),
        ("general_sibling", "li ~ li"),
        ("multiple", "nav > ul > li.active"),
    ];

    for (name, selector) in &combinators {
        group.bench_with_input(BenchmarkId::from_parameter(name), selector, |b, &selector| {
            let mut doc = Document::new();
            let root = build_test_tree(&mut doc);

            b.iter(|| {
                black_box(root.read().query_selector_all(selector))
            });
        });
    }

    group.finish();
}

/// Benchmark pseudo-classes (if implemented)
fn bench_pseudo_classes(c: &mut Criterion) {
    let mut doc = Document::new();
    let root = build_test_tree(&mut doc);

    c.bench_function("first-child", |b| {
        b.iter(|| {
            // Note: Pseudo-classes may not be fully implemented yet
            // This serves as a performance baseline
            black_box(root.read().query_selector("li:first-child"))
        });
    });
}

criterion_group!(
    benches,
    bench_query_selector,
    bench_query_selector_all,
    bench_matches,
    bench_closest,
    bench_selector_parsing,
    bench_combinators,
    bench_pseudo_classes,
);

criterion_main!(benches);
