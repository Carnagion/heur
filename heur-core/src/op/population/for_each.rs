use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    eval::Eval,
    op::{Operator, mutate::Mutate, search::Search},
    solution::{Individual, Population},
};

#[must_use]
pub struct ForEach<T, P, S, E, In = ()> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, S, E, In)>,
}

impl<T, P, S, E, In> Debug for ForEach<T, P, S, E, In>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ForEach")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T, P, S, E, In> Copy for ForEach<T, P, S, E, In> where T: Copy {}

impl<T, P, S, E, In> Clone for ForEach<T, P, S, E, In>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            op: self.op.clone(),
            marker: self.marker,
        }
    }
}

impl<T, P, S, E, In> Eq for ForEach<T, P, S, E, In> where T: Eq {}

impl<T, P, S, E, In> PartialEq for ForEach<T, P, S, E, In>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
    }
}

impl<T, P, S, E, In> Hash for ForEach<T, P, S, E, In>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.op.hash(state);
        self.marker.hash(state);
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for ForEach<T, P, S, E, In>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        population
            .into_iter()
            .map(Individual::from_mut)
            .try_fold(input, |input, solution| {
                self.op.apply(solution, problem, eval, input)
            })
    }
}

impl<T, P, S, E> Mutate<P, S, E> for ForEach<T, P, S, E>
where
    T: Mutate<P, Individual<S::Individual>, E, Output = ()>,
    S: Population,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.into_iter().map(Individual::from_mut) {
            self.op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for ForEach<T, P, S, E>
where
    T: Search<P, Individual<S::Individual>, E, Output = ()>,
    S: Population,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.into_iter().map(Individual::from_mut) {
            self.op.search(solution, problem, eval)?;
        }
        Ok(())
    }
}
