mod cli;
pub mod data;
mod visualization;
mod visualizations;

use rxpect::{CheckResult, OwnedExpectations};
use std::fmt::Debug;

pub(crate) fn extract_failure_message<T: Debug>(expectations: OwnedExpectations<T>) -> String {
    match expectations.check_result().1 {
        CheckResult::Fail(message) => message,
        _ => panic!("Expected failure"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let all = visualizations::all();
    let selected = cli::select(&args, &all);

    if selected.is_empty() && !args.is_empty() {
        println!("No visualizations matched: {}", args.join(" "));
        return;
    }

    for visualization in selected {
        println!("=== {} / {} ===", visualization.header, visualization.name);
        println!("{}", (visualization.message)());
        println!();
    }
}
