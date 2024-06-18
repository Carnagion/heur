use crate::{eval::Eval, solution::Solution};

use super::Operator;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Search<P, S, E>: Operator<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error>;
}

impl<P, S, E> Search<P, S, E> for ()
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for &mut T
where
    T: Search<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Box<T>
where
    T: Search<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Option<T>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.search(problem, solution, eval)?;
        }
        Ok(())
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Search<P, S, E> for either::Either<L, R>
where
    L: Search<P, S, E>,
    R: Search<P, S, E, Output = L::Output, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.search(problem, solution, eval),
            Self::Right(right) => right.search(problem, solution, eval),
        }
    }
}
