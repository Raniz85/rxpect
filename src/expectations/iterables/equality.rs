use crate::{CheckResult, Expectation, ExpectationBuilder};
use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use std::fmt::Debug;

/// Extension trait for equality expectations for iterables
pub trait IterableItemEqualityExpectations<I, C>
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug,
{
    /// Expect an iterable to contain at least one value equal to another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::IterableItemEqualityExpectations;
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
    /// # use rxpect::expectations::IterableItemEqualityExpectations;
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
    /// # use rxpect::expectations::IterableItemEqualityExpectations;
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
    /// # use rxpect::expectations::IterableItemEqualityExpectations;
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

impl<'e, I, C, B> IterableItemEqualityExpectations<I, C> for B
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: PartialEq + Debug + 'e,
    B: ExpectationBuilder<'e, I>,
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
        if self
            .0
            .iter()
            .zip_longest(value.into_iter())
            .all(|pair| match pair {
                Both(a, b) => a.eq(b),
                _ => false,
            })
        {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (a == b)\na: `{:?}`\nb: `{:?}`",
                value, self.0
            ))
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
        let mut remaining: Vec<&C> = self.0.iter().collect();
        let mut extras: Vec<&C> = Vec::new();
        for actual in value.into_iter() {
            if let Some(pos) = remaining.iter().position(|e| (*e).eq(actual)) {
                // Remove matched item; swap_remove is O(1)
                remaining.swap_remove(pos);
            } else {
                // No match found for this actual item; record as extra and continue
                extras.push(actual);
            }
        }
        if remaining.is_empty() && extras.is_empty() {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (a ≅ b, any order)\na: `{:?}`\nb: `{:?}`\nextra: `{:?}`\nunmatched: `{:?}`",
                value, self.0, extras, remaining
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterableItemEqualityExpectations;
    use crate::expect;
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
        let value = vec![];

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
    pub fn thaht_empty_list_is_contained() {
        // Given a vec with a value that implements PartialEq
        let value = vec![1];

        // Expect the to_contain_equal_to_all_of expectation to pass with an empty list
        expect(value).to_contain_equal_to_all_of([]);
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
}
