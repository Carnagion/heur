use std::mem;

use super::Stop;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Iterations(pub usize);

impl Iterations {
    #[inline]
    #[must_use]
    pub fn new(iters: usize) -> Self {
        Self(iters)
    }
}

impl<S, P, E> Stop<S, P, E> for Iterations {
    #[inline]
    #[must_use]
    fn stop(&mut self, _solution: &S, _problem: &P, _eval: &mut E) -> bool {
        let remaining_iters = self.0.saturating_sub(1);
        let iters = mem::replace(&mut self.0, remaining_iters);
        iters == 0
    }
}
