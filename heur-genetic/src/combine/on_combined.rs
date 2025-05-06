use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use alloc::vec::Vec;

use heur_core::{eval::Eval, op::Operator, solution::Population};

#[must_use]
pub struct OnCombined<T, P, S, E> {
    pub(super) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, S, E)>,
}

impl<T, P, S, E> Operator<P, S, E, Vec<S::Individual>> for OnCombined<T, P, S, E>
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
        mut combined: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(&mut combined, problem, eval, ())?;
        Ok(combined)
    }
}

impl<T, P, S, E> Debug for OnCombined<T, P, S, E>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OnCombined")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T, P, S, E> Copy for OnCombined<T, P, S, E> where T: Copy {}

impl<T, P, S, E> Clone for OnCombined<T, P, S, E>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            op: self.op.clone(),
            marker: self.marker,
        }
    }
}

impl<T, P, S, E> Eq for OnCombined<T, P, S, E> where T: Eq {}

impl<T, P, S, E> PartialEq for OnCombined<T, P, S, E>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
    }
}

impl<T, P, S, E> Hash for OnCombined<T, P, S, E>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.op.hash(state);
        self.marker.hash(state);
    }
}
