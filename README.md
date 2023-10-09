# RXpect

A Rust library for fluently building expectations in tests.

## Another library for fluent assertions?

None of the other libraries worked quite like I wanted them to.
I also wanted to test my ideas about how a fluent assertion library in Rust could work.

## What about the name?

All other names I could come up with were already taken.

### What does it mean?

Either _Rust Expect_ or _Raniz Expect_, pick whichever you like best.

## How do I use this thing?

It's pretty simple actually,
wrap whatever you're having expectations on with `expect` and then call the different
extension methods.

```rust
use rxpect::expect;
use rxpect::expectations::EqualityExpectations;

// Expect 1 plus 1 to equal 2
expect(1 + 1).to_equal(2);
```

```shell
running 1 test
test tests::that_one_plus_one_equals_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Failures are neatly reported:

```rust,no_run
use rxpect::expect;
use rxpect::expectations::EqualityExpectations;

// Expect 1 plus 1 to equal 3
expect(1 + 1).to_equal(3);
```

```shell
thread 'main' panicked at 'Expectation failed (expected == actual)
expected: `3`
  actual: `2`'

```



## I don't like it

Use something else.
Here's a bunch of other crates that also does fluent expectations,
in no particular order:

- https://crates.io/crates/totems
- https://crates.io/crates/lets_expect
- https://crates.io/crates/fluent-assertions
- https://crates.io/crates/xpct
- https://crates.io/crates/expect
- https://crates.io/crates/fluent-asserter
- https://crates.io/crates/spectral
- https://crates.io/crates/assertables
- https://crates.io/crates/speculoos
- https://crates.io/crates/assert
- https://crates.io/crates/rassert
