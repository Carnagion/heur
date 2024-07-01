use crate::{eval::Eval, op::Operator, solution::Population};

mod on_combined;
pub use on_combined::OnCombined;

mod uniform;
pub use uniform::{UniformCrossover, UniformCrossoverError};

// TODO: Add `#[diagnostic::on_unimplemented]`
#[doc(alias = "Crossover")]
pub trait Combine<P, S, E>:
    Operator<P, S, E, Vec<S::Individual>, Output = Vec<S::Individual>>
where
    S: Population,
    E: Eval<P, S::Individual>,
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

impl<T, P, S, E> Combine<P, S, E> for &mut T
where
    T: Combine<P, S, E> + ?Sized,
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
        T::combine(self, population, problem, eval, selected)
    }
}

impl<T, P, S, E> Combine<P, S, E> for Box<T>
where
    T: Combine<P, S, E> + ?Sized,
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
        T::combine(self, population, problem, eval, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Combine<P, S, E> for either::Either<L, R>
where
    L: Combine<P, S, E>,
    R: Combine<P, S, E, Error = L::Error>,
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
        match self {
            Self::Left(left) => left.combine(population, problem, eval, selected),
            Self::Right(right) => right.combine(population, problem, eval, selected),
        }
    }
}

#[inline]
pub fn on_combined<P, S, E, T>(op: T) -> OnCombined<T>
where
    T: Operator<P, Vec<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    OnCombined(op)
}
