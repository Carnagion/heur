use crate::{eval::Eval, solution::Solution};

use super::Operator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Pipe<T, U> {
    pub(super) from: T,
    pub(super) to: U,
}

impl<T, U, P, S, E, In> Operator<P, S, E, In> for Pipe<T, U>
where
    T: Operator<P, S, E, In>,
    U: Operator<P, S, E, T::Output, Error = T::Error>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = U::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let intermediate = self.from.apply(problem, solution, eval, input)?;
        let output = self.to.apply(problem, solution, eval, intermediate)?;
        Ok(output)
    }
}
