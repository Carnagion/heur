use crate::op::Operator;

use super::Population;

// TODO: Add `#[diagnostic::on_unimplemented]`
#[doc(alias = "Crossover")]
pub trait Combine<S, P, E>:
    Operator<S, P, E, Vec<S::Individual>, Output = Vec<S::Individual>>
where
    S: Population,
{
    #[doc(alias = "crossover")]
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error>;
}

impl<T, S, P, E> Combine<S, P, E> for &mut T
where
    T: Combine<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        T::combine(self, population, problem, eval, selected)
    }
}

impl<T, S, P, E> Combine<S, P, E> for Box<T>
where
    T: Combine<S, P, E> + ?Sized,
    S: Population,
{
    #[inline]
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        T::combine(self, population, problem, eval, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Combine<S, P, E> for either::Either<L, R>
where
    L: Combine<S, P, E>,
    R: Combine<S, P, E, Error = L::Error>,
    S: Population,
{
    #[inline]
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.combine(population, problem, eval, selected),
            Self::Right(right) => right.combine(population, problem, eval, selected),
        }
    }
}
