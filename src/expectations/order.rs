use crate::ExpectationBuilder;
use crate::expectations::predicate::PredicateExpectation;
use std::fmt::Debug;
use std::ops::RangeBounds;

/// Extension trait for ordering expectations
pub trait OrderExpectations<'e, T> {
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

    /// Expect the value to be inside a range
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = 5;
    /// let range = 1..10;
    /// expect(a).to_be_inside(range);
    /// ```
    /// asserts that `a` is inside `range`
    ///
    /// It works with inclusive ranges as well
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::OrderExpectations;
    ///
    /// let a = 10;
    /// let range = 1..=10;
    /// expect(a).to_be_inside(range);
    /// ```
    fn to_be_inside<R: RangeBounds<T> + Debug + 'e>(self, range: R) -> Self;
}

impl<'e, T, B> OrderExpectations<'e, T> for B
where
    T: PartialOrd + Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn to_be_less_than(self, value: T) -> Self {
        self.to_pass(PredicateExpectation::new(
            value,
            |a: &T, b: &T| a.lt(b),
            |a: &T, b: &T| format!("Expectation failed (a < b)\na: `{:?}`\nb: `{:?}`", b, a),
        ))
    }

    fn to_be_less_than_or_equal(self, value: T) -> Self {
        self.to_pass(PredicateExpectation::new(
            value,
            |a: &T, b: &T| a.le(b),
            |a: &T, b: &T| format!("Expectation failed (a ≤ b)\na: `{:?}`\nb: `{:?}`", b, a),
        ))
    }

    fn to_be_greater_than(self, value: T) -> Self {
        self.to_pass(PredicateExpectation::new(
            value,
            |a: &T, b: &T| a.gt(b),
            |a: &T, b: &T| format!("Expectation failed (a > b)\na: `{:?}`\nb: `{:?}`", b, a),
        ))
    }

    fn to_be_greater_than_or_equal(self, value: T) -> Self {
        self.to_pass(PredicateExpectation::new(
            value,
            |a: &T, b: &T| a.ge(b),
            |a: &T, b: &T| format!("Expectation failed (a ≥ b)\na: `{:?}`\nb: `{:?}`", b, a),
        ))
    }

    fn to_be_inside<R: RangeBounds<T> + Debug + 'e>(self, range: R) -> Self {
        self.to_pass(PredicateExpectation::new(
            range,
            |value: &T, range: &R| range.contains(value),
            |value: &T, range: &R| {
                format!(
                    "Expectation failed (value ∈ range)\nvalue:`{:?}`\nrange:`{:?}`",
                    value, range
                )
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::OrderExpectations;
    use crate::expect;
    use rstest::rstest;
    use std::fmt::Debug;
    use std::ops::Range;
    use std::ops::RangeBounds;

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

    #[test]
    pub fn that_less_than_accepts_greater_value() {
        // Given a value that implements PartialOrd
        let value = 1;

        // Expect the to_be_less_than expectation to pass with a greater value
        expect(value).to_be_less_than(2);
    }

    #[test]
    #[should_panic]
    pub fn that_less_than_does_not_accept_equal_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_less_than expectation to not pass with the same value
        expect(value).to_be_less_than(2);
    }

    #[test]
    #[should_panic]
    pub fn that_less_than_does_not_accept_lesser_value() {
        // Given a value that implements PartialOrd
        let value = 3;

        // Expect the to_be_less_than expectation to not pass with a lesser value
        expect(value).to_be_less_than(2);
    }

    #[test]
    pub fn that_less_than_or_equal_accepts_greater_value() {
        // Given a value that implements PartialOrd
        let value = 1;

        // Expect the to_be_less_than_or_equal expectation to pass with a greater value
        expect(value).to_be_less_than_or_equal(2);
    }

    #[test]
    pub fn that_less_than_or_equal_accepts_equal_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_less_than_or_equal expectation to pass with the same value
        expect(value).to_be_less_than_or_equal(2);
    }

    #[test]
    #[should_panic]
    pub fn that_less_than_or_equal_does_not_accept_lesser_value() {
        // Given a value that implements PartialOrd
        let value = 3;

        // Expect the to_be_less_than_or_equal expectation to not pass with a lesser value
        expect(value).to_be_less_than_or_equal(2);
    }

    #[test]
    pub fn that_greater_than_or_equal_accepts_lesser_value() {
        // Given a value that implements PartialOrd
        let value = 3;

        // Expect the to_be_greater_than_or_equal expectation to pass with a lesser value
        expect(value).to_be_greater_than_or_equal(2);
    }

    #[test]
    pub fn that_greater_than_or_equal_accepts_equal_value() {
        // Given a value that implements PartialOrd
        let value = 2;

        // Expect the to_be_greater_than_or_equal expectation to pass with the same value
        expect(value).to_be_greater_than_or_equal(2);
    }

    #[test]
    #[should_panic]
    pub fn that_greater_than_or_equal_does_not_accept_greater_value() {
        // Given a value that implements PartialOrd
        let value = 1;

        // Expect the to_be_greater_than_or_equal expectation to not pass with a greater value
        expect(value).to_be_greater_than_or_equal(2);
    }

    #[rstest]
    #[case(1..=5)]
    #[case(5..10)]
    pub fn that_inside_does_accept_value_inside_range(
        #[case] range: impl RangeBounds<u32> + Debug,
    ) {
        // Given a value that implements PartialOrd
        let value = 5;

        // Expect the to_be_inside expectation to not pass with a range that does not include it
        expect(value).to_be_inside(range);
    }

    #[rstest]
    #[case(1..5)]
    #[case(6..10)]
    #[should_panic]
    pub fn that_inside_does_not_accept_value_outside_range(#[case] range: Range<u32>) {
        // Given a value that implements PartialOrd
        let value = 5;

        // Expect the to_be_inside expectation to not pass with a range that does not include it
        expect(value).to_be_inside(range);
    }
}
