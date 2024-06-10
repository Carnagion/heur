use super::{init::Init, mutate::Mutate, search::Search, Operator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ignore<T>(pub(super) T);

impl<T, S, P, E, In> Operator<S, P, E, In> for Ignore<T>
where
    T: Operator<S, P, E, In>,
{
    type Output = ();

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(solution, problem, eval, input)?;
        Ok(())
    }
}

impl<T, S, P, E> Init<S, P, E> for Ignore<T>
where
    T: Init<S, P, E>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.0.init(problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.0.init_into(solution, problem, eval)
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Ignore<T>
where
    T: Mutate<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.mutate(solution, problem, eval)
    }
}

impl<T, S, P, E> Search<S, P, E> for Ignore<T>
where
    T: Search<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.0.search(solution, problem, eval)
    }
}
