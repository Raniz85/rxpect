# RXpect

A Rust library for fluently building expectations in tests.

## What is fluent assertions?

Test assertions that are readable, with output that is understandable.

<pre>
failures:

---- fail stdout ----

thread 'main' (100481) panicked at .../entity.rs:57:13
Expectation failed (<span style="background: #ffd7d7">expected</span> == <span style="background: #d7ffd7">actual</span>)
 TestEntity {
<span style="background: #ffd7d7">-     id: <span style="background: #ffafaf">"foo",</span></span>
<span style="background: #d7ffd7">+     id: <span style="background: #afffaf">"bar",</span></span>
      value: 7,
<span style="background: #ffd7d7">-     content: "The quick brown <span style="background: #ffafaf">fox jumps over the dog",</span></span>
<span style="background: #ffd7d7">-     content_length: <span style="background: #ffafaf">43,</span></span>
<span style="background: #d7ffd7">+     content: "The <span style="background: #afffaf">lazy dog eats the </span>quick brow <span style="background: #afffaf">fox",</span></span>
<span style="background: #d7ffd7">+     content_length: <span style="background: #afffaf">37,</span></span>
 }
</pre>

If we're only asserting on equality, `assert_eq!` goes a long way,
but when assertions become more complex, it breaks down.

Consider that you want to make sure that an item is in a vector.
With standard asserts you'd have to write `assert!(haystack.contains(&needle))`. 
If that fails, the error is not the most helpful, it'll only tell you that the assertion failed and repeat the code in the assert macro - it gives you no information about the contents of the haystack, nor what the needle is.

With rxpect, you'll not only get a more readable assertion, the error message is more helpful too.

```rust,no_run
# #[cfg(not(feature = "itertools"))]
# fn main() {}
# #[cfg(feature = "itertools")]
# fn main() {
use rxpect::expect;
use rxpect::expectations::iterables::IterableItemEqualityExpectations;
let haystack = vec![1, 2, 3, 4, 5, 6];
let needle = 7;

// Expect to find the needle in the haystack
expect(haystack).to_contain_equal_to(needle);
# }
```

```shell
thread 'main' (311272) panicked at /home/raniz/src/rxpect/src/root.rs:54:13:
Expectation failed (a ⊇ b)
a: `[1, 2, 3, 4, 5, 6]`
b: `[7]`
```

## Another library for fluent assertions?

None of the other libraries worked quite like I wanted them to.
I also wanted to test my ideas about how a fluent assertion library in Rust could work.

## What about the name?

All other names I could come up with were already taken.

### What does it mean?

Either _Rust Expect_ or _Raniz' Expect_, pick whichever you like best.

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

You can even chain multiple assertions on the same value indefinitely, and all errors will be reported:

```rust,no_run
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, OrderExpectations};

expect(0)
    .to_equal(2)
    .to_be_greater_than(1);
```

```shell
thread 'main' (57062) panicked at /home/raniz/src/rxpect/rxpect/src/root.rs:53:17:
Expectation failed (expected == actual)
expected: `2`
  actual: `0`
Expectation failed (a > b)
a: `0`
b: `1`
```

For complex types, there exists the concept of projections,
which will add expectations on a projected value:

```rust
use rxpect::expect;
use rxpect::ExpectProjection;
use rxpect::expectations::EqualityExpectations;

#[derive(Debug)]
struct MyStruct {
    foo: i32,
}

let value = MyStruct { foo: 7 };

expect(value)
    .projected_by(|s| s.foo)
    .to_equal(7);
```

If you have multiple fields, you can "unproject" and continue with the parent value,
possibly projecting in a different way:

```rust
use rxpect::expect;
use rxpect::ExpectProjection;
use rxpect::expectations::EqualityExpectations;

#[derive(Debug)]
struct MyStruct {
    foo: i32,
    bar: &'static str,
}

let value = MyStruct { foo: 7, bar: "rxpect" };

expect(value)
    .projected_by(|s| s.foo)
        .to_equal(7)
        .unproject()
    .projected_by(|s| s.bar)
        .to_equal("rxpect");
```

You can even nest projections if necessary:


```rust
use rxpect::expect;
use rxpect::ExpectProjection;
use rxpect::expectations::EqualityExpectations;

#[derive(Debug)]
struct Parent {
    child: Child,
}

#[derive(Debug)]
struct Child {
    foo: i32,
}

let value = Parent { child: Child { foo: 7 } };
expect(value)
    .projected_by_ref(|p| &p.child)
        .projected_by(|s| s.foo)
            .to_equal(7);
```
## Finding expectations
All expectations are implemented as extension traits on the `ExpectationBuilder` trait.
This is to ensure extensibility and modularity.
This can make discovering expectations a bit tricky.
The easiest way to find them is to look at the various traits in the [`expectations`] module.

