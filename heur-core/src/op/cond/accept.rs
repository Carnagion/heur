#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Problem,
    eval::Eval,
    solution::{Individual, Solution},
};

use super::{And, Not, Or};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Accept<P, S>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    #[must_use]
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool;

    fn and<U>(self, other: U) -> And<Self, U>
    where
        Self: Sized,
    {
        And {
            first: self,
            second: other,
        }
    }

    fn or<U>(self, other: U) -> Or<Self, U>
    where
        Self: Sized,
    {
        Or {
            first: self,
            second: other,
        }
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }
}

impl<T, P, S> Accept<P, S> for &mut T
where
    T: Accept<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        T::accept(self, solution, prev, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S> Accept<P, S> for Box<T>
where
    T: Accept<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        T::accept(self, solution, prev, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Accept<P, S> for either::Either<L, R>
where
    L: Accept<P, S>,
    R: Accept<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        match self {
            Self::Left(left) => left.accept(solution, prev, eval, problem),
            Self::Right(right) => right.accept(solution, prev, eval, problem),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Improving;

impl<P, S> Accept<P, Individual<S>> for Improving
where
    P: Problem<Individual = S>,
{
    fn accept(
        &mut self,
        solution: &Individual<S>,
        prev: &Individual<S>,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        eval.eval(solution, problem) > eval.eval(prev, problem)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct NonWorsening;

impl<P, S> Accept<P, Individual<S>> for NonWorsening
where
    P: Problem<Individual = S>,
{
    fn accept(
        &mut self,
        solution: &Individual<S>,
        prev: &Individual<S>,
        eval: &mut P::Eval,
        problem: &P,
    ) -> bool {
        eval.eval(solution, problem) >= eval.eval(prev, problem)
    }
}

impl<T, U, P, S> Accept<P, S> for And<T, U>
where
    T: Accept<P, S>,
    U: Accept<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.accept(solution, prev, eval, problem)
            && self.second.accept(solution, prev, eval, problem)
    }
}

impl<T, U, P, S> Accept<P, S> for Or<T, U>
where
    T: Accept<P, S>,
    U: Accept<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.accept(solution, prev, eval, problem)
            || self.second.accept(solution, prev, eval, problem)
    }
}

impl<T, P, S> Accept<P, S> for Not<T>
where
    T: Accept<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn accept(&mut self, solution: &S, prev: &S, eval: &mut P::Eval, problem: &P) -> bool {
        !self.0.accept(solution, prev, eval, problem)
    }
}
