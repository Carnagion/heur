use crate::{
    eval::Eval,
    solution::{Individual, Population},
};

use super::Operator;

mod for_each;
pub use for_each::ForEach;

pub mod select;

pub mod combine;

pub mod insert;

#[inline]
pub fn for_each<P, S, E, In, T>(op: T) -> ForEach<T>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population + ?Sized,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    ForEach(op)
}
