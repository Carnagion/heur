use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Problem, eval::Eval};

use super::Solution;

// TODO: Manually implement common traits
#[repr(transparent)]
pub struct Reencoded<P, S> {
    problem: P,
    marker: PhantomData<fn() -> S>,
}

impl<P, S> Problem for Reencoded<P, S>
where
    P: Problem,
    S: Solution<Individual = <P::Solution as Solution>::Individual>,
{
    type Solution = S;

    type Eval = Reeval<P::Eval>;
}

impl<P, S> Reencoded<P, S> {
    #[must_use]
    pub fn from_ref(ptr: &P) -> &Self {
        // SAFETY: `Reencoded<P, S>` is `repr(transparent)` and only contains a `P`.
        unsafe { &*(ptr as *const P as *const Self) }
    }

    #[must_use]
    pub fn from_mut(ptr: &mut P) -> &mut Self {
        // SAFETY: `Reencoded<P, S>` is `repr(transparent)` and only contains a `P`.
        unsafe { &mut *(ptr as *mut P as *mut Self) }
    }
}

impl<P, S> Deref for Reencoded<P, S> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.problem
    }
}

impl<P, S> DerefMut for Reencoded<P, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.problem
    }
}

impl<P, S> AsRef<P> for Reencoded<P, S> {
    fn as_ref(&self) -> &P {
        &self.problem
    }
}

impl<P, S> AsMut<P> for Reencoded<P, S> {
    fn as_mut(&mut self) -> &mut P {
        &mut self.problem
    }
}

impl<P, S> From<P> for Reencoded<P, S> {
    fn from(problem: P) -> Self {
        Self {
            problem,
            marker: PhantomData,
        }
    }
}

impl<'a, P, S> From<&'a P> for &'a Reencoded<P, S> {
    fn from(ptr: &'a P) -> Self {
        Reencoded::from_ref(ptr)
    }
}

impl<'a, P, S> From<&'a mut P> for &'a mut Reencoded<P, S> {
    fn from(ptr: &'a mut P) -> Self {
        Reencoded::from_mut(ptr)
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Reeval<E>(pub E);

impl<P, S, E> Eval<Reencoded<P, S>> for Reeval<E>
where
    P: Problem<Eval = E>,
    S: Solution<Individual = <P::Solution as Solution>::Individual>,
    E: Eval<P>,
{
    type Objective = E::Objective;

    fn eval(&mut self, solution: &S::Individual, problem: &Reencoded<P, S>) -> Self::Objective {
        self.0.eval(solution, problem)
    }
}

impl<E> Reeval<E> {
    #[must_use]
    pub fn from_ref(ptr: &E) -> &Self {
        // SAFETY: `Reeval<E>` is `repr(transparent)` and only contains an `E`.
        unsafe { &*(ptr as *const E as *const Self) }
    }

    #[must_use]
    pub fn from_mut(ptr: &mut E) -> &mut Self {
        // SAFETY: `Reeval<E>` is `repr(transparent)` and only contains an `E`.
        unsafe { &mut *(ptr as *mut E as *mut Self) }
    }
}

impl<E> From<E> for Reeval<E> {
    fn from(eval: E) -> Self {
        Self(eval)
    }
}

impl<'a, E> From<&'a E> for &'a Reeval<E> {
    fn from(ptr: &'a E) -> Self {
        Reeval::from_ref(ptr)
    }
}

impl<'a, E> From<&'a mut E> for &'a mut Reeval<E> {
    fn from(ptr: &'a mut E) -> Self {
        Reeval::from_mut(ptr)
    }
}

impl<E> AsRef<E> for Reeval<E> {
    fn as_ref(&self) -> &E {
        &self.0
    }
}

impl<E> AsMut<E> for Reeval<E> {
    fn as_mut(&mut self) -> &mut E {
        &mut self.0
    }
}

impl<E> Deref for Reeval<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for Reeval<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
