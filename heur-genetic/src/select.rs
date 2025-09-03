use alloc::{boxed::Box, vec::Vec};

use heur_core::{Problem, op::Operator, solution::Population};

mod tournament;
pub use tournament::{TournamentSelectError, TournamentSelector};

mod elitist;
pub use elitist::ElitistSelector;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Select<P, S>: Operator<P, S, Output = Vec<P::Individual>>
where
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<Vec<P::Individual>, Self::Error>;

    fn select_into(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        *selected = self.select(population, eval, problem)?;
        Ok(())
    }
}

impl<T, P, S> Select<P, S> for &mut T
where
    T: Select<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        T::select(self, population, eval, problem)
    }

    fn select_into(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, eval, problem, selected)
    }
}

impl<T, P, S> Select<P, S> for Box<T>
where
    T: Select<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        T::select(self, population, eval, problem)
    }

    fn select_into(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, eval, problem, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Select<P, S> for either::Either<L, R>
where
    L: Select<P, S>,
    R: Select<P, S, Error = L::Error>,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.select(population, eval, problem),
            Self::Right(right) => right.select(population, eval, problem),
        }
    }

    fn select_into(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut Vec<P::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.select_into(population, eval, problem, selected),
            Self::Right(right) => right.select_into(population, eval, problem, selected),
        }
    }
}
