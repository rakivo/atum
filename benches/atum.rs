use atum::AtomTable;

use std::sync::Arc;
use std::thread;

use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_intern(c: &mut Criterion) {
    let table = AtomTable::new();
    let keys: Vec<String> = (0..10_000).map(|i| format!("key{}", i)).collect();

    c.bench_function("atum intern", |b| {
        b.iter(|| {
            for k in &keys {
                black_box(table.intern(k));
            }
        })
    });
}

fn bench_lookup(c: &mut Criterion) {
    let table = AtomTable::new();
    let keys: Vec<String> = (0..10_000).map(|i| format!("key{}", i)).collect();
    let ids: Vec<_> = keys.iter().map(|k| table.intern(k)).collect();

    c.bench_function("atum lookup_ref", |b| {
        b.iter(|| {
            for id in &ids {
                black_box(table.lookup_ref(*id));
            }
        })
    });
}

fn bench_multithreaded_intern(c: &mut Criterion) {
    let table = Arc::new(AtomTable::new());
    let keys: Vec<String> = (0..10_000).map(|i| format!("key{}", i)).collect();

    c.bench_function("atum multithreaded intern", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..16 {
                let table = table.clone();
                let keys = keys.clone();
                handles.push(thread::spawn(move || {
                    for k in &keys {
                        black_box(table.intern(k));
                    }
                }));
            }
            for h in handles {
                h.join().unwrap();
            }
        })
    });
}

criterion_group!(
    benches,
    bench_intern,
    bench_lookup,
    bench_multithreaded_intern,
);
criterion_main!(benches);
