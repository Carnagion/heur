use crate::{Optimize, Problem, solution::Solution};

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Pipe<T, U> {
    pub(super) from: T,
    pub(super) to: U,
}

impl<T, U, P, S, In> Operator<P, S, In> for Pipe<T, U>
where
    T: Operator<P, S, In>,
    U: Operator<P, S, T::Output, Error = T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = U::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let intermediate = self.from.apply(solution, eval, problem, input)?;
        let output = self.to.apply(solution, eval, problem, intermediate)?;
        Ok(output)
    }
}

impl<T, U, P, S> Init<P, S> for Pipe<T, U>
where
    T: Init<P, S>,
    U: Operator<P, S, Output = (), Error = T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.from.by_ref().then(&mut self.to).init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.from
            .by_ref()
            .then(&mut self.to)
            .init_into(solution, eval, problem)
    }
}

impl<T, U, P, S> Optimize<P, S> for Pipe<T, U>
where
    T: Init<P, S>,
    U: Operator<P, S, Output = (), Error = T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = <Self as Operator<P, S>>::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.init(eval, problem)
    }
}
