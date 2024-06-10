use super::{mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Once<T>(pub(super) Option<T>);

impl<T, S, P, E, In> Operator<S, P, E, In> for Once<T>
where
    T: Operator<S, P, E, In>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    #[inline]
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

impl<T, S, P, E> Mutate<S, P, E> for Once<T>
where
    T: Mutate<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.mutate(solution, problem, eval)
    }
}

impl<T, S, P, E> Search<S, P, E> for Once<T>
where
    T: Search<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.search(solution, problem, eval)
    }
}
