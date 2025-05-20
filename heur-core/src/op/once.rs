use crate::Problem;

use super::Operator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Once<T>(pub(super) Option<T>);

impl<T, P, In> Operator<P, In> for Once<T>
where
    T: Operator<P, In>,
    P: Problem,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut <P as Problem>::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.take().apply(solution, eval, problem, input)
    }
}
