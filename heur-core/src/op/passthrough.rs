use crate::{
    Optimize,
    Problem,
    op::{Operator, init::Init},
    solution::Solution,
};

/// An operator that converts a no-input, no-output operator into an operator that takes any input and returns the same input as
/// its output.
///
/// This type is created by [`Operator::passthrough`]. See its documentation for more details.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Passthrough<T>(pub(super) T);

impl<T, P, S, In> Operator<P, S, In> for Passthrough<T>
where
    T: Operator<P, S, Output = ()>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(solution, eval, problem, ())?;
        Ok(input)
    }
}

impl<T, P, S> Init<P, S> for Passthrough<T>
where
    T: Init<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.0.init(eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, eval, problem)
    }
}

impl<T, P, S> Optimize<P, S> for Passthrough<T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        self.0.optimize(eval, problem)
    }
}
