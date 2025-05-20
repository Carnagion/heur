use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{Problem, solution::Solution};

mod from_fn;
pub use from_fn::FromFn;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Eval<P: Problem> {
    type Objective: PartialOrd;

    #[must_use]
    fn eval(
        &mut self,
        solution: &<P::Solution as Solution>::Individual,
        problem: &P,
    ) -> Self::Objective;
}

impl<T, P> Eval<P> for &mut T
where
    T: Eval<P> + ?Sized,
    P: Problem,
{
    type Objective = T::Objective;

    fn eval(
        &mut self,
        solution: &<P::Solution as Solution>::Individual,
        problem: &P,
    ) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Eval<P> for Box<T>
where
    T: Eval<P> + ?Sized,
    P: Problem,
{
    type Objective = T::Objective;

    fn eval(
        &mut self,
        solution: &<P::Solution as Solution>::Individual,
        problem: &P,
    ) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Eval<P> for either::Either<L, R>
where
    L: Eval<P>,
    R: Eval<P, Objective = L::Objective>,
    P: Problem,
{
    type Objective = L::Objective;

    fn eval(
        &mut self,
        solution: &<P::Solution as Solution>::Individual,
        problem: &P,
    ) -> Self::Objective {
        match self {
            Self::Left(left) => left.eval(solution, problem),
            Self::Right(right) => right.eval(solution, problem),
        }
    }
}

pub fn from_fn<P, O, F>(f: F) -> FromFn<P, O, F>
where
    F: FnMut(&<P::Solution as Solution>::Individual, &P) -> O,
    P: Problem,
    O: PartialOrd,
{
    FromFn {
        f,
        marker: PhantomData,
    }
}
