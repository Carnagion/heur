use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Optimize, Problem, op::Operator};

use super::Init;

#[must_use]
pub struct FromSolver<P, T> {
    pub(super) solver: T,
    pub(super) marker: PhantomData<fn() -> P>,
}

impl<P, T> Operator<P> for FromSolver<P, T>
where
    T: Optimize<P>,
    P: Problem,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, eval, problem)
    }
}

impl<P, T> Init<P> for FromSolver<P, T>
where
    T: Optimize<P>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.solver.optimize(eval, problem)
    }
}

impl<T: Debug, P> Debug for FromSolver<P, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FromSolver")
            .field("solver", &self.solver)
            .finish_non_exhaustive()
    }
}

impl<T: Copy, P> Copy for FromSolver<P, T> {}

impl<T: Clone, P> Clone for FromSolver<P, T> {
    fn clone(&self) -> Self {
        Self {
            solver: self.solver.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Eq, P> Eq for FromSolver<P, T> {}

impl<T: PartialEq, P> PartialEq for FromSolver<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.solver == other.solver
    }
}

impl<T: Hash, P> Hash for FromSolver<P, T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.solver.hash(state);
    }
}
