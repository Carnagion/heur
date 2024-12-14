use std::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    eval::Eval,
    op::Operator,
    solution::{Individual, Population, Solution},
};

use super::Init;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromIndividual<S>(pub(super) S);

impl<P, S, E> Operator<P, Individual<S>, E> for FromIndividual<S>
where
    S: Clone,
    E: Eval<P, S>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut Individual<S>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, problem, eval)
    }
}

impl<P, S, E> Init<P, Individual<S>, E> for FromIndividual<S>
where
    S: Clone,
    E: Eval<P, S>,
{
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<Individual<S>, Self::Error> {
        Ok(Individual::new(self.0.clone()))
    }

    fn init_into(
        &mut self,
        solution: &mut Individual<S>,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        solution.clone_from(Individual::from_ref(&self.0));
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FromPopulation<S>(pub(super) S);

impl<P, S, E> Operator<P, S, E> for FromPopulation<S>
where
    S: Population + Clone,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.init_into(solution, problem, eval)
    }
}

impl<P, S, E> Init<P, S, E> for FromPopulation<S>
where
    S: Population + Clone,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        Ok(self.0.clone())
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        solution.clone_from(&self.0);
        Ok(())
    }
}

#[must_use]
pub struct FromDefault<S>(pub(super) PhantomData<fn() -> S>);

impl<S> Debug for FromDefault<S> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("FromDefault").finish_non_exhaustive()
    }
}

impl<S> Default for FromDefault<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S> Copy for FromDefault<S> {}

impl<S> Clone for FromDefault<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Eq for FromDefault<S> {}

impl<S> PartialEq for FromDefault<S> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<S> Hash for FromDefault<S> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

impl<P, S, E> Operator<P, S, E> for FromDefault<S>
where
    S: Solution + Default,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut S,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        *solution = S::default();
        Ok(())
    }
}

impl<P, S, E> Init<P, S, E> for FromDefault<S>
where
    S: Solution + Default,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, _problem: &P, _eval: &mut E) -> Result<S, Self::Error> {
        Ok(S::default())
    }
}
