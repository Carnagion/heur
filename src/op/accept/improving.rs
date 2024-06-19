use crate::{eval::Eval, solution::Individual};

use super::Accept;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Improving;

impl Improving {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, Individual<S>, E> for Improving
where
    E: Eval<P, S>,
{
    #[inline]
    fn accept(
        &mut self,
        problem: &P,
        solution: &Individual<S>,
        prev_solution: &Individual<S>,
        eval: &mut E,
    ) -> bool {
        eval.eval(problem, solution) > eval.eval(problem, prev_solution)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NonWorsening;

impl NonWorsening {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, Individual<S>, E> for NonWorsening
where
    E: Eval<P, S>,
{
    #[inline]
    fn accept(
        &mut self,
        problem: &P,
        solution: &Individual<S>,
        prev_solution: &Individual<S>,
        eval: &mut E,
    ) -> bool {
        eval.eval(problem, solution) >= eval.eval(problem, prev_solution)
    }
}
