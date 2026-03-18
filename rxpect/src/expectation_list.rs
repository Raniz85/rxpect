use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

/// List of expectations on a value.
pub struct ExpectationList<'e, T>(Vec<Box<dyn Expectation<T> + 'e>>);

impl<'e, T: Debug> ExpectationList<'e, T> {
    /// Creates a new empty list of expectations.
    pub fn new() -> Self {
        ExpectationList(Vec::new())
    }

    /// Add a new expectation to the list.
    pub fn push(&mut self, expectation: impl Expectation<T> + 'e) {
        self.0.push(Box::new(expectation));
    }

    /// Check all expectations on the value.
    ///
    /// Runs _all_ expectations in order.
    ///
    /// # Returns
    /// `CheckResult::Pass` if all expectations pass, otherwise `CheckResult::Fail` with a formatted message.
    /// If multiple failures occur, they are concatenated with newlines.
    pub fn check(&self, value: &T) -> CheckResult {
        let failures = self
            .0
            .iter()
            .map(|e| e.check(value))
            .filter_map(|r| match r {
                CheckResult::Fail(message) => Some(message),
                _ => None,
            })
            .collect::<Vec<String>>();
        if !failures.is_empty() {
            // TODO: ensure messages are nicely formatted
            let message = failures
                .into_iter()
                .fold(String::new(), |a, b| a + &b + "\n")
                .trim()
                .to_owned();
            CheckResult::Fail(message)
        } else {
            CheckResult::Pass
        }
    }
}

impl<'e, T: Debug> Default for ExpectationList<'e, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'e, T> ExpectationBuilder<'e> for ExpectationList<'e, T>
where
    T: Debug + 'e,
{
    type Value = T;

    fn to_pass(mut self, expectation: impl Expectation<T> + 'e) -> Self {
        self.push(expectation);
        self
    }
}
