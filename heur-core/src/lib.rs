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
pub trait Problem: Sized {
    type Solution: Solution;

    type Eval: Eval<Self>;
}

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Optimize<P: Problem> {
    type Error: Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error>;
}

impl<T, P> Optimize<P> for &mut T
where
    T: Optimize<P> + ?Sized,
    P: Problem,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        T::optimize(self, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Optimize<P> for Box<T>
where
    T: Optimize<P> + ?Sized,
    P: Problem,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        T::optimize(self, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Optimize<P> for either::Either<L, R>
where
    L: Optimize<P>,
    R: Optimize<P, Error = L::Error>,
    P: Problem,
{
    type Error = L::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
        match self {
            Self::Left(left) => left.optimize(eval, problem),
            Self::Right(right) => right.optimize(eval, problem),
        }
    }
}
