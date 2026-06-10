use crate::visualization::Visualization;

mod boolean;
mod equality;
mod iterables;
mod option;
mod order;
mod result;
mod string;

/// Every visualization across all headers, in display order.
pub fn all() -> Vec<Visualization> {
    let mut visualizations = Vec::new();
    visualizations.extend(equality::visualizations());
    visualizations.extend(order::visualizations());
    visualizations.extend(boolean::visualizations());
    visualizations.extend(string::visualizations());
    visualizations.extend(option::visualizations());
    visualizations.extend(result::visualizations());
    visualizations.extend(iterables::visualizations());
    visualizations
}

#[cfg(test)]
mod tests {
    use super::all;

    #[test]
    fn that_every_visualization_produces_a_failure_message() {
        // given every registered visualization
        let visualizations = all();

        // when each visualization's message is produced
        let empty: Vec<&str> = visualizations
            .iter()
            .filter(|v| (v.message)().is_empty())
            .map(|v| v.name)
            .collect();

        // then none of them are empty
        assert_eq!(empty, Vec::<&str>::new());
    }
}
