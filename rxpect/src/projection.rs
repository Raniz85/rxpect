use crate::borrow::BorrowedOrOwned;
use crate::expectation_list::ExpectationList;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

/// Indent each line of a string by two spaces
fn indent(message: String) -> String {
    message
        .lines()
        .map(|line| "  ".to_string() + line)
        .fold(String::new(), |a, b| a + &b + "\n")
        .trim_end()
        .to_owned()
}

/// Expectations on a projected value.
pub struct ProjectedExpectations<'e, T, U, F>
where
    T: Debug,
    U: Debug + 'e,
    F: for<'a> Fn(&'a T) -> Option<BorrowedOrOwned<'a, U>>,
{
    expectations: Rc<RefCell<ExpectationList<'e, U>>>,
    extract: F,
    fail_message: fn(&T) -> String,
    _phantom: PhantomData<&'e (T, U)>,
}

impl<'e, T, U, F> ProjectedExpectations<'e, T, U, F>
where
    T: Debug,
    U: Debug + 'e,
    F: for<'a> Fn(&'a T) -> Option<BorrowedOrOwned<'a, U>>,
{
    /// Create new `ProjectedExpectations` paired with its shared expectation list.
    ///
    /// When checked, `extract` is called: `Some` runs inner expectations on the extracted value,
    /// `None` calls `fail_message` to produce the error string.
    ///
    /// # Returns
    /// A tuple containing:
    ///
    /// * The `ProjectedExpectations`
    /// * The `ExpectationList` so that more expectations can be added
    pub fn new(
        extract: F,
        fail_message: fn(&T) -> String,
    ) -> (Self, Rc<RefCell<ExpectationList<'e, U>>>) {
        let expectations = Rc::new(RefCell::new(ExpectationList::new()));
        (
            Self {
                expectations: expectations.clone(),
                extract,
                fail_message,
                _phantom: PhantomData,
            },
            expectations,
        )
    }
}

impl<'e, T, U, F> Expectation<T> for ProjectedExpectations<'e, T, U, F>
where
    T: Debug,
    U: Debug + 'e,
    F: for<'a> Fn(&'a T) -> Option<BorrowedOrOwned<'a, U>>,
{
    fn check(&self, value: &T) -> CheckResult {
        match (self.extract)(value) {
            Some(inner) => {
                let inner_ref = inner.borrow_self();
                match (*self.expectations).borrow().check(inner_ref) {
                    CheckResult::Fail(message) => CheckResult::Fail(indent(message)),
                    pass => pass,
                }
            }
            None => CheckResult::Fail((self.fail_message)(value)),
        }
    }
}

/// Builder for chaining expectations on a projected value.
///
/// Expectations on the projected value are added to the parent expectation builder and will be checked
/// in order when the parent expectations are checked.
pub struct ProjectedExpectationsBuilder<'e, P, T, U>
where
    T: Debug + 'e,
    U: Debug + 'e,
    P: ExpectationBuilder<'e, Value = T>,
{
    parent: P,
    expectations: Rc<RefCell<ExpectationList<'e, U>>>,
    _phantom: PhantomData<&'e T>,
}

impl<'e, P, T, U> ProjectedExpectationsBuilder<'e, P, T, U>
where
    T: Debug + 'e,
    U: Debug + 'e,
    P: ExpectationBuilder<'e, Value = T>,
{
    /// Create a `ProjectedExpectationsBuilder` from a pre-built expectation and its shared list.
    /// Used by variant-extracting builders (e.g. `to_be_ok_and`).
    pub fn from_expectation(
        parent: P,
        expectation: impl Expectation<T> + 'e,
        expectations: Rc<RefCell<ExpectationList<'e, U>>>,
    ) -> Self {
        Self {
            parent: parent.to_pass(expectation),
            expectations,
            _phantom: PhantomData,
        }
    }

    /// Drop the projection and return the parent expectation builder.
    ///
    /// No checks are executed when unprojecting - they live on the parent expectation builder.
    pub fn unproject(self) -> P {
        self.parent
    }
}

impl<'e, P, T, U> ExpectationBuilder<'e> for ProjectedExpectationsBuilder<'e, P, T, U>
where
    T: Debug + 'e,
    U: Debug + 'e,
    P: ExpectationBuilder<'e, Value = T>,
{
    type Value = U;

    fn to_pass(self, expectation: impl Expectation<U> + 'e) -> Self {
        self.expectations.borrow_mut().push(expectation);
        self
    }
}

