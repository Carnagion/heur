#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{Problem, eval::Eval, solution::Individual};

use super::{And, Not, Or};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Accept<P: Problem> {
    #[must_use]
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool;

    fn and<U>(self, cond: U) -> And<Self, U>
    where
        Self: Sized,
        U: Accept<P>,
    {
        And {
            first: self,
            second: cond,
        }
    }

    fn or<U>(self, cond: U) -> Or<Self, U>
    where
        Self: Sized,
        U: Accept<P>,
    {
        Or {
            first: self,
            second: cond,
        }
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }
}

impl<T, P> Accept<P> for &mut T
where
    T: Accept<P> + ?Sized,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        T::accept(self, solution, prev, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Accept<P> for Box<T>
where
    T: Accept<P> + ?Sized,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        T::accept(self, solution, prev, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Accept<P> for either::Either<L, R>
where
    L: Accept<P>,
    R: Accept<P>,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        match self {
            Self::Left(left) => left.accept(solution, prev, eval, problem),
            Self::Right(right) => right.accept(solution, prev, eval, problem),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Improving;

impl<P, S> Accept<P> for Improving
where
    P: Problem<Solution = Individual<S>>,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        eval.eval(solution, problem) > eval.eval(prev, problem)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct NonWorsening;

impl<P, S> Accept<P> for NonWorsening
where
    P: Problem<Solution = Individual<S>>,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        eval.eval(solution, problem) >= eval.eval(prev, problem)
    }
}

impl<T, U, P> Accept<P> for And<T, U>
where
    T: Accept<P>,
    U: Accept<P>,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        self.first.accept(solution, prev, eval, problem)
            && self.second.accept(solution, prev, eval, problem)
    }
}

impl<T, U, P> Accept<P> for Or<T, U>
where
    T: Accept<P>,
    U: Accept<P>,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        self.first.accept(solution, prev, eval, problem)
            || self.second.accept(solution, prev, eval, problem)
    }
}

impl<T, P> Accept<P> for Not<T>
where
    T: Accept<P>,
    P: Problem,
{
    fn accept(
        &mut self,
        solution: &P::Solution,
        prev: &P::Solution,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        !self.0.accept(solution, prev, eval, problem)
    }
}
