use super::predicate::PredicateExpectation;
use crate::ExpectationBuilder;
use std::fmt::Debug;

/// Expectations for strings
pub trait StringExpectations<'e, T>
where
    T: Debug + 'e,
{
    /// Expect that a string contains a substring
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello, world!";
    /// expect(text).to_contain("world");
    /// ```
    /// asserts that `text` contains the substring "world"
    fn to_contain(self, substring: &'e str) -> Self;

    /// Expect that a string does not contain a substring
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello, world!";
    /// expect(text).to_not_contain("foo");
    /// ```
    /// asserts that `text` does not contain the substring "foo"
    fn to_not_contain(self, substring: &'e str) -> Self;

    /// Expect that a string has a specific length
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello";
    /// expect(text).to_have_length(5);
    /// ```
    /// asserts that `text` has a length of 5 characters
    fn to_have_length(self, length: usize) -> Self;

    /// Expect that a string starts with a specific prefix
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello, world!";
    /// expect(text).to_start_with("Hello");
    /// ```
    /// asserts that `text` starts with the prefix "Hello"
    fn to_start_with(self, prefix: &'e str) -> Self;

    /// Expect that a string ends with a specific suffix
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello, world!";
    /// expect(text).to_end_with("world!");
    /// ```
    /// asserts that `text` ends with the suffix "world!"
    fn to_end_with(self, suffix: &'e str) -> Self;

    /// Expect that a string is empty
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "";
    /// expect(text).to_be_empty();
    /// ```
    /// asserts that `text` is an empty string
    fn to_be_empty(self) -> Self;

    /// Expect that a string consists entirely of whitespace characters
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "   \t\n";
    /// expect(text).to_be_all_whitespace();
    /// ```
    /// asserts that `text` consists entirely of whitespace characters
    fn to_be_all_whitespace(self) -> Self;

    /// Expect that a string consists entirely of alphabetic characters
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello";
    /// expect(text).to_be_alphabetic();
    /// ```
    /// asserts that `text` consists entirely of alphabetic characters
    fn to_be_alphabetic(self) -> Self;

    /// Expect that a string consists entirely of numeric characters
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "12345";
    /// expect(text).to_be_numeric();
    /// ```
    /// asserts that `text` consists entirely of numeric characters
    fn to_be_numeric(self) -> Self;

    /// Expect that a string consists entirely of alphanumeric characters
    /// ```
    /// # use rxpect::expect;
    /// # use rxpect::expectations::StringExpectations;
    ///
    /// let text = "Hello123";
    /// expect(text).to_be_alphanumeric();
    /// ```
    /// asserts that `text` consists entirely of alphanumeric characters
    fn to_be_alphanumeric(self) -> Self;
}

impl<'e, T, B> StringExpectations<'e, T> for B
where
    T: AsRef<str> + Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn to_contain(self, substring: &'e str) -> Self {
        self.to_pass(PredicateExpectation::new(
            substring,
            |a: &T, b| a.as_ref().contains(b),
            |a: &T, b| format!("Expected \"{}\" to contain \"{b}\"", a.as_ref()),
        ))
    }

    fn to_not_contain(self, substring: &'e str) -> Self {
        self.to_pass(PredicateExpectation::new(
            substring,
            |a: &T, b| !a.as_ref().contains(b),
            |a: &T, b| format!("Expected \"{}\" to not contain \"{b}\"", a.as_ref()),
        ))
    }

    fn to_have_length(self, length: usize) -> Self {
        self.to_pass(PredicateExpectation::new(
            length,
            |a: &T, &b| a.as_ref().len() == b,
            |a: &T, &b| {
                format!(
                    "Expected \"{}\" to have length {b}, but it has length {}",
                    a.as_ref(),
                    a.as_ref().len()
                )
            },
        ))
    }

    fn to_start_with(self, prefix: &'e str) -> Self {
        self.to_pass(PredicateExpectation::new(
            prefix,
            |a: &T, b| a.as_ref().starts_with(b),
            |a: &T, b| format!("Expected \"{}\" to start with \"{b}\"", a.as_ref()),
        ))
    }

    fn to_end_with(self, suffix: &'e str) -> Self {
        self.to_pass(PredicateExpectation::new(
            suffix,
            |a: &T, b| a.as_ref().ends_with(b),
            |a: &T, b| format!("Expected \"{}\" to end with \"{b}\"", a.as_ref()),
        ))
    }

    fn to_be_empty(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |a: &T, _| a.as_ref().is_empty(),
            |a: &T, _| format!("Expected \"{}\" to be empty", a.as_ref()),
        ))
    }

    fn to_be_all_whitespace(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |a: &T, _| a.as_ref().chars().all(|c| c.is_whitespace()),
            |a: &T, _| format!("Expected \"{}\" to be all whitespace", a.as_ref()),
        ))
    }

    fn to_be_alphabetic(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |a: &T, _| !a.as_ref().is_empty() && a.as_ref().chars().all(|c| c.is_alphabetic()),
            |a: &T, _| format!("Expected \"{}\" to be alphabetic", a.as_ref()),
        ))
    }

    fn to_be_numeric(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |a: &T, _| !a.as_ref().is_empty() && a.as_ref().chars().all(|c| c.is_numeric()),
            |a: &T, _| format!("Expected \"{}\" to be numeric", a.as_ref()),
        ))
    }

    fn to_be_alphanumeric(self) -> Self {
        self.to_pass(PredicateExpectation::new(
            (),
            |a: &T, _| !a.as_ref().is_empty() && a.as_ref().chars().all(|c| c.is_alphanumeric()),
            |a: &T, _| format!("Expected \"{}\" to be alphanumeric", a.as_ref()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::expect;
    use crate::expectations::string::StringExpectations;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("foobar", "foo")]
    #[case("foobar", "bar")]
    #[case("foobar", "oob")]
    #[case("foobar", "")]
    fn that_to_contain_passes_when_string_contains_the_substring(
        #[case] actual: &str,
        #[case] substring: &str,
    ) {
        expect(actual).to_contain(substring);
    }

    #[rstest]
    #[case("", "foo")]
    #[case("foobar", "rab")]
    #[case("foobar", "oof")]
    #[case("foobar", "boo")]
    #[should_panic]
    fn that_to_contain_does_not_pass_when_string_does_not_contain_the_substring(
        #[case] actual: &str,
        #[case] substring: &str,
    ) {
        expect(actual).to_contain(substring);
    }

    #[rstest]
    #[case("", "foo")]
    #[case("foobar", "rab")]
    #[case("foobar", "oof")]
    #[case("foobar", "boo")]
    fn that_to_not_contain_passes_when_string_does_not_contain_the_substring(
        #[case] actual: &str,
        #[case] substring: &str,
    ) {
        expect(actual).to_not_contain(substring);
    }

    #[rstest]
    #[case("", "")]
    #[case("foobar", "foo")]
    #[case("foobar", "bar")]
    #[case("foobar", "oob")]
    #[case("foobar", "")]
    #[should_panic]
    fn that_to_not_contain_does_not_pass_when_string_contains_the_substring(
        #[case] actual: &str,
        #[case] substring: &str,
    ) {
        expect(actual).to_not_contain(substring);
    }

    #[rstest]
    #[case("", 0)]
    #[case("a", 1)]
    #[case("ab", 2)]
    #[case("abc", 3)]
    #[case("abcd", 4)]
    fn that_to_have_length_passes_when_string_has_expected_length(
        #[case] actual: &str,
        #[case] length: usize,
    ) {
        expect(actual).to_have_length(length);
    }

    #[rstest]
    #[case("", 1)]
    #[case("a", 0)]
    #[case("ab", 3)]
    #[case("abc", 2)]
    #[case("abcd", 5)]
    #[should_panic]
    fn that_to_have_length_does_not_pass_when_string_does_not_have_expected_length(
        #[case] actual: &str,
        #[case] length: usize,
    ) {
        expect(actual).to_have_length(length);
    }

    #[rstest]
    #[case("", "")]
    #[case("foobar", "foo")]
    #[case("foobar", "f")]
    #[case("foobar", "")]
    fn that_to_start_with_passes_when_string_starts_with_prefix(
        #[case] actual: &str,
        #[case] prefix: &str,
    ) {
        expect(actual).to_start_with(prefix);
    }

    #[rstest]
    #[case("", "foo")]
    #[case("foobar", "bar")]
    #[case("foobar", "oo")]
    #[case("foobar", "foobar1")]
    #[should_panic]
    fn that_to_start_with_does_not_pass_when_string_does_not_start_with_prefix(
        #[case] actual: &str,
        #[case] prefix: &str,
    ) {
        expect(actual).to_start_with(prefix);
    }

    #[rstest]
    #[case("", "")]
    #[case("foobar", "bar")]
    #[case("foobar", "r")]
    #[case("foobar", "")]
    fn that_to_end_with_passes_when_string_ends_with_suffix(
        #[case] actual: &str,
        #[case] suffix: &str,
    ) {
        expect(actual).to_end_with(suffix);
    }

    #[rstest]
    #[case("", "foo")]
    #[case("foobar", "foo")]
    #[case("foobar", "ba")]
    #[case("foobar", "1foobar")]
    #[should_panic]
    fn that_to_end_with_does_not_pass_when_string_does_not_end_with_suffix(
        #[case] actual: &str,
        #[case] suffix: &str,
    ) {
        expect(actual).to_end_with(suffix);
    }

    #[rstest]
    #[case("")]
    fn that_to_be_empty_passes_when_string_is_empty(#[case] actual: &str) {
        expect(actual).to_be_empty();
    }

    #[rstest]
    #[case("a")]
    #[case("foo")]
    #[case(" ")]
    #[should_panic]
    fn that_to_be_empty_does_not_pass_when_string_is_not_empty(#[case] actual: &str) {
        expect(actual).to_be_empty();
    }

    #[rstest]
    #[case("")]
    #[case(" ")]
    #[case("  ")]
    #[case("\t")]
    #[case("\n")]
    #[case(" \t\n\r")]
    fn that_to_be_all_whitespace_passes_when_string_is_all_whitespace(#[case] actual: &str) {
        expect(actual).to_be_all_whitespace();
    }

    #[rstest]
    #[case("a")]
    #[case("foo")]
    #[case(" a")]
    #[case("a ")]
    #[case(" a ")]
    #[should_panic]
    fn that_to_be_all_whitespace_does_not_pass_when_string_is_not_all_whitespace(
        #[case] actual: &str,
    ) {
        expect(actual).to_be_all_whitespace();
    }

    #[rstest]
    #[case("a")]
    #[case("abc")]
    #[case("Abc")]
    #[case("ABC")]
    #[case("абв")]
    fn that_to_be_alphabetic_passes_when_string_is_alphabetic(#[case] actual: &str) {
        expect(actual).to_be_alphabetic();
    }

    #[rstest]
    #[case("")]
    #[case("123")]
    #[case("a1")]
    #[case("1a")]
    #[case("a b")]
    #[case("a-b")]
    #[should_panic]
    fn that_to_be_alphabetic_does_not_pass_when_string_is_not_alphabetic(#[case] actual: &str) {
        expect(actual).to_be_alphabetic();
    }

    #[rstest]
    #[case("0")]
    #[case("123")]
    #[case("١٢٣")] // Arabic numerals
    fn that_to_be_numeric_passes_when_string_is_numeric(#[case] actual: &str) {
        expect(actual).to_be_numeric();
    }

    #[rstest]
    #[case("")]
    #[case("abc")]
    #[case("1a")]
    #[case("a1")]
    #[case("1 2")]
    #[case("1-2")]
    #[should_panic]
    fn that_to_be_numeric_does_not_pass_when_string_is_not_numeric(#[case] actual: &str) {
        expect(actual).to_be_numeric();
    }
}
