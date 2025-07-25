#[cfg(feature = "alloc")]
use alloc::boxed::Box;

pub mod accept;

pub mod stop;

pub trait Condition {
    fn and<U>(self, other: U) -> And<Self, U>
    where
        Self: Sized,
    {
        And {
            first: self,
            second: other,
        }
    }

    fn or<U>(self, other: U) -> Or<Self, U>
    where
        Self: Sized,
    {
        Or {
            first: self,
            second: other,
        }
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }
}

impl<T> Condition for &mut T where T: Condition + ?Sized {}

#[cfg(feature = "alloc")]
impl<T> Condition for Box<T> where T: Condition + ?Sized {}

#[cfg(feature = "either")]
impl<L, R> Condition for either::Either<L, R>
where
    L: Condition,
    R: Condition,
{
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct And<T, U> {
    first: T,
    second: U,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Or<T, U> {
    first: T,
    second: U,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct Not<T>(T);

impl<T, U> Condition for And<T, U>
where
    T: Condition,
    U: Condition,
{
}

impl<T, U> Condition for Or<T, U>
where
    T: Condition,
    U: Condition,
{
}

impl<T> Condition for Not<T> where T: Condition {}
