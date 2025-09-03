use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Problem,
    eval::Eval,
    solution::{Individual, Iter, Population, Solution},
};

use super::{And, Condition, Not, Or};

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Stop<P, S>: Condition
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    #[must_use]
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool;
}

impl<T, P, S> Stop<P, S> for &mut T
where
    T: Stop<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        T::stop(self, solution, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S> Stop<P, S> for Box<T>
where
    T: Stop<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        T::stop(self, solution, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Stop<P, S> for either::Either<L, R>
where
    L: Stop<P, S>,
    R: Stop<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        match self {
            Self::Left(left) => left.stop(solution, eval, problem),
            Self::Right(right) => right.stop(solution, eval, problem),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Iterations(pub usize);

impl Condition for Iterations {}

impl<P, S> Stop<P, S> for Iterations
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, _: &S, _: &mut P::Eval, _: &P) -> bool {
        let remaining = self.0.saturating_sub(1);
        let iters = mem::replace(&mut self.0, remaining);
        iters == 0
    }
}

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

impl<O, S> Condition for Optimum<O, S> {}

impl<P, S, O> Stop<P, Individual<S>> for Optimum<O, Individual<S>>
where
    P: Problem<Individual = S>,
    P::Eval: Eval<P, Objective = O>,
    O: PartialOrd,
{
    fn stop(&mut self, solution: &Individual<S>, eval: &mut P::Eval, problem: &P) -> bool {
        eval.eval(solution, problem) >= self.optimum
    }
}

impl<P, S, O> Stop<P, S> for Optimum<O, S>
where
    P: Problem,
    S: Population<Individual = P::Individual> + for<'a> Iter<'a, Item = S::Individual>,
    P::Eval: Eval<P, Objective = O>,
    O: PartialOrd,
{
    fn stop(&mut self, population: &S, eval: &mut P::Eval, problem: &P) -> bool {
        population
            .iter()
            .any(|solution| eval.eval(solution, problem) >= self.optimum)
    }
}

impl<O: Debug, S> Debug for Optimum<O, S> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Optimum")
            .field("optimum", &self.optimum)
            .finish_non_exhaustive()
    }
}

impl<O: Copy, S> Copy for Optimum<O, S> {}

impl<O: Clone, S> Clone for Optimum<O, S> {
    fn clone(&self) -> Self {
        Self {
            optimum: self.optimum.clone(),
            marker: self.marker,
        }
    }
}

impl<O: Eq, S> Eq for Optimum<O, S> {}

impl<O: PartialEq, S> PartialEq for Optimum<O, S> {
    fn eq(&self, other: &Self) -> bool {
        self.optimum.eq(&other.optimum)
    }
}

impl<O: Hash, S> Hash for Optimum<O, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.optimum.hash(state);
    }
}

impl<T, U, P, S> Stop<P, S> for And<T, U>
where
    T: Stop<P, S>,
    U: Stop<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.stop(solution, eval, problem) && self.second.stop(solution, eval, problem)
    }
}

impl<T, U, P, S> Stop<P, S> for Or<T, U>
where
    T: Stop<P, S>,
    U: Stop<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        self.first.stop(solution, eval, problem) || self.second.stop(solution, eval, problem)
    }
}

impl<T, P, S> Stop<P, S> for Not<T>
where
    T: Stop<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn stop(&mut self, solution: &S, eval: &mut P::Eval, problem: &P) -> bool {
        !self.0.stop(solution, eval, problem)
    }
}
