use std::fmt::Debug;
use crate::{CheckResult, Expectation, ExpectationBuilder};

/// Extension trait for equality expectations for iterables
pub trait IterableItemEqualityExpectations<I, C>
where
    I: Debug,
    for <'a> &'a I: IntoIterator<Item=&'a C>,
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
    fn to_contain_equal_to_all_of(self, values: impl IntoIterator<Item=C>) -> Self;
}

impl<'e, I, C, B> IterableItemEqualityExpectations<I, C> for B
where
    I: Debug,
    for <'a> &'a I: IntoIterator<Item=&'a C>,
    C: PartialEq + Debug + 'e,
    B: ExpectationBuilder<'e, I>,
{
    fn to_contain_equal_to(self, value: C) -> Self {
        self.to_pass(ContainsEqualToExpectation(vec![value]))
    }

    fn to_contain_equal_to_all_of(self, values: impl IntoIterator<Item=C>) -> Self {
        self.to_pass(ContainsEqualToExpectation(values.into_iter().collect()))
    }
}

/// Expectation for to_equal
struct ContainsEqualToExpectation<T>(Vec<T>);

impl<I, C> Expectation<I> for ContainsEqualToExpectation<C>
where
    I: Debug,
    for <'a> &'a I: IntoIterator<Item=&'a C>,
    C: PartialEq + Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        if self.0.iter().all(|needle| value.into_iter()
            .any(|candidate| candidate.eq(needle))) {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (expected == actual)\nexpected: `{:?}`\n  actual: `{:?}`",
                &self.0, value
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterableItemEqualityExpectations;
    use crate::expect;

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
    pub fn that_inequal_values_are_not_considered_contained() {
        // Given a value that implements PartialEq
        let value = vec![1];

        // Expect the to_contain_equal_to expectation to fail with a different value
        expect(value).to_contain_equal_to(2);
    }

    #[test]
    pub fn that_order_of_items_is_insignificant_for_contains_all_of() {
        // Given a vector with multiple values
        let value = vec![1, 3, 5, 7, 8, 9];

        // Expect the to_contain_equal_to_all_of expectation to pass with values in different order
        expect(value).to_contain_equal_to_all_of([5, 1]);
    }
}
