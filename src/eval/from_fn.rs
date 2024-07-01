use std::fmt::{self, Debug, Formatter};

use super::Eval;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromFn<F>(pub(super) F);

impl<F, P, S, O> Eval<P, S> for FromFn<F>
where
    F: FnMut(&S, &P) -> O,
    O: Ord,
{
    type Objective = O;

    #[inline]
    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        (self.0)(solution, problem)
    }
}

impl<F> Debug for FromFn<F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("FromFn").finish()
    }
}
