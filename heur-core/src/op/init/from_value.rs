use core::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    Problem,
    op::Operator,
    solution::{Individual, Population},
};

use super::Init;

#[must_use]
pub struct FromIndividual<P, S> {
    pub(super) solution: S,
    pub(super) marker: PhantomData<fn() -> P>,
}

impl<P, S> Operator<P> for FromIndividual<P, S>
where
    P: Problem<Solution = Individual<S>>,
    S: Clone,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut Individual<S>,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, eval, problem)
    }
}

impl<P, S> Init<P> for FromIndividual<P, S>
where
    P: Problem<Solution = Individual<S>>,
    S: Clone,
{
    fn init(&mut self, _: &mut P::Eval, _: &P) -> Result<Individual<S>, Self::Error> {
        Ok(Individual(self.solution.clone()))
    }

    fn init_into(
        &mut self,
        solution: &mut Individual<S>,
        _: &mut P::Eval,
        _: &P,
    ) -> Result<(), Self::Error> {
        solution.clone_from(Individual::from_ref(&self.solution));
        Ok(())
    }
}

#[must_use]
pub struct FromPopulation<P, S> {
    pub(super) population: S,
    pub(super) marker: PhantomData<fn() -> P>,
}

impl<P, S> Operator<P> for FromPopulation<P, S>
where
    P: Problem<Solution = S>,
    S: Population + Clone,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(population, eval, problem)
    }
}

impl<P, S> Init<P> for FromPopulation<P, S>
where
    P: Problem<Solution = S>,
    S: Population + Clone,
{
    fn init(&mut self, _: &mut P::Eval, _: &P) -> Result<S, Self::Error> {
        Ok(self.population.clone())
    }

    fn init_into(&mut self, population: &mut S, _: &mut P::Eval, _: &P) -> Result<(), Self::Error> {
        population.clone_from(&self.population);
        Ok(())
    }
}

impl<S: Debug, P> Debug for FromIndividual<P, S> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FromIndividual")
            .field("solution", &self.solution)
            .finish_non_exhaustive()
    }
}

impl<S: Copy, P> Copy for FromIndividual<P, S> {}

impl<S: Clone, P> Clone for FromIndividual<P, S> {
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            marker: PhantomData,
        }
    }
}

impl<S: Eq, P> Eq for FromIndividual<P, S> {}

impl<S: PartialEq, P> PartialEq for FromIndividual<P, S> {
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution
    }
}

impl<S: Hash, P> Hash for FromIndividual<P, S> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.solution.hash(state);
    }
}

impl<S: Debug, P> Debug for FromPopulation<P, S> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FromPopulation")
            .field("population", &self.population)
            .finish_non_exhaustive()
    }
}

impl<S: Copy, P> Copy for FromPopulation<P, S> {}

impl<S: Clone, P> Clone for FromPopulation<P, S> {
    fn clone(&self) -> Self {
        Self {
            population: self.population.clone(),
            marker: PhantomData,
        }
    }
}

impl<S: Eq, P> Eq for FromPopulation<P, S> {}

impl<S: PartialEq, P> PartialEq for FromPopulation<P, S> {
    fn eq(&self, other: &Self) -> bool {
        self.population == other.population
    }
}

impl<S: Hash, P> Hash for FromPopulation<P, S> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.population.hash(state);
    }
}
