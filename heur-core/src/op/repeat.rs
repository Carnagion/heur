use crate::{Problem, solution::Solution};

use super::{Operator, cond::stop::Stop};

/// An operator that repeatedly applies its inner operator a given number of times.
///
/// This type is created by [`Operator::repeat`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Repeat<T> {
    pub(super) op: T,
    pub(super) times: usize,
}

impl<T, P, S, In> Operator<P, S, In> for Repeat<T>
where
    T: Operator<P, S, In, Output = In>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
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

/// An operator that repeatedly applies its inner operator until a stopping condition is met.
///
/// This type is created by [`Operator::repeat_until`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct RepeatUntil<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, S, In> Operator<P, S, In> for RepeatUntil<T, F>
where
    T: Operator<P, S, In, Output = In>,
    F: Stop<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
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
