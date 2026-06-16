# RXpect

A Rust library for fluently building expectations in tests.

This GitHub repository is a mirror of [the real repository on Codeberg](https://codeberg.org/raniz/rxpect),
please use that as this repository may experience force pushes.

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
```

## Workspace contents

* [rxpect](./rxpect/README.md) - the core _rxpect_ crate

## Contributing

Contributions are always welcome.

Before contributing anything, however,
make sure there is an open [Issue](https://codeberg.org/raniz/rxpect/issues) for your intended contribution.

If there is not already an open issue,
please create one and describe your use case so we can discuss if it fits within RXpect or is better suited to an extension crate.

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
