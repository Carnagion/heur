use alloc::{boxed::Box, vec::Vec};

use heur_core::{
    eval::Eval,
    op::{Hint, Ignore, Operator, Unwrapped},
    solution::Population,
};

mod elitist;
pub use elitist::ElitistInserter;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Insert<P, S, E>: Operator<P, S, E, Vec<S::Individual>>
where
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error>;
}

impl<T, P, S, E> Insert<P, S, E> for &mut T
where
    T: Insert<P, S, E> + ?Sized,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, problem, eval, combined)
    }
}

impl<T, P, S, E> Insert<P, S, E> for Box<T>
where
    T: Insert<P, S, E> + ?Sized,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, problem, eval, combined)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Insert<P, S, E> for either::Either<L, R>
where
    L: Insert<P, S, E>,
    R: Insert<P, S, E, Output = L::Output, Error = L::Error>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.insert(population, problem, eval, combined),
            Self::Right(right) => right.insert(population, problem, eval, combined),
        }
    }
}

impl<T, P, S, E> Insert<P, S, E> for Ignore<T>
where
    T: Insert<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.as_mut().insert(population, problem, eval, combined)
    }
}

impl<T, P, S, E> Insert<P, S, E> for Unwrapped<T>
where
    T: Insert<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.as_mut()
            .insert(population, problem, eval, combined)
            .unwrap();
        Ok(())
    }
}

impl<T, P, S, E> Insert<P, S, E> for Hint<T, P, S, E, Vec<S::Individual>>
where
    T: Insert<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.as_mut().insert(population, problem, eval, combined)
    }
}
