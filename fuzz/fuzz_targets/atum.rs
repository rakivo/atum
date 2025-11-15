#![no_main]

use atum::AtomTable;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = match std::str::from_utf8(data) {
        Ok(v) => v,
        Err(_) => return,
    };

    let table = AtomTable::new();

    let id1 = table.intern(s);
    let id2 = table.intern(s);
    let s1 = table.lookup_ref(id1);
    let s2 = table.lookup_ref(id2);

    // invariants
    assert_eq!(id1, id2);
    assert_eq!(s1.as_ref(), s2.as_ref());
});

