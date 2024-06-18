use std::{convert::Infallible, error::Error, marker::PhantomData};

use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Manually impl common traits
#[must_use]
pub struct Todo<P, S, E, In = (), Out = (), Err = Infallible>(
    #[allow(clippy::type_complexity)] pub(super) PhantomData<fn() -> (P, S, E, In, Out, Err)>,
);

impl<P, S, E, In, Out, Err> Operator<P, S, E, In> for Todo<P, S, E, In, Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    #[inline]
    fn apply(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
        _input: In,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl<P, S, E, Out, Err> Init<P, S, E> for Todo<P, S, E, (), Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    #[inline]
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        todo!()
    }
}

impl<P, S, E, Out, Err> Mutate<P, S, E> for Todo<P, S, E, (), Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    #[inline]
    fn mutate(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<P, S, E, Out, Err> Search<P, S, E> for Todo<P, S, E, (), Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    #[inline]
    fn search(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
