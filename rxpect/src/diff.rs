use colored::Colorize;
use itertools::Itertools;
use similar::{ChangeTag, InlineChange, TextDiff};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    RemovedRow = 224,
    AddedRow = 194,
    RemovedInline = 217,
    AddedInline = 157,
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        color as u8
    }
}

fn format_inline_change(
    prefix: char,
    change: InlineChange<'_, str>,
    color: Color,
    inline_color: Color,
) -> String {
    let mut line = prefix.to_string().on_ansi_color(color).to_string();
    for (emphasized, text) in change.iter_strings_lossy() {
        let text = text.trim_end_matches('\n');
        if text.is_empty() {
            continue;
        }
        if emphasized {
            line.push_str(&text.on_ansi_color(inline_color).to_string());
        } else {
            line.push_str(&text.on_ansi_color(color).to_string());
        }
    }
    line.push('\n');
    line
}

pub fn diff_pretty_debug<T: Debug, U: Debug>(a: &T, b: &U) -> String {
    let a = format!("{:#?}", a);
    let b = format!("{:#?}", b);
    let diff = TextDiff::from_lines(&a, &b);
    let mut output = Vec::new();
    for change in diff
        .ops()
        .iter()
        .flat_map(|op| diff.iter_inline_changes(op))
    {
        match change.tag() {
            ChangeTag::Delete => output.push(format_inline_change(
                '-',
                change,
                Color::RemovedRow,
                Color::RemovedInline,
            )),
            ChangeTag::Insert => output.push(format_inline_change(
                '+',
                change,
                Color::AddedRow,
                Color::AddedInline,
            )),
            ChangeTag::Equal => output.push(format!(" {}", change)),
        }
    }
    output.iter().join("").trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use crate::diff::{Color, diff_pretty_debug};
    use crate::expect;
    use crate::expectations::EqualityExpectations;
    use colored::ColoredString;
    use colored::Colorize;
    use itertools::Itertools;
    use rstest::rstest;
    use std::fmt::Debug;

    #[derive(Debug)]
    #[allow(unused)]
    struct TestEntity {
        id: String,
        value: i32,
    }

    impl TestEntity {
        pub fn new(id: impl Into<String>, value: impl Into<i32>) -> Self {
            Self {
                id: id.into(),
                value: value.into(),
            }
        }
    }

    #[rstest]
    #[case("foo")]
    #[case("foo\nbar")]
    #[case(TestEntity::new("foo", 7))]
    fn that_no_diff_returns_original(#[case] input: impl Debug) {
        // Given two strings with the same content

        // When they are diffed
        let diff = diff_pretty_debug(&input, &input);

        // Then the diff contains no changes
        let padded_lines = format!("{:#?}", input)
            .split("\n")
            .map(|line| " ".to_string() + line)
            .join("\n");
        expect(diff).to_equal(padded_lines);
    }

    fn rr(s: &str) -> ColoredString {
        s.on_ansi_color(Color::RemovedRow)
    }

    fn ar(s: &str) -> ColoredString {
        s.on_ansi_color(Color::AddedRow)
    }
    fn ri(s: &str) -> ColoredString {
        s.on_ansi_color(Color::RemovedInline)
    }
    fn ai(s: &str) -> ColoredString {
        s.on_ansi_color(Color::AddedInline)
    }

    fn n(s: &str) -> ColoredString {
        s.normal().clear()
    }

    #[rstest]
    #[case::singleline("foo", "bar", vec![rr("-"), rr("\"foo\""), n("\n"), ar("+"), ar("\"bar\"")])]
    #[case::multiline("foo\nbar", "bar\nbar", vec![rr("-"), rr("\"foo\\nbar\""), n("\n"), ar("+"), ar("\"bar\\nbar\"")])]
    #[case::entity_id(TestEntity::new("foo", 7), TestEntity::new("foo", 8),
        vec![
            n(" TestEntity {\n     id: \"foo\",\n"),
            rr("-"), rr("    value: "), ri("7,"), n("\n"),
            ar("+"), ar("    value: "), ai("8,"), n("\n }")
        ]
    )]
    #[case::entity_value(TestEntity::new("foo", 7), TestEntity::new("bar", 7),
        vec![
            n(" TestEntity {\n"),
            rr("-"), rr("    id: "), ri("\"foo\","), n("\n"),
            ar("+"), ar("    id: "), ai("\"bar\","), n("\n"),
            n("     value: 7,"), n("\n }")
        ]
    )]
    #[case::entity_multiple(TestEntity::new("foo", 7), TestEntity::new("bar", 9),
        vec![
            n(" TestEntity {\n"),
            rr("-"), rr("    id: "), ri("\"foo\","), n("\n"),
            rr("-"), rr("    value: "), ri("7,"), n("\n"),
            ar("+"), ar("    id: "), ai("\"bar\","), n("\n"),
            ar("+"), ar("    value: "), ai("9,"), n("\n }")
        ]
    )]
    fn that_different_strings_return_a_diff<T: Debug>(
        #[case] a: T,
        #[case] b: T,
        #[case] expected: Vec<ColoredString>,
    ) {
        // Given two strings with different content

        // When they are diffed
        let diff = diff_pretty_debug(&a, &b);

        // Then the diff contains the changes
        let expected = expected.into_iter().join("");
        expect(diff).to_equal(expected);
    }
}
