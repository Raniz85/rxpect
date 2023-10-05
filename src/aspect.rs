use crate::expectation_list::ExpectationList;
use crate::{CheckResult, Expectation, ExpectationBuilder};
use std::cell::RefCell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct AspectExpectations<'e, P, PT: 'e, T: 'e> {
    parent: P,
    expectations: Rc<RefCell<ExpectationList<'e, T>>>,
    value: Rc<T>,
    _pt: PhantomData<PT>,
}

impl<'e, P, PT, T> AspectExpectations<'e, P, PT, T>
where
    P: ExpectationBuilder<'e, PT>,
    PT: Debug + 'e,
    T: Debug + 'e,
{
    pub(crate) fn new(mut parent: P, value: T) -> Self {
        let value = Rc::new(value);
        let (expectation, expectations) = AggregatedAspectExpectation::new(value.clone());
        parent.add_expectation(expectation);
        AspectExpectations {
            parent,
            expectations,
            value,
            _pt: Default::default(),
        }
    }

    pub fn aspect<U: Debug + 'e>(
        self,
        transform: impl FnOnce(&T) -> U,
    ) -> AspectExpectations<'e, Self, T, U> {
        let value = transform(&self.value);
        AspectExpectations::new(self, value)
    }

    pub fn parent(self) -> P {
        self.parent
    }
}

impl<'e, P, PT, T> ExpectationBuilder<'e, T> for AspectExpectations<'e, P, PT, T>
where
    PT: Debug + 'e,
    T: Debug + 'e,
{
    /// Add an expectation to the list of expectations
    fn add_expectation(&mut self, expectation: impl Expectation<T> + 'e) -> &mut Self {
        self.expectations.borrow_mut().push(expectation);
        self
    }
}

struct AggregatedAspectExpectation<'e, PT, T> {
    value: Rc<T>,
    expectations: Rc<RefCell<ExpectationList<'e, T>>>,
    _pt: PhantomData<PT>,
}

impl<'e, PT, T> AggregatedAspectExpectation<'e, PT, T>
where
    PT: Debug + 'e,
    T: Debug + 'e,
{
    fn new(
        value: Rc<T>,
    ) -> (
        AggregatedAspectExpectation<'e, PT, T>,
        Rc<RefCell<ExpectationList<'e, T>>>,
    ) {
        let expectations = Rc::new(RefCell::new(ExpectationList::new()));
        (
            AggregatedAspectExpectation {
                value,
                expectations: expectations.clone(),
                _pt: Default::default(),
            },
            expectations,
        )
    }
}

impl<'e, PT, T> Expectation<PT> for AggregatedAspectExpectation<'e, PT, T>
where
    PT: Debug + 'e,
    T: Debug + 'e,
{
    fn check(&self, _: &PT) -> CheckResult {
        match self.expectations.borrow().check(self.value.as_ref()) {
            CheckResult::Fail(message) => CheckResult::Fail(message.lines()
                .map(|line| "  ".to_string() + line)
                .fold(String::new(), |a, b| a + &b + "\n").trim_end().to_owned()),
            pass => pass,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::tests::TestExpectation;
    use crate::{expect, CheckResult, ExpectationBuilder, Expectation};
    use crate::aspect::AggregatedAspectExpectation;

    #[test]
    pub fn that_aspect_runs_all_expectations() {
        // Given two expectations that both pass
        let (expectation1, expected1) = TestExpectation::new(CheckResult::Pass);
        let (expectation2, expected2) = TestExpectation::new(CheckResult::Pass);

        // And an aspect expectation containing those
        let mut aspect = expect(true).aspect(|_| 1);
        aspect.add_expectation(expectation1);
        aspect.add_expectation(expectation2);

        // When the aspect is dropped
        drop(aspect);

        // Then both expectations were run
        assert!(*expected1.lock().unwrap());
        assert!(*expected2.lock().unwrap());
    }

    #[test]
    pub fn that_aspect_indents_output() {
        // Given an aggregated aspect expectation
        let (aggregated, expectations) = AggregatedAspectExpectation::new(Rc::new(1));

        // And expectation that fails
        let (expectation, _) = TestExpectation::new(CheckResult::Fail("this\nis\na\nmultiline\nmessage".to_string()));
        expectations.borrow_mut().push(expectation);

        // When the aggregated expectation is checked
        let result = aggregated.check(&true);

        // Then each line of the error message starts with two spaces
        if let CheckResult::Fail(message) = result {
            message.lines()
                .for_each(|line| assert!(line.starts_with("  ")));
        } else {
            panic!("Result was a pass when failure was expected");
        }
    }
}
