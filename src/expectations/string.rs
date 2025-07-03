use std::fmt::Debug;
use std::marker::PhantomData;
use crate::{CheckResult, Expectation, ExpectationBuilder};

/// Expectations for strings
pub trait StringExpectations<'e, T>
where T: Debug + 'e {
    /// Expect that a string contains a substring
    fn to_contain(self, substring: &'e str) -> Self;
}

impl<'e, T, B> StringExpectations<'e, T> for B
where T: AsRef<str> + Debug + 'e,
    B: ExpectationBuilder<'e, T>
{
    fn to_contain(self, substring: &'e str) -> Self {
        self.to_pass(StringPredicateExpectation::new(substring, |a, b| a.contains(b), |a, b| format!("Expected \"{a}\" to contain \"{b}\"")))
    }
}

pub struct StringPredicateExpectation<T, U> {
    expected_value: U,
    predicate: for<'a> fn(&'a str, &U) -> bool,
    error_message: for<'a> fn(&'a str, &U) -> String,
    phantom_data: PhantomData<T>
}

impl<'e, U, T> StringPredicateExpectation<T, U>
where U: Debug + 'e,
    T: AsRef<str> + Debug + 'e
{
    fn new(expected_value: U, predicate: for<'a> fn(&'a str, &U) -> bool, error_message: for<'a> fn(&'a str, &U) -> String) -> Self {
        Self {
            expected_value,
            predicate,
            error_message,
            phantom_data: Default::default(),
        }
    }
}

impl <'e, T, U> Expectation<T> for StringPredicateExpectation<T, U>
where T: AsRef<str> + Debug + 'e,
      U: Debug + 'e
{
    fn check(&self, value: &T) -> CheckResult {
        let value_str = value.as_ref();
        if (self.predicate)(value_str, &self.expected_value) {
            CheckResult::Pass
        } else {
            CheckResult::Fail((self.error_message)(value_str, &self.expected_value))
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::expect;
    use crate::expectations::string::StringExpectations;

    #[rstest]
    #[case("", "")]
    #[case("foobar", "foo")]
    #[case("foobar", "bar")]
    #[case("foobar", "oob")]
    #[case("foobar", "")]
    fn that_to_contain_passes_when_string_contains_the_substring(#[case] actual: &str, #[case] substring: &str) {
        expect(actual).to_contain(substring);
    }

    #[rstest]
    #[case("", "foo")]
    #[case("foobar", "rab")]
    #[case("foobar", "oof")]
    #[case("foobar", "boo")]
    #[should_panic]
    fn that_to_contain_does_not_pass_when_string_does_not_contain_the_substring(#[case] actual: &str, #[case] substring: &str) {
        expect(actual).to_contain(substring);
    }
}