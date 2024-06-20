use crate::{eval::Eval, op::Operator, solution::Population};

// TODO: Add `#[diagnostic::on_unimplemented]`
#[doc(alias = "Crossover")]
pub trait Combine<P, S, E>:
    Operator<P, S, E, Vec<S::Individual>, Output = Vec<S::Individual>>
where
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[doc(alias = "crossover")]
    fn combine(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error>;
}

impl<T, P, S, E> Combine<P, S, E> for &mut T
where
    T: Combine<P, S, E> + ?Sized,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn combine(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<<S>::Individual>, Self::Error> {
        T::combine(self, problem, population, eval, selected)
    }
}

impl<T, P, S, E> Combine<P, S, E> for Box<T>
where
    T: Combine<P, S, E> + ?Sized,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn combine(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<<S>::Individual>, Self::Error> {
        T::combine(self, problem, population, eval, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Combine<P, S, E> for either::Either<L, R>
where
    L: Combine<P, S, E>,
    R: Combine<P, S, E, Error = L::Error>,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn combine(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.combine(problem, population, eval, selected),
            Self::Right(right) => right.combine(problem, population, eval, selected),
        }
    }
}
