use crate::data::Parent;
use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::{EqualityExpectations, ResultExpectations};

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "result",
            name: "to_be_ok",
            message: || extract_failure_message(expect(Err::<i32, &str>("kaboom")).to_be_ok()),
        },
        Visualization {
            header: "result",
            name: "to_be_ok_complex",
            message: || extract_failure_message(expect(Err::<Parent, &str>("kaboom")).to_be_ok()),
        },
        Visualization {
            header: "result",
            name: "to_be_err",
            message: || extract_failure_message(expect(Ok::<i32, &str>(1)).to_be_err()),
        },
        Visualization {
            header: "result",
            name: "to_be_err_complex",
            message: || {
                extract_failure_message(
                    expect(Ok::<Parent, &str>(Parent::hammersmith())).to_be_err(),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_ok_matching",
            message: || {
                extract_failure_message(expect(Ok::<i32, &str>(1)).to_be_ok_matching(|v| *v > 40))
            },
        },
        Visualization {
            header: "result",
            name: "to_be_ok_matching_complex",
            message: || {
                extract_failure_message(
                    expect(Ok::<Parent, &str>(Parent::quincey())).to_be_ok_matching(|_| false),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_err_matching",
            message: || {
                extract_failure_message(
                    expect(Err::<i32, &str>("oops")).to_be_err_matching(|e| *e == "expected"),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_ok_and",
            message: || {
                extract_failure_message(
                    expect(Err::<i32, &str>("kaboom"))
                        .to_be_ok_and()
                        .to_equal(1)
                        .unproject(),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_ok_and_complex",
            message: || {
                extract_failure_message(
                    expect(Err::<Parent, &str>("kaboom"))
                        .to_be_ok_and()
                        .to_equal(Parent::quincey())
                        .unproject(),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_err_and",
            message: || {
                extract_failure_message(
                    expect(Ok::<i32, &str>(1))
                        .to_be_err_and()
                        .to_equal("expected")
                        .unproject(),
                )
            },
        },
        Visualization {
            header: "result",
            name: "to_be_err_and_complex",
            message: || {
                extract_failure_message(
                    expect(Ok::<i32, Parent>(1))
                        .to_be_err_and()
                        .to_equal(Parent::quincey())
                        .unproject(),
                )
            },
        },
    ]
}
