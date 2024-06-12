use std::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use super::Operator;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FromFn<F>(pub(super) F);

impl<F, S, P, E, In, Out, Err> Operator<S, P, E, In> for FromFn<F>
where
    F: FnMut(&mut S, &P, &mut E, In) -> Result<Out, Err>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        (self.0)(solution, problem, eval, input)
    }
}

impl<F> Debug for FromFn<F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("FromFn").finish()
    }
}
