use crate::solve::Solve;

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Then<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U, S, P, E> Operator<S, P, E> for Then<T, U>
where
    T: Operator<S, P, E>,
    U: Operator<S, P, E, Error = T::Error>,
{
    type Output = ();

    type Error = T::Error;

    #[inline]
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

impl<T, U, S, P, E> Init<S, P, E> for Then<T, U>
where
    T: Init<S, P, E>,
    U: Operator<S, P, E, Error = T::Error>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        let mut solution = self.first.init(problem, eval)?;
        self.second.apply(&mut solution, problem, eval, ())?;
        Ok(solution)
    }

    #[inline]
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

impl<T, U, S, P, E> Mutate<S, P, E> for Then<T, U>
where
    T: Mutate<S, P, E>,
    U: Mutate<S, P, E, Error = T::Error>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.first.mutate(solution, problem, eval)?;
        self.second.mutate(solution, problem, eval)?;
        Ok(())
    }
}

impl<T, U, S, P, E> Search<S, P, E> for Then<T, U>
where
    T: Search<S, P, E>,
    U: Search<S, P, E, Error = T::Error>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.first.search(solution, problem, eval)?;
        self.second.search(solution, problem, eval)?;
        Ok(())
    }
}

impl<T, U, S, P, E> Solve<S, P, E> for Then<T, U>
where
    T: Init<S, P, E>,
    U: Operator<S, P, E, Error = T::Error>,
{
    type Error = T::Error;

    #[inline]
    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.init(problem, eval)
    }
}
