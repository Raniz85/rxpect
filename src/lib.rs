pub mod expect;
mod expectation_list;
mod projection;
mod root;

pub use projection::*;
pub use root::*;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum CheckResult {
    Pass,
    Fail(String),
}

/// An expectation on a value
pub trait Expectation<T: Debug> {
    /// Check this expectation
    fn check(&self, value: &T) -> CheckResult;
}

/// Trait to enable fluent building of expectations
pub trait ExpectationBuilder<'e, T: Debug> {
    fn to_pass(self, expectation: impl Expectation<T> + 'e) -> Self;
}

/// Create expectations for a value.
/// Used as an entrypoint for fluently building expectations
/// ```
/// use fluent_rs::expect;
/// use fluent_rs::expect::EqualityExpectations;
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
