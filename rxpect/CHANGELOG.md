## 0.11.0

### Breaking changes

* `RootExpectations` is split into `OwnedExpectations` and
  `RefExpectations` - you shouldn't rely on these struct names though
* `ProjectedResult-`/`OptionExpectations` merged into their respective
  "normal" extension trait
* `to_be_all_whitespace` no longer accepts empty strings, this aligns it
  with the other assertions on string content

### Additions

* Support for coloured diffs in error messages for expectations where it
  makes sense (gated behind the default feature _diff_)
* Add `StringExpectations::to_equal_str` which uses the string itself
  for the coloured diff instead of the debug representation
* Add `check_result` to get the owned value out of `OwnedExpectations`
  together with the result of the checks (i.e. does not panic on
  failure)

### Misc

* Lots of documentation improvements

## 0.10.0

### Additions

* Add projected_by_ref which works with references

### Misc

* Improved documentation
* Improve compilation by making T in ExpectationBuilder an associated
  type

## 0.9.0

### Breaking changes

* Rework projections

### Misc

* Lots of cleanup and clarifications

This release introduces backwards-incompatible changes with 0.8.0

## 0.8.0

### Changes

* Expanded `to_equal` to include all implementations of PartialEq<Rhs>
  instead of just `T: PartialEq<T>`

## 0.7.0

### Additions

* `to_be_equivalent_in_any_order(iterable)`: For checking that two
  iterables contain exactly the same items, regardless of order
* `length()`: Projects the count of an iterable, consuming it and
  counting the number of items.
* `to_be_empty()`: Asserts that an iterable contains no items
* `to_not_be_empty()`: Asserts that an iterable contains at least one
  item

## 0.6.0

### Additions

* More expectations on strings
* Chainable expectations for Result: ok_and, err_and

## 0.5.0

### Additions

* Add order.to_be_inside for ranges
* Add expectations for Result<T, E>

## 0.4.0

### Additions

* Add expectations for iterables

## 0.3.0

### Additions

* Add expectations for true/false

## 0.2.0

### Additions

* Add expectations for PartialOrd

## 0.1.1

Name change

## 0.1.0

Initial release, POC with expect(x).to_equal(y)

