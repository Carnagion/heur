use std::mem;

use crate::op::Operator;

use super::Population;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Insert<S, P, E>: Operator<S, P, E, Vec<S::Individual>>
where
    S: Population,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error>;

    #[inline]
    fn insert_from(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.insert(population, problem, eval, mem::take(combined))
    }
}

impl<T, S, P, E> Insert<S, P, E> for &mut T
where
    T: Insert<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, problem, eval, combined)
    }

    #[inline]
    fn insert_from(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert_from(self, population, problem, eval, combined)
    }
}

impl<T, S, P, E> Insert<S, P, E> for Box<T>
where
    T: Insert<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert(self, population, problem, eval, combined)
    }

    #[inline]
    fn insert_from(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::insert_from(self, population, problem, eval, combined)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Insert<S, P, E> for either::Either<L, R>
where
    L: Insert<S, P, E>,
    R: Insert<S, P, E, Output = L::Output, Error = L::Error>,
    S: Population,
{
    #[inline]
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.insert(population, problem, eval, combined),
            Self::Right(right) => right.insert(population, problem, eval, combined),
        }
    }

    #[inline]
    fn insert_from(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.insert_from(population, problem, eval, combined),
            Self::Right(right) => right.insert_from(population, problem, eval, combined),
        }
    }
}
