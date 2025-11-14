//! Demo of the Atom (string interning) system

use dom_types::AtomTable;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Atom System Demo ===\n");

    // Create atom table
    let table = AtomTable::new();
    println!(
        "1. Created AtomTable with {} pre-interned strings",
        table.len()
    );

    // String deduplication
    let atom1 = table.get_or_intern("hello");
    let atom2 = table.get_or_intern("hello");
    let atom3 = table.get_or_intern("world");

    println!("\n2. String Deduplication:");
    println!("   atom1 (hello) == atom2 (hello): {}", atom1 == atom2);
    println!("   atom1 (hello) == atom3 (world): {}", atom1 == atom3);

    // Atom resolution
    println!("\n3. Atom Resolution:");
    println!("   atom1 resolves to: '{}'", table.resolve(&atom1));
    println!("   atom3 resolves to: '{}'", table.resolve(&atom3));

    // Pre-interned strings
    println!("\n4. Pre-interned DOM strings:");
    let div = table.get_or_intern("div");
    let class = table.get_or_intern("class");
    let html_ns = table.get_or_intern("http://www.w3.org/1999/xhtml");

    println!("   Tag 'div': {}", table.resolve(&div));
    println!("   Attribute 'class': {}", table.resolve(&class));
    println!("   HTML namespace: {}", table.resolve(&html_ns));

    // Thread safety demo
    println!("\n5. Thread Safety:");
    let table = Arc::new(table);
    let mut handles = vec![];

    for i in 0..3 {
        let table_clone = Arc::clone(&table);
        let handle = thread::spawn(move || {
            let atom = table_clone.get_or_intern(&format!("thread_{}", i));
            (i, atom)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (thread_id, atom) = handle.join().unwrap();
        println!(
            "   Thread {} created atom for: '{}'",
            thread_id,
            table.resolve(&atom)
        );
    }

    println!("\n6. Final table size: {} unique strings", table.len());
    println!("\n=== Demo Complete ===");
}
