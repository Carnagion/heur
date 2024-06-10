use std::{convert::Infallible, marker::PhantomData};

use crate::op::Operator;

use super::Init;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FromValue<S>(pub(super) S);

impl<S, P, E> Operator<S, P, E> for FromValue<S>
where
    S: Clone,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        solution.clone_from(&self.0);
        Ok(())
    }
}

impl<S, P, E> Init<S, P, E> for FromValue<S>
where
    S: Clone,
{
    #[inline]
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        Ok(self.0.clone())
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        solution.clone_from(&self.0);
        Ok(())
    }
}

// TODO: Manually impl common traits
pub struct FromDefault<S>(pub(super) PhantomData<fn() -> S>);

impl<S, P, E> Operator<S, P, E> for FromDefault<S>
where
    S: Default,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        *solution = S::default();
        Ok(())
    }
}

impl<S, P, E> Init<S, P, E> for FromDefault<S>
where
    S: Default,
{
    #[inline]
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        Ok(S::default())
    }
}