## Custom expectations
RXpect is built with extensibility in mind.
In fact, all bundled expectations are implemented in the same way as custom expectations should be - as extension traits.

To add a custom expectation, add a new extension trait and implement it for the `ExpectationBuilder` trait,
adding any restrictions on trait implementations of the type under test that you need through the type system (i.e. using generics or `where` clauses).
Have a look at the existing extension traits for inspiration.

```rust
use rxpect::expect;
use rxpect::ExpectationBuilder;
use rxpect::Expectation;
use rxpect::CheckResult;
use rxpect::expectations::PredicateExpectation;

// This is the extension trat that defines the extension methods
pub trait ToBeEvenExpectations {
    fn to_be_even(self) -> Self;
    fn to_be_odd(self) -> Self;
}

// implementation of the extension trait for ExpectationBuilder<'e, Value = u32>
impl<'e, B: ExpectationBuilder<'e, Value = u32>> ToBeEvenExpectations for B
{
    fn to_be_even(self) -> B {
        // Expectation implementation with a custom expectation implementation
        // Better if you need complex logic or more context,
        // also gives full control over the error handling
        self.to_pass(EvenExpectation)
    }
    
    fn to_be_odd(self) -> B {
        // Expectation implementation with a predicate
        // suitable for simpler checks
        self.to_pass(PredicateExpectation::new(
            // The expected/reference value, passed to both the predicate
            // and the error message producer. We don't use one here
            (),
            // The check, returns a bool
            |actual, _reference| actual % 2 != 0,
            // This is called to get the error message in case the check fails
            |actual, _reference| format!("Expected odd value, but got {actual}")
        ))
    }
}

// Custom expectation to implement the check
struct EvenExpectation;

impl Expectation<u32> for EvenExpectation {
    fn check(&self, value: &u32) -> CheckResult {
        if value % 2 == 0 {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!("Expected even value, but was {value}"))
        }
    }
}

expect(2).to_be_even();
expect(3).to_be_odd();
```

## Features

The following features are available and enabled by default:

* _iterables_, expectations regarding iterables. Pulls in [itertools](https://lib.rs/itertools) as a dependency.
* _diff, colored diffing of certain error messages. Pulls in [colored](https://lib.rs/colored) and [similar](https://lib.rs/similar) as dependencies

### Colour

`Colored` automatically determines if colours should be enabled or not. Sometimes it gets it wrong though,
and this can be controlled via the `CLICOLOR_FORCE` [environment variable](https://docs.rs/colored/latest/colored/control/struct.ShouldColorize.html#method.from_env).

This needs to be set, for example, for coloured output to show when running tests in [JetBrains RustRover](https://www.jetbrains.com/rust/).

## Credits
Fluent assertions is not my idea. Plenty of other projects implement the idea, both in Rust and other languages.

Here's a list of similar tools that I have used before and may or may not have inspired features in RXpect:

- [https://fluentassertions.com](Fluent Assertions (.NET))
- [https://jestjs.io](Jest (JS))
- [https://assertj.github.io](AssertJ (Java))

Huge thanks also goes out to the dependencies (which I try to keep minimal), without which, some functionality would have been a lot harder to implement:

- [Similar](https://insta.rs/similar) for diffing
- [Itertools](https://github.com/rust-itertools/itertools) for working with iterables
- [Colored](https://github.com/colored-rs/colored) for coloring output
- [rstest](https://github.com/la10736/rstest) for making writing parameterised tests bearable

## Contributing

Contributions are always welcome.

Before contributing anything, however,
make sure there is an open [Issue](https://github.com/raniz85/rxpect/issues) for your intended contribution.

If there is not already an open issue, please create one and describe your use case so we can discuss if it fits within RXxpect or is better suited to an extension crate.

Please keep pull requests focused and self-contained.
Stick to one expectation area (i.e. string, iterables, equality, etc.) per PR.

Fixup commits are explicitly forbidden, if you address comments on your pull request, squash them into the relevant commit.

`git commit --fixup` is a fantastic tool, [git-rebase.io](https://git-rebase.io/) contains more tips about a rebasing workflow.

I try my best to avoid merge commits, so favor rebasing on top of _main_ over merging.
There is a high chance that I will merge your contribution manually by rebasing and signing it with my private GPG key,
something that I cannot do with the GitHub UI.

I will not change authorship, your contributions are yours, but please sign your commits yourself too.

### Model-assisted Contributions

Responsible, well-thought-through, model-assisted contributions are more than welcome, slop is not.

_You_ are responsible for the output of the models that you wield.
If I can tell that your contribution (issue or code) is heavily generated without human oversight or editing,
I will reject it outright without further consideration.

My (Raniz') normal flow for model-assisted development in RXpect is to design a new expectation by hand and then have the model generate the other variants I want for me.
They are typically very good at that.