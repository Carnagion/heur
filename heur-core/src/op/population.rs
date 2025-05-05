use std::marker::PhantomData;

use crate::{
    eval::Eval,
    solution::{Individual, IterMut, Population},
};

use super::Operator;

mod for_each;
pub use for_each::ForEach;

pub fn for_each<P, S, E, In, T>(op: T) -> ForEach<T, P, S, E, In>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population + for<'a> IterMut<'a, Item = S::Individual>,
    E: Eval<P, S::Individual>,
{
    ForEach {
        op,
        marker: PhantomData,
    }
}
