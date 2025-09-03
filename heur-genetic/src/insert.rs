use alloc::{boxed::Box, vec::Vec};

use heur_core::{Problem, op::Operator, solution::Population};

mod elitist;
pub use elitist::ElitistInserter;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Insert<P, S>: Operator<P, S, Vec<P::Individual>, Output = ()>
where
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        combined: Vec<P::Individual>,
    ) -> Result<(), Self::Error>;
}

impl<T, P, S> Insert<P, S> for &mut T
where
    T: Insert<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        combined: Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, eval, problem, combined)
    }
}

impl<T, P, S> Insert<P, S> for Box<T>
where
    T: Insert<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        combined: Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, eval, problem, combined)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Insert<P, S> for either::Either<L, R>
where
    L: Insert<P, S>,
    R: Insert<P, S, Error = L::Error>,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        combined: Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.insert(population, eval, problem, combined),
            Self::Right(right) => right.insert(population, eval, problem, combined),
        }
    }
}
