use core::marker::PhantomData;

use alloc::boxed::Box;

use heur_core::{
    Problem,
    op::Operator,
    solution::{
        Population,
        reencode::{Reencoded, Reeval},
    },
};

use super::VecPopulation;

mod uniform;
pub use uniform::{UniformCrossover, UniformCrossoverError};

// TODO: Add `#[diagnostic::on_unimplemented]`
#[doc(alias = "Crossover")]
pub trait Combine<P>: Operator<P, VecPopulation<P>, Output = VecPopulation<P>>
where
    P: Problem<Solution: Population>,
{
    #[doc(alias = "crossover")]
    fn combine(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: VecPopulation<P>,
    ) -> Result<VecPopulation<P>, Self::Error>;
}

impl<T, P> Combine<P> for &mut T
where
    T: Combine<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn combine(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: VecPopulation<P>,
    ) -> Result<VecPopulation<P>, Self::Error> {
        T::combine(self, population, eval, problem, selected)
    }
}

impl<T, P> Combine<P> for Box<T>
where
    T: Combine<P> + ?Sized,
    P: Problem<Solution: Population>,
{
    fn combine(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: VecPopulation<P>,
    ) -> Result<VecPopulation<P>, Self::Error> {
        T::combine(self, population, eval, problem, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Combine<P> for either::Either<L, R>
where
    L: Combine<P>,
    R: Combine<P, Error = L::Error>,
    P: Problem<Solution: Population>,
{
    fn combine(
        &mut self,
        population: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: VecPopulation<P>,
    ) -> Result<VecPopulation<P>, Self::Error> {
        match self {
            Self::Left(left) => left.combine(population, eval, problem, selected),
            Self::Right(right) => right.combine(population, eval, problem, selected),
        }
    }
}

// TODO: Manually implement common traits
#[must_use]
pub struct OnCombined<T, P> {
    op: T,
    marker: PhantomData<fn() -> P>,
}

impl<T, P> Operator<P, VecPopulation<P>> for OnCombined<T, P>
where
    T: Operator<Reencoded<P, VecPopulation<P>>, Output = ()>,
    P: Problem<Solution: Population>,
{
    type Output = VecPopulation<P>;

    type Error = T::Error;

    fn apply(
        &mut self,
        _: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        mut combined: VecPopulation<P>,
    ) -> Result<Self::Output, Self::Error> {
        let eval = Reeval::from_mut(eval);
        let problem = Reencoded::from_ref(problem);
        self.op.apply(&mut combined, eval, problem, ())?;
        Ok(combined)
    }
}

pub fn on_combined<T, P>(op: T) -> OnCombined<T, P>
where
    T: Operator<Reencoded<P, VecPopulation<P>>, Output = ()>,
    P: Problem<Solution: Population>,
{
    OnCombined {
        op,
        marker: PhantomData,
    }
}
