use crate::{Optimize, Problem};

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Then<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U, P> Operator<P> for Then<T, U>
where
    T: Operator<P, Output = ()>,
    U: Operator<P, Output = (), Error = T::Error>,
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
        self.first.apply(solution, eval, problem, ())?;
        self.second.apply(solution, eval, problem, ())?;
        Ok(())
    }
}

impl<T, U, P> Init<P> for Then<T, U>
where
    T: Init<P, Output = ()>,
    U: Operator<P, Output = (), Error = T::Error>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        let mut solution = self.first.init(eval, problem)?;
        self.second.apply(&mut solution, eval, problem, ())?;
        Ok(solution)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.first.init_into(solution, eval, problem)?;
        self.second.apply(solution, eval, problem, ())?;
        Ok(())
    }
}

impl<T, U, P> Optimize<P> for Then<T, U>
where
    T: Init<P, Output = ()>,
    U: Operator<P, Output = (), Error = T::Error>,
    P: Problem,
{
    type Error = <Self as Operator<P>>::Error;

    fn optimize(
        &mut self,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<<P as Problem>::Solution, Self::Error> {
        self.init(eval, problem)
    }
}
