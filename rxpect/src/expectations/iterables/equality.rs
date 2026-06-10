#[cfg(feature = "diff")]
use crate::diff::{Color, diff_pretty_debug, format_flagged_list};
use crate::{CheckResult, Expectation, ExpectationBuilder};
#[cfg(feature = "diff")]
use colored::Colorize;
use dedent::dedent;
use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use std::fmt::Debug;

/// Extension trait for equality expectations for iterables
pub trait IterableItemEqualityExpectations<'e, B, C>
where
    B: ExpectationBuilder<'e>,
    C: PartialEq + Debug + 'e,
{
    /// Expect an iterable to contain at least one value equal to another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::iterables::IterableItemEqualityExpectations;
    ///
    /// let haystack = vec!["bar", "foo", "foo"];
    /// let needle = "foo";
    /// expect(haystack).to_contain_equal_to(needle);
    /// ```
    /// asserts that `haystack` contains at least one item equal to `needle`
    fn to_contain_equal_to(self, value: C) -> Self;

    /// Expect an iterable to contain at least one value equal to another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::iterables::IterableItemEqualityExpectations;
    ///
    /// let haystack = vec!["apple", "orange", "pear", "apple", "peach"];
    /// let needles = ["orange", "apple"];
    /// expect(haystack).to_contain_equal_to_all_of(needles);
    /// ```
    /// asserts that `haystack` contains at least one item equal to each item in `needles`
    fn to_contain_equal_to_all_of(self, values: impl IntoIterator<Item = C>) -> Self;

    /// Expect an iterable to be equivalent to another iterable
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::iterables::IterableItemEqualityExpectations;
    ///
    /// let a = vec!["apple", "orange", "pear", "apple", "peach"];
    /// let b = ["apple", "orange", "pear", "apple", "peach"];
    /// expect(a).to_be_equivalent_to(b);
    /// ```
    /// asserts that `a` contains exactly the same items in the same order as `b`
    fn to_be_equivalent_to(self, values: impl IntoIterator<Item = C>) -> Self;

    /// Expect an iterable to be equivalent to another iterable, ignoring the order of items
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::iterables::IterableItemEqualityExpectations;
    ///
    /// let a = vec!["apple", "orange", "pear", "apple", "peach"];
    /// let b = ["orange", "peach", "apple", "apple", "pear"];
    /// let c = ["peach", "apple", "pear", "orange", "apple"];
    /// expect(a.clone()).to_be_equivalent_to_in_any_order(b);
    /// expect(a).to_be_equivalent_to_in_any_order(c);
    /// expect(b).to_be_equivalent_to_in_any_order(c);
    /// ```
    /// asserts that `a` contains exactly the same items in the same order as `b`
    fn to_be_equivalent_to_in_any_order(self, values: impl IntoIterator<Item = C>) -> Self;
}

impl<'e, I, C, B> IterableItemEqualityExpectations<'e, B, C> for B
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug + 'e,
    B: ExpectationBuilder<'e, Value = I>,
{
    fn to_contain_equal_to(self, value: C) -> Self {
        self.to_pass(ContainsEqualToExpectation(vec![value]))
    }

    fn to_contain_equal_to_all_of(self, values: impl IntoIterator<Item = C>) -> Self {
        self.to_pass(ContainsEqualToExpectation(values.into_iter().collect()))
    }

    fn to_be_equivalent_to(self, values: impl IntoIterator<Item = C>) -> Self {
        self.to_pass(IterableIsEquivalentToExpectation(
            values.into_iter().collect(),
        ))
    }

    fn to_be_equivalent_to_in_any_order(self, values: impl IntoIterator<Item = C>) -> Self {
        self.to_pass(IterableIsEquivalentToInAnyOrderExpectation(
            values.into_iter().collect(),
        ))
    }
}

struct ContainsEqualToExpectation<T>(Vec<T>);

struct IterableIsEquivalentToExpectation<T>(Vec<T>);

struct IterableIsEquivalentToInAnyOrderExpectation<T>(Vec<T>);

impl<I, C> Expectation<I> for ContainsEqualToExpectation<C>
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        if self
            .0
            .iter()
            .all(|needle| value.into_iter().any(|candidate| candidate.eq(needle)))
        {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (a ⊇ b)\na: `{:?}`\nb: `{:?}`",
                value, self.0
            ))
        }
    }
}

impl<I, C> Expectation<I> for IterableIsEquivalentToExpectation<C>
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        if self.0.iter().zip_longest(value).all(|pair| match pair {
            Both(a, b) => a.eq(b),
            _ => false,
        }) {
            CheckResult::Pass
        } else {
            #[cfg(feature = "diff")]
            {
                let diff = diff_pretty_debug(&self.0, value);
                CheckResult::Fail(format!(
                    "Expectation failed ({} == {})\n{diff}",
                    "expected".on_ansi_color(Color::RemovedRow),
                    "actual".on_ansi_color(Color::AddedRow)
                ))
            }
            #[cfg(not(feature = "diff"))]
            {
                CheckResult::Fail(format!(
                    "Expectation failed (a == b)\na: `{:?}`\nb: `{:?}`",
                    value, self.0
                ))
            }
        }
    }
}

