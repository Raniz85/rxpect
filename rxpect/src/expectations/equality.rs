#[cfg(feature = "diff")]
use crate::diff::{Color, diff_pretty_debug};
use crate::{CheckResult, Expectation, ExpectationBuilder};
#[cfg(feature = "diff")]
use colored::Colorize;
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
    /// It works with differing types as well, as long as `T: PartialEq<U>` holds true
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::EqualityExpectations;
    ///
    /// let a = "foo".to_string();
    /// let b = "foo";
    /// expect(a).to_equal(b);
    /// ```
    /// asserts that `a.eq(b)` is true
    fn to_equal(self, value: U) -> Self;
}

impl<'e, T, U, B> EqualityExpectations<T, U> for B
where
    T: PartialEq<U> + Debug + 'e,
    U: Debug + 'e,
    B: ExpectationBuilder<'e, Value = T>,
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
            #[cfg(feature = "diff")]
            {
                let diff = diff_pretty_debug(&self.0, value);
                CheckResult::Fail(format!(
                    "Expectation failed ({} == {})\n{diff}",
                    "expected".on_ansi_color(Color::RemovedRow),
                    "actual".on_ansi_color(Color::AddedRow)
                ))
            }
            #[cfg(not(feature = "diff"))]
            {
                CheckResult::Fail(format!(
                    "Expectation failed (expected == actual)\nexpected: `{:?}`\n  actual: `{:?}`",
                    &self.0, value
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EqualityExpectations;
    #[cfg(feature = "diff")]
    use super::ToEqualExpectation;
    #[cfg(feature = "diff")]
    use crate::diff::{Color, diff_pretty_debug};
    use crate::expect;
    #[cfg(feature = "diff")]
    use crate::{CheckResult, ExpectProjection, Expectation};
    #[cfg(feature = "diff")]
    use colored::Colorize;
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

    #[cfg(feature = "diff")]
    #[test]
    pub fn that_equal_with_diffing_returns_colored_diff() {
        // Given two strings that differ
        let a = "One two three".to_string();
        let b = "One three two".to_string();

        let result = ToEqualExpectation(b.clone()).check(&a);
        expect(result)
            .projected_by(|r| match r {
                CheckResult::Fail(message) => message.clone(),
                _ => "Passed".to_string(),
            })
            .to_equal(format!(
                "Expectation failed ({} == {})\n{}",
                "expected".on_ansi_color(Color::RemovedRow),
                "actual".on_ansi_color(Color::AddedRow),
                diff_pretty_debug(&b, &a)
            ));
    }
}
