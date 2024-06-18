use std::ops::{Deref, DerefMut};

// TODO: Add `#[diagnostic::on_unimplemented]` and impl `Solution` for types from `smallvec`, `arrayvec`, `im`,
//       `heapless`, and maybe `tinyvec`
pub trait Solution {
    type Individual;
}

// TODO: Impl `Display`, `FromStr`, and other appropriate traits from `std`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Individual<T>(pub T);

impl<T> Individual<T> {
    #[inline]
    #[must_use]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    // NOTE: This is an associated function and not a method to prevent confusion with any methods named `into_inner` on `T`
    //       (since `Individual<T>` derefs to `T`).
    #[inline]
    #[must_use]
    pub fn into_inner(this: Self) -> T {
        this.0
    }
}

impl<T> From<T> for Individual<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Individual<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Individual<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// NOTE: We need to wrap up "individual" solutions (as opposed to population-based solutions) in an `Individual<T>` because
//       specialisation does not exist yet.
// TODO: Change this to a blanket impl over all `T` if possible and if specialisation ever stabilises
impl<T> Solution for Individual<T> {
    type Individual = T;
}

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Population: Solution {}

impl<T> Solution for Vec<T> {
    type Individual = T;
}

impl<T> Population for Vec<T> {}

impl<T, const N: usize> Solution for [T; N] {
    type Individual = T;
}

impl<T, const N: usize> Population for [T; N] {}

// NOTE: While it wouldn't be possible to produce `[T]` as the output of a metaheuristic or `Solve` type, this impl still
//       allows passing a `&mut [T]` as a population type to population operators. This can be useful if eg. applying operators
//       in isolation or to an already-initialised population without having to `Init`.
impl<T> Solution for [T] {
    type Individual = T;
}

impl<T> Population for [T] {}

impl<S> Solution for Box<S>
where
    S: Solution,
{
    type Individual = S::Individual;
}

impl<S> Population for Box<S> where S: Population {}
