use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;

pub(crate) struct ExpectationList<'e, T>(Vec<Box<dyn Expectation<T> + 'e>>);

impl<'e, T: Debug> ExpectationList<'e, T> {
    pub(crate) fn new() -> Self {
        ExpectationList(Vec::new())
    }

    pub(crate) fn push(&mut self, expectation: impl Expectation<T> + 'e) {
        self.0.push(Box::new(expectation));
    }

    pub(crate) fn check(&self, value: &T) -> CheckResult {
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

impl<'e, T> ExpectationBuilder<'e, T> for ExpectationList<'e, T>
where
    T: Debug + 'e,
{
    fn to_pass(mut self, expectation: impl Expectation<T> + 'e) -> Self {
        self.push(expectation);
        self
    }
}
