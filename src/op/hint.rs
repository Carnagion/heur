use std::marker::PhantomData;

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Manually impl common traits
pub struct Hint<T, S, P, E, In> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) _marker: PhantomData<fn() -> (S, P, E, In)>,
}

impl<T, S, P, E, In> Operator<S, P, E, In> for Hint<T, S, P, E, In>
where
    T: Operator<S, P, E, In>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, problem, eval, input)
    }
}

impl<T, S, P, E> Init<S, P, E> for Hint<T, S, P, E, ()>
where
    T: Init<S, P, E>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, problem, eval)
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Hint<T, S, P, E, ()>
where
    T: Mutate<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval)
    }
}

impl<T, S, P, E> Search<S, P, E> for Hint<T, S, P, E, ()>
where
    T: Search<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(solution, problem, eval)
    }
}
