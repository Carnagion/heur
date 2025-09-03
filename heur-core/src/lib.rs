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
    type Individual;

    type Eval: Eval<Self>;
}

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Optimize<P, S>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error: Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error>;
}

impl<T, P, S> Optimize<P, S> for &mut T
where
    T: Optimize<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::optimize(self, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S> Optimize<P, S> for Box<T>
where
    T: Optimize<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = T::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::optimize(self, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Optimize<P, S> for either::Either<L, R>
where
    L: Optimize<P, S>,
    R: Optimize<P, S, Error = L::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Error = L::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.optimize(eval, problem),
            Self::Right(right) => right.optimize(eval, problem),
        }
    }
}
