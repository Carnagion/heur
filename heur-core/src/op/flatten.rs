use std::fmt::{self, Debug, Formatter};

use crate::{eval::Eval, solution::Solution};

use super::Operator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Flatten<T>(pub(crate) T);

impl<T, P, S, E, In> Operator<P, S, E, In> for Flatten<T>
where
    T: Operator<P, S, E, In, Output: Operator<P, S, E, Error = T::Error>>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = <T::Output as Operator<P, S, E>>::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0
            .apply(solution, problem, eval, input)?
            .apply(solution, problem, eval, ())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FlatMap<T, F> {
    pub(crate) op: T,
    pub(crate) f: F,
}

impl<T, F> Debug for FlatMap<T, F>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FlatMap")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T, U, F, P, S, E, In> Operator<P, S, E, In> for FlatMap<T, F>
where
    T: Operator<P, S, E, In>,
    U: Operator<P, S, E, Error = T::Error>,
    F: FnMut(T::Output) -> U,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = U::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let output = self.op.apply(solution, problem, eval, input)?;
        let mut op = (self.f)(output);
        op.apply(solution, problem, eval, ())
    }
}
