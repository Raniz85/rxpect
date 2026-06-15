use crate::expectation_list::Combinator;
use crate::{CheckResult, Expectation, ExpectationBuilder, ExpectationList};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

/// Extensions for grouping expectations
///
/// Create groups of expectations that can be evaluated together as a group.
/// This can be used to achieve OR-semantics between expectations:
///
/// ```rust
/// use rxpect::expect;
/// use rxpect::GroupExpectations;
/// use rxpect::expectations::StringExpectations;
/// use rxpect::expectations::OrderExpectations;
///
/// expect("hello123")
///     .any_of()
///         .to_be_all_whitespace()
///         .to_be_alphanumeric()
///     .close_group()
///     .length()
///          .to_be_greater_than(5)
///     ;
/// ```
pub trait GroupExpectations<'e>
where
    Self: Sized + ExpectationBuilder<'e>,
{
    /// Create a new group using the semantics of the given combinator
    fn group(self, combinator: Combinator) -> GroupedExpectationsBuilder<'e, Self, Self::Value>;

    /// Create a new group using OR semantics
    ///
    /// # Example
    /// ```rust
    /// use rxpect::expect;
    /// use rxpect::GroupExpectations;
    /// use rxpect::expectations::StringExpectations;
    ///
    /// expect("hello123")
    ///     .any_of()
    ///         .to_be_all_whitespace()
    ///         .to_be_alphanumeric()
    ///     ;
    /// ```
    fn any_of(self) -> GroupedExpectationsBuilder<'e, Self, Self::Value> {
        self.group(Combinator::Or)
    }

    /// Create a new group using AND semantics
    ///
    /// <div class="warning">Note that this is the default semantics,
    /// this should most often only be used when nested inside a group with OR-semantics.</div>
    ///
    /// ## Example
    /// ```rust
    /// use rxpect::expect;
    /// use rxpect::GroupExpectations;
    /// use rxpect::expectations::StringExpectations;
    /// use rxpect::expectations::OrderExpectations;
    ///
    /// // Expects the string to be either all whitespace OR all numbers OR alphabetic with a length greater than 5
    /// expect("helloworld")
    ///     .any_of()
    ///         .to_be_all_whitespace()
    ///         .all_of()
    ///              .to_be_alphabetic()
    ///              .length()
    ///                  .to_be_greater_than(5)
    ///              .unproject()
    ///         .close_group()
    ///         .to_be_numeric()
    ///     ;
    /// ```
    fn all_of(self) -> GroupedExpectationsBuilder<'e, Self, Self::Value> {
        self.group(Combinator::And)
    }
}

impl<'e, B> GroupExpectations<'e> for B
where
    B: Sized + ExpectationBuilder<'e>,
{
    fn group(self, combinator: Combinator) -> GroupedExpectationsBuilder<'e, Self, Self::Value> {
        let (expectation, expectations) = GroupedExpectations::new(combinator);
        GroupedExpectationsBuilder {
            parent: self.to_pass(expectation),
            expectations,
        }
    }
}

struct GroupedExpectations<'e, T>
where
    T: Debug + 'e,
{
    combinator: Combinator,
    expectations: Rc<RefCell<ExpectationList<'e, T>>>,
}

impl<'e, T> GroupedExpectations<'e, T>
where
    T: Debug + 'e,
{
    pub fn new(
        combinator: Combinator,
    ) -> (
        GroupedExpectations<'e, T>,
        Rc<RefCell<ExpectationList<'e, T>>>,
    ) {
        let expectations = Rc::new(RefCell::new(ExpectationList::new()));
        (
            GroupedExpectations {
                combinator,
                expectations: expectations.clone(),
            },
            expectations,
        )
    }
}

/// Builder for grouped expectations
pub struct GroupedExpectationsBuilder<'e, B, T>
where
    B: ExpectationBuilder<'e, Value = T>,
{
    parent: B,
    expectations: Rc<RefCell<ExpectationList<'e, T>>>,
}

impl<'e, B, T> GroupedExpectationsBuilder<'e, B, T>
where
    B: ExpectationBuilder<'e, Value = T>,
{
    /// Close the group, returning the parent builder
    pub fn close_group(self) -> B {
        self.parent
    }
}

impl<'e, B> ExpectationBuilder<'e> for GroupedExpectationsBuilder<'e, B, B::Value>
where
    B: ExpectationBuilder<'e>,
{
    type Value = B::Value;

    fn to_pass(self, expectation: impl Expectation<Self::Value> + 'e) -> Self {
        self.expectations.borrow_mut().push(expectation);
        self
    }
}

impl<'e, T> Expectation<T> for GroupedExpectations<'e, T>
where
    T: Debug + 'e,
{
    fn check(&self, value: &T) -> CheckResult {
        self.expectations
            .borrow()
            .check_with_combinator(value, self.combinator)
    }
}

/// These tests are more about the API since all the and/or functionality is tested in the `expectation_list` module.
#[cfg(test)]
mod tests {
    use super::GroupExpectations;
    use crate::expect;
    use crate::expectations::{OrderExpectations, StringExpectations};
    use rstest::rstest;

    #[rstest]
    #[case::not_whitespace("hello123")]
    #[case::not_alphanumeric("      ")]
    #[case::empty("")]
    #[should_panic]
    pub fn that_all_of_fails_if_not_all_expectations_pass(#[case] input: impl AsRef<str>) {
        // Expect an all-of expectation for alphanumerics and all whitespace to fail
        expect(input.as_ref())
            .all_of()
            .to_be_alphanumeric()
            .to_be_all_whitespace();
    }

    #[test]
    pub fn that_all_of_pass_if_all_expectations_pass() {
        // Given an alphanumeric string
        let value = "hello123";

        // Then an all-of expectation for alphanumerics and length greater than 0 passes
        expect(value)
            .all_of()
            .to_be_alphanumeric()
            .length()
            .to_be_greater_than(0);
    }

    #[test]
    #[should_panic]
    pub fn that_any_of_fails_if_no_expectations_pass() {
        // Given an alphanumeric string
        let value = "hello123";

        // Then an any-of expectation for numeric and all whitespace fails
        expect(value)
            .any_of()
            .to_be_numeric()
            .to_be_all_whitespace();
    }

    #[test]
    pub fn that_any_of_passes_if_at_least_one_expectation_passes() {
        // Given a numeric string
        let value = "123";

        // Then an any-of expectation for numeric and all whitespace passes
        expect(value)
            .any_of()
            .to_be_numeric()
            .to_be_all_whitespace();
    }
}
