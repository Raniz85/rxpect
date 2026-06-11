# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

RXpect is a Rust library for fluently building expectations (fluent assertions) in
tests. The published crate is `rxpect`; the workspace also contains a `visualize`
binary used to render example failure output.

## Commands

The `Justfile` is the source of truth for development commands:

- `just test` — run tests (`cargo test`, default features)
- `just feature_tests` — run tests across feature permutations (`--no-default-features`,
  then each of `diff`, `iterables`, `diff,iterables`). Run this when touching anything
  feature-gated, since the default `cargo test` only exercises the default feature set.
- `just lint` — `cargo clippy --all-targets --tests --all-features`
- `just fix_lints` — clippy with `--fix --allow-dirty`
- `just checkFormat` / `just format` — `cargo fmt --all [--check]`
- `just ci` — `checkFormat lint test` (what CI runs)
- `just prep_commit` — `format fix_lints test`

Run a single test: `cargo test <test_name>` (e.g. `cargo test that_one_plus_one_equals_two`).
Tests use `rstest` for parameterised cases.

The `README.md` files (workspace root and `rxpect/`) are compiled as doctests
(`rxpect/src/lib.rs` does `#![doc=include_str!("../README.md")]`), so code examples in the
README must compile and pass.

## Architecture

The whole library is built around a small set of core traits in `rxpect/src/lib.rs`,
with every concrete assertion living in an **extension trait**. This is deliberate:
extensibility and modularity are the central design goal — bundled expectations are
implemented exactly the way a third-party custom expectation would be.

Core pieces:

- `Expectation<T>` (`lib.rs`) — a single check: `fn check(&self, value: &T) -> CheckResult`,
  where `CheckResult` is `Pass` or `Fail(String)`.
- `ExpectationBuilder<'e>` (`lib.rs`) — the fluent surface. Has an associated
  `Value` type and `to_pass(self, expectation) -> Self`. **Extension traits are written
  against `ExpectationBuilder`, not against the concrete containers**, so they work
  uniformly on owned values, references, and projections.
- `expect(value)` / `expect_ref(&value)` (`lib.rs`) — entry points returning
  `OwnedExpectations` / `RefExpectations` (`root.rs`).
- `ExpectationList` (`expectation_list.rs`) — holds boxed expectations and runs **all**
  of them in order, concatenating every failure message (assertions do not short-circuit).
- Containers (`root.rs`) run their expectations **on `Drop`** and panic on failure. For
  `OwnedExpectations` you can instead call `.check()` to recover the owned value, or
  `.check_result()` to get `(value, CheckResult)` without panicking.

Projections (`projection.rs`, `ExpectProjection` trait): `.projected_by(|s| s.field)`
makes expectations on a derived value, `.unproject()` returns to the parent, and
projections nest. Extraction can fail (returns `Option`); a `None` produces a failure
message. `BorrowedOrOwned` (`borrow.rs`) lets a projection return either a borrow or an
owned value.

`Predicate<T>` (`predicate.rs`) and `PredicateExpectation` (`expectations/predicate.rs`)
are the simple path for custom checks — pass a closure plus a message producer instead of
writing a full `Expectation` impl.

The `diff` feature (`diff.rs`) produces coloured, inline diffs of `Debug` output for
equality-style failures via `similar` + `colored`. Colour is auto-detected; force it with
`CLICOLOR_FORCE`.

### Adding an expectation

Follow the existing pattern (see `expectations/equality.rs` for the canonical example):

1. Define a public extension trait (e.g. `FooExpectations`) with the fluent method(s).
2. `impl<'e, ..., B: ExpectationBuilder<'e, Value = T>> FooExpectations for B`, encoding any
   constraints on `T` via `where` clauses.
3. Inside, call `self.to_pass(...)` with either a `PredicateExpectation` (simple) or a
   custom `struct` implementing `Expectation<T>` (complex / richer error messages).
4. Re-export it from `expectations/mod.rs` (`mod foo; pub use foo::*;`).

Keep one expectation area (string, iterables, equality, …) per change; the project values
small, focused, cohesive diffs.

## Features

- `iterables` (default) — iterable expectations; pulls in `itertools`.
- `diff` (default) — coloured diffing of failure messages; pulls in `colored` + `similar`.

When adding feature-gated code, gate it with `#[cfg(feature = "...")]` and verify with
`just feature_tests` so it still compiles and passes with the feature off.

## Constraints from the maintainer

- Repository of record is Codeberg (`https://codeberg.org/raniz/rxpect`);
  Commits are GPG-signed by the maintainer.
- Contributions stay focused: one expectation area per PR, no fixup/merge commits (rebase
  onto `main`).