/// Extension trait for adding expectations on a projected value.
pub trait ExpectProjection<'e, T, U>
where
    T: Debug + 'e,
    U: Debug + 'e,
{
    /// Add expectations on a projected value.
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
    /// expect(MyStruct{ foo: 7 }).projected_by(|it| it.foo).to_equal(7);
    /// ```
    fn projected_by<F>(self, projection: F) -> ProjectedExpectationsBuilder<'e, Self, T, U>
    where
        F: Fn(&T) -> U + 'e,
        Self: Sized + ExpectationBuilder<'e, Value = T>;

    /// Add expectations on a projected reference.
    ///
    /// ```
    /// use rxpect::expect;
    /// use rxpect::expectations::EqualityExpectations;
    /// use rxpect::ExpectProjection;
    ///
    /// #[derive(Debug)]
    /// struct Parent {
    ///     child: Child,
    /// }
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct Child {
    ///     number: u32,
    /// }
    ///
    /// let value = Parent {
    ///     child: Child {
    ///         number: 7
    ///     },
    /// };
    /// expect(value)
    ///     .projected_by_ref(|s| &s.child)
    ///     .to_equal(Child { number: 7 });
    /// ```
    fn projected_by_ref<F>(self, projection: F) -> ProjectedExpectationsBuilder<'e, Self, T, U>
    where
        F: (for<'a> Fn(&'a T) -> &'a U) + 'e,
        Self: Sized + ExpectationBuilder<'e, Value = T>;
}

impl<'e, T, U, B> ExpectProjection<'e, T, U> for B
where
    T: Debug + 'e,
    U: Debug + 'e,
    B: ExpectationBuilder<'e, Value = T>,
{
    fn projected_by<F>(self, projection: F) -> ProjectedExpectationsBuilder<'e, Self, T, U>
    where
        F: Fn(&T) -> U + 'e,
    {
        let (expectation, expectations) = ProjectedExpectations::new(
            move |value| Some(BorrowedOrOwned::Owned(projection(value))),
            |_| unreachable!(),
        );
        ProjectedExpectationsBuilder {
            parent: self.to_pass(expectation),
            expectations,
            _phantom: PhantomData,
        }
    }

    fn projected_by_ref<F>(self, projection: F) -> ProjectedExpectationsBuilder<'e, Self, T, U>
    where
        F: (for<'a> Fn(&'a T) -> &'a U) + 'e,
    {
        let (expectation, expectations) = ProjectedExpectations::new(
            move |value| Some(BorrowedOrOwned::Borrowed(projection(value))),
            |_| unreachable!(),
        );
        ProjectedExpectationsBuilder {
            parent: self.to_pass(expectation),
            expectations,
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::borrow::BorrowedOrOwned;
    use crate::expectations::EqualityExpectations;
    use crate::projection::ProjectedExpectations;
    use crate::tests::TestExpectation;
    use crate::{CheckResult, ExpectProjection, Expectation, ExpectationBuilder, expect};

    #[test]
    pub fn that_projection_runs_all_expectations() {
        // Given two expectations that both pass
        let (expectation1, expected1) = TestExpectation::new(CheckResult::Pass);
        let (expectation2, expected2) = TestExpectation::new(CheckResult::Pass);

        // And a projection expectation containing those
        let projection = expect(true)
            .projected_by(|_| 1)
            .to_pass(expectation1)
            .to_pass(expectation2);

        // When the projection is dropped
        drop(projection);

        // Then both expectations were run
        assert!(*expected1.lock().unwrap());
        assert!(*expected2.lock().unwrap());
    }

    #[test]
    pub fn that_projection_indents_output() {
        // Given an expectation that fails
        let (expectation, _) = TestExpectation::new(CheckResult::Fail(
            "this\nis\na\nmultiline\nmessage".to_string(),
        ));

        // And a projection expectation
        let (projected, expectations) = ProjectedExpectations::new(
            |v: &bool| Some(BorrowedOrOwned::Owned(*v)),
            |_| unreachable!(),
        );
        expectations.borrow_mut().push(expectation);

        // When the expectation is checked
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
    pub fn that_variant_projection_indents_output() {
        // Given an expectation that fails
        let (expectation, _) = TestExpectation::new(CheckResult::Fail(
            "this\nis\na\nmultiline\nmessage".to_string(),
        ));

        // And a variant projection expectation
        let (projected, expectations) = ProjectedExpectations::new(
            |v: &bool| Some(BorrowedOrOwned::Borrowed(v)),
            |_| unreachable!(),
        );
        expectations.borrow_mut().push(expectation);

        // When the expectation is checked
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
        let expectations = expect(true)
            .projected_by(|_: &bool| 1i32)
            .projected_by(|_: &i32| 1.0f64)
            .projected_by(|_: &f64| "foo")
            .to_pass(expectation);

        // When the expectations are checked
        drop(expectations);

        // Then the expectation was checked
        assert!(*expected.lock().unwrap());
    }

    #[test]
    pub fn that_projection_by_ref_works() {
        #[derive(Debug)]
        struct Parent {
            child: Child,
        }

        #[derive(Debug, Eq, PartialEq)]
        struct Child {
            number: u32,
        }
        let value = Parent {
            child: Child { number: 7 },
        };
        expect(value)
            .projected_by_ref(|s| &s.child)
            .to_equal(Child { number: 7 });
    }
}
