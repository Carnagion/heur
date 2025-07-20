use core::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::Problem;

use super::Operator;

type OperatorFn<P, In, Out, Err> =
    fn(&mut <P as Problem>::Solution, &mut <P as Problem>::Eval, &P, In) -> Result<Out, Err>;

#[must_use]
pub struct FromFn<P, In = (), Out = (), Err = Infallible, F = OperatorFn<P, In, Out, Err>> {
    pub(super) f: F,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, In, Out, Err)>,
}

impl<P, In, Out, Err, F> Operator<P, In> for FromFn<P, In, Out, Err, F>
where
    F: FnMut(&mut P::Solution, &mut P::Eval, &P, In) -> Result<Out, Err>,
    P: Problem,
    Err: Error,
{
    type Output = Out;

    type Error = Err;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        (self.f)(solution, eval, problem, input)
    }
}

impl<P, In, Out, Err, F> Debug for FromFn<P, In, Out, Err, F> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("FromFn").finish_non_exhaustive()
    }
}

impl<P, In, Out, Err, F: Copy> Copy for FromFn<P, In, Out, Err, F> {}

impl<P, In, Out, Err, F: Clone> Clone for FromFn<P, In, Out, Err, F> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: self.marker,
        }
    }
}

impl<P, In, Out, Err, F: Eq> Eq for FromFn<P, In, Out, Err, F> {}

impl<P, In, Out, Err, F: PartialEq> PartialEq for FromFn<P, In, Out, Err, F> {
    fn eq(&self, other: &Self) -> bool {
        self.f.eq(&other.f)
    }
}

impl<P, In, Out, Err, F: Hash> Hash for FromFn<P, In, Out, Err, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.f.hash(state);
    }
}
