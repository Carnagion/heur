use core::fmt::{self, Debug, Formatter};

use crate::{Problem, solution::Solution};

use super::Operator;

/// An operator that flattens one level of nesting in its inner operator, whose output is itself an operator.
///
/// This type is created by [`Operator::flatten`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Flatten<T>(pub(crate) T);

impl<T, P, S, In> Operator<P, S, In> for Flatten<T>
where
    T: Operator<P, S, In, Output: Operator<P, S, Error = T::Error>>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = <T::Output as Operator<P, S>>::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0
            .apply(solution, eval, problem, input)?
            .apply(solution, eval, problem, ())
    }
}

/// An operator that maps its inner operator's output to another operator, and yields the mapped operator's output as
/// its own output.
///
/// This type is created by [`Operator::flat_map`]. See its documentation for more details.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FlatMap<T, F> {
    pub(crate) op: T,
    pub(crate) f: F,
}

impl<T, U, F, P, S, In> Operator<P, S, In> for FlatMap<T, F>
where
    T: Operator<P, S, In>,
    U: Operator<P, S, Error = T::Error>,
    F: FnMut(T::Output) -> U,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = U::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let output = self.op.apply(solution, eval, problem, input)?;
        let mut op = (self.f)(output);
        op.apply(solution, eval, problem, ())
    }
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
