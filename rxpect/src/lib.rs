#![doc=include_str!("../README.md")]
mod borrow;
mod expectation_list;
pub mod expectations;
mod projection;
mod root;

pub use borrow::BorrowedOrOwned;
pub use expectation_list::ExpectationList;
pub use projection::ExpectProjection;
pub use projection::ProjectedExpectationsBuilder;
pub use root::RootExpectations;

use std::fmt::Debug;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Result of an expectation check
#[derive(Clone, Debug)]
pub enum CheckResult {
    /// The expectation passed
    Pass,
    /// The expectation failed, contains a message describing the failure
    Fail(String),
}

/// An expectation on a value
pub trait Expectation<T: Debug> {
    /// Check this expectation
    /// Returns CheckResult::Pass if the expectation pass
    /// and CheckResult::Fail with a descriptive message if it didn't
    fn check(&self, value: &T) -> CheckResult;
}

/// Trait to enable fluent building of expectations
pub trait ExpectationBuilder<'e> {
    /// Target value type for this builder
    type Value: Debug + 'e;

    /// Expect the value to pass an expectation
    /// This is intended to be used in extension methods to add expectations to the builder
    fn to_pass(self, expectation: impl Expectation<Self::Value> + 'e) -> Self;
}

/// Create expectations for a value.
/// Used as an entrypoint for fluently building expectations
/// ```
/// use rxpect::expect;
/// use rxpect::expectations::EqualityExpectations;
///
/// expect(1).to_equal(1);
/// ```
pub fn expect<'e, T: Debug>(value: T) -> RootExpectations<'e, T> {
    RootExpectations::new(value)
}

/// Create expectations for a reference to a value.
/// Used as an entrypoint for fluently building expectations
/// ```
/// use rxpect::expect_ref;
/// use rxpect::expectations::EqualityExpectations;
///
/// expect_ref(&1).to_equal(1);
/// ```
pub fn expect_ref<T: Debug>(value: &'_ T) -> RootExpectations<'_, T> {
    RootExpectations::new_ref(value)
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{CheckResult, Expectation};
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::sync::Mutex;

    pub(crate) struct TestExpectation {
        pub asserted: Rc<Mutex<bool>>,
        result: CheckResult,
    }

    impl TestExpectation {
        pub fn new(result: CheckResult) -> (TestExpectation, Rc<Mutex<bool>>) {
            let asserted = Rc::new(Mutex::new(false));
            (
                TestExpectation {
                    asserted: asserted.clone(),
                    result,
                },
                asserted,
            )
        }
    }

    impl<T: Debug> Expectation<T> for TestExpectation {
        fn check(&self, _: &T) -> CheckResult {
            let mut asserted = self.asserted.lock().unwrap();
            *asserted = true;
            self.result.clone()
        }
    }
}
