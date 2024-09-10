use crate::{eval::Eval, solution::Solution};

use super::Operator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flatten<T>(pub(crate) T);

impl<T, P, S, E, In> Operator<P, S, E, In> for Flatten<T>
where
    T: Operator<P, S, E, In, Output: Operator<P, S, E, Error = T::Error>>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = <T::Output as Operator<P, S, E>>::Output;

    type Error = T::Error;

    #[inline]
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

// TODO: Manually impl `Debug`
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlatMap<T, F> {
    pub(crate) op: T,
    pub(crate) f: F,
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

    #[inline]
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
