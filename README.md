# `atum`

Lock-free bidirectional Atom Table, optimized for maximum performance in multi-threaded workloads.

Designed for compilers, interpreters, and any system that needs fast, concurrent string interning.

## Example
```rust
use atum::AtomTable;

let tbl = AtomTable::new();
let atom = tbl.intern("Hello, Sailor!");

assert_eq!(tbl.lookup_ref(atom).as_ref(), "Hello, Sailor!");
assert_eq!(tbl.intern("Hello, Sailor!"), atom);

let guard = tbl.pin();
let strings = &[
    "unfortunately", "there's", "a",
    "radio", "connected", "to", "my", "brain"
];
for &s in strings {
    let atom = tbl.intern_with_guard(s, &guard);
    assert_eq!(tbl.lookup_ref(atom).as_ref(), s);
    assert_eq!(tbl.intern_with_guard(s, &guard), atom);
}
```

# Why `atum`?
> Benchmarks are going to be here soon ..

# NOTICE
> This is the first version of atum. It doesn’t yet include all advanced optimizations, but it’s already as fast or faster than `lasso` in multithreaded scenarios. Future updates may improve performance further.
> Also: the naming is experimental, it might change.

# License

MIT OR Apache-2.0
