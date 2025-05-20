use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    Problem,
    eval::Eval,
    solution::{Individual, IterMut, Population, Solution},
};

use super::Operator;

// TODO: 1. Manually implement common traits
//       2. Do we really need this combinator?
#[must_use]
pub struct ForEach<T, P> {
    op: T,
    marker: PhantomData<fn() -> P>,
}

impl<T, P> Operator<P> for ForEach<T, P>
where
    T: Operator<AsIndividual<P>, Output = ()>,
    P: Problem,
    P::Solution: Population + for<'a> IterMut<'a, Item = <P::Solution as Solution>::Individual>,
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
        let eval = AsIndividual::from_mut(eval);
        let problem = AsIndividual::from_ref(problem);
        population
            .iter_mut()
            .map(Individual::from_mut)
            .try_for_each(|solution| self.op.apply(solution, eval, problem, ()))
    }
}

pub fn for_each<T, P>(op: T) -> ForEach<T, P>
where
    T: Operator<AsIndividual<P>, Output = ()>,
    P: Problem,
    P::Solution: Population + for<'a> IterMut<'a, Item = <P::Solution as Solution>::Individual>,
{
    ForEach {
        op,
        marker: PhantomData,
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct AsIndividual<T>(T);

impl<T> AsIndividual<T> {
    #[must_use]
    pub fn from_ref(ptr: &T) -> &Self {
        // SAFETY: `AsIndividual<T>` is `repr(transparent)` and only contains a `T`.
        unsafe { &*(ptr as *const T as *const Self) }
    }

    #[must_use]
    pub fn from_mut(ptr: &mut T) -> &mut Self {
        // SAFETY: `AsIndividual<T>` is `repr(transparent)` and only contains a `T`.
        unsafe { &mut *(ptr as *mut T as *mut Self) }
    }
}

impl<P: Problem> Problem for AsIndividual<P> {
    type Solution = Individual<<P::Solution as Solution>::Individual>;

    type Eval = AsIndividual<P::Eval>;
}

impl<E, P> Eval<AsIndividual<P>> for AsIndividual<E>
where
    E: Eval<P>,
    P: Problem,
{
    type Objective = E::Objective;

    fn eval(
        &mut self,
        solution: &<<AsIndividual<P> as Problem>::Solution as Solution>::Individual,
        problem: &AsIndividual<P>,
    ) -> Self::Objective {
        self.0.eval(solution, problem)
    }
}

impl<T> Deref for AsIndividual<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AsIndividual<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
