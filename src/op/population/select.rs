use crate::op::Operator;

use super::Population;

mod tournament;
pub use tournament::Tournament;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Select<S, P, E>: Operator<S, P, E, Output = Vec<S::Individual>>
where
    S: Population,
{
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error>;

    #[inline]
    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        *selected = self.select(population, problem, eval)?;
        Ok(())
    }
}

impl<T, S, P, E> Select<S, P, E> for &mut T
where
    T: Select<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        T::select(self, population, problem, eval)
    }

    #[inline]
    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, problem, eval, selected)
    }
}

impl<T, S, P, E> Select<S, P, E> for Box<T>
where
    T: Select<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        T::select(self, population, problem, eval)
    }

    #[inline]
    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, population, problem, eval, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Select<S, P, E> for either::Either<L, R>
where
    L: Select<S, P, E>,
    R: Select<S, P, E, Error = L::Error>,
    S: Population,
{
    #[inline]
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.select(population, problem, eval),
            Self::Right(right) => right.select(population, problem, eval),
        }
    }

    #[inline]
    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.select_into(population, problem, eval, selected),
            Self::Right(right) => right.select_into(population, problem, eval, selected),
        }
    }
}
