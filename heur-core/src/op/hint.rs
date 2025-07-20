use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::Problem;

use super::{Operator, init::Init};

#[must_use]
pub struct Hint<T, P, In = (), Out = (), Err = Infallible> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, In, Out, Err)>,
}

impl<T, P, In, Out, Err> Operator<P, In> for Hint<T, P, In, Out, Err>
where
    T: Operator<P, In, Output = Out, Error = Err>,
    P: Problem,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut <P as Problem>::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, eval, problem, input)
    }
}

impl<T, P, Err> Init<P> for Hint<T, P, (), (), Err>
where
    T: Init<P, Error = Err>,
    P: Problem,
    Err: Error,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.op.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, eval, problem)
    }
}

impl<T: Debug, P, In, Out, Err> Debug for Hint<T, P, In, Out, Err> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Hint")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T: Copy, P, In, Out, Err> Copy for Hint<T, P, In, Out, Err> {}

impl<T: Clone, P, In, Out, Err> Clone for Hint<T, P, In, Out, Err> {
    fn clone(&self) -> Self {
        Self {
            op: self.op.clone(),
            marker: self.marker,
        }
    }
}

impl<T: Eq, P, In, Out, Err> Eq for Hint<T, P, In, Out, Err> {}

impl<T: PartialEq, P, In, Out, Err> PartialEq for Hint<T, P, In, Out, Err> {
    fn eq(&self, other: &Self) -> bool {
        self.op.eq(&other.op)
    }
}

impl<T: Hash, P, In, Out, Err> Hash for Hint<T, P, In, Out, Err> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.op.hash(state);
    }
}
