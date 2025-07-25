use crate::{
    Optimize,
    Problem,
    op::{Operator, init::Init},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Passthrough<T>(pub(super) T);

impl<T, P, In> Operator<P, In> for Passthrough<T>
where
    T: Operator<P, Output = ()>,
    P: Problem,
{
    type Output = In;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(solution, eval, problem, ())?;
        Ok(input)
    }
}

impl<T, P> Init<P> for Passthrough<T>
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

impl<T, P> Optimize<P> for Passthrough<T>
where
    T: Optimize<P>,
    P: Problem,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        self.0.optimize(eval, problem)
    }
}
