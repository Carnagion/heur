use core::marker::PhantomData;

use crate::{
    Problem,
    solution::{Individual, IterMut, Population},
};

use super::Operator;

// TODO: Manually implement common traits
#[must_use]
pub struct ForEach<T, P, S> {
    op: T,
    marker: PhantomData<fn() -> (P, S)>,
}

impl<T, P, S> Operator<P, S> for ForEach<T, P, S>
where
    T: Operator<P, Individual<P::Individual>, Output = ()>,
    P: Problem,
    S: Population<Individual = P::Individual> + for<'a> IterMut<'a, Item = P::Individual>,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        population
            .iter_mut()
            .map(Individual::from_mut)
            .try_for_each(|solution| self.op.apply(solution, eval, problem, ()))
    }
}

pub fn for_each<T, P, S>(op: T) -> ForEach<T, P, S>
where
    T: Operator<P, Individual<P::Individual>, Output = ()>,
    P: Problem,
    S: Population<Individual = P::Individual> + for<'a> IterMut<'a, Item = P::Individual>,
{
    ForEach {
        op,
        marker: PhantomData,
    }
}
