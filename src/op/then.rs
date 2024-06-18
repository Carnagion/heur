use crate::{eval::Eval, solution::Solution, solve::Solve};

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
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.first.apply(problem, solution, eval, ())?;
        self.second.apply(problem, solution, eval, ())?;
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
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        let mut solution = self.first.init(problem, eval)?;
        self.second.apply(problem, &mut solution, eval, ())?;
        Ok(solution)
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.first.init_into(problem, solution, eval)?;
        self.second.apply(problem, solution, eval, ())?;
        Ok(())
    }
}

impl<T, U, P, S, E> Mutate<P, S, E> for Then<T, U>
where
    T: Mutate<P, S, E, Output = ()>,
    U: Mutate<P, S, E, Output = (), Error = T::Error>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.first.mutate(problem, solution, eval)?;
        self.second.mutate(problem, solution, eval)?;
        Ok(())
    }
}

impl<T, U, P, S, E> Search<P, S, E> for Then<T, U>
where
    T: Search<P, S, E, Output = ()>,
    U: Search<P, S, E, Output = (), Error = T::Error>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.first.search(problem, solution, eval)?;
        self.second.search(problem, solution, eval)?;
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

    #[inline]
    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.init(problem, eval)
    }
}
