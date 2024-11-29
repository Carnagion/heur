use crate::{
    eval::Eval,
    op::Operator,
    solution::{Solution, Solve},
};

use super::Init;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromSolver<T>(pub(super) T);

impl<T, P, S, E> Operator<P, S, E> for FromSolver<T>
where
    T: Solve<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, problem, eval)
    }
}

impl<T, P, S, E> Init<P, S, E> for FromSolver<T>
where
    T: Solve<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.0.solve(problem, eval)
    }
}
