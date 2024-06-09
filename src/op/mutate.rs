use super::Operator;

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Mutate<S, P, E>: Operator<S, P, E> {
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error>;
}

impl<S, P, E> Mutate<S, P, E> for () {
    #[inline]
    fn mutate(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T, S, P, E> Mutate<S, P, E> for &mut T
where
    T: Mutate<S, P, E> + ?Sized,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Box<T>
where
    T: Mutate<S, P, E> + ?Sized,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Mutate<S, P, E> for Option<T>
where
    T: Mutate<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Mutate<S, P, E> for either::Either<L, R>
where
    L: Mutate<S, P, E>,
    R: Mutate<S, P, E, Output = L::Output, Error = L::Error>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.mutate(solution, problem, eval),
            Self::Right(right) => right.mutate(solution, problem, eval),
        }
    }
}
