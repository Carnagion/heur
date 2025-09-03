use core::convert::Infallible;

use crate::{Optimize, Problem, solution::Solution};

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Unwrapped<T>(pub(super) T);

impl<T, P, S, In> Operator<P, S, In> for Unwrapped<T>
where
    T: Operator<P, S, In>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let output = self.0.apply(solution, eval, problem, input).unwrap();
        Ok(output)
    }
}

impl<T, P, S> Init<P, S> for Unwrapped<T>
where
    T: Init<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        let solution = self.0.init(eval, problem).unwrap();
        Ok(solution)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, eval, problem).unwrap();
        Ok(())
    }
}

impl<T, P, S> Optimize<P, S> for Unwrapped<T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = Infallible;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        let solution = self.0.optimize(eval, problem).unwrap();
        Ok(solution)
    }
}
