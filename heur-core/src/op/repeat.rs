use crate::Problem;

use super::{Operator, cond::stop::Stop};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Repeat<T> {
    pub(super) op: T,
    pub(super) times: usize,
}

impl<T, P, In> Operator<P, In> for Repeat<T>
where
    T: Operator<P, In, Output = In>,
    P: Problem,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        for _ in 0..self.times {
            input = self.op.apply(solution, eval, problem, input)?;
        }
        Ok(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct RepeatUntil<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, In> Operator<P, In> for RepeatUntil<T, F>
where
    T: Operator<P, In, Output = In>,
    F: Stop<P>,
    P: Problem,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        while !self.cond.stop(solution, eval, problem) {
            input = self.op.apply(solution, eval, problem, input)?;
        }
        Ok(input)
    }
}
