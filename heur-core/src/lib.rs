#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::error::Error;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use eval::Eval;

use solution::Solution;

pub mod solution;

pub mod eval;

pub mod op;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Optimize<P, S, E>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error: Error;

    fn optimize(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;
}

impl<T, P, S, E> Optimize<P, S, E> for &mut T
where
    T: Optimize<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::optimize(self, problem, eval)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S, E> Optimize<P, S, E> for Box<T>
where
    T: Optimize<P, S, E> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::optimize(self, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Optimize<P, S, E> for either::Either<L, R>
where
    L: Optimize<P, S, E>,
    R: Optimize<P, S, E, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Error = L::Error;

    fn optimize(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.optimize(problem, eval),
            Self::Right(right) => right.optimize(problem, eval),
        }
    }
}
