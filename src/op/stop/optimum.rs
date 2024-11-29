use crate::{eval::Eval, solution::Individual};

use super::Stop;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Optimum<O>(pub O);

impl<O> Optimum<O> {
    #[must_use]
    pub fn new(optimum: O) -> Self {
        Self(optimum)
    }
}

impl<P, S, E, O> Stop<P, Individual<S>, E> for Optimum<O>
where
    E: Eval<P, S, Objective = O>,
    O: Ord,
{
    fn stop(&mut self, solution: &Individual<S>, problem: &P, eval: &mut E) -> bool {
        eval.eval(solution, problem) >= self.0
    }
}
