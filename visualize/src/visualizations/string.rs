use crate::extract_failure_message;
use crate::visualization::Visualization;
use rxpect::expect;
use rxpect::expectations::StringExpectations;

pub fn visualizations() -> Vec<Visualization> {
    vec![
        Visualization {
            header: "string",
            name: "to_contain",
            message: || extract_failure_message(expect("Hello, world!").to_contain("goodbye")),
        },
        Visualization {
            header: "string",
            name: "to_not_contain",
            message: || extract_failure_message(expect("Hello, world!").to_not_contain("world")),
        },
        Visualization {
            header: "string",
            name: "to_have_length",
            message: || extract_failure_message(expect("Hello").to_have_length(3)),
        },
        Visualization {
            header: "string",
            name: "to_start_with",
            message: || extract_failure_message(expect("Hello, world!").to_start_with("Howdy")),
        },
        Visualization {
            header: "string",
            name: "to_end_with",
            message: || extract_failure_message(expect("Hello, world!").to_end_with("planet!")),
        },
        Visualization {
            header: "string",
            name: "to_be_empty",
            message: || extract_failure_message(expect("Hello").to_be_empty()),
        },
        Visualization {
            header: "string",
            name: "to_be_all_whitespace",
            message: || extract_failure_message(expect("Hello").to_be_all_whitespace()),
        },
        Visualization {
            header: "string",
            name: "to_be_alphabetic",
            message: || extract_failure_message(expect("Hello123").to_be_alphabetic()),
        },
        Visualization {
            header: "string",
            name: "to_be_numeric",
            message: || extract_failure_message(expect("12a45").to_be_numeric()),
        },
        Visualization {
            header: "string",
            name: "to_be_alphanumeric",
            message: || extract_failure_message(expect("Hello, world!").to_be_alphanumeric()),
        },
    ]
}
