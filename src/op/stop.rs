use crate::{eval::Eval, solution::Solution};

mod iters;
pub use iters::Iterations;

mod optimum;
pub use optimum::Optimum;

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Stop<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[must_use]
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool;
}

impl<T, P, S, E> Stop<P, S, E> for &mut T
where
    T: Stop<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        T::stop(self, solution, problem, eval)
    }
}

impl<T, P, S, E> Stop<P, S, E> for Box<T>
where
    T: Stop<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        T::stop(self, solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Stop<P, S, E> for either::Either<L, R>
where
    L: Stop<P, S, E>,
    R: Stop<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    
    fn stop(&mut self, solution: &S, problem: &P, eval: &mut E) -> bool {
        match self {
            Self::Left(left) => left.stop(solution, problem, eval),
            Self::Right(right) => right.stop(solution, problem, eval),
        }
    }
}
