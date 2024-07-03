use std::{
    collections::{vec_deque, VecDeque},
    slice,
};

use super::Solution;

// TODO: 1. Impl `Population` for types from `smallvec`, `arrayvec`, `tinyvec`, `heapless`, and/or `im`
//       2. Add `#[diagnostic::on_unimplemented]`
pub trait Population: Solution {
    type Iter<'a>: Iterator<Item = &'a Self::Individual>
    where
        Self: 'a;

    type IterMut<'a>: Iterator<Item = &'a mut Self::Individual>
    where
        Self: 'a;

    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    fn get(&self, index: usize) -> Option<&Self::Individual>;

    #[must_use]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual>;

    #[must_use]
    fn iter(&self) -> Self::Iter<'_>;

    #[must_use]
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<T> Solution for Vec<T> {
    type Individual = T;
}

// NOTE: While it wouldn't actually be possible to use a `[T]` as a solution (it's unsized and therefore can't be initialised
//       with an initialisation operator), this impl allows any `Box<S>` to impl `Solution` as long as `S: Solution`, including
//       the case when `S` = `[T]`.
impl<T> Solution for [T] {
    type Individual = T;
}

impl<T, const N: usize> Solution for [T; N] {
    type Individual = T;
}

impl<S> Solution for Box<S>
where
    S: Solution + ?Sized,
{
    type Individual = S::Individual;
}

impl<T> Solution for VecDeque<T> {
    type Individual = T;
}

impl<T> Population for Vec<T> {
    type Iter<'a> = slice::Iter<'a, T>
    where
        T: 'a;

    type IterMut<'a> = slice::IterMut<'a, T>
    where
        T: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Individual> {
        self.as_slice().get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual> {
        self.as_mut_slice().get_mut(index)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, const N: usize> Population for [T; N] {
    type Iter<'a> = slice::Iter<'a, T>
    where
        T: 'a;

    type IterMut<'a> = slice::IterMut<'a, T>
    where
        T: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Individual> {
        self.as_slice().get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual> {
        self.as_mut_slice().get_mut(index)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

// NOTE: See the note above on the `Solution` impl for `[T]`.
impl<T> Population for [T] {
    type Iter<'a> = slice::Iter<'a, T>
    where
        T: 'a;

    type IterMut<'a> = slice::IterMut<'a, T>
    where
        T: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Individual> {
        self.get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual> {
        self.get_mut(index)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<S> Population for Box<S>
where
    S: Population + ?Sized,
{
    type Iter<'a> = S::Iter<'a>
    where
        S: 'a;

    type IterMut<'a> = S::IterMut<'a>
    where
        S: 'a;

    #[inline]
    fn len(&self) -> usize {
        S::len(self)
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Individual> {
        self.as_ref().get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual> {
        self.as_mut().get_mut(index)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        S::iter(self)
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        S::iter_mut(self)
    }
}

impl<T> Population for VecDeque<T> {
    type Iter<'a> = vec_deque::Iter<'a, T>
    where
        T: 'a;

    type IterMut<'a> = vec_deque::IterMut<'a, T>
    where
        T: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Individual> {
        self.get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Individual> {
        self.get_mut(index)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}
