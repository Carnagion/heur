use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Problem, eval::Eval};

use super::Solution;

// TODO: Manually implement common traits
#[repr(transparent)]
pub struct Reencoded<T, U> {
    inner: T,
    marker: PhantomData<fn() -> U>,
}

impl<P, S> Problem for Reencoded<P, S>
where
    P: Problem,
    S: Solution<Individual = <P::Solution as Solution>::Individual>,
{
    type Solution = S;

    type Eval = Reencoded<P::Eval, S>;
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
