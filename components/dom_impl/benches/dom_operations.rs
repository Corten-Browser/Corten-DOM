use browser_dom_impl::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn create_simple_tree() -> ParsedNode {
    ParsedNode {
        node_type: ParsedNodeType::Element,
        tag_name: Some("div".to_string()),
        attributes: HashMap::new(),
        text_content: None,
        children: vec![],
    }
}

fn benchmark_parsed_document(c: &mut Criterion) {
    c.bench_function("handle parsed document", |b| {
        b.iter(|| {
            let mut component = DomComponent::new(DomConfig::default());
            let msg = DomComponentMessage::ParsedDocument {
                request_id: 1,
                root: create_simple_tree(),
                doctype: None,
            };
            black_box(component.handle_message(msg));
        });
    });
}

fn benchmark_script_manipulation(c: &mut Criterion) {
    c.bench_function("handle script manipulation", |b| {
        b.iter(|| {
            let mut component = DomComponent::new(DomConfig::default());
            let msg = DomComponentMessage::ScriptManipulation {
                operation: DomOperation::SetAttribute {
                    element_id: 1,
                    name: "class".to_string(),
                    value: "test".to_string(),
                },
                node_id: 1,
                params: OperationParams::default(),
            };
            black_box(component.handle_message(msg));
        });
    });
}

criterion_group!(
    benches,
    benchmark_parsed_document,
    benchmark_script_manipulation
);
criterion_main!(benches);
