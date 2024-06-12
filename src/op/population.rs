pub mod select;

pub mod combine;

pub mod insert;

// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Population {
    type Individual;
}

impl<T> Population for Vec<T> {
    type Individual = T;
}

impl<T, const N: usize> Population for [T; N] {
    type Individual = T;
}
