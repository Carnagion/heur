use core::{
    error::Error,
    fmt::{self, Debug, Formatter},
};

use crate::{eval::Eval, solution::Solution};

use super::Operator;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromFn<F>(pub(super) F);

impl<F, P, S, E, In, Out, Err> Operator<P, S, E, In> for FromFn<F>
where
    F: FnMut(&mut S, &P, &mut E, In) -> Result<Out, Err>,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

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
        formatter.debug_tuple("FromFn").finish_non_exhaustive()
    }
}
