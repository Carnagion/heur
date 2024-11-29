use crate::{eval::Eval, solution::Solution};

use super::{mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Once<T>(pub(super) Option<T>);

impl<T, P, S, E, In> Operator<P, S, E, In> for Once<T>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.take().apply(solution, problem, eval, input)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Once<T>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.take().mutate(solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Once<T>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.take().search(solution, problem, eval)
    }
}
