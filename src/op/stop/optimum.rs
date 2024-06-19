use crate::{eval::Eval, solution::Individual};

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

impl<P, S, E, O> Stop<P, Individual<S>, E> for Optimum<O>
where
    E: Eval<P, S, Objective = O>,
    O: Ord,
{
    #[inline]
    fn stop(&mut self, problem: &P, solution: &Individual<S>, eval: &mut E) -> bool {
        eval.eval(problem, solution) >= self.0
    }
}
