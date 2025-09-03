use crate::{Problem, solution::Solution};

use super::{Operator, cond::accept::Accept};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct AcceptIf<T, F> {
    pub(super) op: T,
    pub(super) cond: F,
}

impl<T, F, P, S, In> Operator<P, S, In> for AcceptIf<T, F>
where
    T: Operator<P, S, In>,
    F: Accept<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual> + Clone,
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
