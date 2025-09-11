use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Optimize, Problem, op::Operator, solution::Solution};

use super::Init;

/// An operator that initialises solutions by running a solver that implements [`Optimize`].
///
/// This type is created by [`init::from_solver`](super::from_solver()). See its documentation for more details.
#[must_use]
pub struct FromSolver<P, S, T> {
    pub(super) solver: T,
    pub(super) marker: PhantomData<fn() -> (P, S)>,
}

impl<P, S, T> Operator<P, S> for FromSolver<P, S, T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, eval, problem)
    }
}

impl<P, S, T> Init<P, S> for FromSolver<P, S, T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.solver.optimize(eval, problem)
    }
}

impl<T: Debug, P, S> Debug for FromSolver<P, S, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FromSolver")
            .field("solver", &self.solver)
            .finish_non_exhaustive()
    }
}

impl<T: Copy, P, S> Copy for FromSolver<P, S, T> {}

impl<T: Clone, P, S> Clone for FromSolver<P, S, T> {
    fn clone(&self) -> Self {
        Self {
            solver: self.solver.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Eq, P, S> Eq for FromSolver<P, S, T> {}

impl<T: PartialEq, P, S> PartialEq for FromSolver<P, S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.solver == other.solver
    }
}

impl<T: Hash, P, S> Hash for FromSolver<P, S, T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.solver.hash(state);
    }
}
