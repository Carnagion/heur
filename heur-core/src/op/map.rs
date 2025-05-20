use core::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use crate::Problem;

use super::Operator;

// TODO: Should this impl `Init`, `Mutate`, and/or `Search`?
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Map<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, In, Out> Operator<P, In> for Map<T, F>
where
    T: Operator<P, In>,
    F: FnMut(T::Output) -> Out,
    P: Problem,
{
    type Output = Out;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
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

impl<T, F, P, In, Err> Operator<P, In> for MapErr<T, F>
where
    T: Operator<P, In>,
    F: FnMut(T::Error) -> Err,
    P: Problem,
    Err: Error,
{
    type Output = T::Output;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, eval, problem, input)
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

// TODO: Should this impl `Init`, `Mutate`, and/or `Search`?
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct TryMap<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, In, Out> Operator<P, In> for TryMap<T, F>
where
    T: Operator<P, In>,
    F: FnMut(T::Output) -> Result<Out, T::Error>,
    P: Problem,
{
    type Output = Out;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
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
