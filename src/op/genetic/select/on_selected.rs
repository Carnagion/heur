use std::marker::PhantomData;

use crate::{eval::Eval, op::Operator, solution::Population};

// TODO: Manually impl common traits
#[must_use]
pub struct OnSelected<T, P, S, E> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) _marker: PhantomData<fn() -> (P, S, E)>,
}

impl<T, P, S, E> Operator<P, S, E, Vec<S::Individual>> for OnSelected<T, P, S, E>
where
    T: Operator<P, Vec<S::Individual>, E, Output = ()>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    type Output = Vec<S::Individual>;

    type Error = T::Error;

    fn apply(
        &mut self,
        _solution: &mut S,
        problem: &P,
        eval: &mut E,
        mut selected: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(&mut selected, problem, eval, ())?;
        Ok(selected)
    }
}
