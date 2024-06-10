use std::{convert::Infallible, ops::IndexMut};

use rand::{
    distributions::{Bernoulli, BernoulliError, Distribution},
    rngs::ThreadRng,
    Rng,
};

use crate::op::Operator;

use super::Mutate;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlipBit<R> {
    rng: R,
}

impl FlipBit<ThreadRng> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl<R> FlipBit<R>
where
    R: Rng,
{
    #[inline]
    #[must_use]
    pub fn with_rng(rng: R) -> Self {
        Self { rng }
    }

    #[inline]
    fn flip_bit<S>(&mut self, len: usize, solution: &mut S)
    where
        S: IndexMut<usize, Output = bool>,
    {
        // NOTE: We need to check that the solution is not empty, because `Rng::gen_range` panics on empty ranges.
        if len > 0 {
            let idx = self.rng.gen_range(0..len);
            solution[idx] = !solution[idx];
        }
    }
}

impl<P, E, R> Operator<Vec<bool>, P, E> for FlipBit<R>
where
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Vec<bool>,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.flip_bit(solution.len(), solution);
        Ok(())
    }
}

impl<P, E, R, const N: usize> Operator<[bool; N], P, E> for FlipBit<R>
where
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut [bool; N],
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.flip_bit(solution.len(), solution);
        Ok(())
    }
}

impl<S, P, E, R> Mutate<S, P, E> for FlipBit<R>
where
    Self: Operator<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.apply(solution, problem, eval, ())?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlipAllBits<R> {
    rng: R,
    dist: Bernoulli,
}

impl FlipAllBits<ThreadRng> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            dist: Bernoulli::new(0.5).unwrap(), // PANICS: 0.5 is a valid probability
        }
    }
}

impl<R> FlipAllBits<R>
where
    R: Rng,
{
    #[inline]
    #[must_use]
    pub fn with_prob_and_rng(prob: f64, rng: R) -> Result<Self, BernoulliError> {
        let dist = Bernoulli::new(prob)?;
        Ok(Self { rng, dist })
    }

    #[inline]
    fn flip_all_bits<'a, S>(&mut self, solution: S)
    where
        S: IntoIterator<Item = &'a mut bool>,
    {
        for bit in solution {
            if self.dist.sample(&mut self.rng) {
                *bit = !*bit;
            }
        }
    }
}

impl<P, E, R> Operator<Vec<bool>, P, E> for FlipAllBits<R>
where
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Vec<bool>,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.flip_all_bits(solution);
        Ok(())
    }
}

impl<P, E, R, const N: usize> Operator<[bool; N], P, E> for FlipAllBits<R>
where
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut [bool; N],
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.flip_all_bits(solution);
        Ok(())
    }
}

impl<S, P, E, R> Mutate<S, P, E> for FlipAllBits<R>
where
    Self: Operator<S, P, E>,
{
    #[inline]
    fn mutate(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.apply(solution, problem, eval, ())?;
        Ok(())
    }
}
