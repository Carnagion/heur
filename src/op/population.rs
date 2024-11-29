use std::marker::PhantomData;

use crate::{
    eval::Eval,
    solution::{Individual, Population},
};

use super::Operator;

mod for_each;
pub use for_each::ForEach;


pub fn for_each<P, S, E, In, T>(op: T) -> ForEach<T, P, S, E, In>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    ForEach {
        op,
        _marker: PhantomData,
    }
}
