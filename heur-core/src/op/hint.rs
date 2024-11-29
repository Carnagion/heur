use std::marker::PhantomData;

use crate::{eval::Eval, solution::Solution};

use super::{init::Init, mutate::Mutate, search::Search, Operator};

// TODO: Manually impl common traits
#[must_use]
pub struct Hint<T, P, S, E, In = ()> {
    pub op: T,
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

impl<T, P, S, E> Init<P, S, E> for Hint<T, P, S, E>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Hint<T, P, S, E>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Hint<T, P, S, E>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(solution, problem, eval)
    }
}
