use core::convert::Infallible;

use crate::Problem;

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Unwrapped<T>(pub(super) T);

impl<T, P, In> Operator<P, In> for Unwrapped<T>
where
    T: Operator<P, In>,
    P: Problem,
{
    type Output = T::Output;

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut <P as Problem>::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let output = self.0.apply(solution, eval, problem, input).unwrap();
        Ok(output)
    }
}

impl<T, P> Init<P> for Unwrapped<T>
where
    T: Init<P>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        let solution = self.0.init(eval, problem).unwrap();
        Ok(solution)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, eval, problem).unwrap();
        Ok(())
    }
}
