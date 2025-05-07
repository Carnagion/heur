use crate::{eval::Eval, solution::Solution};

use super::{Operator, init::Init, mutate::Mutate, search::Search};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Ignore<T>(pub(super) T);

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

impl<T, P, S, E, In> Operator<P, S, E, In> for Ignore<T>
where
    T: Operator<P, S, E, In>,
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
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(solution, problem, eval, input)?;
        Ok(())
    }
}

impl<T, P, S, E> Init<P, S, E> for Ignore<T>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.0.init(problem, eval)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Ignore<T>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.mutate(solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Ignore<T>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.search(solution, problem, eval)
    }
}
