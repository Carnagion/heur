use crate::eval::Eval;

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

impl<S, P, E> Accept<S, P, E> for Improving
where
    E: Eval<S, P>,
{
    #[inline]
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        eval.eval(solution, problem) > eval.eval(prev_solution, problem)
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

impl<S, P, E> Accept<S, P, E> for NonWorsening
where
    E: Eval<S, P>,
{
    #[inline]
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        eval.eval(solution, problem) >= eval.eval(prev_solution, problem)
    }
}
