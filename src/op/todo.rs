use std::{convert::Infallible, error::Error, marker::PhantomData};

use crate::{eval::Eval, solution::Solution};

use super::Operator;

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
