use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for
pub trait EqualityExpectations<T> {

    /// Expect the value to equal another value
    /// ```
    /// # use rexpect::expect;
    /// # use rexpect::expectations::EqualityExpectations;
    ///
    /// let a = "foo";
    /// let b = "foo";
    /// expect(a).to_equal(b);
    /// ```
    /// asserts that `b.eq(a)` is true
    fn to_equal(self, value: T) -> Self;
}

impl<'e, T, B> EqualityExpectations<T> for B
where
    T: PartialEq + Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn to_equal(self, value: T) -> Self {
        self.to_pass(ToEqualExpectation(value))
    }
}

/// Expectation for to_equal
struct ToEqualExpectation<T>(T);

impl<T: PartialEq + Debug> Expectation<T> for ToEqualExpectation<T> {
    fn check(&self, value: &T) -> CheckResult {
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

#[cfg(test)]
mod tests {
    use super::EqualityExpectations;
    use crate::expect;

    #[test]
    pub fn that_to_equal_accepts_equal_values() {
        // Given a value that implements PartialEq
        let value = 1;

        // Expect the to_equal expectation to pass with an identical value
        expect(value).to_equal(1);
    }

    #[test]
    #[should_panic]
    pub fn that_to_equal_does_not_accept_unequal_values() {
        // Given a value that implements PartialEq
        let value = 1;

        // Expect the to_equal expectation to fail with an identical value
        expect(value).to_equal(2);
    }
}
