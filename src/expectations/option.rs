use crate::expectation_list::ExpectationList;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for Option expectations
pub trait OptionExpectations<T>
where
    T: Debug,
{
    /// Expect the Option to be Some
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OptionExpectations;
    ///
    /// let option: Option<i32> = Some(42);
    /// expect(option).to_be_some();
    /// ```
    /// asserts that the Option is Some
    fn to_be_some(self) -> Self;

    /// Expect the Option to be None
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OptionExpectations;
    ///
    /// let option: Option<i32> = None;
    /// expect(option).to_be_none();
    /// ```
    /// asserts that the Option is None
    fn to_be_none(self) -> Self;

    /// Expect the Option to be Some and the Some value to match a predicate
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OptionExpectations;
    ///
    /// let option: Option<i32> = Some(42);
    /// expect(option).to_be_some_matching(|v| *v > 40);
    /// ```
    /// asserts that the Option is Some and the predicate returns true when applied to the Some value
    fn to_be_some_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + 'static;
}

pub trait ProjectedOptionExpectations<'e, T, TB>
where
    T: Debug + 'e,
    TB: ExpectationBuilder<'e, T>,
{
    /// Expect the Option to be Some and then chain into further expectations
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::{EqualityExpectations, ProjectedOptionExpectations};
    ///
    /// let option: Option<i32> = Some(42);
    /// expect(option).to_be_some_and(|foo| foo.to_equal(42));
    /// ```
    /// asserts that the Option is Some and the predicate returns true when applied to the Some value
    fn to_be_some_and(self, config: impl FnOnce(TB) -> TB) -> Self;
}

impl<'e, T, B> OptionExpectations<T> for B
where
    T: Debug + 'e,
    B: ExpectationBuilder<'e, Option<T>>,
{
    fn to_be_some(self) -> Self {
        self.to_pass(IsSomeExpectation)
    }

    fn to_be_none(self) -> Self {
        self.to_pass(IsNoneExpectation)
    }

    fn to_be_some_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.to_pass(IsSomeMatchingExpectation(predicate))
    }
}

impl<'e, T, B> ProjectedOptionExpectations<'e, T, ExpectationList<'e, T>> for B
where
    T: Debug + 'e,
    B: ExpectationBuilder<'e, Option<T>>,
{
    fn to_be_some_and(
        self,
        config: impl FnOnce(ExpectationList<'e, T>) -> ExpectationList<'e, T>,
    ) -> Self {
        let expectations = config(ExpectationList::new());
        self.to_pass(OptionSomeProjectionExpectation {
            expectations,
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Expectation for option Some with projection
struct OptionSomeProjectionExpectation<'e, T> {
    expectations: ExpectationList<'e, T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'e, T: Debug + 'e> Expectation<Option<T>> for OptionSomeProjectionExpectation<'e, T> {
    fn check(&self, value: &Option<T>) -> CheckResult {
        match value {
            Some(some_value) => self.expectations.check(some_value),
            None => {
                CheckResult::Fail("Expectation failed (expected Some)\n  actual: None".to_string())
            }
        }
    }
}

/// Expectation for to_be_some
struct IsSomeExpectation;

impl<T: Debug> Expectation<Option<T>> for IsSomeExpectation {
    fn check(&self, value: &Option<T>) -> CheckResult {
        match value {
            Some(_) => CheckResult::Pass,
            None => {
                CheckResult::Fail("Expectation failed (expected Some)\n  actual: None".to_string())
            }
        }
    }
}

/// Expectation for to_be_none
struct IsNoneExpectation;

impl<T: Debug> Expectation<Option<T>> for IsNoneExpectation {
    fn check(&self, value: &Option<T>) -> CheckResult {
        match value {
            None => CheckResult::Pass,
            Some(v) => CheckResult::Fail(format!(
                "Expectation failed (expected None)\n  actual: Some({:?})",
                v
            )),
        }
    }
}

/// Expectation for to_be_some_matching
struct IsSomeMatchingExpectation<F>(F);

impl<T: Debug, F: Fn(&T) -> bool> Expectation<Option<T>> for IsSomeMatchingExpectation<F> {
    fn check(&self, value: &Option<T>) -> CheckResult {
        match value {
            Some(v) => {
                if (self.0)(v) {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail(format!(
                        "Expectation failed (expected Some value to match predicate)\n  actual: Some({:?})",
                        v
                    ))
                }
            }
            None => {
                CheckResult::Fail("Expectation failed (expected Some)\n  actual: None".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;
    use crate::expectations::option::{OptionExpectations, ProjectedOptionExpectations};
    use crate::expectations::EqualityExpectations;

    #[test]
    pub fn that_to_be_some_accepts_some_values() {
        // Given an Option that is Some
        let option: Option<i32> = Some(42);

        // Expect the to_be_some expectation to pass
        expect(option).to_be_some();
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_some_does_not_accept_none_values() {
        // Given an Option that is None
        let option: Option<i32> = None;

        // Expect the to_be_some expectation to fail
        expect(option).to_be_some();
    }

    #[test]
    pub fn that_to_be_none_accepts_none_values() {
        // Given an Option that is None
        let option: Option<i32> = None;

        // Expect the to_be_none expectation to pass
        expect(option).to_be_none();
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_none_does_not_accept_some_values() {
        // Given an Option that is Some
        let option: Option<i32> = Some(42);

        // Expect the to_be_none expectation to fail
        expect(option).to_be_none();
    }

    #[test]
    pub fn that_to_be_some_matching_accepts_some_values_that_match_predicate() {
        // Given an Option that is Some with a value that matches the predicate
        let option: Option<i32> = Some(42);

        // Expect the to_be_some_matching expectation to pass
        expect(option).to_be_some_matching(|v| *v > 40);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_some_matching_does_not_accept_some_values_that_do_not_match_predicate() {
        // Given an Option that is Some with a value that does not match the predicate
        let option: Option<i32> = Some(42);

        // Expect the to_be_some_matching expectation to fail
        expect(option).to_be_some_matching(|v| *v < 40);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_some_matching_does_not_accept_none_values() {
        // Given an Option that is None
        let option: Option<i32> = None;

        // Expect the to_be_some_matching expectation to fail
        expect(option).to_be_some_matching(|v| *v > 40);
    }

    #[test]
    pub fn that_to_be_some_and_accepts_some_values_with_matching_expectations() {
        // Given an Option that is Some with a value that matches the expectation
        let option: Option<i32> = Some(42);

        // Expect the to_be_some_and expectation to pass
        expect(option).to_be_some_and(|v| v.to_equal(42));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_some_and_does_not_accept_some_values_with_non_matching_expectations() {
        // Given an Option that is Some with a value that does not match the expectation
        let option: Option<i32> = Some(42);

        // Expect the to_be_some_and expectation to fail
        expect(option).to_be_some_and(|v| v.to_equal(43));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_some_and_does_not_accept_none_values() {
        // Given an Option that is None
        let option: Option<i32> = None;

        // Expect the to_be_some_and expectation to fail
        expect(option).to_be_some_and(|v| v.to_equal(42));
    }
}
