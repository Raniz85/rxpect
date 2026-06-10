use crate::data::Child;
use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::iterables::{
    IterableCountExpectations, IterableItemEqualityExpectations,
};

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "iterables",
            name: "to_contain_equal_to",
            message: || extract_failure_message(expect(vec![1, 2, 3]).to_contain_equal_to(5)),
        },
        Visualization {
            header: "iterables",
            name: "to_contain_equal_to_complex",
            message: || {
                extract_failure_message(
                    expect(vec![Child::theodore(), Child::clementine()])
                        .to_contain_equal_to(Child::rosalind()),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_contain_equal_to_all_of",
            message: || {
                extract_failure_message(expect(vec![1, 2, 3]).to_contain_equal_to_all_of([2, 5]))
            },
        },
        Visualization {
            header: "iterables",
            name: "to_contain_equal_to_all_of_complex",
            message: || {
                extract_failure_message(
                    expect(vec![Child::theodore(), Child::clementine()])
                        .to_contain_equal_to_all_of([Child::theodore(), Child::rosalind()]),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_equivalent_to",
            message: || {
                extract_failure_message(expect(vec![1, 2, 3]).to_be_equivalent_to([1, 4, 3]))
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_equivalent_to_complex",
            message: || {
                extract_failure_message(
                    expect(vec![Child::theodore(), Child::clementine()])
                        .to_be_equivalent_to([Child::theodore(), Child::rosalind()]),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_equivalent_to_in_any_order",
            message: || {
                extract_failure_message(
                    expect(vec![1, 2, 3]).to_be_equivalent_to_in_any_order([3, 5, 1]),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_equivalent_to_in_any_order_complex_single",
            message: || {
                extract_failure_message(
                    expect(vec![
                        Child::theodore(),
                        Child::clementine(),
                        Child::rosalind(),
                    ])
                    .to_be_equivalent_to_in_any_order([
                        Child::theodore(),
                        Child::maximilian(),
                        Child::rosalind(),
                    ]),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_equivalent_to_in_any_order_complex_multi",
            message: || {
                extract_failure_message(
                    expect(vec![
                        Child::theodore(),
                        Child::clementine(),
                        Child::rosalind(),
                    ])
                    .to_be_equivalent_to_in_any_order([
                        Child::theodore(),
                        Child::maximilian(),
                        Child::genevieve(),
                    ]),
                )
            },
        },
        Visualization {
            header: "iterables",
            name: "to_be_empty",
            message: || extract_failure_message(expect(vec![1, 2, 3]).to_be_empty()),
        },
        Visualization {
            header: "iterables",
            name: "to_be_empty_complex",
            message: || extract_failure_message(expect(vec![Child::theodore()]).to_be_empty()),
        },
        Visualization {
            header: "iterables",
            name: "to_not_be_empty",
            message: || extract_failure_message(expect(Vec::<i32>::new()).to_not_be_empty()),
        },
    ]
}
