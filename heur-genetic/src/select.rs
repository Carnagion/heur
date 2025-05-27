use alloc::boxed::Box;

use heur_core::{Problem, op::Operator, solution::Population};

use super::VecPopulation;

mod tournament;
pub use tournament::{TournamentSelectError, TournamentSelector};

mod elitist;
pub use elitist::ElitistSelector;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Select<P>: Operator<P, Output = VecPopulation<P>>
where
    P: Problem<Solution: Population>,
{
    fn select(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<VecPopulation<P>, Self::Error>;

    fn select_into(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        *selected = self.select(population, eval, problem)?;
        Ok(())
    }
}

impl<T, P> Select<P> for &mut T
where
    T: Select<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn select(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<VecPopulation<P>, Self::Error> {
        T::select(self, population, eval, problem)
    }

    fn select_into(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, eval, problem, selected)
    }
}

impl<T, P> Select<P> for Box<T>
where
    T: Select<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn select(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<VecPopulation<P>, Self::Error> {
        T::select(self, population, eval, problem)
    }

    fn select_into(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, eval, problem, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Select<P> for either::Either<L, R>
where
    L: Select<P>,
    R: Select<P, Error = L::Error>,
    P: Problem<Solution: Population>,
{
    fn select(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<VecPopulation<P>, Self::Error> {
        match self {
            Self::Left(left) => left.select(population, eval, problem),
            Self::Right(right) => right.select(population, eval, problem),
        }
    }

    fn select_into(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.select_into(population, eval, problem, selected),
            Self::Right(right) => right.select_into(population, eval, problem, selected),
        }
    }
}
