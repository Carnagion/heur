use crate::{Optimize, Problem};

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Pipe<T, U> {
    pub(super) from: T,
    pub(super) to: U,
}

impl<T, U, P, In> Operator<P, In> for Pipe<T, U>
where
    T: Operator<P, In>,
    U: Operator<P, T::Output, Error = T::Error>,
    P: Problem,
{
    type Output = U::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let intermediate = self.from.apply(solution, eval, problem, input)?;
        let output = self.to.apply(solution, eval, problem, intermediate)?;
        Ok(output)
    }
}

impl<T, U, P> Init<P> for Pipe<T, U>
where
    T: Init<P>,
    U: Operator<P, Output = (), Error = T::Error>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.from.by_ref().then(&mut self.to).init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.from
            .by_ref()
            .then(&mut self.to)
            .init_into(solution, eval, problem)
    }
}

impl<T, U, P> Optimize<P> for Pipe<T, U>
where
    T: Init<P>,
    U: Operator<P, Output = (), Error = T::Error>,
    P: Problem,
{
    type Error = <Self as Operator<P>>::Error;

    fn optimize(
        &mut self,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<<P as Problem>::Solution, Self::Error> {
        self.init(eval, problem)
    }
}
