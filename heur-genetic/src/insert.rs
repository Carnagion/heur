use alloc::boxed::Box;

use heur_core::{Problem, op::Operator, solution::Population};

use super::VecPopulation;

mod elitist;
pub use elitist::ElitistInserter;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Insert<P>: Operator<P, VecPopulation<P>, Output = ()>
where
    P: Problem<Solution: Population>,
{
    fn insert(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        combined: VecPopulation<P>,
    ) -> Result<(), Self::Error>;
}

impl<T, P> Insert<P> for &mut T
where
    T: Insert<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn insert(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        combined: VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, eval, problem, combined)
    }
}

impl<T, P> Insert<P> for Box<T>
where
    T: Insert<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn insert(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        combined: VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, eval, problem, combined)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Insert<P> for either::Either<L, R>
where
    L: Insert<P>,
    R: Insert<P, Error = L::Error>,
    P: Problem<Solution: Population>,
{
    fn insert(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        combined: VecPopulation<P>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.insert(population, eval, problem, combined),
            Self::Right(right) => right.insert(population, eval, problem, combined),
        }
    }
}
