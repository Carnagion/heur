use core::error::Error;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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

#[cfg(feature = "alloc")]
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

// NOTE: We need these traits due to a possible bug in `rustc` where trying to prove that `Evaluated<S, O>` impls
//       `IntoIterator` puts the trait solver into a loop and leads to an overflow. Conceptually, `T: for<'a> Iter<'a>`
//       is exactly the same as `for<'a> &'a T: IntoIterator<Item = &'a U>`, but the latter leads to E0275 ("overflow
//       evaluating the requirement ...") while the former works perfectly.
pub trait Iter<'a> {
    type Item: 'a;

    type Iter: Iterator<Item = &'a Self::Item>;

    fn iter(&'a self) -> Self::Iter;
}

// NOTE: See the above note on `Iter<'a>`.
pub trait IterMut<'a>: Iter<'a> {
    type IterMut: Iterator<Item = &'a mut Self::Item>;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}

impl<'a, I, T> Iter<'a> for I
where
    I: 'a,
    &'a I: IntoIterator<Item = &'a T>,
    T: 'a,
{
    type Item = T;

    type Iter = <&'a Self as IntoIterator>::IntoIter;

    fn iter(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a, I, T> IterMut<'a> for I
where
    I: 'a,
    &'a I: IntoIterator<Item = &'a T>,
    &'a mut I: IntoIterator<Item = &'a mut T>,
    T: 'a,
{
    type IterMut = <&'a mut Self as IntoIterator>::IntoIter;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.into_iter()
    }
}
