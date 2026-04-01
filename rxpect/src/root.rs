use crate::expectation_list::ExpectationList;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Container for expectations on an owned value.
///
/// Returned by [expect](crate::expect)
///
/// Contains the value under test and all expectations.
/// Expectations are automatically run when `OwnedExpectations` is dropped.
/// Alternatively, call [check](OwnedExpectations::check) to run expectations and recover ownership of the value.
///
/// Expectation failures result in a panic
pub struct OwnedExpectations<'e, T: Debug> {
    value: Option<T>,
    expectations: ExpectationList<'e, T>,
}

impl<'e, T: Debug> OwnedExpectations<'e, T> {
    /// Create new `OwnedExpectations` on an owned value
    pub fn new(value: T) -> Self {
        Self {
            expectations: ExpectationList::new(),
            value: Some(value),
        }
    }

    /// Run all expectations and return the owned value.
    pub fn check(mut self) -> T {
        let value = self
            .value
            .take()
            .expect("Check can only be called once, hence value must be Some");
        let result = self.expectations.check(&value);
        if let CheckResult::Fail(message) = result {
            panic!("{}", message);
        }
        value
    }
}

impl<'e, T: Debug + 'e> ExpectationBuilder<'e> for OwnedExpectations<'e, T> {
    type Value = T;

    fn to_pass(mut self, expectation: impl Expectation<T> + 'e) -> Self {
        self.expectations.push(expectation);
        self
    }
}

impl<'e, T: Debug> Drop for OwnedExpectations<'e, T> {
    fn drop(&mut self) {
        // If check has been called value is None here and we shouldn't recheck
        if let Some(value) = self.value.take()
            && let CheckResult::Fail(message) = self.expectations.check(&value)
        {
            panic!("{}", message);
        }
    }
}

/// Container for expectations on a borrowed value.
///
/// Returned by [expect_ref](crate::expect_ref)
///
/// Contains a reference to the value under test and all expectations.
/// Expectations are automatically run when `RefExpectations` is dropped.
///
/// Expectation failures result in a panic
pub struct RefExpectations<'e, T: Debug> {
    value: &'e T,
    expectations: ExpectationList<'e, T>,
}

impl<'e, T: Debug> RefExpectations<'e, T> {
    /// Create new `RefExpectations` on a borrowed value
    pub fn new(value: &'e T) -> Self {
        Self {
            expectations: ExpectationList::new(),
            value,
        }
    }

    /// Manually run all the expectations
    pub fn check(self) {
        drop(self)
    }
}

impl<'e, T: Debug + 'e> ExpectationBuilder<'e> for RefExpectations<'e, T> {
    type Value = T;

    fn to_pass(mut self, expectation: impl Expectation<T> + 'e) -> Self {
        self.expectations.push(expectation);
        self
    }
}

impl<'e, T: Debug> Drop for RefExpectations<'e, T> {
    fn drop(&mut self) {
        if let CheckResult::Fail(message) = self.expectations.check(self.value) {
            panic!("{}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestExpectation;
    use crate::{CheckResult, ExpectationBuilder, expect, expect_ref};

    #[test]
    pub fn that_assert_runs_an_expectation() {
        // Given an expectation
        let (expectation, expected) = TestExpectation::new(CheckResult::Pass);

        // And expectations containing it
        let expectations = expect(true).to_pass(expectation);

        // When the expectations are checked
        expectations.check();

        // Then the expectation was run
        assert!(*expected.lock().unwrap());
    }

    #[test]
    pub fn that_check_returns_the_value() {
        // Given some owned value
        let value = vec![1, 2, 3];

        // When we run expectations and call check
        let returned = expect(value).check();

        // Then we have ownership of the value again
        assert_eq!(returned, vec![1, 2, 3]);
    }

    #[test]
    pub fn that_assert_works_on_references() {
        // Given an expectation
        let (expectation, _) = TestExpectation::new(CheckResult::Pass);

        // Expect a reference to work
        let value = true;
        expect_ref(&value).to_pass(expectation);
    }

    #[test]
    pub fn that_check_runs_all_expectations() {
        // Given two expectations that both pass
        let (expectation1, expected1) = TestExpectation::new(CheckResult::Pass);
        let (expectation2, expected2) = TestExpectation::new(CheckResult::Pass);

        // And expectations containing those
        let expectations = expect(true).to_pass(expectation1).to_pass(expectation2);

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
        let expectations = expect(true).to_pass(expectation);

        // Expect a panic when checked
        expectations.check();
    }
}
