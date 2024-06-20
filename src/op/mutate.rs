use crate::{eval::Eval, solution::Solution};

use super::Operator;

mod bit_flip;
pub use bit_flip::{FlipAllBits, FlipBit};

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Mutate<P, S, E>: Operator<P, S, E>
where
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error>;
}

impl<P, S, E> Mutate<P, S, E> for ()
where
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T, P, S, E> Mutate<P, S, E> for &mut T
where
    T: Mutate<P, S, E> + ?Sized,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Box<T>
where
    T: Mutate<P, S, E> + ?Sized,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Option<T>
where
    T: Mutate<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.mutate(problem, solution, eval)?;
        }
        Ok(())
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Mutate<P, S, E> for either::Either<L, R>
where
    L: Mutate<P, S, E>,
    R: Mutate<P, S, E, Output = L::Output, Error = L::Error>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, solution: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.mutate(problem, solution, eval),
            Self::Right(right) => right.mutate(problem, solution, eval),
        }
    }
}
