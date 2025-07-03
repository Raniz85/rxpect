use crate::expectation_list::ExpectationList;
use crate::expectations::predicate::PredicateExpectation;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for Result expectations
pub trait ResultExpectations<T, E>
where
    T: Debug,
    E: Debug,
{
    /// Expect the Result to be Ok
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::ResultExpectations;
    ///
    /// let result: Result<i32, &str> = Ok(42);
    /// expect(result).to_be_ok();
    /// ```
    /// asserts that the Result is Ok
    fn to_be_ok(self) -> Self;

    /// Expect the Result to be Err
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::ResultExpectations;
    ///
    /// let result: Result<i32, &str> = Err("error");
    /// expect(result).to_be_err();
    /// ```
    /// asserts that the Result is Err
    fn to_be_err(self) -> Self;

    /// Expect the Result to be Ok and the Ok value to match a predicate
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::ResultExpectations;
    ///
    /// let result: Result<i32, &str> = Ok(42);
    /// expect(result).to_be_ok_matching(|v| *v > 40);
    /// ```
    /// asserts that the Result is Ok and the predicate returns true when applied to the Ok value
    fn to_be_ok_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + 'static;

    /// Expect the Result to be Err and the Err value to match a predicate
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::ResultExpectations;
    ///
    /// let result: Result<i32, &str> = Err("error");
    /// expect(result).to_be_err_matching(|e| *e == "error");
    /// ```
    /// asserts that the Result is Err and the predicate returns true when applied to the Err value
    fn to_be_err_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&E) -> bool + 'static;
}

pub trait ProjectedResultExpectations<'e, T, E, TB, EB>
where
    T: Debug + 'e,
    E: Debug + 'e,
    TB: ExpectationBuilder<'e, T>,
    EB: ExpectationBuilder<'e, E>,
{
    /// Expect the Result to be Ok and then chain into further expectations
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::{EqualityExpectations, ProjectedResultExpectations};
    ///
    /// let result: Result<i32, &str> = Ok(42);
    /// expect(result).to_be_ok_and(|foo| foo.to_equal(42));
    /// ```
    /// asserts that the Result is Ok and the predicate returns true when applied to the Ok value
    fn to_be_ok_and(self, config: impl FnOnce(TB) -> TB) -> Self;

    /// Expect the Result to be Ok and then chain into further expectations
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::{EqualityExpectations, ProjectedResultExpectations};
    ///
    /// let result: Result<i32, &str> = Err("Error message");
    /// expect(result).to_be_err_and(|foo| foo.to_equal("Error message"));
    /// ```
    /// asserts that the Result is Ok and the predicate returns true when applied to the Ok value
    fn to_be_err_and(self, config: impl FnOnce(EB) -> EB) -> Self;
}

impl<'e, T, E, B> ResultExpectations<T, E> for B
where
    T: Debug + 'e,
    E: Debug + 'e,
    B: ExpectationBuilder<'e, Result<T, E>>,
{
    fn to_be_ok(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |value: &Result<T, E>, _| value.is_ok(),
            |value: &Result<T, E>, _| format!("Expectation failed (expected Ok)\n  actual: {:?}", value)
        ))
    }

    fn to_be_err(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |value: &Result<T, E>, _| value.is_err(),
            |value: &Result<T, E>, _| format!("Expectation failed (expected Err)\n  actual: {:?}", value)
        ))
    }

    fn to_be_ok_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.to_pass(IsOkMatchingExpectation(predicate))
    }

    fn to_be_err_matching<F>(self, predicate: F) -> Self
    where
        F: Fn(&E) -> bool + 'static,
    {
        self.to_pass(IsErrMatchingExpectation(predicate))
    }
}
impl<'e, T, E, B>
    ProjectedResultExpectations<'e, T, E, ExpectationList<'e, T>, ExpectationList<'e, E>> for B
where
    T: Debug + 'e,
    E: Debug + 'e,
    B: ExpectationBuilder<'e, Result<T, E>>,
{
    fn to_be_ok_and(
        self,
        config: impl FnOnce(ExpectationList<'e, T>) -> ExpectationList<'e, T>,
    ) -> Self {
        let expectations = config(ExpectationList::new());
        self.to_pass(ResultOkProjectionExpectation {
            expectations,
            _phantom: std::marker::PhantomData,
        })
    }

    fn to_be_err_and(
        self,
        config: impl FnOnce(ExpectationList<'e, E>) -> ExpectationList<'e, E>,
    ) -> Self {
        let expectations = config(ExpectationList::new());
        self.to_pass(ResultErrProjectionExpectation {
            expectations,
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Expectation for result Ok with projection
struct ResultOkProjectionExpectation<'e, T, E> {
    expectations: ExpectationList<'e, T>,
    _phantom: std::marker::PhantomData<E>,
}

impl<'e, T: Debug + 'e, E: Debug> Expectation<Result<T, E>>
    for ResultOkProjectionExpectation<'e, T, E>
{
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Ok(ok_value) => self.expectations.check(ok_value),
            Err(e) => CheckResult::Fail(format!(
                "Expectation failed (expected Ok)\n  actual: Err({:?})",
                e
            )),
        }
    }
}

/// Expectation for result Err with projection
struct ResultErrProjectionExpectation<'e, T, E> {
    expectations: ExpectationList<'e, E>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'e, T: Debug + 'e, E: Debug> Expectation<Result<T, E>>
    for ResultErrProjectionExpectation<'e, T, E>
{
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Ok(value) => CheckResult::Fail(format!(
                "Expectation failed (expected Err)\n  actual: Ok({:?})",
                value
            )),
            Err(err) => self.expectations.check(err),
        }
    }
}

/// Expectation for to_be_ok_matching
struct IsOkMatchingExpectation<F>(F);

impl<T: Debug, E: Debug, F: Fn(&T) -> bool> Expectation<Result<T, E>>
    for IsOkMatchingExpectation<F>
{
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Ok(v) => {
                if (self.0)(v) {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail(format!(
                        "Expectation failed (expected Ok value to match predicate)\n  actual: Ok({:?})",
                        v
                    ))
                }
            }
            Err(e) => CheckResult::Fail(format!(
                "Expectation failed (expected Ok)\n  actual: Err({:?})",
                e
            )),
        }
    }
}

