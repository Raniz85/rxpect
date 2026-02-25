use crate::borrow::BorrowedOrOwned;
use crate::expectations::predicate::PredicateExpectation;
use crate::projection::{ProjectedExpectations, ProjectedExpectationsBuilder};
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

pub trait ProjectedResultExpectations<'e, T, E>
where
    T: Debug + 'e,
    E: Debug + 'e,
{
    /// Expect the Result to be Ok and then chain into further expectations
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::{EqualityExpectations, ProjectedResultExpectations};
    ///
    /// let result: Result<i32, &str> = Ok(42);
    /// expect(result).to_be_ok_and().to_equal(42);
    /// ```
    /// asserts that the Result is Ok and the chained expectations hold for the Ok value
    fn to_be_ok_and(self) -> ProjectedExpectationsBuilder<'e, Self, Result<T, E>, T>
    where
        Self: Sized + ExpectationBuilder<'e, Result<T, E>>;

    /// Expect the Result to be Err and then chain into further expectations
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::{EqualityExpectations, ProjectedResultExpectations};
    ///
    /// let result: Result<i32, &str> = Err("Error message");
    /// expect(result).to_be_err_and().to_equal("Error message");
    /// ```
    /// asserts that the Result is Err and the chained expectations hold for the Err value
    fn to_be_err_and(self) -> ProjectedExpectationsBuilder<'e, Self, Result<T, E>, E>
    where
        Self: Sized + ExpectationBuilder<'e, Result<T, E>>;
}

fn ok_extract<T: Debug, E: Debug>(result: &Result<T, E>) -> Option<BorrowedOrOwned<'_, T>> {
    result.as_ref().ok().map(BorrowedOrOwned::Borrowed)
}

fn ok_fail_message<T: Debug, E: Debug>(result: &Result<T, E>) -> String {
    format!("Expectation failed (expected Ok)\n  actual: {:?}", result)
}

fn err_extract<T: Debug, E: Debug>(result: &Result<T, E>) -> Option<BorrowedOrOwned<'_, E>> {
    result.as_ref().err().map(BorrowedOrOwned::Borrowed)
}

fn err_fail_message<T: Debug, E: Debug>(result: &Result<T, E>) -> String {
    format!("Expectation failed (expected Err)\n  actual: {:?}", result)
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
            |value: &Result<T, E>, _| {
                format!("Expectation failed (expected Ok)\n  actual: {:?}", value)
            },
        ))
    }

    fn to_be_err(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |value: &Result<T, E>, _| value.is_err(),
            |value: &Result<T, E>, _| {
                format!("Expectation failed (expected Err)\n  actual: {:?}", value)
            },
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

impl<'e, T, E, B> ProjectedResultExpectations<'e, T, E> for B
where
    T: Debug + 'e,
    E: Debug + 'e,
    B: ExpectationBuilder<'e, Result<T, E>>,
{
    fn to_be_ok_and(self) -> ProjectedExpectationsBuilder<'e, Self, Result<T, E>, T> {
        let (expectation, expectations) =
            ProjectedExpectations::new(ok_extract::<T, E>, ok_fail_message::<T, E>);
        ProjectedExpectationsBuilder::from_expectation(self, expectation, expectations)
    }

    fn to_be_err_and(self) -> ProjectedExpectationsBuilder<'e, Self, Result<T, E>, E> {
        let (expectation, expectations) =
            ProjectedExpectations::new(err_extract::<T, E>, err_fail_message::<T, E>);
        ProjectedExpectationsBuilder::from_expectation(self, expectation, expectations)
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
    use crate::expectations::EqualityExpectations;
    use crate::expectations::result::{ProjectedResultExpectations, ResultExpectations};
    use crate::{expect, expect_ref};

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
        expect(result).to_be_ok_and().to_equal(42);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_and_does_not_accept_ok_values_with_non_matching_expectations() {
        // Given a Result that is Ok with a value that does not match the expectation
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok_and expectation to fail
        expect(result).to_be_ok_and().to_equal(43);
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_ok_and_does_not_accept_err_values() {
        // Given a Result that is Err
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_ok_and expectation to fail
        expect(result).to_be_ok_and().to_equal(42);
    }

    #[test]
    pub fn that_to_be_err_and_accepts_err_values_with_matching_expectations() {
        // Given a Result that is Err with a value that matches the expectation
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_and expectation to pass
        expect(result).to_be_err_and().to_equal("error");
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_and_does_not_accept_err_values_with_non_matching_expectations() {
        // Given a Result that is Err with a value that does not match the expectation
        let result: Result<i32, &str> = Err("error");

        // Expect the to_be_err_and expectation to fail
        expect(result).to_be_err_and().to_equal("other error");
    }

    #[test]
    #[should_panic]
    pub fn that_to_be_err_and_does_not_accept_ok_values() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_err_and expectation to fail
        expect(result).to_be_err_and().to_equal("error");
    }

    #[test]
    pub fn that_to_be_ok_accepts_reference() {
        // Given a Result that is Ok
        let result: Result<i32, &str> = Ok(42);

        // Expect the to_be_ok expectation to pass for a reference to the result
        expect_ref(&result).to_be_ok();
    }
}
