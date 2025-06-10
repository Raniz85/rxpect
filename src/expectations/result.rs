use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for Result expectations
pub trait ResultExpectations<T, E> {
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

impl<'e, T, E, B> ResultExpectations<T, E> for B
where
    T: Debug + 'e,
    E: Debug + 'e,
    B: ExpectationBuilder<'e, Result<T, E>>,
{
    fn to_be_ok(self) -> Self {
        self.to_pass(IsOkExpectation)
    }

    fn to_be_err(self) -> Self {
        self.to_pass(IsErrExpectation)
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

/// Expectation for to_be_ok
struct IsOkExpectation;

impl<T: Debug, E: Debug> Expectation<Result<T, E>> for IsOkExpectation {
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Ok(_) => CheckResult::Pass,
            Err(e) => CheckResult::Fail(format!(
                "Expectation failed (expected Ok)\n  actual: Err({:?})",
                e
            )),
        }
    }
}

/// Expectation for to_be_err
struct IsErrExpectation;

impl<T: Debug, E: Debug> Expectation<Result<T, E>> for IsErrExpectation {
    fn check(&self, value: &Result<T, E>) -> CheckResult {
        match value {
            Err(_) => CheckResult::Pass,
            Ok(v) => CheckResult::Fail(format!(
                "Expectation failed (expected Err)\n  actual: Ok({:?})",
                v
            )),
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
    use super::ResultExpectations;
    use crate::expect;

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
}
