use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Optimize,
    Problem,
    solution::{Individual, Population, Solution},
};

use super::Operator;

mod from_value;
pub use from_value::{FromIndividual, FromPopulation};

mod from_solver;
pub use from_solver::FromSolver;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Init<P, S>: Operator<P, S, Output = ()>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error>;

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        *solution = self.init(eval, problem)?;
        Ok(())
    }
}

impl<T, P, S> Init<P, S> for &mut T
where
    T: Init<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S> Init<P, S> for Box<T>
where
    T: Init<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Init<P, S> for either::Either<L, R>
where
    L: Init<P, S>,
    R: Init<P, S, Error = L::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.init(eval, problem),
            Self::Right(right) => right.init(eval, problem),
        }
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.init_into(solution, eval, problem),
            Self::Right(right) => right.init_into(solution, eval, problem),
        }
    }
}

pub fn from_individual<P, S>(solution: S) -> FromIndividual<P, S>
where
    P: Problem<Individual = S>,
    S: Clone,
{
    FromIndividual {
        solution: Individual(solution),
        marker: PhantomData,
    }
}

pub fn from_population<P, S>(population: S) -> FromPopulation<P, S>
where
    P: Problem,
    S: Population<Individual = P::Individual> + Clone,
{
    FromPopulation {
        population,
        marker: PhantomData,
    }
}

pub fn from_solver<P, S, T>(solver: T) -> FromSolver<P, S, T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    FromSolver {
        solver,
        marker: PhantomData,
    }
}
