use crate::{Problem, solution::Solution};

use super::Operator;

/// An operator that enforces single-use application for its inner operator.
///
/// This type is created by [`Operator::once`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Once<T>(pub(super) Option<T>);

impl<T, P, S, In> Operator<P, S, In> for Once<T>
where
    T: Operator<P, S, In>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.take().apply(solution, eval, problem, input)
    }
}
