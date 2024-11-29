use std::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Should this impl `Init`, `Mutate`, and/or `Search`?
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Map<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, P, S, E, In, Out> Operator<P, S, E, In> for Map<T, F>
where
    T: Operator<P, S, E, In>,
    F: FnMut(T::Output) -> Out,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = Out;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, problem, eval, input)
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

impl<T, F, P, S, E, In, Err> Operator<P, S, E, In> for MapErr<T, F>
where
    T: Operator<P, S, E, In>,
    F: FnMut(T::Error) -> Err,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    type Output = T::Output;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, problem, eval, input)
            .map_err(&mut self.f)
    }
}

impl<T, F, P, S, E, Err> Init<P, S, E> for MapErr<T, F>
where
    T: Init<P, S, E>,
    F: FnMut(T::Error) -> Err,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval).map_err(&mut self.f)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op
            .init_into(solution, problem, eval)
            .map_err(&mut self.f)
    }
}

impl<T, F, P, S, E, Err> Mutate<P, S, E> for MapErr<T, F>
where
    T: Mutate<P, S, E>,
    F: FnMut(T::Error) -> Err,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval).map_err(&mut self.f)
    }
}

impl<T, F, P, S, E, Err> Search<P, S, E> for MapErr<T, F>
where
    T: Search<P, S, E>,
    F: FnMut(T::Error) -> Err,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(solution, problem, eval).map_err(&mut self.f)
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

impl<T, F, P, S, E, In, Out> Operator<P, S, E, In> for TryMap<T, F>
where
    T: Operator<P, S, E, In>,
    F: FnMut(T::Output) -> Result<Out, T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = Out;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op
            .apply(solution, problem, eval, input)
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
