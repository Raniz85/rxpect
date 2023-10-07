pub mod expectations;
mod expectation_list;
mod projection;
mod root;

pub use projection::*;
pub use root::*;
use std::fmt::Debug;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[derive(Clone, Debug)]
pub enum CheckResult {
    Pass,
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
pub trait ExpectationBuilder<'e, T: Debug> {
    /// Expect the value to pass an expectation
    /// This is intended to be used in extension methods to add expectations to the builder
    fn to_pass(self, expectation: impl Expectation<T> + 'e) -> Self;
}

/// Create expectations for a value.
/// Used as an entrypoint for fluently building expectations
/// ```
/// use rexpect::expect;
/// use rexpect::expectations::EqualityExpectations;
///
/// expect(1).to_equal(1);
/// ```
pub fn expect<'e, T: Debug>(value: T) -> RootExpectations<'e, T> {
    RootExpectations::new(value)
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
