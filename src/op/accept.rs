mod unconditional;
pub use unconditional::{Always, Never};

mod improving;
pub use improving::{Improving, NonWorsening};

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Accept<S, P, E> {
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool;
}

impl<T, S, P, E> Accept<S, P, E> for &mut T
where
    T: Accept<S, P, E> + ?Sized,
{
    #[inline]
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        T::accept(self, solution, prev_solution, problem, eval)
    }
}

impl<T, S, P, E> Accept<S, P, E> for Box<T>
where
    T: Accept<S, P, E> + ?Sized,
{
    #[inline]
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        T::accept(self, solution, prev_solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Accept<S, P, E> for either::Either<L, R>
where
    L: Accept<S, P, E>,
    R: Accept<S, P, E>,
{
    #[inline]
    #[must_use]
    fn accept(&mut self, solution: &S, prev_solution: &S, problem: &P, eval: &mut E) -> bool {
        match self {
            Self::Left(left) => left.accept(solution, prev_solution, problem, eval),
            Self::Right(right) => right.accept(solution, prev_solution, problem, eval),
        }
    }
}
