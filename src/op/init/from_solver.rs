use crate::solve::Solve;

use super::{Init, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FromSolver<T>(pub(super) T);

impl<T, S, P, E> Operator<S, P, E> for FromSolver<T>
where
    T: Solve<S, P, E>,
{
    type Output = ();

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        *solution = self.0.solve(problem, eval)?;
        Ok(())
    }
}

impl<T, S, P, E> Init<S, P, E> for FromSolver<T>
where
    T: Solve<S, P, E>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.0.solve(problem, eval)
    }
}
