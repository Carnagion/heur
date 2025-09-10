use core::fmt::{self, Debug, Formatter};

use crate::{
    Problem,
    op::{Operator, init::Init},
    solution::Solution,
};

/// An operator that calls a function on the output produced by its inner operator.
///
/// This type is created by [`Operator::inspect`]. See its documentation for more details.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Inspect<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, In> Operator<P, S, In> for Inspect<T, F>
where
    T: Operator<P, S, In>,
    F: FnMut(&T::Output),
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, eval, problem, input)
            .inspect(&mut self.f)
    }
}

impl<T, F> Debug for Inspect<T, F>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Map")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

/// An operator that calls a function on any errors produced by its inner operator.
///
/// This type is created by [`Operator::inspect_err`]. See its documentation for more details.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct InspectErr<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, In> Operator<P, S, In> for InspectErr<T, F>
where
    T: Operator<P, S, In>,
    F: FnMut(&T::Error),
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, eval, problem, input)
            .inspect_err(&mut self.f)
    }
}

impl<T, F, P, S> Init<P, S> for InspectErr<T, F>
where
    T: Init<P, S>,
    F: FnMut(&T::Error),
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.op.init(eval, problem).inspect_err(&mut self.f)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.op
            .init_into(solution, eval, problem)
            .inspect_err(&mut self.f)
    }
}

impl<T, F> Debug for InspectErr<T, F>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MapErr")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}
