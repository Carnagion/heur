use crate::{eval::Eval, solution::Solution};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Stop<P, S, E>
where
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[must_use]
    fn stop(&mut self, problem: &P, solution: &S, eval: &mut E) -> bool;
}

impl<T, P, S, E> Stop<P, S, E> for &mut T
where
    T: Stop<P, S, E> + ?Sized,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn stop(&mut self, problem: &P, solution: &S, eval: &mut E) -> bool {
        T::stop(self, problem, solution, eval)
    }
}

impl<T, P, S, E> Stop<P, S, E> for Box<T>
where
    T: Stop<P, S, E> + ?Sized,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn stop(&mut self, problem: &P, solution: &S, eval: &mut E) -> bool {
        T::stop(self, problem, solution, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Stop<P, S, E> for either::Either<L, R>
where
    L: Stop<P, S, E>,
    R: Stop<P, S, E>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn stop(&mut self, problem: &P, solution: &S, eval: &mut E) -> bool {
        match self {
            Self::Left(left) => left.stop(problem, solution, eval),
            Self::Right(right) => right.stop(problem, solution, eval),
        }
    }
}
