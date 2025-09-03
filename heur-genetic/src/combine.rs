use core::marker::PhantomData;

use alloc::{boxed::Box, vec::Vec};

use heur_core::{Problem, op::Operator, solution::Population};

mod uniform;
pub use uniform::{UniformCrossover, UniformCrossoverError};

// TODO: Add `#[diagnostic::on_unimplemented]`
#[doc(alias = "Crossover")]
pub trait Combine<P, S>: Operator<P, S, Vec<P::Individual>, Output = Vec<P::Individual>>
where
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    #[doc(alias = "crossover")]
    fn combine(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: Vec<P::Individual>,
    ) -> Result<Vec<P::Individual>, Self::Error>;
}

impl<T, P, S> Combine<P, S> for &mut T
where
    T: Combine<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn combine(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: Vec<P::Individual>,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        T::combine(self, population, eval, problem, selected)
    }
}

impl<T, P, S> Combine<P, S> for Box<T>
where
    T: Combine<P, S> + ?Sized,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn combine(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: Vec<P::Individual>,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        T::combine(self, population, eval, problem, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Combine<P, S> for either::Either<L, R>
where
    L: Combine<P, S>,
    R: Combine<P, S, Error = L::Error>,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    fn combine(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: Vec<P::Individual>,
    ) -> Result<Vec<P::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.combine(population, eval, problem, selected),
            Self::Right(right) => right.combine(population, eval, problem, selected),
        }
    }
}

// TODO: Manually implement common traits
#[must_use]
pub struct OnCombined<T, P, S> {
    op: T,
    marker: PhantomData<fn() -> (P, S)>,
}

impl<T, P, S> Operator<P, S, Vec<P::Individual>> for OnCombined<T, P, S>
where
    T: Operator<P, Vec<P::Individual>, Output = ()>,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    type Output = Vec<P::Individual>;

    type Error = T::Error;

    fn apply(
        &mut self,
        _: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        mut combined: Vec<P::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(&mut combined, eval, problem, ())?;
        Ok(combined)
    }
}

pub fn on_combined<T, P, S>(op: T) -> OnCombined<T, P, S>
where
    T: Operator<P, Vec<P::Individual>, Output = ()>,
    P: Problem,
    S: Population<Individual = P::Individual>,
{
    OnCombined {
        op,
        marker: PhantomData,
    }
}
