use core::{convert::Infallible, error::Error, marker::PhantomData};

use crate::Problem;

use super::{Operator, init::Init};

// TODO: Manually implement common traits
#[must_use]
pub struct Hint<T, P, In = (), Out = (), Err = Infallible> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, In, Out, Err)>,
}

impl<T, P, In, Out, Err> Operator<P, In> for Hint<T, P, In, Out, Err>
where
    T: Operator<P, In, Output = Out, Error = Err>,
    P: Problem,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut <P as Problem>::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, eval, problem, input)
    }
}

impl<T, P, Err> Init<P> for Hint<T, P, (), (), Err>
where
    T: Init<P, Error = Err>,
    P: Problem,
    Err: Error,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.op.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, eval, problem)
    }
}
