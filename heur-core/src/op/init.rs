use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Optimize,
    eval::Eval,
    solution::{Population, Solution},
};

use super::Operator;

mod from_value;
pub use from_value::{FromDefault, FromIndividual, FromPopulation};

mod from_solver;
pub use from_solver::FromSolver;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Init<P, S, E>: Operator<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        *solution = self.init(problem, eval)?;
        Ok(())
    }
}

impl<T, P, S, E> Init<P, S, E> for &mut T
where
    T: Init<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, problem, eval)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S, E> Init<P, S, E> for Box<T>
where
    T: Init<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Init<P, S, E> for either::Either<L, R>
where
    L: Init<P, S, E>,
    R: Init<P, S, E, Output = L::Output, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.init(problem, eval),
            Self::Right(right) => right.init(problem, eval),
        }
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.init_into(solution, problem, eval),
            Self::Right(right) => right.init_into(solution, problem, eval),
        }
    }
}

pub fn from_individual<S>(individual: S) -> FromIndividual<S>
where
    S: Clone,
{
    FromIndividual(individual)
}

pub fn from_population<S>(population: S) -> FromPopulation<S>
where
    S: Population + Clone,
{
    FromPopulation(population)
}

pub fn from_default<S>() -> FromDefault<S>
where
    S: Solution + Default,
{
    FromDefault(PhantomData)
}

pub fn from_solver<P, S, E, T>(solver: T) -> FromSolver<T>
where
    T: Optimize<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    FromSolver(solver)
}
