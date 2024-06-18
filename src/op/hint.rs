use std::marker::PhantomData;

use crate::{eval::Eval, solution::Solution};

use super::Operator;

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
