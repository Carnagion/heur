use super::Operator;

mod bit_climb;
pub use bit_climb::{FirstAscentBitClimb, SteepestAscentBitClimb};

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Search<S, P, E>: Operator<S, P, E> {
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error>;
}

impl<S, P, E> Search<S, P, E> for () {
    #[inline]
    fn search(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T, S, P, E> Search<S, P, E> for &mut T
where
    T: Search<S, P, E> + ?Sized,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Search<S, P, E> for Box<T>
where
    T: Search<S, P, E> + ?Sized,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Search<S, P, E> for Option<T>
where
    T: Search<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.search(solution, problem, eval)?;
        }
        Ok(())
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Search<S, P, E> for either::Either<L, R>
where
    L: Search<S, P, E>,
    R: Search<S, P, E, Output = L::Output, Error = L::Error>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.search(solution, problem, eval),
            Self::Right(right) => right.search(solution, problem, eval),
        }
    }
}
