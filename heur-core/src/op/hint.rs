use core::{
    convert::Infallible,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Optimize, Problem, solution::Solution};

use super::{Operator, init::Init};

#[must_use]
pub struct Hint<T, P, S, In = (), Out = (), Err = Infallible> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, S, In, Out, Err)>,
}

impl<T, P, S, In> Operator<P, S, In> for Hint<T, P, S, In, T::Output, T::Error>
where
    T: Operator<P, S, In>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, eval, problem, input)
    }
}

impl<T, P, S> Init<P, S> for Hint<T, P, S, (), (), T::Error>
where
    T: Init<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.op.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, eval, problem)
    }
}

impl<T, P, S> Optimize<P, S> for Hint<T, P, S, (), (), T::Error>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.op.optimize(eval, problem)
    }
}

impl<T: Debug, P, S, In, Out, Err> Debug for Hint<T, P, S, In, Out, Err> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Hint")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T: Copy, P, S, In, Out, Err> Copy for Hint<T, P, S, In, Out, Err> {}

impl<T: Clone, P, S, In, Out, Err> Clone for Hint<T, P, S, In, Out, Err> {
    fn clone(&self) -> Self {
        Self {
            op: self.op.clone(),
            marker: self.marker,
        }
    }
}

impl<T: Eq, P, S, In, Out, Err> Eq for Hint<T, P, S, In, Out, Err> {}

impl<T: PartialEq, P, S, In, Out, Err> PartialEq for Hint<T, P, S, In, Out, Err> {
    fn eq(&self, other: &Self) -> bool {
        self.op.eq(&other.op)
    }
}

impl<T: Hash, P, S, In, Out, Err> Hash for Hint<T, P, S, In, Out, Err> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.op.hash(state);
    }
}
