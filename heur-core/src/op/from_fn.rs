use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{Problem, solution::Solution};

use super::Operator;

type OperatorFn<P, S, In, Out, Err> =
    fn(&mut S, &mut <P as Problem>::Eval, &P, In) -> Result<Out, Err>;

#[must_use]
pub struct FromFn<P, S, In = (), Out = (), Err = Infallible, F = OperatorFn<P, S, In, Out, Err>> {
    pub(super) f: F,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, S, In, Out, Err)>,
}

impl<P, S, In, Out, Err, F> Operator<P, S, In> for FromFn<P, S, In, Out, Err, F>
where
    F: FnMut(&mut S, &mut P::Eval, &P, In) -> Result<Out, Err>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        (self.f)(solution, eval, problem, input)
    }
}

impl<P, S, In, Out, Err, F> Debug for FromFn<P, S, In, Out, Err, F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("FromFn").finish_non_exhaustive()
    }
}

impl<P, S, In, Out, Err, F: Copy> Copy for FromFn<P, S, In, Out, Err, F> {}

impl<P, S, In, Out, Err, F: Clone> Clone for FromFn<P, S, In, Out, Err, F> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: self.marker,
        }
    }
}

impl<P, S, In, Out, Err, F: Eq> Eq for FromFn<P, S, In, Out, Err, F> {}

impl<P, S, In, Out, Err, F: PartialEq> PartialEq for FromFn<P, S, In, Out, Err, F> {
    fn eq(&self, other: &Self) -> bool {
        self.f.eq(&other.f)
    }
}

impl<P, S, In, Out, Err, F: Hash> Hash for FromFn<P, S, In, Out, Err, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.f.hash(state);
    }
}
