use super::{accept::Accept, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AcceptIf<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, S, P, E, In> Operator<S, P, E, In> for AcceptIf<T, F>
where
    T: Operator<S, P, E, In>,
    F: Accept<S, P, E>,
    S: Clone,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let prev_solution = solution.clone();

        let output = self.op.apply(solution, problem, eval, input)?;
        if self.cond.accept(solution, &prev_solution, problem, eval) {
            Ok(Some(output))
        } else {
            *solution = prev_solution;
            Ok(None)
        }
    }
}

impl<T, F, S, P, E> Mutate<S, P, E> for AcceptIf<T, F>
where
    T: Mutate<S, P, E>,
    F: Accept<S, P, E>,
    S: Clone,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        let prev_solution = solution.clone();

        self.op.mutate(solution, problem, eval)?;
        if !self.cond.accept(solution, &prev_solution, problem, eval) {
            *solution = prev_solution;
        }

        Ok(())
    }
}

impl<T, F, S, P, E> Search<S, P, E> for AcceptIf<T, F>
where
    T: Search<S, P, E>,
    F: Accept<S, P, E>,
    S: Clone,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        let prev_solution = solution.clone();

        self.op.search(solution, problem, eval)?;
        if !self.cond.accept(solution, &prev_solution, problem, eval) {
            *solution = prev_solution;
        }

        Ok(())
    }
}
