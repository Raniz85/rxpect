use crate::expectation_list::ExpectationList;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::fmt::Debug;
use std::marker::PhantomData;

struct ProjectedExpectations<'e, F, T, U>
where
    F: Fn(&T) -> U,
    T: Debug,
    U: Debug + 'e,
{
    projection: F,
    expectations: ExpectationList<'e, U>,
    _t: PhantomData<&'e T>,
}

impl<'e, F, T, U> Expectation<T> for ProjectedExpectations<'e, F, T, U>
where
    F: (Fn(&T) -> U) + 'e,
    T: Debug,
    U: Debug + 'e,
{
    fn check(&self, value: &T) -> CheckResult {
        let projected = (self.projection)(value);
        match self.expectations.check(&projected) {
            CheckResult::Fail(message) => CheckResult::Fail(
                message
                    .lines()
                    .map(|line| "  ".to_string() + line)
                    .fold(String::new(), |a, b| a + &b + "\n")
                    .trim_end()
                    .to_owned(),
            ),
            pass => pass,
        }
    }
}

pub trait ExpectProjection<'e, F, T, U, B>
where
    F: (Fn(&T) -> U) + 'e,
    T: Debug + 'e,
    U: Debug + 'e,
    B: ExpectationBuilder<'e, U>,
{
    /// Add expectations on a projected value
    ///
    /// ```
    /// use rxpect::expect;
    /// use rxpect::expectations::EqualityExpectations;
    /// use rxpect::ExpectProjection;
    ///
    /// #[derive(Debug)]
    /// pub struct MyStruct {
    ///     pub foo: u32
    /// }
    /// expect(MyStruct{ foo: 7 }).projected_by(|it| it.foo, |foo| foo
    ///     .to_equal(7)
    /// );
    /// ```
    fn projected_by(self, projection: F, config: impl FnOnce(B) -> B) -> Self;
}

impl<'e, F, T, U, B> ExpectProjection<'e, F, T, U, ExpectationList<'e, U>> for B
where
    F: (Fn(&T) -> U) + 'e,
    T: Debug + 'e,
    U: Debug + 'e,
    B: ExpectationBuilder<'e, T>,
{
    fn projected_by(
        self,
        projection: F,
        config: impl FnOnce(ExpectationList<'e, U>) -> ExpectationList<'e, U>,
    ) -> Self {
        let expectations = config(ExpectationList::new());
        self.to_pass(ProjectedExpectations {
            projection,
            expectations,
            _t: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::expectation_list::ExpectationList;
    use crate::projection::ProjectedExpectations;
    use crate::tests::TestExpectation;
    use crate::{CheckResult, ExpectProjection, Expectation, ExpectationBuilder, expect};

    #[test]
    pub fn that_projection_runs_all_expectations() {
        // Given two expectations that both pass
        let (expectation1, expected1) = TestExpectation::new(CheckResult::Pass);
        let (expectation2, expected2) = TestExpectation::new(CheckResult::Pass);

        // And a projection expectation containing those
        let projection =
            expect(true).projected_by(|_| 1, |it| it.to_pass(expectation1).to_pass(expectation2));

        // When the projection is dropped
        drop(projection);

        // Then both expectations were run
        assert!(*expected1.lock().unwrap());
        assert!(*expected2.lock().unwrap());
    }

    #[test]
    pub fn that_projection_indents_output() {
        // Given expectation that fails
        let (expectation, _) = TestExpectation::new(CheckResult::Fail(
            "this\nis\na\nmultiline\nmessage".to_string(),
        ));

        // And an aggregated projection expectation
        let mut projected = ProjectedExpectations {
            expectations: ExpectationList::new(),
            projection: |_| 1,
            _t: Default::default(),
        };
        projected.expectations.push(expectation);

        // When the aggregated expectation is checked
        let result = projected.check(&true);

        // Then each line of the error message starts with two spaces
        if let CheckResult::Fail(message) = result {
            message
                .lines()
                .for_each(|line| assert!(line.starts_with("  ")));
        } else {
            panic!("Result was a pass when failure was expected");
        }
    }

    #[test]
    pub fn that_projections_can_be_nested_deeply() {
        // Given an expectation that passes
        let (expectation, expected) = TestExpectation::new(CheckResult::Pass);

        // And a deeply projected expectation
        let expectations = expect(true).projected_by(
            |_| 1,
            |it| {
                it.projected_by(
                    |_| 1.0,
                    |it| it.projected_by(|_| "foo", |it| it.to_pass(expectation)),
                )
            },
        );

        // When the expectations are checked
        drop(expectations);

        // Then the expectation was checked
        assert!(*expected.lock().unwrap());
    }
}
