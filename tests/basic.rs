use atum::prelude::*;

#[test]
fn test_small_strings() {
    let table = AtomTable::new();

    let id1 = table.intern("hello");
    let id2 = table.intern("world");
    let id3 = table.intern("hello");

    assert_eq!(id1, id3);
    assert_ne!(id1, id2);
}

#[test]
fn test_large_strings() {
    let table = AtomTable::new();

    let long_str = "this_is_a_very_long_identifier_name";
    let id1 = table.intern(long_str);
    let id2 = table.intern(long_str);

    assert_eq!(id1, id2);
    assert_eq!(table.len(), 1);
}

#[test]
fn test_resolve() {
    let table = AtomTable::new();

    let id = table.intern("test");
    let resolved = table.lookup_owned(id);

    assert_eq!(resolved, "test");
}
