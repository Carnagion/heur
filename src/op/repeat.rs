use crate::{eval::Eval, solution::Solution};

use super::{mutate::Mutate, search::Search, stop::Stop, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Repeat<T> {
    pub(super) op: T,
    pub(super) times: usize,
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Repeat<T>
where
    T: Operator<P, S, E, In, Output = In>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        for _ in 0..self.times {
            input = self.op.apply(problem, solution, eval, input)?;
        }
        Ok(input)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Repeat<T>
where
    T: Mutate<P, S, E, Output = ()>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        for _ in 0..self.times {
            self.op.mutate(problem, solution, eval)?;
        }
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for Repeat<T>
where
    T: Search<P, S, E, Output = ()>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        for _ in 0..self.times {
            self.op.search(problem, solution, eval)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct RepeatUntil<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, S, E, In> Operator<P, S, E, In> for RepeatUntil<T, F>
where
    T: Operator<P, S, E, In, Output = In>,
    F: Stop<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        while !self.cond.stop(problem, solution, eval) {
            input = self.op.apply(problem, solution, eval, input)?;
        }
        Ok(input)
    }
}

impl<T, F, P, S, E> Mutate<P, S, E> for RepeatUntil<T, F>
where
    T: Mutate<P, S, E, Output = ()>,
    F: Stop<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        while !self.cond.stop(problem, solution, eval) {
            self.op.mutate(problem, solution, eval)?;
        }
        Ok(())
    }
}

impl<T, F, P, S, E> Search<P, S, E> for RepeatUntil<T, F>
where
    T: Search<P, S, E, Output = ()>,
    F: Stop<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        while !self.cond.stop(problem, solution, eval) {
            self.op.search(problem, solution, eval)?;
        }
        Ok(())
    }
}
