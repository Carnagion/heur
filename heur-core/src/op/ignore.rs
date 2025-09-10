use crate::{Optimize, Problem, solution::Solution};

use super::{Operator, init::Init};

/// An operator that ignores the output produced by its inner operator, producing `()` instead.
///
/// This type is created by [`Operator::ignore`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Ignore<T>(pub(super) T);

impl<T, P, S, In> Operator<P, S, In> for Ignore<T>
where
    T: Operator<P, S, In>,
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
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let _ = self.0.apply(solution, eval, problem, input)?;
        Ok(())
    }
}

impl<T, P, S> Init<P, S> for Ignore<T>
where
    T: Init<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.0.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, eval, problem)
    }
}

impl<T, P, S> Optimize<P, S> for Ignore<T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.0.optimize(eval, problem)
    }
}

impl<T> AsRef<T> for Ignore<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Ignore<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
