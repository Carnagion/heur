#[cfg(feature = "alloc")]
use alloc::{boxed::Box, collections::VecDeque, vec::Vec};

use super::Solution;

// TODO: 1. Impl `Population` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, and/or `im`
//       2. Add `#[diagnostic::on_unimplemented]`
pub trait Population: Solution {}

#[cfg(feature = "alloc")]
impl<T> Solution for Vec<T> {
    type Individual = T;
}

#[cfg(feature = "alloc")]
impl<T> Population for Vec<T> {}

// NOTE: While it wouldn't actually be possible to use a `[T]` as a solution (it's unsized and therefore can't be initialised
//       with an initialisation operator), this impl allows any `Box<S>` to impl `Solution` as long as `S: Solution`, including
//       the case when `S` = `[T]`.
impl<T> Solution for [T] {
    type Individual = T;
}

// NOTE: See the note above on the `Solution` impl for `[T]`.
impl<T> Population for [T] {}

impl<T, const N: usize> Solution for [T; N] {
    type Individual = T;
}

impl<T, const N: usize> Population for [T; N] {}

#[cfg(feature = "alloc")]
impl<S> Solution for Box<S>
where
    S: Solution + ?Sized,
{
    type Individual = S::Individual;
}

#[cfg(feature = "alloc")]
impl<P> Population for Box<P> where P: Population + ?Sized {}

#[cfg(feature = "alloc")]
impl<T> Solution for VecDeque<T> {
    type Individual = T;
}

#[cfg(feature = "alloc")]
impl<T> Population for VecDeque<T> {}
