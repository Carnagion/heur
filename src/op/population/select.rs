use crate::{eval::Eval, op::Operator, solution::Population};

mod tournament;
pub use tournament::Tournament;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Select<P, S, E>: Operator<P, S, E, Output = Vec<S::Individual>>
where
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    fn select(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error>;

    #[inline]
    fn select_into(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        *selected = self.select(problem, population, eval)?;
        Ok(())
    }
}

impl<T, P, S, E> Select<P, S, E> for &mut T
where
    T: Select<P, S, E> + ?Sized,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn select(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
    ) -> Result<Vec<<S>::Individual>, Self::Error> {
        T::select(self, problem, population, eval)
    }

    #[inline]
    fn select_into(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: &mut Vec<<S>::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, problem, population, eval, selected)
    }
}

impl<T, P, S, E> Select<P, S, E> for Box<T>
where
    T: Select<P, S, E> + ?Sized,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn select(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
    ) -> Result<Vec<<S>::Individual>, Self::Error> {
        T::select(self, problem, population, eval)
    }

    #[inline]
    fn select_into(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: &mut Vec<<S>::Individual>,
    ) -> Result<(), Self::Error> {
        T::select_into(self, problem, population, eval, selected)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E> Select<P, S, E> for either::Either<L, R>
where
    L: Select<P, S, E>,
    R: Select<P, S, E, Error = L::Error>,
    S: Population<Individual: Sized> + ?Sized,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn select(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        match self {
            Self::Left(left) => left.select(problem, population, eval),
            Self::Right(right) => right.select(problem, population, eval),
        }
    }

    #[inline]
    fn select_into(
        &mut self,
        problem: &P,
        population: &S,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.select_into(problem, population, eval, selected),
            Self::Right(right) => right.select_into(problem, population, eval, selected),
        }
    }
}
