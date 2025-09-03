use crate::{Optimize, Problem, solution::Solution};

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Then<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U, P, S> Operator<P, S> for Then<T, U>
where
    T: Operator<P, S, Output = ()>,
    U: Operator<P, S, Output = (), Error = T::Error>,
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
        self.first.apply(solution, eval, problem, ())?;
        self.second.apply(solution, eval, problem, ())?;
        Ok(())
    }
}

impl<T, U, P, S> Init<P, S> for Then<T, U>
where
    T: Init<P, S>,
    U: Operator<P, S, Output = (), Error = T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        let mut solution = self.first.init(eval, problem)?;
        self.second.apply(&mut solution, eval, problem, ())?;
        Ok(solution)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.first.init_into(solution, eval, problem)?;
        self.second.apply(solution, eval, problem, ())?;
        Ok(())
    }
}

impl<T, U, P, S> Optimize<P, S> for Then<T, U>
where
    T: Init<P, S>,
    U: Operator<P, S, Output = (), Error = T::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = <Self as Operator<P, S>>::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.init(eval, problem)
    }
}
