use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for
pub trait EqualityExpectations<T> {
    fn to_equal(&mut self, value: T) -> &mut Self;
}

impl<'e, T, E> EqualityExpectations<T> for E
where
    T: PartialEq + Debug + 'e,
    E: ExpectationBuilder<'e, T>,
{
    fn to_equal(&mut self, value: T) -> &mut Self {
        self.add_expectation(ToEqualExpectation(value))
    }
}

struct ToEqualExpectation<T>(T);

impl<T: PartialEq + Debug> Expectation<T> for ToEqualExpectation<T> {
    fn check(&self, value: &T) -> CheckResult {
        if value.eq(&self.0) {
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
