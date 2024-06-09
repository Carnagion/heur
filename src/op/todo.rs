use std::{convert::Infallible, error::Error, marker::PhantomData};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Manually impl common traits
pub struct Todo<S, P, E, In = (), Out = (), Err = Infallible>(
    pub(super) PhantomData<(S, P, E, In, Out, Err)>,
);

impl<S, P, E, In, Out, Err> Operator<S, P, E, In> for Todo<S, P, E, In, Out, Err>
where
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    #[inline]
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

impl<S, P, E, Out, Err> Init<S, P, E> for Todo<S, P, E, (), Out, Err>
where
    Err: Error,
{
    #[inline]
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        todo!()
    }
}

impl<S, P, E, Out, Err> Mutate<S, P, E> for Todo<S, P, E, (), Out, Err>
where
    Err: Error,
{
    #[inline]
    fn mutate(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<S, P, E, Out, Err> Search<S, P, E> for Todo<S, P, E, (), Out, Err>
where
    Err: Error,
{
    #[inline]
    fn search(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
