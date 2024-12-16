use crate::{eval::Eval, solution::Solution};

use super::Accept;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Always;

impl Always {
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, S, E> for Always
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn accept(&mut self, _solution: &S, _prev_solution: &S, _problem: &P, _eval: &mut E) -> bool {
        true
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Never;

impl Never {
    pub fn new() -> Self {
        Self
    }
}

impl<P, S, E> Accept<P, S, E> for Never
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn accept(&mut self, _solution: &S, _prev_solution: &S, _problem: &P, _eval: &mut E) -> bool {
        false
    }
}
