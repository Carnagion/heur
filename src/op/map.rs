use std::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Map<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, S, P, E, In, Out> Operator<S, P, E, In> for Map<T, F>
where
    T: Operator<S, P, E, In>,
    F: FnMut(T::Output) -> Out,
{
    type Output = Out;

    type Error = T::Error;

    #[inline]
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
pub struct MapErr<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, S, P, E, In, Err> Operator<S, P, E, In> for MapErr<T, F>
where
    T: Operator<S, P, E, In>,
    F: FnMut(T::Error) -> Err,
    Err: Error,
{
    type Output = T::Output;

    type Error = Err;

    #[inline]
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

impl<T, F, S, P, E, Err> Init<S, P, E> for MapErr<T, F>
where
    T: Init<S, P, E>,
    F: FnMut(T::Error) -> Err,
    Err: Error,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval).map_err(&mut self.f)
    }

    #[inline]
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

impl<T, F, S, P, E, Err> Mutate<S, P, E> for MapErr<T, F>
where
    T: Mutate<S, P, E>,
    F: FnMut(T::Error) -> Err,
    Err: Error,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval).map_err(&mut self.f)
    }
}

impl<T, F, S, P, E, Err> Search<S, P, E> for MapErr<T, F>
where
    T: Search<S, P, E>,
    F: FnMut(T::Error) -> Err,
    Err: Error,
{
    #[inline]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TryMap<T, F> {
    pub(super) op: T,
    pub(super) f: F,
}

impl<T, F, S, P, E, In, Out> Operator<S, P, E, In> for TryMap<T, F>
where
    T: Operator<S, P, E, In>,
    F: FnMut(T::Output) -> Result<Out, T::Error>,
{
    type Output = Out;

    type Error = T::Error;

    #[inline]
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
