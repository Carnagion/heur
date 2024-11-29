use std::mem;

use crate::{eval::Eval, solution::Solution};

use super::Stop;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Iterations(pub usize);

impl Iterations {
    
    #[must_use]
    pub fn new(iters: usize) -> Self {
        Self(iters)
    }
}

impl<P, S, E> Stop<P, S, E> for Iterations
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn stop(&mut self, _solution: &S, _problem: &P, _eval: &mut E) -> bool {
        let remaining_iters = self.0.saturating_sub(1);
        let iters = mem::replace(&mut self.0, remaining_iters);
        iters == 0
    }
}
