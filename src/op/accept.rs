use crate::{eval::Eval, solution::Solution};

mod unconditional;
pub use unconditional::{Always, Never};

mod improving;
pub use improving::{Improving, NonWorsening};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Accept<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool;
}

impl<T, P, S, E> Accept<P, S, E> for &mut T
where
    T: Accept<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        T::accept(self, solution, prev_solution, problem, eval)
    }
}

impl<T, P, S, E> Accept<P, S, E> for Box<T>
where
    T: Accept<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        T::accept(self, solution, prev_solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Accept<P, S, E> for either::Either<L, R>
where
    L: Accept<P, S, E>,
    R: Accept<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        match self {
            Self::Left(left) => left.accept(solution, prev_solution, problem, eval),
            Self::Right(right) => right.accept(solution, prev_solution, problem, eval),
        }
    }
}
