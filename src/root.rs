use crate::expectation_list::ExpectationList;
use crate::{AspectExpectations, CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

pub struct RootExpectations<'e, T: Debug> {
    value: T,
    expectations: ExpectationList<'e, T>,
}

impl<'e, T: Debug> RootExpectations<'e, T> {
    pub(crate) fn new(value: T) -> Self {
        RootExpectations {
            expectations: ExpectationList::new(),
            value,
        }
    }

    pub fn aspect<U: Debug + 'e>(
        self,
        transform: impl FnOnce(&T) -> U,
    ) -> AspectExpectations<'e, Self, T, U> {
        let value = transform(&self.value);
        AspectExpectations::new(self, value)
    }

    /// Manually run all the expectations
    pub fn check(self) {
        drop(self)
    }
}

impl<'e, T: Debug> ExpectationBuilder<'e, T> for RootExpectations<'e, T> {
    /// Add an expectation to the list of expectations
    fn add_expectation(&mut self, expectation: impl Expectation<T> + 'e) -> &mut Self {
        self.expectations.push(expectation);
        self
    }
}

impl<'e, T: Debug> Drop for RootExpectations<'e, T> {
    fn drop(&mut self) {
        if let CheckResult::Fail(message) = self.expectations.check(&self.value) {
            panic!("{}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestExpectation;
    use crate::{expect, CheckResult, ExpectationBuilder};

    #[test]
    pub fn that_assert_runs_an_expectation() {
        // Given an expectation
        let (expectation, expected) = TestExpectation::new(CheckResult::Pass);

        // And expectations containing it
        let mut expectations = expect(true);
        expectations.add_expectation(expectation);

        // When the expectations are checked
        expectations.check();

        // Then the expectation was run
        assert!(*expected.lock().unwrap());
    }

    #[test]
    pub fn that_check_runs_all_expectations() {
        // Given two expectations that both pass
        let (expectation1, expected1) = TestExpectation::new(CheckResult::Pass);
        let (expectation2, expected2) = TestExpectation::new(CheckResult::Pass);

        // And expectations containing those
        let mut expectations = expect(true);
        expectations.add_expectation(expectation1);
        expectations.add_expectation(expectation2);

        // When the expectations are checked
        expectations.check();

        // Then both expectations were run
        assert!(*expected1.lock().unwrap());
        assert!(*expected2.lock().unwrap());
    }

    #[test]
    #[should_panic]
    pub fn that_failure_panics() {
        // Given an expectation that fails
        let (expectation, _) = TestExpectation::new(CheckResult::Fail("message".to_owned()));

        // And expectations containing it
        let mut expectations = expect(true);
        expectations.add_expectation(expectation);

        // Expect a panic when checked
        expectations.check();
    }
}