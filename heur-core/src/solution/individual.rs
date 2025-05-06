use core::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Deref, DerefMut},
};

use super::Solution;

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Individual<T>(pub T);

impl<T> Individual<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn from_ref(ptr: &T) -> &Self {
        // SAFETY: `Individual<T>` is `repr(transparent)` and only contains a `T`.
        let ptr = ptr as *const T as *const Individual<T>;
        unsafe { &*ptr }
    }

    pub fn from_mut(ptr: &mut T) -> &mut Self {
        // SAFETY: `Individual<T>` is `repr(transparent)` and only contains a `T`.
        let ptr = ptr as *mut T as *mut Individual<T>;
        unsafe { &mut *ptr }
    }

    // NOTE: This is an associated function and not a method to prevent confusion with any methods named `into_inner` on `T`
    //       (since `Individual<T>` derefs to `T`).
    pub fn into_inner(this: Self) -> T {
        this.0
    }
}

impl<T> Solution for Individual<T> {
    type Individual = T;
}

impl<T> Debug for Individual<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Display for Individual<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> AsRef<T> for Individual<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Individual<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
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
