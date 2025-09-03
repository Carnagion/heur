use core::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use crate::{Problem, op::init::Init, solution::Solution};

use super::Operator;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Map<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, In, Out> Operator<P, S, In> for Map<T, F>
where
    T: Operator<P, S, In>,
    F: FnMut(T::Output) -> Out,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = Out;

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
            .map(&mut self.f)
    }
}

impl<T, F> Debug for Map<T, F>
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct MapErr<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, In, Err> Operator<P, S, In> for MapErr<T, F>
where
    T: Operator<P, S, In>,
    F: FnMut(T::Error) -> Err,
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    type Output = T::Output;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, eval, problem, input)
            .map_err(&mut self.f)
    }
}

impl<T, F, P, S, Err> Init<P, S> for MapErr<T, F>
where
    T: Init<P, S>,
    F: FnMut(T::Error) -> Err,
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.op.init(eval, problem).map_err(&mut self.f)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.op
            .init_into(solution, eval, problem)
            .map_err(&mut self.f)
    }
}

impl<T, F> Debug for MapErr<T, F>
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct TryMap<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, In, Out> Operator<P, S, In> for TryMap<T, F>
where
    T: Operator<P, S, In>,
    F: FnMut(T::Output) -> Result<Out, T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = Out;

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
            .and_then(&mut self.f)
    }
}

impl<T, F> Debug for TryMap<T, F>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TryMap")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}
