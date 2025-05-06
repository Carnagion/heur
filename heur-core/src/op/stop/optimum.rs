use crate::{
    eval::Eval,
    solution::{Individual, Iter, Population},
};

use super::Stop;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Optimum<O>(pub O);

impl<O> Optimum<O> {
    pub fn new(optimum: O) -> Self {
        Self(optimum)
    }
}

impl<P, S, E> Stop<P, Individual<S>, E> for Optimum<E::Objective>
where
    E: Eval<P, S>,
{
    fn stop(&mut self, solution: &Individual<S>, problem: &P, eval: &mut E) -> bool {
        eval.eval(solution, problem) >= self.0
    }
}

impl<P, S, E> Stop<P, S, E> for Optimum<E::Objective>
where
    S: Population + for<'a> Iter<'a, Item = S::Individual>,
    E: Eval<P, S::Individual>,
{
    fn stop(&mut self, population: &S, problem: &P, eval: &mut E) -> bool {
        population
            .iter()
            .any(|solution| eval.eval(solution, problem) >= self.0)
    }
}
