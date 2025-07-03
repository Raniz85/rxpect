use crate::{CheckResult, Expectation};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A generic predicate-based expectation that can be used for any type
pub struct PredicateExpectation<T, U> {
    expected_value: U,
    predicate: for<'a> fn(&'a T, &U) -> bool,
    error_message: for<'a> fn(&'a T, &U) -> String,
    phantom_data: PhantomData<T>,
}

impl<U, T> PredicateExpectation<T, U>
where
    U: Debug,
    T: Debug,
{
    pub fn new(
        expected_value: U,
        predicate: for<'a> fn(&'a T, &U) -> bool,
        error_message: for<'a> fn(&'a T, &U) -> String,
    ) -> Self {
        Self {
            expected_value,
            predicate,
            error_message,
            phantom_data: Default::default(),
        }
    }
}

impl<T, U> Expectation<T> for PredicateExpectation<T, U>
where
    T: Debug,
    U: Debug,
{
    fn check(&self, value: &T) -> CheckResult {
        if (self.predicate)(value, &self.expected_value) {
            CheckResult::Pass
        } else {
            CheckResult::Fail((self.error_message)(value, &self.expected_value))
        }
    }
}
