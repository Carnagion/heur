use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[must_use]
pub struct Todo<P, S, E, In = (), Out = (), Err = Infallible>(
    #[allow(clippy::type_complexity)] pub(super) PhantomData<fn() -> (P, S, E, In, Out, Err)>,
);

impl<P, S, E, In, Out, Err> Debug for Todo<P, S, E, In, Out, Err> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Todo").finish_non_exhaustive()
    }
}

impl<P, S, E, In, Out, Err> Default for Todo<P, S, E, In, Out, Err> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P, S, E, In, Out, Err> Copy for Todo<P, S, E, In, Out, Err> {}

impl<P, S, E, In, Out, Err> Clone for Todo<P, S, E, In, Out, Err> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, S, E, In, Out, Err> Eq for Todo<P, S, E, In, Out, Err> {}

impl<P, S, E, In, Out, Err> PartialEq for Todo<P, S, E, In, Out, Err> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<P, S, E, In, Out, Err> Hash for Todo<P, S, E, In, Out, Err> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

impl<P, S, E, In, Out, Err> Operator<P, S, E, In> for Todo<P, S, E, In, Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        _solution: &mut S,
        _problem: &P,
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
    fn mutate(
        &mut self,
        _solution: &mut S,
        _problem: &P,
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
    fn search(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
