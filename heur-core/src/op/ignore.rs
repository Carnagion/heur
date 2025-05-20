use crate::Problem;

use super::{Operator, init::Init};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Ignore<T>(pub(super) T);

impl<T, P, In> Operator<P, In> for Ignore<T>
where
    T: Operator<P, In>,
    P: Problem,
{
    type Output = ();

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let _ = self.0.apply(solution, eval, problem, input)?;
        Ok(())
    }
}

impl<T, P> Init<P> for Ignore<T>
where
    T: Init<P>,
    P: Problem,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.0.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, eval, problem)
    }
}

impl<T> AsRef<T> for Ignore<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Ignore<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
