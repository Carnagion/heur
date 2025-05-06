#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{eval::Eval, solution::Solution};

use super::Operator;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Search<P, S, E>: Operator<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error>;
}

impl<P, S, E> Search<P, S, E> for ()
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(
        &mut self,
        _solution: &mut S,
        _problem: &P,
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
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, solution, problem, eval)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S, E> Search<P, S, E> for Box<T>
where
    T: Search<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::search(self, solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Option<T>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.search(solution, problem, eval)?;
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
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.search(solution, problem, eval),
            Self::Right(right) => right.search(solution, problem, eval),
        }
    }
}
