use std::fmt::{self, Debug, Formatter};

use super::Eval;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromFn<F>(pub(super) F);

impl<F, P, S, O> Eval<P, S> for FromFn<F>
where
    F: FnMut(&P, &S) -> O,
    O: Ord,
{
    type Objective = O;

    #[inline]
    fn eval(&mut self, problem: &P, solution: &S) -> Self::Objective {
        (self.0)(problem, solution)
    }
}

impl<F> Debug for FromFn<F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("FromFn").finish()
    }
}
