use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

use crate::Problem;

use super::Operator;

type OperatorFn<P, In, Out, Err> =
    fn(&mut <P as Problem>::Solution, &mut <P as Problem>::Eval, &P, In) -> Result<Out, Err>;

// TODO: Manually implement common traits
#[must_use]
pub struct FromFn<P, In = (), Out = (), Err = Infallible, F = OperatorFn<P, In, Out, Err>> {
    pub(super) f: F,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, In, Out, Err)>,
}

impl<P, In, Out, Err, F> Operator<P, In> for FromFn<P, In, Out, Err, F>
where
    F: FnMut(&mut P::Solution, &mut P::Eval, &P, In) -> Result<Out, Err>,
    P: Problem,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        (self.f)(solution, eval, problem, input)
    }
}

impl<P, In, Out, Err, F> Debug for FromFn<P, In, Out, Err, F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("FromFn").finish_non_exhaustive()
    }
}
