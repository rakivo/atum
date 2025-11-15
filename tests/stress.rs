use atum::prelude::*;

use std::sync::Arc;
use std::thread;

#[test]
fn threaded_stress() {
    let table = Arc::new(AtomTable::new());

    let handles = (0..8)
        .map(|_| {
            let table = table.clone();
            thread::spawn(move || {
                for i in 0..100_000 {
                    let s = format!("key{}", i % 100);
                    let id = table.intern(&s);
                    assert_eq!(table.lookup_ref(id).as_ref(), s);
                    assert_eq!(table.intern(&s), id);
                }
            })
        })
        .collect::<Vec<_>>();

    for h in handles {
        h.join().unwrap();
    }
}
