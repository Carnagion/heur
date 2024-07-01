use crate::{eval::Eval, op::Operator, solution::Population};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct OnSelected<T>(pub(super) T);

impl<T, P, S, E> Operator<P, S, E, Vec<S::Individual>> for OnSelected<T>
where
    T: Operator<P, Vec<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    type Output = Vec<S::Individual>;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        _solution: &mut S,
        problem: &P,
        eval: &mut E,
        mut selected: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.0.apply(&mut selected, problem, eval, ())?;
        Ok(selected)
    }
}
