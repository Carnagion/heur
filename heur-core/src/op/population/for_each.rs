use std::marker::PhantomData;

use crate::{
    eval::Eval,
    op::{mutate::Mutate, search::Search, Operator},
    solution::{Individual, Population},
};

// TODO: Manually impl common traits
#[must_use]
pub struct ForEach<T, P, S, E, In = ()> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) _marker: PhantomData<fn() -> (P, S, E, In)>,
}

impl<T, P, S, E, In> Operator<P, S, E, In> for ForEach<T, P, S, E, In>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population,
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
            .iter_mut()
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
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.iter_mut().map(Individual::from_mut) {
            self.op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for ForEach<T, P, S, E>
where
    T: Search<P, Individual<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.iter_mut().map(Individual::from_mut) {
            self.op.search(solution, problem, eval)?;
        }
        Ok(())
    }
}
