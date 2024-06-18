use crate::{eval::Eval, solution::Solution};

use super::Operator;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Init<P, S, E>: Operator<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        *solution = self.init(problem, eval)?;
        Ok(())
    }
}

impl<T, P, S, E> Init<P, S, E> for &mut T
where
    T: Init<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Init<P, S, E> for Box<T>
where
    T: Init<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, problem, solution, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Init<P, S, E> for either::Either<L, R>
where
    L: Init<P, S, E>,
    R: Init<P, S, E, Output = L::Output, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.init(problem, eval),
            Self::Right(right) => right.init(problem, eval),
        }
    }

    #[inline]
    fn init_into(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.init_into(problem, solution, eval),
            Self::Right(right) => right.init_into(problem, solution, eval),
        }
    }
}
