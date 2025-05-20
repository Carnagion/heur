use core::{marker::PhantomData, mem};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Problem,
    eval::Eval,
    solution::{Individual, Iter, Population},
};

use super::{And, Not, Or};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Stop<P: Problem> {
    #[must_use]
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool;

    fn and<U>(self, cond: U) -> And<Self, U>
    where
        Self: Sized,
        U: Stop<P>,
    {
        And {
            first: self,
            second: cond,
        }
    }

    fn or<U>(self, cond: U) -> Or<Self, U>
    where
        Self: Sized,
        U: Stop<P>,
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

impl<T, P> Stop<P> for &mut T
where
    T: Stop<P> + ?Sized,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        T::stop(self, solution, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P> Stop<P> for Box<T>
where
    T: Stop<P> + ?Sized,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        T::stop(self, solution, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P> Stop<P> for either::Either<L, R>
where
    L: Stop<P>,
    R: Stop<P>,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        match self {
            Self::Left(left) => left.stop(solution, eval, problem),
            Self::Right(right) => right.stop(solution, eval, problem),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Iterations(pub usize);

impl<P: Problem> Stop<P> for Iterations {
    fn stop(&mut self, _: &P::Solution, _: &mut P::Eval, _: &P) -> bool {
        let remaining = self.0.saturating_sub(1);
        let iters = mem::replace(&mut self.0, remaining);
        iters == 0
    }
}

// TODO: Manually implement common traits
#[must_use]
pub struct Optimum<O, S> {
    optimum: O,
    marker: PhantomData<fn() -> S>,
}

impl<O, S> Optimum<O, S> {
    pub fn new(optimum: O) -> Self {
        Self {
            optimum,
            marker: PhantomData,
        }
    }
}

impl<P, S, O> Stop<P> for Optimum<O, Individual<S>>
where
    P: Problem<Solution = Individual<S>>,
    P::Eval: Eval<P, Objective = O>,
    O: PartialOrd,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        eval.eval(solution, problem) >= self.optimum
    }
}

impl<P, S, O> Stop<P> for Optimum<O, S>
where
    P: Problem<Solution = S>,
    S: Population + for<'a> Iter<'a, Item = S::Individual>,
    P::Eval: Eval<P, Objective = O>,
    O: PartialOrd,
{
    fn stop(&mut self, population: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        population
            .iter()
            .any(|solution| eval.eval(solution, problem) >= self.optimum)
    }
}

impl<T, U, P> Stop<P> for And<T, U>
where
    T: Stop<P>,
    U: Stop<P>,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.stop(solution, eval, problem) && self.second.stop(solution, eval, problem)
    }
}

impl<T, U, P> Stop<P> for Or<T, U>
where
    T: Stop<P>,
    U: Stop<P>,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.stop(solution, eval, problem) || self.second.stop(solution, eval, problem)
    }
}

impl<T, P> Stop<P> for Not<T>
where
    T: Stop<P>,
    P: Problem,
{
    fn stop(&mut self, solution: &P::Solution, eval: &mut P::Eval, problem: &P) -> bool {
        !self.0.stop(solution, eval, problem)
    }
}
