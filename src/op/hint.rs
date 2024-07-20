use std::marker::PhantomData;

use crate::{
    eval::Eval,
    solution::{Population, Solution},
};

use super::{
    genetic::{combine::Combine, insert::Insert, select::Select},
    init::Init,
    mutate::Mutate,
    search::Search,
    Operator,
};

// TODO: Manually impl common traits
#[must_use]
pub struct Hint<T, P, S, E, In = ()> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) _marker: PhantomData<fn() -> (P, S, E, In)>,
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Hint<T, P, S, E, In>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, problem, eval, input)
    }
}

impl<T, P, S, E> Init<P, S, E> for Hint<T, P, S, E>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Hint<T, P, S, E>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Hint<T, P, S, E>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(solution, problem, eval)
    }
}

impl<T, P, S, E> Select<P, S, E> for Hint<T, P, S, E>
where
    T: Select<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        self.op.select(population, problem, eval)
    }

    #[inline]
    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.op.select_into(population, problem, eval, selected)
    }
}

impl<T, P, S, E> Combine<P, S, E> for Hint<T, P, S, E, Vec<S::Individual>>
where
    T: Combine<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        self.op.combine(population, problem, eval, selected)
    }
}

impl<T, P, S, E> Insert<P, S, E> for Hint<T, P, S, E, Vec<S::Individual>>
where
    T: Insert<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.op.insert(population, problem, eval, combined)
    }
}
