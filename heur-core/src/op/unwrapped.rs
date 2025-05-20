use core::convert::Infallible;

use crate::Problem;

use super::Operator;

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
