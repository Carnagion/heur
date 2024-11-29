use crate::{eval::Eval, solution::Solution};

use super::Operator;

mod bit_flip;
pub use bit_flip::{FlipAllBits, FlipBit};

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Mutate<P, S, E>: Operator<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error>;
}

impl<P, S, E> Mutate<P, S, E> for ()
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T, P, S, E> Mutate<P, S, E> for &mut T
where
    T: Mutate<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Box<T>
where
    T: Mutate<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        T::mutate(self, solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Option<T>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        if let Some(op) = self {
            op.mutate(solution, problem, eval)?;
        }
        Ok(())
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Mutate<P, S, E> for either::Either<L, R>
where
    L: Mutate<P, S, E>,
    R: Mutate<P, S, E, Output = L::Output, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.mutate(solution, problem, eval),
            Self::Right(right) => right.mutate(solution, problem, eval),
        }
    }
}
