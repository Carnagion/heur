use std::{collections::VecDeque, convert::Infallible};

use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

use crate::{eval::Eval, op::Operator, solution::Individual};

use super::Mutate;

// TODO: Impl `Operator` and `Mutate` for bitstring types from crate like `bitvec` and `im::Vector<bool>`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlipBit<R> {
    rng: R,
}

impl<R> FlipBit<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

macro_rules! make_flip_bit_operator {
    (
        $({ $($generics:tt)* })? Operator<$p:ty, $s:ty, $e:ty> for $op:ty
        $(where $($bounds:tt)*)?
    ) => {
        impl $(< $($generics)* >)? Operator<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            type Output = ();

            type Error = Infallible;

            #[inline]
            fn apply(
                &mut self,
                _problem: &$p,
                solution: &mut $s,
                _eval: &mut $e,
                _input: (),
            ) -> Result<Self::Output, Self::Error> {
                // NOTE: We need to check that the solution is not empty, because `Rng::gen_range` panics on empty ranges.
                if !solution.is_empty() {
                    let idx = self.rng.gen_range(0..solution.len());
                    solution[idx] = !solution[idx];
                }
                Ok(())
            }
        }

        impl $(< $($generics)* >)? Mutate<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            #[inline]
            fn mutate(
                &mut self,
                problem: &$p,
                solution: &mut $s,
                eval: &mut $e,
            ) -> Result<(), Self::Error> {
                self.apply(problem, solution, eval, ())
            }
        }
    };
}

make_flip_bit_operator! {
    {P, E, R} Operator<P, Individual<Vec<bool>>, E> for FlipBit<R>
    where
        E: Eval<P, Vec<bool>>,
        R: Rng,
}

make_flip_bit_operator! {
    {P, E, R, const N: usize} Operator<P, Individual<[bool; N]>, E> for FlipBit<R>
    where
        E: Eval<P, [bool; N]>,
        R: Rng,
}

make_flip_bit_operator! {
    {P, E, R} Operator<P, Individual<[bool]>, E> for FlipBit<R>
    where
        E: Eval<P, [bool]>,
        R: Rng,
}

make_flip_bit_operator! {
    {P, E, R} Operator<P, Individual<Box<[bool]>>, E> for FlipBit<R>
    where
        E: Eval<P, Box<[bool]>>,
        R: Rng,
}

make_flip_bit_operator! {
    {P, E, R} Operator<P, Individual<VecDeque<bool>>, E> for FlipBit<R>
    where
        E: Eval<P, VecDeque<bool>>,
        R: Rng,
}

// TODO: Impl `Operator` and `Mutate` for bitstring types from crate like `bitvec` and `im::Vector<bool>`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlipAllBits<R> {
    dist: Bernoulli,
    rng: R,
}

impl<R> FlipAllBits<R> {
    pub fn new(dist: Bernoulli, rng: R) -> Self {
        Self { dist, rng }
    }
}

impl<R> Default for FlipAllBits<R>
where
    R: Default,
{
    fn default() -> Self {
        let dist = Bernoulli::new(0.5).unwrap(); // PANICS: 0.5 is a valid probability
        Self::new(dist, R::default())
    }
}

macro_rules! make_flip_all_bits_operator {
    (
        $({ $($generics:tt)* })? Operator<$p:ty, $s:ty, $e:ty> for $op:ty
        $(where $($bounds:tt)*)?
    ) => {
        impl $(< $($generics)* >)? Operator<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            type Output = ();

            type Error = Infallible;

            #[inline]
            fn apply(
                &mut self,
                _problem: &$p,
                solution: &mut $s,
                _eval: &mut $e,
                _input: (),
            ) -> Result<Self::Output, Self::Error> {
                for bit in solution.iter_mut() {
                    if self.dist.sample(&mut self.rng) {
                        *bit = !*bit;
                    }
                }
                Ok(())
            }
        }

        impl $(< $($generics)* >)? Mutate<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            #[inline]
            fn mutate(
                &mut self,
                problem: &$p,
                solution: &mut $s,
                eval: &mut $e,
            ) -> Result<(), Self::Error> {
                self.apply(problem, solution, eval, ())
            }
        }
    };
}

make_flip_all_bits_operator! {
    {P, E, R} Operator<P, Individual<Vec<bool>>, E> for FlipAllBits<R>
    where
        E: Eval<P, Vec<bool>>,
        R: Rng,
}

make_flip_all_bits_operator! {
    {P, E, R, const N: usize} Operator<P, Individual<[bool; N]>, E> for FlipAllBits<R>
    where
        E: Eval<P, [bool; N]>,
        R: Rng,
}

make_flip_all_bits_operator! {
    {P, E, R} Operator<P, Individual<[bool]>, E> for FlipAllBits<R>
    where
        E: Eval<P, [bool]>,
        R: Rng,
}

make_flip_all_bits_operator! {
    {P, E, R} Operator<P, Individual<Box<[bool]>>, E> for FlipAllBits<R>
    where
        E: Eval<P, Box<[bool]>>,
        R: Rng,
}

make_flip_all_bits_operator! {
    {P, E, R} Operator<P, Individual<VecDeque<bool>>, E> for FlipAllBits<R>
    where
        E: Eval<P, VecDeque<bool>>,
        R: Rng,
}
