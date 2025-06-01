use core::marker::PhantomData;

use crate::{
    Problem,
    solution::{Individual, IterMut, Population, Reencoded, Solution},
};

use super::Operator;

// TODO: 1. Manually implement common traits
//       2. Do we really need this combinator?
#[must_use]
pub struct ForEach<T, P> {
    op: T,
    marker: PhantomData<fn() -> P>,
}

impl<T, P, S> Operator<P> for ForEach<T, P>
where
    T: Operator<Reencoded<P, Individual<S::Individual>>, Output = ()>,
    P: Problem<Solution = S>,
    S: Population + for<'a> IterMut<'a, Item = <P::Solution as Solution>::Individual>,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        let eval = Reencoded::from_mut(eval);
        let problem = Reencoded::from_ref(problem);
        population
            .iter_mut()
            .map(Individual::from_mut)
            .try_for_each(|solution| self.op.apply(solution, eval, problem, ()))
    }
}

pub fn for_each<T, P, S>(op: T) -> ForEach<T, P>
where
    T: Operator<Reencoded<P, Individual<S::Individual>>, Output = ()>,
    P: Problem<Solution = S>,
    S: Population + for<'a> IterMut<'a, Item = <P::Solution as Solution>::Individual>,
{
    ForEach {
        op,
        marker: PhantomData,
    }
}
