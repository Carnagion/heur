use crate::{
    eval::Eval,
    op::{mutate::Mutate, search::Search, Operator},
    solution::{Individual, Population},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct ForEach<T>(pub(super) T);

impl<T, P, S, E, In> Operator<P, S, E, In> for ForEach<T>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
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
                self.0.apply(solution, problem, eval, input)
            })
    }
}

impl<T, P, S, E> Mutate<P, S, E> for ForEach<T>
where
    T: Mutate<P, Individual<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.iter_mut().map(Individual::from_mut) {
            self.0.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for ForEach<T>
where
    T: Search<P, Individual<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, population: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population.iter_mut().map(Individual::from_mut) {
            self.0.search(solution, problem, eval)?;
        }
        Ok(())
    }
}
