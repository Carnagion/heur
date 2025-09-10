use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Problem, solution::Solution};

use super::Operator;

/// An operator that can be used in place of any other operator and panics in its implementation of [`apply`](Operator::apply).
///
/// This type is created by [`op::todo`](crate::op::todo). See its documentation for more details.
#[must_use]
pub struct Todo<P, S, In = (), Out = (), Err = Infallible>(
    #[allow(clippy::type_complexity)] pub(super) PhantomData<fn() -> (P, S, In, Out, Err)>,
);

impl<P, S, In, Out, Err> Operator<P, S, In> for Todo<P, S, In, Out, Err>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        _: &mut S,
        _: &mut P::Eval,
        _: &P,
        _: In,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl<P, S, In, Out, Err> Debug for Todo<P, S, In, Out, Err> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Todo").finish_non_exhaustive()
    }
}

impl<P, S, In, Out, Err> Default for Todo<P, S, In, Out, Err> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P, S, In, Out, Err> Copy for Todo<P, S, In, Out, Err> {}

impl<P, S, In, Out, Err> Clone for Todo<P, S, In, Out, Err> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, S, In, Out, Err> Eq for Todo<P, S, In, Out, Err> {}

impl<P, S, In, Out, Err> PartialEq for Todo<P, S, In, Out, Err> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P, S, In, Out, Err> Hash for Todo<P, S, In, Out, Err> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}