/// Expectation for to_be_err_matching
struct IsErrMatchingExpectation<F>(F);

impl<T: Debug, E: Debug, F: Fn(&E) -> bool> Expectation<Result<T, E>>
    for IsErrMatchingExpectation<F>
{
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Err(e) => {
                if (self.0)(e) {
                    CheckResult::Pass
                } else {
                    CheckResult::Fail(format!(
                        "Expectation failed (expected Err value to match predicate)\n  actual: Err({:?})",
                        e
                    ))
                }
            }
            Ok(v) => CheckResult::Fail(format!(
                "Expectation failed (expected Err)\n  actual: Ok({:?})",
                v
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;
    use crate::expectations::result::{ProjectedResultExpectations, ResultExpectations};
    use crate::expectations::EqualityExpectations;

    #[test]
    pub fn that_to_be_ok_accepts_ok_values() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok expectation to pass
        expect(result).to_be_ok();
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_does_not_accept_err_values() {
        // Given a Result that is Err
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_ok expectation to fail
        expect(result).to_be_ok();
    }

    #[test]
    pub fn that_to_be_err_accepts_err_values() {
        // Given a Result that is Err
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err expectation to pass
        expect(result).to_be_err();
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_does_not_accept_ok_values() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_err expectation to fail
        expect(result).to_be_err();
    }

    #[test]
    pub fn that_to_be_ok_matching_accepts_ok_values_that_match_predicate() {
        // Given a Result that is Ok with a value that matches the predicate
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok_matching expectation to pass
        expect(result).to_be_ok_matching(|v| *v > 40);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_matching_does_not_accept_ok_values_that_do_not_match_predicate() {
        // Given a Result that is Ok with a value that does not match the predicate
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok_matching expectation to fail
        expect(result).to_be_ok_matching(|v| *v < 40);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_matching_does_not_accept_err_values() {
        // Given a Result that is Err
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_ok_matching expectation to fail
        expect(result).to_be_ok_matching(|v| *v > 40);
    }

    #[test]
    pub fn that_to_be_err_matching_accepts_err_values_that_match_predicate() {
        // Given a Result that is Err with a value that matches the predicate
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_matching expectation to pass
        expect(result).to_be_err_matching(|e| *e == "error");
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_matching_does_not_accept_err_values_that_do_not_match_predicate() {
        // Given a Result that is Err with a value that does not match the predicate
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_matching expectation to fail
        expect(result).to_be_err_matching(|e| *e == "other error");
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_matching_does_not_accept_ok_values() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_err_matching expectation to fail
        expect(result).to_be_err_matching(|e| *e == "error");
    }

    #[test]
    pub fn that_to_be_ok_and_accepts_ok_values_with_matching_expectations() {
        // Given a Result that is Ok with a value that matches the expectation
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok_and expectation to pass
        expect(result).to_be_ok_and(|v| v.to_equal(42));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_and_does_not_accept_ok_values_with_non_matching_expectations() {
        // Given a Result that is Ok with a value that does not match the expectation
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok_and expectation to fail
        expect(result).to_be_ok_and(|v| v.to_equal(43));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_and_does_not_accept_err_values() {
        // Given a Result that is Err
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_ok_and expectation to fail
        expect(result).to_be_ok_and(|v| v.to_equal(42));
    }

    #[test]
    pub fn that_to_be_err_and_accepts_err_values_with_matching_expectations() {
        // Given a Result that is Err with a value that matches the expectation
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_and expectation to pass
        expect(result).to_be_err_and(|e| e.to_equal("error"));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_and_does_not_accept_err_values_with_non_matching_expectations() {
        // Given a Result that is Err with a value that does not match the expectation
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_and expectation to fail
        expect(result).to_be_err_and(|e| e.to_equal("other error"));
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_and_does_not_accept_ok_values() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_err_and expectation to fail
        expect(result).to_be_err_and(|e| e.to_equal("error"));
    }
}
