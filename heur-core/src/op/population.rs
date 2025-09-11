//! Population-based (a.k.a. multipoint) operators.

use core::marker::PhantomData;

use crate::{
    Problem,
    solution::{Individual, IterMut, Population},
};

use super::Operator;

// TODO: Manually implement common traits
/// An operator that calls a provided single-point (i.e. individual solution) operator on each individual solution in a
/// population of solutions.
///
/// This type is created by [`population::for_each`](for_each()). See its documentation for more details.
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

/// Calls the provided single-point (i.e. individual solution) operator on each individual solution in a population of solutions.
///
/// The operator provided must have a solution type of [`Individual<T>`]. Additionally, it must take no input and produce no
/// output, i.e. `In` = `()` and [`Output`](Operator::Output) = `()`. Iteration stops when all individuals in the solution have
/// been covered, or when an error is returned; i.e. applying `population::for_each(a)` to a solution is equivalent to the
/// following:
/// ```
/// # use heur_core::{op::Operator, solution::{Individual, IterMut, Population}, Problem};
/// #
/// # fn test<T, P, S>(
/// #     mut a: T,
/// #     solution: &mut S,
/// #     eval: &mut P::Eval,
/// #     problem: &P,
/// # ) -> Result<(), T::Error>
/// # where
/// #     T: Operator<P, Individual<P::Individual>, Output = ()>,
/// #     P: Problem,
/// #     S: Population<Individual = P::Individual> + for<'a> IterMut<'a, Item = P::Individual>,
/// # {
/// for indv in solution.iter_mut() {
///     let indv = Individual::from_mut(indv);
///     a.apply(indv, eval, problem, ())?;
/// }
/// Ok(())
/// # }
/// ```
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
