pub mod accept;

pub mod stop;

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
