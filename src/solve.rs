use std::error::Error;

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Solve<S, P, E> {
    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;
}

impl<T, S, P, E> Solve<S, P, E> for &mut T
where
    T: Solve<S, P, E> + ?Sized,
{
    type Error = T::Error;

    #[inline]
    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::solve(self, problem, eval)
    }
}

impl<T, S, P, E> Solve<S, P, E> for Box<T>
where
    T: Solve<S, P, E> + ?Sized,
{
    type Error = T::Error;

    #[inline]
    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::solve(self, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Solve<S, P, E> for either::Either<L, R>
where
    L: Solve<S, P, E>,
    R: Solve<S, P, E, Error = L::Error>,
{
    type Error = L::Error;

    #[inline]
    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.solve(problem, eval),
            Self::Right(right) => right.solve(problem, eval),
        }
    }
}
