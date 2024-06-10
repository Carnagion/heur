use super::Operator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pipe<T, U> {
    pub(super) from: T,
    pub(super) to: U,
}

impl<T, U, S, P, E, In> Operator<S, P, E, In> for Pipe<T, U>
where
    T: Operator<S, P, E, In>,
    U: Operator<S, P, E, T::Output, Error = T::Error>,
{
    type Output = U::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        let intermediate = self.from.apply(solution, problem, eval, input)?;
        let output = self.to.apply(solution, problem, eval, intermediate)?;
        Ok(output)
    }
}
