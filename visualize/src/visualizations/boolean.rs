use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::BooleanExpectations;

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "boolean",
            name: "to_be_true",
            message: || extract_failure_message(expect(false).to_be_true()),
        },
        Visualization {
            header: "boolean",
            name: "to_be_false",
            message: || extract_failure_message(expect(true).to_be_false()),
        },
    ]
}
