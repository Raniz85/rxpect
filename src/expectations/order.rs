use std::fmt::Debug;
use crate::{CheckResult, Expectation, ExpectationBuilder};

/// Extension trait for ordering expectations
pub trait OrderExpectations<T> {

    /// Expect the value to be less than another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = "abc";
    /// let b = "def";
    /// expect(a).to_be_less_than(b);
    /// ```
    /// asserts that `a.lt(b)` is true
    fn to_be_less_than(self, value: T) -> Self;

    /// Expect the value to be less than or equal to another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = "abc";
    /// let b = "abc";
    /// let c = "def";
    /// expect(a).to_be_less_than_or_equal(b);
    /// expect(a).to_be_less_than_or_equal(c);
    /// ```
    /// asserts that `a.le(b)` is true
    fn to_be_less_than_or_equal(self, value: T) -> Self;

    /// Expect the value to be greater than another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = "def";
    /// let b = "abc";
    /// expect(a).to_be_greater_than(b);
    /// ```
    /// asserts that `a.gt(b)` is true
    fn to_be_greater_than(self, value: T) -> Self;

    /// Expect the value to be greater than or equal to another value
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = "def";
    /// let b = "def";
    /// let c = "abc";
    /// expect(a).to_be_greater_than_or_equal(b);
    /// expect(a).to_be_greater_than_or_equal(c);
    /// ```
    /// asserts that `a.ge(b)` is true
    fn to_be_greater_than_or_equal(self, value: T) -> Self;
}

impl<'e, T, B> OrderExpectations<T> for B
where
    T: PartialOrd + Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn to_be_less_than(self, value: T) -> Self {
        self.to_pass(OrderExpectation {
            operator_fn: PartialOrd::lt,
            operator_char: '<',
            operand: value
        })
    }

    fn to_be_less_than_or_equal(self, value: T) -> Self {
        self.to_pass(OrderExpectation {
            operator_fn: PartialOrd::le,
            operator_char: '≤',
            operand: value
        })
    }


    fn to_be_greater_than(self, value: T) -> Self {
        self.to_pass(OrderExpectation {
            operator_fn: PartialOrd::gt,
            operator_char: '>',
            operand: value
        })
    }

    fn to_be_greater_than_or_equal(self, value: T) -> Self {
        self.to_pass(OrderExpectation {
            operator_fn: PartialOrd::ge,
            operator_char: '≥',
            operand: value
        })
    }
}

/// Expectation for to_equal
struct OrderExpectation<T> {
    operator_fn: fn(&T, &T) -> bool,
    operator_char: char,
    operand: T,
}

impl<T: PartialOrd + Debug> Expectation<T> for OrderExpectation<T> {
    fn check(&self, value: &T) -> CheckResult {
        if (self.operator_fn)(value, &self.operand) {
            CheckResult::Pass
        } else {
            CheckResult::Fail(format!(
                "Expectation failed (a {} b)\na: `{:?}`\nb: `{:?}`",
                self.operator_char, &self.operand, value
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OrderExpectations;
    use crate::expect;

    #[test]
    pub fn that_greater_than_accepts_lesser_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_greater_than expectation to pass with a lesser value
        expect(value).to_be_greater_than(1);
    }

    #[test]
    #[should_panic]
    pub fn that_greater_than_does_not_accept_equal_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_greater_than expectation to not pass with the same value
        expect(value).to_be_greater_than(2);
    }

    #[test]
    #[should_panic]
    pub fn that_greater_than_does_not_accept_greater_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_greater_than expectation to not pass with a greater value
        expect(value).to_be_greater_than(3);
    }
}
