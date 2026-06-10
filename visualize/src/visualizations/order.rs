use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::OrderExpectations;

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "order",
            name: "to_be_less_than",
            message: || extract_failure_message(expect(5).to_be_less_than(3)),
        },
        Visualization {
            header: "order",
            name: "to_be_less_than_or_equal",
            message: || extract_failure_message(expect(5).to_be_less_than_or_equal(3)),
        },
        Visualization {
            header: "order",
            name: "to_be_greater_than",
            message: || extract_failure_message(expect(3).to_be_greater_than(5)),
        },
        Visualization {
            header: "order",
            name: "to_be_greater_than_or_equal",
            message: || extract_failure_message(expect(3).to_be_greater_than_or_equal(5)),
        },
        Visualization {
            header: "order",
            name: "to_be_inside",
            message: || extract_failure_message(expect(10).to_be_inside(0..5)),
        },
    ]
}
