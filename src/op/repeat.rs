use super::{mutate::Mutate, search::Search, stop::Stop, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Repeat<T> {
    pub(super) op: T,
    pub(super) times: usize,
}

impl<T, S, P, E, In> Operator<S, P, E, In> for Repeat<T>
where
    T: Operator<S, P, E, In, Output = In>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        for _ in 0..self.times {
            input = self.op.apply(solution, problem, eval, input)?;
        }
        Ok(input)
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Repeat<T>
where
    T: Mutate<S, P, E, Output = ()>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for _ in 0..self.times {
            self.op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

impl<T, S, P, E> Search<S, P, E> for Repeat<T>
where
    T: Search<S, P, E, Output = ()>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for _ in 0..self.times {
            self.op.search(solution, problem, eval)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RepeatUntil<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, S, P, E, In> Operator<S, P, E, In> for RepeatUntil<T, F>
where
    T: Operator<S, P, E, In, Output = In>,
    F: Stop<S, P, E>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        while !self.cond.stop(solution, problem, eval) {
            input = self.op.apply(solution, problem, eval, input)?;
        }
        Ok(input)
    }
}

impl<T, F, S, P, E> Mutate<S, P, E> for RepeatUntil<T, F>
where
    T: Mutate<S, P, E, Output = ()>,
    F: Stop<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        while !self.cond.stop(solution, problem, eval) {
            self.op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

impl<T, F, S, P, E> Search<S, P, E> for RepeatUntil<T, F>
where
    T: Search<S, P, E, Output = ()>,
    F: Stop<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        while !self.cond.stop(solution, problem, eval) {
            self.op.search(solution, problem, eval)?;
        }
        Ok(())
    }
}
