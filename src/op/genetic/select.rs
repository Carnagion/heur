use crate::{eval::Eval, op::Operator, solution::Population};

mod tournament;
pub use tournament::Tournament;

mod on_selected;
pub use on_selected::OnSelected;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Select<P, S, E>: Operator<P, S, E, Output = Vec<S::Individual>>
where
    S: Population,
    E: Eval<P, S::Individual>,
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

impl<T, P, S, E> Select<P, S, E> for &mut T
where
    T: Select<P, S, E> + ?Sized,
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

impl<T, P, S, E> Select<P, S, E> for Box<T>
where
    T: Select<P, S, E> + ?Sized,
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
impl<L, R, P, S, E> Select<P, S, E> for either::Either<L, R>
where
    L: Select<P, S, E>,
    R: Select<P, S, E, Error = L::Error>,
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

#[inline]
pub fn on_selected<P, S, E, T>(op: T) -> OnSelected<T>
where
    T: Operator<P, Vec<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    OnSelected(op)
}
