use std::error::Error;

use crate::eval::Eval;

mod individual;
pub use individual::Individual;

mod population;
pub use population::Population;

mod evaluated;
pub use evaluated::Evaluated;

// TODO: 1. Impl `Solution` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, and/or `im`
//       2. Add `#[diagnostic::on_unimplemented]`
pub trait Solution {
    type Individual;
}

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Solve<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;
}

impl<T, P, S, E> Solve<P, S, E> for &mut T
where
    T: Solve<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = T::Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::solve(self, problem, eval)
    }
}

impl<T, P, S, E> Solve<P, S, E> for Box<T>
where
    T: Solve<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = T::Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::solve(self, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Solve<P, S, E> for either::Either<L, R>
where
    L: Solve<P, S, E>,
    R: Solve<P, S, E, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = L::Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.solve(problem, eval),
            Self::Right(right) => right.solve(problem, eval),
        }
    }
}
