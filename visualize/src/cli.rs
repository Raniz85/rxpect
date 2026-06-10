use crate::visualization::Visualization;

/// Selects the visualizations to run from the given positional args.
///
/// - `[]` selects every visualization
/// - `[header]` selects every visualization under that header
/// - `[header, name]` selects the single visualization matching both
/// - anything else selects nothing
pub fn select<'a>(args: &[String], visualizations: &'a [Visualization]) -> Vec<&'a Visualization> {
    match args {
        [] => visualizations.iter().collect(),
        [header] => visualizations
            .iter()
            .filter(|v| v.header == header.as_str())
            .collect(),
        [header, name] => visualizations
            .iter()
            .filter(|v| v.header == header.as_str() && v.name == name.as_str())
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::select;
    use crate::visualization::Visualization;

    fn visualization(header: &'static str, name: &'static str) -> Visualization {
        Visualization {
            header,
            name,
            message: || "x".to_string(),
        }
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| part.to_string()).collect()
    }

    fn selected_pairs(visualizations: &[&Visualization]) -> Vec<(&'static str, &'static str)> {
        visualizations.iter().map(|v| (v.header, v.name)).collect()
    }

    #[test]
    fn that_no_args_selects_all_visualizations() {
        // given visualizations across two headers
        let visualizations = vec![
            visualization("string", "to_start_with"),
            visualization("order", "to_be_less_than"),
        ];

        // when selecting with no args
        let selected = select(&args(&[]), &visualizations);

        // then every visualization is selected
        assert_eq!(
            selected_pairs(&selected),
            vec![("string", "to_start_with"), ("order", "to_be_less_than"),]
        );
    }

    #[test]
    fn that_single_header_arg_selects_only_that_headers_visualizations() {
        // given visualizations in the "string" and "order" headers
        let visualizations = vec![
            visualization("string", "to_start_with"),
            visualization("string", "to_end_with"),
            visualization("order", "to_be_less_than"),
        ];

        // when selecting with the "string" header
        let selected = select(&args(&["string"]), &visualizations);

        // then only the "string" visualizations are selected
        assert_eq!(
            selected_pairs(&selected),
            vec![("string", "to_start_with"), ("string", "to_end_with"),]
        );
    }

    #[test]
    fn that_header_and_name_args_select_the_single_matching_visualization() {
        // given a "string" header with two visualizations
        let visualizations = vec![
            visualization("string", "to_start_with"),
            visualization("string", "to_end_with"),
        ];

        // when selecting with the "string" header and "to_start_with" name
        let selected = select(&args(&["string", "to_start_with"]), &visualizations);

        // then only the matching visualization is selected
        assert_eq!(selected_pairs(&selected), vec![("string", "to_start_with")]);
    }

    #[test]
    fn that_unknown_header_selects_nothing() {
        // given visualizations only in the "string" header
        let visualizations = vec![visualization("string", "to_start_with")];

        // when selecting with an unknown header
        let selected = select(&args(&["nonsense"]), &visualizations);

        // then nothing is selected
        assert_eq!(selected_pairs(&selected), Vec::new());
    }

    #[test]
    fn that_known_header_with_unknown_name_selects_nothing() {
        // given a "string" header without a "to_explode" visualization
        let visualizations = vec![visualization("string", "to_start_with")];

        // when selecting with a known header and unknown name
        let selected = select(&args(&["string", "to_explode"]), &visualizations);

        // then nothing is selected
        assert_eq!(selected_pairs(&selected), Vec::new());
    }

    #[test]
    fn that_a_name_matching_in_a_different_header_is_not_selected() {
        // given the same "to_be_empty" name under two headers
        let visualizations = vec![
            visualization("string", "to_be_empty"),
            visualization("iterables", "to_be_empty"),
        ];

        // when selecting with the "iterables" header and "to_be_empty" name
        let selected = select(&args(&["iterables", "to_be_empty"]), &visualizations);

        // then only the "iterables" visualization is selected
        assert_eq!(
            selected_pairs(&selected),
            vec![("iterables", "to_be_empty")]
        );
    }

    #[test]
    fn that_more_than_two_args_selects_nothing() {
        // given a "string" header with one visualization
        let visualizations = vec![visualization("string", "to_start_with")];

        // when selecting with more than two args
        let selected = select(
            &args(&["string", "to_start_with", "extra"]),
            &visualizations,
        );

        // then nothing is selected
        assert_eq!(selected_pairs(&selected), Vec::new());
    }
}
