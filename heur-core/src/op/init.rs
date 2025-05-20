use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Optimize,
    Problem,
    solution::{Individual, Population},
};

use super::Operator;

mod from_value;
pub use from_value::{FromIndividual, FromPopulation};

mod from_solver;
pub use from_solver::FromSolver;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Init<P: Problem>: Operator<P> {
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error>;

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        *solution = self.init(eval, problem)?;
        Ok(())
    }
}

impl<T, P> Init<P> for &mut T
where
    T: Init<P> + ?Sized,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Init<P> for Box<T>
where
    T: Init<P> + ?Sized,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Init<P> for either::Either<L, R>
where
    L: Init<P>,
    R: Init<P, Output = L::Output, Error = L::Error>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        match self {
            Self::Left(left) => left.init(eval, problem),
            Self::Right(right) => right.init(eval, problem),
        }
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
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
    P: Problem<Solution = Individual<S>>,
    S: Clone,
{
    FromIndividual {
        solution,
        marker: PhantomData,
    }
}

pub fn from_population<P, S>(population: S) -> FromPopulation<P, S>
where
    P: Problem<Solution = S>,
    S: Population + Clone,
{
    FromPopulation {
        population,
        marker: PhantomData,
    }
}

pub fn from_solver<P, T>(solver: T) -> FromSolver<P, T>
where
    T: Optimize<P>,
    P: Problem,
{
    FromSolver {
        solver,
        marker: PhantomData,
    }
}
