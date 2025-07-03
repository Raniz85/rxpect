use std::fmt::Debug;
use crate::ExpectationBuilder;
use super::predicate::PredicateExpectation;

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
        self.to_pass(PredicateExpectation::new(substring, |a: &T, b| a.as_ref().contains(b), |a: &T, b| format!("Expected \"{}\" to contain \"{b}\"", a.as_ref())))
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