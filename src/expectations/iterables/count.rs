use crate::expectation_list::ExpectationList;
use crate::{CheckResult, ExpectProjection, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for equality expectations for iterables
pub trait IterableCountExpectations<'e, I, C>
where
    I: Debug + 'e,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: Debug,
{
    /// Make expectations on the number of items in the iterable.
    ///
    /// Don't call this on never-ending iterables.
    ///
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::IterableCountExpectations;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let items = vec!["bar", "foo", "foo"];
    /// expect(items).count(|count| count.to_be_greater_than_or_equal(2));
    /// ```
    /// asserts that `items` contains at least 2 items
    fn count(
        self,
        config: impl FnOnce(ExpectationList<'e, usize>) -> ExpectationList<'e, usize>,
    ) -> Self;

    /// Expect an iterable to not be empty.
    ///
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::IterableCountExpectations;
    ///
    /// let items = vec!["bar", "foo", "foo"];
    /// expect(items).to_not_be_empty();
    /// ```
    /// asserts that `items` contains at least one item
    fn to_not_be_empty(self) -> Self;

    /// Expect an iterable to be empty.
    ///
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::IterableCountExpectations;
    ///
    /// let items: Vec<u8> = vec![];
    /// expect(items).to_be_empty();
    /// ```
    /// asserts that `items` contains no items
    fn to_be_empty(self) -> Self;
}

impl<'e, I, C, B> IterableCountExpectations<'e, I, C> for B
where
    I: Debug + 'e,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: Debug,
    B: ExpectationBuilder<'e, I>,
{
    fn count(
        self,
        config: impl FnOnce(ExpectationList<'e, usize>) -> ExpectationList<'e, usize>,
    ) -> Self {
        self.projected_by(|it| it.into_iter().count(), config)
    }

    fn to_not_be_empty(self) -> Self {
        self.to_pass(NotEmtpyExpectation {})
    }

    fn to_be_empty(self) -> Self {
        self.to_pass(EmtpyExpectation {})
    }
}

struct EmtpyExpectation;

impl<I, C> Expectation<I> for EmtpyExpectation
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        if value.into_iter().next().is_none() {
            CheckResult::Pass
        } else {
            CheckResult::Fail(
                "Expected iterable to be empty, but it had at least one item".to_string(),
            )
        }
    }
}

struct NotEmtpyExpectation;

impl<I, C> Expectation<I> for NotEmtpyExpectation
where
    I: Debug,
    for<'a> &'a I: IntoIterator<Item = &'a C>,
    C: Debug,
{
    fn check(&self, value: &I) -> CheckResult {
        if value.into_iter().next().is_some() {
            CheckResult::Pass
        } else {
            CheckResult::Fail("Expected iterable to not be empty, but it was".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IterableCountExpectations;
    use crate::expect;
    use crate::expectations::EqualityExpectations;
    use rstest::rstest;

    #[test]
    pub fn that_to_be_empty_accepts_empty_iterable() {
        // Given an empty vector
        let value: Vec<u32> = vec![];

        // Expect to_be_empty to pass
        expect(value).to_be_empty();
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_empty_does_not_accept_non_empty_iterable() {
        // Given a non-empty vector
        let value = vec![1];

        // Expect to_be_empty to fail
        expect(value).to_be_empty();
    }

    #[test]
    pub fn that_to_not_be_empty_accepts_non_empty_iterable() {
        // Given a non-empty vector
        let value = vec![1];

        // Expect to_not_be_empty to pass
        expect(value).to_not_be_empty();
    }

    #[test]
    #[should_panic]
    pub fn that_to_not_be_empty_does_not_accept_empty_iterable() {
        // Given an empty vector
        let value: Vec<u32> = vec![];

        // Expect to_not_be_empty to fail
        expect(value).to_not_be_empty();
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![1])]
    #[case(vec![1, 2])]
    #[case(vec![1, 2, 3])]
    pub fn that_count_projects_correctly(#[case] items: Vec<u32>) {
        let expected_count = items.len();
        // Expect the length() projection to project the length of the iterable
        expect(items).count(|count| count.to_equal(expected_count));
    }
}
