use std::{collections::VecDeque, convert::Infallible};

use crate::{eval::Eval, op::Operator, solution::Individual};

use super::Search;

// TODO: Impl `Operator` and `Search` for bitstring types from crate like `bitvec` and `im::Vector<bool>`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FirstAscentBitClimb;

impl FirstAscentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

macro_rules! make_first_ascent_bit_climb_operator {
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
                problem: &$p,
                solution: &mut $s,
                eval: &mut $e,
                _input: (),
            ) -> Result<Self::Output, Self::Error> {
                let solution = &mut **solution;

                let objective = eval.eval(problem, solution);

                // NOTE: We iterate over indices here because it's not possible to iterate over the solution mutably
                //       while also passing it immutably to `eval` for evaluation.
                let next = (0..solution.len()).find_map(|idx| {
                    let bit = solution[idx];
                    solution[idx] = !bit;
                    let improved = (eval.eval(problem, solution) > objective).then_some(idx);
                    solution[idx] = bit;
                    improved
                });

                if let Some(idx) = next {
                    solution[idx] = !solution[idx];
                }

                Ok(())
            }
        }

        impl $(< $($generics)* >)? Search<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            #[inline]
            fn search(
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

make_first_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<Vec<bool>>, E> for FirstAscentBitClimb
    where
        E: Eval<P, Vec<bool>>,
}

make_first_ascent_bit_climb_operator! {
    {P, E, const N: usize} Operator<P, Individual<[bool; N]>, E> for FirstAscentBitClimb
    where
        E: Eval<P, [bool; N]>,
}

make_first_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<[bool]>, E> for FirstAscentBitClimb
    where
        E: Eval<P, [bool]>,
}

make_first_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<Box<[bool]>>, E> for FirstAscentBitClimb
    where
        E: Eval<P, Box<[bool]>>,
}

make_first_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<VecDeque<bool>>, E> for FirstAscentBitClimb
    where
        E: Eval<P, VecDeque<bool>>,
}

// TODO: Impl `Operator` and `Search` for bitstring types from crate like `bitvec` and `im::Vector<bool>`
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SteepestAscentBitClimb;

impl SteepestAscentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

macro_rules! make_steepest_ascent_bit_climb_operator {
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
                problem: &$p,
                solution: &mut $s,
                eval: &mut $e,
                _input: (),
            ) -> Result<Self::Output, Self::Error> {
                let solution = &mut **solution;

                // NOTE: See the note above regarding iteration over indices.
                let best = (0..solution.len()).max_by_key(|&idx| {
                    let bit = solution[idx];
                    solution[idx] = !bit;
                    let objective = eval.eval(problem, solution);
                    solution[idx] = bit;
                    objective
                });

                if let Some(idx) = best {
                    solution[idx] = !solution[idx];
                }

                Ok(())
            }
        }

        impl $(< $($generics)* >)? Search<$p, $s, $e> for $op
        $(where $($bounds)*)?
        {
            #[inline]
            fn search(
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

make_steepest_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<Vec<bool>>, E> for SteepestAscentBitClimb
    where
        E: Eval<P, Vec<bool>>,
}

make_steepest_ascent_bit_climb_operator! {
    {P, E, const N: usize} Operator<P, Individual<[bool; N]>, E> for SteepestAscentBitClimb
    where
        E: Eval<P, [bool; N]>,
}

make_steepest_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<[bool]>, E> for SteepestAscentBitClimb
    where
        E: Eval<P, [bool]>,
}

make_steepest_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<Box<[bool]>>, E> for SteepestAscentBitClimb
    where
        E: Eval<P, Box<[bool]>>,
}

make_steepest_ascent_bit_climb_operator! {
    {P, E} Operator<P, Individual<VecDeque<bool>>, E> for SteepestAscentBitClimb
    where
        E: Eval<P, VecDeque<bool>>,
}
