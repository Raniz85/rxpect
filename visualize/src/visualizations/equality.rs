use crate::data::Parent;
use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::EqualityExpectations;

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "equality",
            name: "to_equal",
            message: || extract_failure_message(expect(1).to_equal(2)),
        },
        Visualization {
            header: "equality",
            name: "to_equal_complex",
            message: || {
                extract_failure_message(expect(Parent::hammersmith()).to_equal(Parent::quincey()))
            },
        },
    ]
}
