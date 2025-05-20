use crate::Problem;

use super::{Operator, cond::accept::Accept};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct AcceptIf<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, In> Operator<P, In> for AcceptIf<T, F>
where
    T: Operator<P, In>,
    F: Accept<P>,
    P: Problem<Solution: Clone>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let prev = solution.clone();

        let output = self.op.apply(solution, eval, problem, input)?;
        if self.cond.accept(solution, &prev, eval, problem) {
            Ok(Some(output))
        } else {
            *solution = prev;
            Ok(None)
        }
    }
}
