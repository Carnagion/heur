use crate::eval::Eval;

use super::Stop;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Optimum<O>(pub O);

impl<O> Optimum<O> {
    #[inline]
    #[must_use]
    pub fn new(optimum: O) -> Self {
        Self(optimum)
    }
}

impl<S, P, E, O> Stop<S, P, E> for Optimum<O>
where
    E: Eval<S, P, Objective = O>,
    O: Ord,
{
    #[inline]
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        eval.eval(solution, problem) >= self.0
    }
}
