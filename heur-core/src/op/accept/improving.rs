use crate::{eval::Eval, solution::Individual};

use super::Accept;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Improving;

impl Improving {
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, Individual<S>, E> for Improving
where
    E: Eval<P, S>,
{
    fn accept(
        &mut self,
        solution: &Individual<S>,
        prev_solution: &Individual<S>,
        problem: &P,
        eval: &mut E,
    ) -> bool {
        eval.eval(solution, problem) > eval.eval(prev_solution, problem)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct NonWorsening;

impl NonWorsening {
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, Individual<S>, E> for NonWorsening
where
    E: Eval<P, S>,
{
    fn accept(
        &mut self,
        solution: &Individual<S>,
        prev_solution: &Individual<S>,
        problem: &P,
        eval: &mut E,
    ) -> bool {
        eval.eval(solution, problem) >= eval.eval(prev_solution, problem)
    }
}
