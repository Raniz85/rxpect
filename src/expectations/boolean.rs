use crate::expectations::EqualityExpectations;
use crate::{CheckResult, Expectation, ExpectationBuilder};

/// Extension trait for
pub trait BooleanExpectations {

    /// Expect the value to be true
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::BooleanExpectations;
    ///
    /// let a = true;
    /// expect(a).to_be_true();
    /// ```
    /// asserts that `a` is true
    fn to_be_true(self) -> Self;

    /// Expect the value to be false
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::BooleanExpectations;
    ///
    /// let a = false;
    /// expect(a).to_be_false();
    /// ```
    /// asserts that `a` is false
    fn to_be_false(self) -> Self;
}

impl<'e, B> BooleanExpectations for B
where
    B: ExpectationBuilder<'e, bool> + EqualityExpectations<bool>,
{
    fn to_be_true(self) -> Self {
        self.to_pass(BooleanEqualityExpectation(true))
    }

    fn to_be_false(self) -> Self {
        self.to_pass(BooleanEqualityExpectation(false))
    }
}

/// Expectation for to_be_true/false
struct BooleanEqualityExpectation(bool);

impl Expectation<bool> for BooleanEqualityExpectation {
    fn check(&self, value: &bool) -> CheckResult {
        if self.0.eq(value) {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (expected == actual)\nexpected: `{:?}`\n  actual: `{:?}`",
                &self.0, value
            ))
        }
    }
}

