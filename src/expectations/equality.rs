use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// Extension trait for equality expectations
pub trait EqualityExpectations<T, U> {
    /// Expect the value to equal another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::EqualityExpectations;
    ///
    /// let a = "foo";
    /// let b = "foo";
    /// expect(a).to_equal(b);
    /// ```
    ///
    /// It works with differing types as well, as well as T: Eq<U> holds true
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::EqualityExpectations;
    ///
    /// let a = "foo".to_string();
    /// let b = "foo";
    /// expect(a).to_equal(b);
    /// ```
    /// asserts that `b.eq(a)` is true
    fn to_equal(self, value: U) -> Self;
}

impl<'e, T, U, B> EqualityExpectations<T, U> for B
where
    T: PartialEq<U> + Debug + 'e,
    U: Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn to_equal(self, value: U) -> Self {
        self.to_pass(ToEqualExpectation(value))
    }
}

/// Expectation for to_equal
struct ToEqualExpectation<U>(U);

impl<T, U> Expectation<T> for ToEqualExpectation<U>
where
    T: PartialEq<U> + Debug,
    U: Debug,
{
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
    use rstest::rstest;
    use std::fmt::Debug;

    #[rstest]
    #[case(1, 1)]
    #[case("&str", "&str")]
    #[case("String".to_string(), "String".to_string())]
    #[case("String and &str".to_string(), "String and &str")]
    pub fn that_to_equal_accepts_equal_values<T, U>(#[case] a: T, #[case] b: U)
    where
        T: PartialEq<U> + Debug,
        U: Debug,
    {
        // Expect the two values to be equal
        expect(a).to_equal(b);
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
