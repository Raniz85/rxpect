use crate::data::{Child, Parent};
use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, OptionExpectations, ProjectedOptionExpectations};

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "option",
            name: "to_be_some",
            message: || extract_failure_message(expect(None::<i32>).to_be_some()),
        },
        Visualization {
            header: "option",
            name: "to_be_some_complex",
            message: || extract_failure_message(expect(None::<Parent>).to_be_some()),
        },
        Visualization {
            header: "option",
            name: "to_be_none",
            message: || extract_failure_message(expect(Some(1)).to_be_none()),
        },
        Visualization {
            header: "option",
            name: "to_be_none_complex",
            message: || extract_failure_message(expect(Some(Parent::hammersmith())).to_be_none()),
        },
        Visualization {
            header: "option",
            name: "to_be_some_matching",
            message: || extract_failure_message(expect(Some(1)).to_be_some_matching(|v| *v > 40)),
        },
        Visualization {
            header: "option",
            name: "to_be_some_matching_complex",
            message: || {
                extract_failure_message(
                    expect(Some(Child::theodore())).to_be_some_matching(|_| false),
                )
            },
        },
        Visualization {
            header: "option",
            name: "to_be_some_and",
            message: || {
                extract_failure_message(
                    expect(None::<i32>).to_be_some_and().to_equal(1).unproject(),
                )
            },
        },
        Visualization {
            header: "option",
            name: "to_be_some_and_complex",
            message: || {
                extract_failure_message(
                    expect(None::<Parent>)
                        .to_be_some_and()
                        .to_equal(Parent::quincey())
                        .unproject(),
                )
            },
        },
    ]
}
