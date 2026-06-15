use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Combinator for evaluating multiple expectations together
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Combinator {
    /// OR semantics - i.e. at least one of the expectations must pass
    Or,
    /// AND semantics - i.e. all of the expectations must pass
    And,
}

impl Combinator {
    /// Check if the given number of failures and passes is a pass according to this combinator
    pub fn is_pass(&self, num_failures: usize, num_passes: usize) -> bool {
        match self {
            Combinator::Or => num_passes > 0,
            Combinator::And => num_failures == 0,
        }
    }
}

/// List of expectations on a value.
pub struct ExpectationList<'e, T>(Vec<Box<dyn Expectation<T> + 'e>>);

impl<'e, T: Debug> ExpectationList<'e, T> {
    /// Creates a new empty list of expectations.
    pub fn new() -> Self {
        ExpectationList(Vec::new())
    }

    /// Add a new expectation to the list.
    pub fn push(&mut self, expectation: impl Expectation<T> + 'e) {
        self.0.push(Box::new(expectation));
    }

    /// Check all expectations on the value.
    ///
    /// Runs _all_ expectations in order.
    ///
    /// # Returns
    /// `CheckResult::Pass` if all expectations pass, otherwise `CheckResult::Fail` with a formatted message.
    /// If multiple failures occur, they are concatenated with newlines.
    pub fn check(&self, value: &T) -> CheckResult {
        self.check_with_combinator(value, Combinator::And)
    }

    /// Check all expectations on the value.
    ///
    /// Runs _all_ expectations in order, evaluating the result according to a Combinator.
    ///
    /// # Returns
    /// `CheckResult::Pass` if the combinator deems the evaluation to be a pass, otherwise `CheckResult::Fail` with a formatted message.
    /// If multiple failures occur, they are concatenated with newlines.
    pub fn check_with_combinator(&self, value: &T, combinator: Combinator) -> CheckResult {
        let failures = self
            .0
            .iter()
            .map(|e| e.check(value))
            .filter_map(|r| match r {
                CheckResult::Fail(message) => Some(message),
                _ => None,
            })
            .collect::<Vec<String>>();
        if combinator.is_pass(failures.len(), self.0.len() - failures.len()) {
            CheckResult::Pass
        } else {
            CheckResult::Fail(failures.join("\n"))
        }
    }
}

impl<'e, T: Debug> Default for ExpectationList<'e, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'e, T> ExpectationBuilder<'e> for ExpectationList<'e, T>
where
    T: Debug + 'e,
{
    type Value = T;

    fn to_pass(mut self, expectation: impl Expectation<T> + 'e) -> Self {
        self.push(expectation);
        self
    }
}
#[cfg(test)]
mod tests {
    use super::Combinator;
    use super::ExpectationList;
    use crate::expectations::PredicateExpectation;
    use crate::{CheckResult, Expectation, ExpectationBuilder, expect};
    use rstest::rstest;
    use std::fmt::Debug;

    struct TestExpectation(bool);
    impl<T> Expectation<T> for TestExpectation
    where
        T: Debug,
    {
        fn check(&self, _value: &T) -> CheckResult {
            if self.0 {
                CheckResult::Pass
            } else {
                CheckResult::Fail("Failure".to_string())
            }
        }
    }

    trait CheckResultExpectations {
        fn to_be_a_pass(self) -> Self;
        fn to_be_a_failure(self) -> Self;
    }

    impl<'e, B> CheckResultExpectations for B
    where
        B: ExpectationBuilder<'e, Value = CheckResult>,
    {
        fn to_be_a_pass(self) -> Self {
            self.to_pass(PredicateExpectation::new(
                (),
                |it, ()| matches!(it, CheckResult::Pass),
                |it, _| format!("Expected CheckResult::Pass, got {:?}", it),
            ))
        }

        fn to_be_a_failure(self) -> Self {
            self.to_pass(PredicateExpectation::new(
                (),
                |it, ()| matches!(it, CheckResult::Fail(_)),
                |it, _| format!("Expected CheckResult::Pass, got {:?}", it),
            ))
        }
    }

    fn expectations_from_booleans<'e, T: Debug + 'e>(
        booleans: impl IntoIterator<Item = bool>,
    ) -> ExpectationList<'e, T> {
        let mut list = ExpectationList::new();
        for b in booleans {
            list.push(TestExpectation(b))
        }
        list
    }

    #[rstest]
    #[case::all_fail_single([false])]
    #[case::all_fail_double([false, false])]
    #[case::all_fail_triple([false, false, false])]
    #[case::one_fail_double([false, true])]
    #[case::one_fail_triple([false, true, true])]
    #[case::two_fail_triple([false, false, true])]
    pub fn that_evaluating_with_and_fails_when_there_is_at_least_one_failure(
        #[case] input: impl IntoIterator<Item = bool>,
    ) {
        // Given a list of expectations that may or may not fail
        let list: ExpectationList<'_, ()> = expectations_from_booleans(input);

        // When evaluating the list using AND-semantics
        let result = list.check_with_combinator(&(), Combinator::And);

        // Then the result should be a failure
        expect(result).to_be_a_failure();
    }

    #[rstest]
    #[case::single([true])]
    #[case::double([true, true])]
    #[case::triple([true, true, true])]
    pub fn that_evaluating_with_and_passes_when_there_is_only_passes(
        #[case] input: impl IntoIterator<Item = bool>,
    ) {
        // Given a list of expectations that all pass
        let list: ExpectationList<'_, ()> = expectations_from_booleans(input);

        // When evaluating the list using AND-semantics
        let result = list.check_with_combinator(&(), Combinator::And);

        // Then the result should be a pass
        expect(result).to_be_a_pass();
    }

    #[rstest]
    #[case::all_fail_single([false])]
    #[case::all_fail_double([false, false])]
    #[case::all_fail_triple([false, false, false])]
    pub fn that_evaluating_with_or_fails_when_there_is_only_failures(
        #[case] input: impl IntoIterator<Item = bool>,
    ) {
        // Given a list of expectations that all fail
        let list: ExpectationList<'_, ()> = expectations_from_booleans(input);

        // When evaluating the list using OR-semantics
        let result = list.check_with_combinator(&(), Combinator::Or);

        // Then the result should be a failure
        expect(result).to_be_a_failure();
    }

    #[rstest]
    #[case::all_pass_single([true])]
    #[case::all_pass_double([true, true])]
    #[case::all_pass_triple([true, true, true])]
    #[case::one_pass_double([false, true])]
    #[case::one_pass_triple([false, false, true])]
    #[case::two_pass_triple([false, true, true])]
    pub fn that_evaluating_with_or_passes_when_there_is_at_least_one_pass(
        #[case] input: impl IntoIterator<Item = bool>,
    ) {
        // Given a list of expectations that all pass
        let list: ExpectationList<'_, ()> = expectations_from_booleans(input);

        // When evaluating the list using OR-semantics
        let result = list.check_with_combinator(&(), Combinator::Or);

        // Then the result should be a pass
        expect(result).to_be_a_pass();
    }
}
