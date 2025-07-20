use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Problem, solution::Solution};

use super::Eval;

type EvalFn<P, O> = fn(&<<P as Problem>::Solution as Solution>::Individual, &P) -> O;

#[must_use]
pub struct FromFn<P, O, F = EvalFn<P, O>> {
    pub(super) f: F,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, O)>,
}

impl<P, O, F> Eval<P> for FromFn<P, O, F>
where
    F: FnMut(&<P::Solution as Solution>::Individual, &P) -> O,
    P: Problem,
    O: PartialOrd,
{
    type Objective = O;

    fn eval(
        &mut self,
        solution: &<P::Solution as Solution>::Individual,
        problem: &P,
    ) -> Self::Objective {
        (self.f)(solution, problem)
    }
}

impl<P, O, F> Debug for FromFn<P, O, F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("FromFn").finish_non_exhaustive()
    }
}

impl<P, O, F: Copy> Copy for FromFn<P, O, F> {}

impl<P, O, F: Clone> Clone for FromFn<P, O, F> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: self.marker,
        }
    }
}

impl<P, O, F: Eq> Eq for FromFn<P, O, F> {}

impl<P, O, F: PartialEq> PartialEq for FromFn<P, O, F> {
    fn eq(&self, other: &Self) -> bool {
        self.f.eq(&other.f)
    }
}

impl<P, O, F: Hash> Hash for FromFn<P, O, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.f.hash(state);
    }
}
