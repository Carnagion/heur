pub trait Population {
    type Individual;
}

impl<T> Population for Vec<T> {
    type Individual = T;
}

impl<T, const N: usize> Population for [T; N] {
    type Individual = T;
}
