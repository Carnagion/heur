use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Problem, eval::Eval};

use super::Solution;

#[repr(transparent)]
pub struct Reencoded<T, U> {
    inner: T,
    marker: PhantomData<fn() -> U>,
}

impl<T, U> Reencoded<T, U> {
    #[must_use]
    pub fn from_ref(ptr: &T) -> &Self {
        // SAFETY: `Reencoded<T, U>` is `repr(transparent)` and only contains a `T`.
        unsafe { &*(ptr as *const T as *const Self) }
    }

    #[must_use]
    pub fn from_mut(ptr: &mut T) -> &mut Self {
        // SAFETY: `Reencoded<T, U>` is `repr(transparent)` and only contains a `T`.
        unsafe { &mut *(ptr as *mut T as *mut Self) }
    }
}

impl<P, S> Problem for Reencoded<P, S>
where
    P: Problem,
    S: Solution<Individual = <P::Solution as Solution>::Individual>,
{
    type Solution = S;

    type Eval = Reencoded<P::Eval, S>;
}

impl<P, S, E> Eval<Reencoded<P, S>> for Reencoded<E, S>
where
    P: Problem<Eval = E>,
    S: Solution<Individual = <P::Solution as Solution>::Individual>,
    E: Eval<P>,
{
    type Objective = E::Objective;

    fn eval(&mut self, solution: &S::Individual, problem: &Reencoded<P, S>) -> Self::Objective {
        self.inner.eval(solution, problem)
    }
}

impl<T: Debug, U> Debug for Reencoded<T, U> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Reencoded")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<T: Copy, U> Copy for Reencoded<T, U> {}

impl<T: Clone, U> Clone for Reencoded<T, U> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: self.marker,
        }
    }
}

impl<T: Eq, U> Eq for Reencoded<T, U> {}

impl<T: PartialEq, U> PartialEq for Reencoded<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Hash, U> Hash for Reencoded<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T, U> Deref for Reencoded<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, U> DerefMut for Reencoded<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, U> AsRef<T> for Reencoded<T, U> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T, U> AsMut<T> for Reencoded<T, U> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T, U> From<T> for Reencoded<T, U> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}

impl<'a, T, U> From<&'a T> for &'a Reencoded<T, U> {
    fn from(ptr: &'a T) -> Self {
        Reencoded::from_ref(ptr)
    }
}

impl<'a, T, U> From<&'a mut T> for &'a mut Reencoded<T, U> {
    fn from(ptr: &'a mut T) -> Self {
        Reencoded::from_mut(ptr)
    }
}
