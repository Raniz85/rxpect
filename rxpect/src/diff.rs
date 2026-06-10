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

fn contains_ref<T>(haystack: &[&T], needle: &T) -> bool {
    haystack.iter().any(|item| std::ptr::eq(*item, needle))
}

pub fn format_flagged_list<T: Debug>(
    items: &[&T],
    flagged_items: &[&T],
    prefix: char,
    color: Color,
) -> String {
    let diff_items = items
        .iter()
        .map(|item| {
            if contains_ref(flagged_items, item) {
                format!("{:#?},", item)
                    .split('\n')
                    .map(|line| format!("{}    {}", prefix, line).on_ansi_color(color))
                    .join("\n")
            } else {
                format!("{:#?},", item)
                    .split('\n')
                    .map(|line| format!("     {}", line))
                    .join("\n")
            }
        })
        .map(|item| format!("{item}\n"))
        .collect_vec();
    format!("[\n{}]", diff_items.iter().join(""))
}

#[cfg(test)]
mod tests {
    use crate::diff::{Color, diff_pretty_debug, format_flagged_list};
    use crate::expect;
    use crate::expectations::EqualityExpectations;
    use colored::ColoredString;
    use colored::Colorize;
    use dedent::dedent;
    use itertools::Itertools;
    use rstest::rstest;
    use std::fmt::Debug;

    #[derive(Debug, PartialEq)]
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

    #[rstest]
    #[case(&[1, 2, 3, 4, 5], &[1, 4], format!("[\n     1,\n{}\n     3,\n     4,\n{}\n]",
            "-    2,".on_ansi_color(Color::RemovedRow),
            "-    5,".on_ansi_color(Color::RemovedRow),
    ))]
    #[case(&[
        TestEntity::new("foo", 1),
        TestEntity::new("bar", 2),
        TestEntity::new("foobar", 3),
        TestEntity::new("barfoo", 4),
        TestEntity::new("paj", 5),
        ], &[1, 4], format!(dedent!(r#"
            [
                 TestEntity {{
                     id: "foo",
                     value: 1,
                 }},
            {}
            {}
            {}
            {}
                 TestEntity {{
                     id: "foobar",
                     value: 3,
                 }},
                 TestEntity {{
                     id: "barfoo",
                     value: 4,
                 }},
            {}
            {}
            {}
            {}
            ]"#),
            r#"-    TestEntity {"#.on_ansi_color(Color::RemovedRow),
            r#"-        id: "bar","#.on_ansi_color(Color::RemovedRow),
            r#"-        value: 2,"#.on_ansi_color(Color::RemovedRow),
            r#"-    },"#.on_ansi_color(Color::RemovedRow),
            r#"-    TestEntity {"#.on_ansi_color(Color::RemovedRow),
            r#"-        id: "paj","#.on_ansi_color(Color::RemovedRow),
            r#"-        value: 5,"#.on_ansi_color(Color::RemovedRow),
            r#"-    },"#.on_ansi_color(Color::RemovedRow),
    ))]
    fn that_flag_list_items_are_rendering_flagged_items_correctly<T: PartialEq + Debug>(
        #[case] items: &[T],
        #[case] flagged_indices: &[usize],
        #[case] expected_output: impl AsRef<str>,
    ) {
        // And a list of items to flag
        let flagged_items = flagged_indices.iter().map(|&i| &items[i]).collect_vec();

        // When flagged items is rendered
        let output = format_flagged_list(
            &items.iter().collect_vec(),
            &flagged_items,
            '-',
            Color::RemovedRow,
        );

        // Then the flagged items are in the rendered color
        expect(output).to_equal(expected_output.as_ref());
    }
}
