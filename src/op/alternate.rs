use crate::{eval::Eval, solution::Solution};

use super::{mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Alternate<T, U> {
    pub(super) first: T,
    pub(super) second: U,
    pub(super) is_first: bool,
}

impl<T, U, P, S, E, In> Operator<P, S, E, In> for Alternate<T, U>
where
    T: Operator<P, S, E, In>,
    U: Operator<P, S, E, In, Output = T::Output, Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.is_first = !self.is_first;
        if self.is_first {
            self.first.apply(solution, problem, eval, input)
        } else {
            self.second.apply(solution, problem, eval, input)
        }
    }
}

impl<T, U, P, S, E> Mutate<P, S, E> for Alternate<T, U>
where
    T: Mutate<P, S, E>,
    U: Mutate<P, S, E, Output = T::Output, Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.is_first = !self.is_first;
        if self.is_first {
            self.first.mutate(solution, problem, eval)
        } else {
            self.second.mutate(solution, problem, eval)
        }
    }
}

impl<T, U, P, S, E> Search<P, S, E> for Alternate<T, U>
where
    T: Search<P, S, E>,
    U: Search<P, S, E, Output = T::Output, Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.is_first = !self.is_first;
        if self.is_first {
            self.first.search(solution, problem, eval)
        } else {
            self.second.search(solution, problem, eval)
        }
    }
}
