use std::convert::Infallible;

use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Unwrapped<T>(pub(super) T);

impl<T, S, P, E, In> Operator<S, P, E, In> for Unwrapped<T>
where
    T: Operator<S, P, E, In>,
{
    type Output = T::Output;

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let output = self.0.apply(solution, problem, eval, input).unwrap();
        Ok(output)
    }
}

impl<T, S, P, E> Init<S, P, E> for Unwrapped<T>
where
    T: Init<S, P, E>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        Ok(self.0.init(problem, eval).unwrap())
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, problem, eval).unwrap();
        Ok(())
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Unwrapped<T>
where
    T: Mutate<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.mutate(solution, problem, eval).unwrap();
        Ok(())
    }
}

impl<T, S, P, E> Search<S, P, E> for Unwrapped<T>
where
    T: Search<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.search(solution, problem, eval).unwrap();
        Ok(())
    }
}
