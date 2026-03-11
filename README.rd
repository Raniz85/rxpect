# RXpect

A Rust library for fluently building expectations in tests.

```rust,no_run
use rxpect::expect;
use rxpect::expectations::iterables::IterableItemEqualityExpectations;
let haystack = vec![1, 2, 3, 4, 5, 6];
let needle = 7;

// Expect to find the needle in the haystack
expect(haystack).to_contain_equal_to(needle);
```

```shell
thread 'main' (311272) panicked at /home/raniz/src/rxpect/src/root.rs:54:13:
Expectation failed (a ⊇ b)
a: `[1, 2, 3, 4, 5, 6]`
b: `[7]`

## Workspace contents

* [rxpect](/rxpect/README.md) - the core _rxpect_ crate
