use crate::solution::Evaluated;

use super::Eval;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Cached<T>(pub(super) T);

impl<T, P, S> Eval<P, Evaluated<S, T::Objective>> for Cached<T>
where
    T: Eval<P, S, Objective: Copy>,
{
    type Objective = T::Objective;

    
    fn eval(&mut self, solution: &Evaluated<S, T::Objective>, problem: &P) -> Self::Objective {
        solution.objective_or_eval(|solution| self.0.eval(solution, problem))
    }
}
