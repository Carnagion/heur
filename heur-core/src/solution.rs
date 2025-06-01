use core::ops::{Deref, DerefMut};

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};

mod reencode;
pub use reencode::Reencoded;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Solution {
    type Individual;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Individual<T>(pub T);

impl<T> Individual<T> {
    #[must_use]
    pub fn from_ref(ptr: &T) -> &Self {
        // SAFETY: `Individual<T>` is `repr(transparent)` and only contains a `T`.
        unsafe { &*(ptr as *const T as *const Self) }
    }

    #[must_use]
    pub fn from_mut(ptr: &mut T) -> &mut Self {
        // SAFETY: `Individual<T>` is `repr(transparent)` and only contains a `T`.
        unsafe { &mut *(ptr as *mut T as *mut Self) }
    }
}

impl<T> Solution for Individual<T> {
    type Individual = T;
}

impl<T> From<T> for Individual<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<'a, T> From<&'a T> for &'a Individual<T> {
    fn from(ptr: &'a T) -> Self {
        Individual::from_ref(ptr)
    }
}

impl<'a, T> From<&'a mut T> for &'a mut Individual<T> {
    fn from(ptr: &'a mut T) -> Self {
        Individual::from_mut(ptr)
    }
}

impl<T> AsRef<T> for Individual<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for Individual<T> {
    fn as_mut(&mut self) -> &mut T {
        self
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

// TODO: 1. Impl `Population` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, and/or `im`
//       2. Add `#[diagnostic::on_unimplemented]`
pub trait Population: Solution {}

impl<T, const N: usize> Solution for [T; N] {
    type Individual = T;
}

impl<T, const N: usize> Population for [T; N] {}

#[cfg(feature = "alloc")]
impl<T> Solution for Vec<T> {
    type Individual = T;
}

#[cfg(feature = "alloc")]
impl<T> Population for Vec<T> {}

#[cfg(feature = "alloc")]
impl<T> Solution for Box<[T]> {
    type Individual = T;
}

#[cfg(feature = "alloc")]
impl<T> Population for Box<[T]> {}

#[cfg(feature = "alloc")]
impl<T, const N: usize> Solution for Box<[T; N]> {
    type Individual = T;
}

#[cfg(feature = "alloc")]
impl<T, const N: usize> Population for Box<[T; N]> {}

// NOTE: We need these traits due to a possible bug in `rustc` where trying to prove that `Evaluated<S, O>` impls
//       `IntoIterator` puts the trait solver into a loop and leads to an overflow. Conceptually, `T: for<'a> Iter<'a>`
//       is exactly the same as `for<'a> &'a T: IntoIterator<Item = &'a U>`, but the latter leads to E0275 ("overflow
//       evaluating the requirement ...") while the former works perfectly.
pub trait Iter<'a> {
    type Item: 'a;

    type Iter: Iterator<Item = &'a Self::Item>;

    fn iter(&'a self) -> Self::Iter;
}

// NOTE: See the above note on `Iter<'a>`.
pub trait IterMut<'a>: Iter<'a> {
    type IterMut: Iterator<Item = &'a mut Self::Item>;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}

impl<'a, I, T> Iter<'a> for I
where
    I: 'a,
    &'a I: IntoIterator<Item = &'a T>,
    T: 'a,
{
    type Item = T;

    type Iter = <&'a Self as IntoIterator>::IntoIter;

    fn iter(&'a self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a, I, T> IterMut<'a> for I
where
    I: 'a,
    &'a I: IntoIterator<Item = &'a T>,
    &'a mut I: IntoIterator<Item = &'a mut T>,
    T: 'a,
{
    type IterMut = <&'a mut Self as IntoIterator>::IntoIter;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.into_iter()
    }
}
