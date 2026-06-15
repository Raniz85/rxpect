---
name: add-expectation
description: Scaffold a new RXpect expectation end-to-end (extension trait, Expectation impl, re-export, visualization, and TDD tests) following the repo's established patterns. Use when adding, extending, or implementing an assertion/matcher/expectation in the rxpect crate (e.g. "add a to_be_empty expectation", "add a new string matcher", "implement to_contain").
---

# Add an expectation

A new expectation in RXpect touches a fixed set of places. Do them in TDD order and
skip none — a half-finished expectation (missing re-export, missing visualization, or no
test for the feature-off build) is the most common failure mode.

Work in small batches and confirm the design with the maintainer before generating many
variants. The maintainer's normal flow is to hand-write one expectation and then have the
model generate the sibling variants from it.

## 0. Decide the area

Every expectation belongs to an *area*: `equality`, `order`, `boolean`, `string`,
`option`, `result`, `iterables`. Each area is one file under
`rxpect/src/expectations/<area>.rs` (iterables is a directory, `iterables/mod.rs` + submodules).

- Extending an existing area → edit that file's extension trait.
- A genuinely new area → new file + a `mod`/`pub use` line in
  `rxpect/src/expectations/mod.rs`. Keep one area per change/PR.

`iterables` is feature-gated. If the new area needs a dependency, add an optional dep and
a feature in `rxpect/Cargo.toml` and gate the module with `#[cfg(feature = "...")]`.

## 1. Write the failing tests first (red)

Tests live in a `#[cfg(test)] mod tests` at the bottom of the same area file. Match the
existing style exactly:

- given / when / then comments; multiple sections after the first start with "and".
- One `when` per test. Assert on content/equality, avoid asserting on length.
- A passing case (`expect(good).to_foo(...)`) and `#[should_panic]` failing cases.
- Use `#[rstest]` with `#[case(...)]` for parameterised cases (see `order.rs`).
- Stubs must return a real value — never `panic!`/`todo!` in the implementation to get red.
  Red must come from a *failing assertion*, not a compile error or a panic stub.

Run `just test` (or `cargo test <name>`) and confirm the new tests fail for the right reason.

## 2. Extension trait + impl (green)

Define a public extension trait named `<Area>Expectations` with the fluent method(s), each
carrying a doctest (these compile via the README/lib doc include, so they must pass):

```rust
/// Extension trait for <area> expectations
pub trait FooExpectations<'e, T> {
    /// Expect the value to ...
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::FooExpectations;
    /// expect(good).to_foo(arg);
    /// ```
    fn to_foo(self, value: T) -> Self;
}
```

Implement it for **any** `ExpectationBuilder`, not the concrete containers, so it works on
owned/ref/projected values:

```rust
impl<'e, T, B> FooExpectations<'e, T> for B
where
    T: /* constraints, e.g. PartialOrd */ + Debug + 'e,
    B: ExpectationBuilder<'e, Value = T>,
{
    fn to_foo(self, value: T) -> Self {
        self.to_pass(/* see step 3 */)
    }
}
```

Composite expectations may bound on another extension trait and delegate — e.g.
`boolean.rs`'s `to_be_true` requires `EqualityExpectations<bool, bool>` and calls
`self.to_equal(true)`. Prefer this when the check is just a special case of an existing one.

## 3. Pick the expectation backing

**Simple check → `PredicateExpectation`** (see `order.rs`). Pass the reference value, a
predicate `|actual, reference| -> bool`, and a message producer:

```rust
self.to_pass(PredicateExpectation::new(
    value,
    |a: &T, b: &T| a.lt(b),
    |a: &T, b: &T| format!("Expectation failed (a < b)\na: `{:?}`\nb: `{:?}`", a, b),
))
```

**Complex / richer message → custom `struct` implementing `Expectation<T>`** (see
`equality.rs`'s `ToEqualExpectation`). Return `CheckResult::Pass` / `CheckResult::Fail(msg)`.
If the message benefits from a coloured diff, gate it on the `diff` feature and provide a
plain fallback for `#[cfg(not(feature = "diff"))]`:

```rust
#[cfg(feature = "diff")]
{ /* diff_pretty_debug(...) message */ }
#[cfg(not(feature = "diff"))]
{ /* plain `{:?}` message */ }
```

Failure-message convention: first line `Expectation failed (<short symbolic form>)`, then
the operands on labelled lines (`a:`/`b:`, `expected:`/`  actual:`, `value:`/`range:`).

## 4. Re-export

For a new area file, add to `rxpect/src/expectations/mod.rs`:

```rust
mod foo;
pub use foo::*;
```

(Existing areas are already re-exported.)

## 5. Add a visualization

Every public-facing failure gets a visualization so its rendered message is exercised. In
`visualize/src/visualizations/<area>.rs`, add an entry per method that constructs a
**failing** assertion:

```rust
Visualization {
    header: "foo",
    name: "to_foo",
    message: || extract_failure_message(expect(bad).to_foo(arg)),
},
```

If you created a new area, add a `mod foo;` and a `visualizations.extend(foo::visualizations());`
line in `visualize/src/visualizations/mod.rs`. The `that_every_visualization_produces_a_failure_message`
test asserts none of them are empty — a passing assertion here will fail that test.

## 6. Verify (refactor stays green)

Run, in order, and make output pristine:

1. `just feature_tests` — proves it compiles and passes with features off and in every
   permutation (critical for anything `diff`- or `iterables`-gated, and for the plain
   fallback message).
2. `just lint` — clippy, all features, all targets.
3. `just checkFormat` — formatting.

`just prep_commit` (`format fix_lints test`) is the convenience bundle once green.

## Checklist

- [ ] Failing tests written first, given/when/then style, content assertions
- [ ] Extension trait `<Area>Expectations` with passing doctests
- [ ] `impl ... for B: ExpectationBuilder<'e, Value = T>` with proper `where` bounds
- [ ] `PredicateExpectation` or custom `Expectation` struct (diff-gated message if relevant)
- [ ] Re-exported from `expectations/mod.rs` (new areas only)
- [ ] Visualization added + registered (new areas only)
- [ ] `just feature_tests`, `just lint`, `just checkFormat` all clean
