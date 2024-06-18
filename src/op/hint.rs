use std::marker::PhantomData;

use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Manually impl common traits
#[must_use]
pub struct Hint<T, P, S, E, In = ()> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) _marker: PhantomData<fn() -> (P, S, E, In)>,
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Hint<T, P, S, E, In>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(problem, solution, eval, input)
    }
}

impl<T, P, S, E> Init<P, S, E> for Hint<T, P, S, E>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op.init_into(problem, solution, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Hint<T, P, S, E, ()>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(problem, solution, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Hint<T, P, S, E, ()>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(problem, solution, eval)
    }
}
