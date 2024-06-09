mod iters;
pub use iters::Iterations;

mod optimum;
pub use optimum::Optimum;

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Stop<S, P, E> {
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool;
}

impl<T, S, P, E> Stop<S, P, E> for &mut T
where
    T: Stop<S, P, E> + ?Sized,
{
    #[inline]
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        T::stop(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Stop<S, P, E> for Box<T>
where
    T: Stop<S, P, E> + ?Sized,
{
    #[inline]
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        T::stop(self, solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Stop<S, P, E> for either::Either<L, R>
where
    L: Stop<S, P, E>,
    R: Stop<S, P, E>,
{
    #[inline]
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        match self {
            Self::Left(left) => left.stop(solution, problem, eval),
            Self::Right(right) => right.stop(solution, problem, eval),
        }
    }
}
