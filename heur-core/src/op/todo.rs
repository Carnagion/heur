use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::Problem;

use super::Operator;

#[must_use]
pub struct Todo<P, In = (), Out = (), Err = Infallible>(
    #[allow(clippy::type_complexity)] pub(super) PhantomData<fn() -> (P, In, Out, Err)>,
);

impl<P, In, Out, Err> Operator<P, In> for Todo<P, In, Out, Err>
where
    P: Problem,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        _: &mut P::Solution,
        _: &mut P::Eval,
        _: &P,
        _: In,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

impl<P, In, Out, Err> Debug for Todo<P, In, Out, Err> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Todo").finish_non_exhaustive()
    }
}

impl<P, In, Out, Err> Default for Todo<P, In, Out, Err> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<P, In, Out, Err> Copy for Todo<P, In, Out, Err> {}

impl<P, In, Out, Err> Clone for Todo<P, In, Out, Err> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, In, Out, Err> Eq for Todo<P, In, Out, Err> {}

impl<P, In, Out, Err> PartialEq for Todo<P, In, Out, Err> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P, In, Out, Err> Hash for Todo<P, In, Out, Err> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}
