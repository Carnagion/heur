use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{eval::Eval, solution::Solution};

use super::{Operator, init::Init, mutate::Mutate, search::Search};

#[must_use]
pub struct Hint<T, P, S, E, In = ()> {
    pub(crate) op: T,
    #[allow(clippy::type_complexity)]
    pub(super) marker: PhantomData<fn() -> (P, S, E, In)>,
}

impl<T, P, S, E, In> Debug for Hint<T, P, S, E, In>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Hint")
            .field("op", &self.op)
            .finish_non_exhaustive()
    }
}

impl<T, P, S, E, In> Default for Hint<T, P, S, E, In>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            op: T::default(),
            marker: PhantomData,
        }
    }
}

impl<T, P, S, E, In> Copy for Hint<T, P, S, E, In> where T: Copy {}

impl<T, P, S, E, In> Clone for Hint<T, P, S, E, In>
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

impl<T, P, S, E, In> Eq for Hint<T, P, S, E, In> where T: Eq {}

impl<T, P, S, E, In> PartialEq for Hint<T, P, S, E, In>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
    }
}

impl<T, P, S, E, In> Hash for Hint<T, P, S, E, In>
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

impl<T, P, S, E, In> AsRef<T> for Hint<T, P, S, E, In> {
    fn as_ref(&self) -> &T {
        &self.op
    }
}

impl<T, P, S, E, In> AsMut<T> for Hint<T, P, S, E, In> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.op
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Hint<T, P, S, E, In>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.op.apply(solution, problem, eval, input)
    }
}

impl<T, P, S, E> Init<P, S, E> for Hint<T, P, S, E>
where
    T: Init<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        self.op.init(problem, eval)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        self.op.init_into(solution, problem, eval)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for Hint<T, P, S, E>
where
    T: Mutate<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.mutate(solution, problem, eval)
    }
}

impl<T, P, S, E> Search<P, S, E> for Hint<T, P, S, E>
where
    T: Search<P, S, E>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.op.search(solution, problem, eval)
    }
}
