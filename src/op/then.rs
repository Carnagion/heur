use crate::{
    eval::Eval,
    solution::{Solution, Solve},
};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Then<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U, P, S, E> Operator<P, S, E> for Then<T, U>
where
    T: Operator<P, S, E, Output = ()>,
    U: Operator<P, S, E, Output = (), Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.first.apply(solution, problem, eval, ())?;
        self.second.apply(solution, problem, eval, ())?;
        Ok(())
    }
}

impl<T, U, P, S, E> Init<P, S, E> for Then<T, U>
where
    T: Init<P, S, E, Output = ()>,
    U: Operator<P, S, E, Output = (), Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        let mut solution = self.first.init(problem, eval)?;
        self.second.apply(&mut solution, problem, eval, ())?;
        Ok(solution)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.first.init_into(solution, problem, eval)?;
        self.second.apply(solution, problem, eval, ())?;
        Ok(())
    }
}

impl<T, U, P, S, E> Mutate<P, S, E> for Then<T, U>
where
    T: Mutate<P, S, E, Output = ()>,
    U: Mutate<P, S, E, Output = (), Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.first.mutate(solution, problem, eval)?;
        self.second.mutate(solution, problem, eval)?;
        Ok(())
    }
}

impl<T, U, P, S, E> Search<P, S, E> for Then<T, U>
where
    T: Search<P, S, E, Output = ()>,
    U: Search<P, S, E, Output = (), Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.first.search(solution, problem, eval)?;
        self.second.search(solution, problem, eval)?;
        Ok(())
    }
}

impl<T, U, P, S, E> Solve<P, S, E> for Then<T, U>
where
    T: Init<P, S, E, Output = ()>,
    U: Operator<P, S, E, Output = (), Error = T::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = T::Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.init(problem, eval)
    }
}
