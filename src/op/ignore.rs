use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Ignore<T>(pub(super) T);

impl<T, P, S, E, In> Operator<P, S, E, In> for Ignore<T>
where
    T: Operator<P, S, E, In>,
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
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(problem, solution, eval, input)?;
        Ok(())
    }
}

impl<T, P, S, E> Init<P, S, E> for Ignore<T>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.0.init(problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.0.init_into(problem, solution, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Ignore<T>
where
    T: Mutate<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.0.mutate(problem, solution, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Ignore<T>
where
    T: Search<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.0.search(problem, solution, eval)
    }
}
