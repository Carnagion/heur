use crate::{eval::Eval, solution::Solution};

use super::{accept::Accept, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct AcceptIf<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, S, E, In> Operator<P, S, E, In> for AcceptIf<T, F>
where
    T: Operator<P, S, E, In>,
    F: Accept<P, S, E>,
    S: Solution + Clone,
    E: Eval<P, S::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let prev_solution = solution.clone();

        let output = self.op.apply(problem, solution, eval, input)?;
        if self.cond.accept(problem, solution, &prev_solution, eval) {
            Ok(Some(output))
        } else {
            *solution = prev_solution;
            Ok(None)
        }
    }
}

impl<T, F, P, S, E> Mutate<P, S, E> for AcceptIf<T, F>
where
    T: Mutate<P, S, E>,
    F: Accept<P, S, E>,
    S: Solution + Clone,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        let prev_solution = solution.clone();

        self.op.mutate(problem, solution, eval)?;
        if !self.cond.accept(problem, solution, &prev_solution, eval) {
            *solution = prev_solution;
        }

        Ok(())
    }
}

impl<T, F, P, S, E> Search<P, S, E> for AcceptIf<T, F>
where
    T: Search<P, S, E>,
    F: Accept<P, S, E>,
    S: Solution + Clone,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        let prev_solution = solution.clone();

        self.op.search(problem, solution, eval)?;
        if !self.cond.accept(problem, solution, &prev_solution, eval) {
            *solution = prev_solution;
        }

        Ok(())
    }
}
