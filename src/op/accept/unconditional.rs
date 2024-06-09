use super::Accept;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Always;

impl Always {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<S, P, E> Accept<S, P, E> for Always {
    #[inline]
    #[must_use]
    fn accept(&mut self, _solution: &S, _prev_solution: &S, _problem: &P, _eval: &mut E) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Never;

impl Never {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<S, P, E> Accept<S, P, E> for Never {
    #[inline]
    #[must_use]
    fn accept(&mut self, _solution: &S, _prev_solution: &S, _problem: &P, _eval: &mut E) -> bool {
        false
    }
}