impl<I, C> Expectation<I> for IterableIsEquivalentToInAnyOrderExpectation<C>
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        let value = value.into_iter().collect_vec();
        let mut remaining: Vec<&C> = self.0.iter().collect();
        let mut extras: Vec<&C> = Vec::new();
        for actual in value.iter() {
            if let Some(pos) = remaining.iter().position(|e| (*e).eq(actual)) {
                // Remove matched item to keep track of remaining ones
                remaining.remove(pos);
            } else {
                // No match found for this item; record as extra and continue
                extras.push(actual);
            }
        }
        if remaining.is_empty() && extras.is_empty() {
            CheckResult::Pass
        } else {
            #[cfg(feature = "diff")]
            {
                if remaining.len() == 1 && extras.len() == 1 {
                    // Special case when there is only one element differing
                    let remaining = remaining[0];
                    let extra = extras[0];
                    let diff = diff_pretty_debug(remaining, extra);
                    CheckResult::Fail(format!(
                        "Expectation failed ({} == {}, any order)\nSingle differing element\n{diff}",
                        "expected".on_ansi_color(Color::RemovedRow),
                        "actual".on_ansi_color(Color::AddedRow)
                    ))
                } else {
                    CheckResult::Fail(format!(
                        dedent!(
                            r#"
                        Expectation failed ({} == {}, any order)
                        expected: {}
                        actual: {}"#
                        ),
                        "expected".on_ansi_color(Color::RemovedRow),
                        "actual".on_ansi_color(Color::AddedRow),
                        format_flagged_list(
                            &self.0.iter().collect_vec(),
                            &remaining,
                            '-',
                            Color::RemovedRow
                        ),
                        format_flagged_list(&value, &extras, '+', Color::AddedRow)
                    ))
                }
            }
            #[cfg(not(feature = "diff"))]
            {
                CheckResult::Fail(format!(
                    dedent!(
                        r#"
                    Expectation failed (expected == actual, any order)
                    actual: {}
                    expected: {}
                    extra: {}
                    unmatched: {}"#
                    ),
                    format!("{:#?}", value),
                    format!("{:#?}", self.0),
                    format!("{:#?}", extras),
                    format!("{:#?}", remaining)
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterableItemEqualityExpectations;
    #[cfg(feature = "diff")]
    use super::{IterableIsEquivalentToExpectation, IterableIsEquivalentToInAnyOrderExpectation};
    #[cfg(feature = "diff")]
    use crate::diff::{Color, diff_pretty_debug, format_flagged_list};
    use crate::expect;
    #[cfg(feature = "diff")]
    use crate::expectations::EqualityExpectations;
    #[cfg(feature = "diff")]
    use crate::{CheckResult, Expectation};
    #[cfg(feature = "diff")]
    use colored::Colorize;
    #[cfg(feature = "diff")]
    use dedent::dedent;
    #[cfg(feature = "diff")]
    use itertools::Itertools;
    use rstest::rstest;

    #[test]
    pub fn that_singleton_vec_contains_the_one_item() {
        // Given a vector with a single value that implements PartialEq
        let value = vec![1];

        // Expect the to_contain_equal_to expectation to pass with an identical value
        expect(value).to_contain_equal_to(1);
    }

    #[test]
    #[should_panic]
    pub fn that_empty_vec_does_not_contain_an_item() {
        // Given an empty vec
        let value: Vec<u32> = vec![];

        // Expect the to_contain_equal_to expectation to fail
        expect(value).to_contain_equal_to(1);
    }

    #[test]
    #[should_panic]
    pub fn that_unequal_values_are_not_considered_contained() {
        // Given a vec with a value that implements PartialEq
        let value = vec![1];

        // Expect the to_contain_equal_to expectation to fail with a different value
        expect(value).to_contain_equal_to(2);
    }

    #[test]
    pub fn that_empty_list_is_contained() {
        // Given a vec with a value that implements PartialEq
        let value = vec![1];

        // Expect the to_contain_equal_to_all_of expectation to pass with an empty list
        expect(value).to_contain_equal_to_all_of(Vec::<i32>::new());
    }

    #[test]
    pub fn that_order_of_items_is_insignificant_for_contains_all_of() {
        // Given a vector with multiple values
        let value = vec![1, 3, 5, 7, 8, 9];

        // Expect the to_contain_equal_to_all_of expectation to pass with values in different order
        expect(value).to_contain_equal_to_all_of([5, 1]);
    }

    #[rstest]
    #[case(vec![5, 1])]
    #[case(vec![1, 3, 5, 7, 8])]
    #[case(vec![3, 5, 7, 8, 9])]
    #[case(vec![1, 5, 3, 7, 8, 9])]
    #[case(vec![9, 8, 7, 5, 3, 1])]
    #[should_panic]
    pub fn that_nonequivalent_collections_are_not_considered_equal(
        #[case] non_equivalent: Vec<u32>,
    ) {
        // Given a vector with multiple values
        let value = vec![1, 3, 5, 7, 8, 9];

        // Expect the to_be_equivalent_to expectation to fail with an unequal collection
        expect(value).to_be_equivalent_to(non_equivalent);
    }

    #[rstest]
    #[case(vec![1, 1, 3, 5, 7, 8, 3, 9])]
    #[case(vec![1, 3, 1, 5, 7, 8, 3, 9])]
    #[case(vec![9, 3, 8, 7, 5, 3, 1, 1])]
    #[case(vec![7, 3, 1, 9, 1, 3, 8, 5])]
    pub fn that_equivalent_collections_are_considered_equivalent_regardless_of_order(
        #[case] non_equivalent: Vec<u32>,
    ) {
        // Given a vector with multiple values
        let value = vec![1, 1, 3, 5, 7, 8, 3, 9];

        // Expect the to_be_equivalent_to expectation to pass with an unequal collection
        expect(value).to_be_equivalent_to_in_any_order(non_equivalent);
    }

    #[rstest]
    #[case(vec![5, 1])]
    #[case(vec![1, 3, 5, 7, 8])]
    #[case(vec![3, 5, 7, 8, 9])]
    #[case(vec![1, 5, 3, 7, 8, 9])]
    #[case(vec![1, 3, 3, 5, 7, 8, 3, 9])]
    #[case(vec![1, 1, 1, 5, 7, 8, 3, 9])]
    #[case(vec![1, 1, 3, 7, 7, 8, 3, 9])]
    #[case(vec![1, 1, 3, 6, 7, 8, 3, 9])]
    #[should_panic]
    pub fn that_nonequivalent_collections_are_not_considered_equal_regardless_of_order(
        #[case] non_equivalent: Vec<u32>,
    ) {
        // Given a vector with multiple values
        let value = vec![1, 1, 3, 5, 7, 8, 3, 9];

        // Expect the to_be_equivalent_to expectation to fail with an unequal collection
        expect(value).to_be_equivalent_to_in_any_order(non_equivalent);
    }

    #[cfg(feature = "diff")]
    #[test]
    pub fn that_equivalent_to_with_diffing_returns_colored_diff() {
        // Given an actual collection and an expected collection that differ
        let actual = vec![1, 2, 3];
        let expected = vec![1, 4, 3];

        // When the equivalence expectation is checked
        let result = IterableIsEquivalentToExpectation(expected.clone()).check(&actual);

        // Then the failure message contains the colored diff of expected against actual
        let message = match result {
            CheckResult::Fail(message) => message,
            _ => "Passed".to_string(),
        };
        expect(message).to_equal(format!(
            "Expectation failed ({} == {})\n{}",
            "expected".on_ansi_color(Color::RemovedRow),
            "actual".on_ansi_color(Color::AddedRow),
            diff_pretty_debug(&expected, &actual)
        ));
    }

    #[cfg(feature = "diff")]
    #[test]
    pub fn that_equivalent_to_in_any_order_with_single_difference_returns_colored_diff() {
        // Given an actual collection and an expected collection that differ in a single element
        let actual = vec![1, 2, 3];
        let expected = vec![1, 4, 3];

        // When the any-order equivalence expectation is checked
        let result = IterableIsEquivalentToInAnyOrderExpectation(expected).check(&actual);

        // Then the failure message contains the colored diff of the single differing element
        let message = match result {
            CheckResult::Fail(message) => message,
            _ => "Passed".to_string(),
        };
        expect(message).to_equal(format!(
            "Expectation failed ({} == {}, any order)\nSingle differing element\n{}",
            "expected".on_ansi_color(Color::RemovedRow),
            "actual".on_ansi_color(Color::AddedRow),
            diff_pretty_debug(&4, &2)
        ));
    }

    #[cfg(feature = "diff")]
    #[test]
    pub fn that_equivalent_to_in_any_order_with_multiple_differences_returns_colored_set_difference()
     {
        // Given an actual collection and an expected collection that differ in multiple elements
        let actual = vec![1, 2, 9, 8];
        let expected = vec![1, 2, 3, 4];

        // When the any-order equivalence expectation is checked
        let result = IterableIsEquivalentToInAnyOrderExpectation(expected.clone()).check(&actual);

        // Then the failure message contains the colored set difference of missing and extra items
        let message = match result {
            CheckResult::Fail(message) => message,
            _ => "Passed".to_string(),
        };
        expect(message).to_equal(format!(
            dedent!(
                r#"
            Expectation failed ({} == {}, any order)
            expected: {}
            actual: {}
            "#
            ),
            "expected".on_ansi_color(Color::RemovedRow),
            "actual".on_ansi_color(Color::AddedRow),
            format_flagged_list(
                &expected.iter().collect_vec(),
                &[&expected[2], &expected[3]],
                '-',
                Color::RemovedRow
            ),
            format_flagged_list(
                &actual.iter().collect_vec(),
                &[&actual[2], &actual[3]],
                '+',
                Color::AddedRow
            )
        ));
    }
}
