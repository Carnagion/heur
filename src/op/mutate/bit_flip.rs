use std::convert::Infallible;

use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

use crate::{
    eval::Eval,
    op::Operator,
    solution::{bits::Bits, Individual},
};

use super::Mutate;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlipBit<R> {
    rng: R,
}

impl<R> FlipBit<R> {
    #[inline]
    #[must_use]
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<P, B, E, R> Operator<P, Individual<B>, E> for FlipBit<R>
where
    B: Bits,
    E: Eval<P, B>,
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.mutate(solution, problem, eval)
    }
}

impl<P, B, E, R> Mutate<P, Individual<B>, E> for FlipBit<R>
where
    B: Bits,
    E: Eval<P, B>,
    R: Rng,
{
    #[inline]
    fn mutate(
        &mut self,
        solution: &mut Individual<B>,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        // NOTE: We need to check that the solution is not empty, because `Rng::gen_range` panics on empty ranges.
        if !solution.is_empty() {
            let idx = self.rng.gen_range(0..solution.len());
            solution.flip(idx).unwrap(); // PANICS: We know that the index is valid
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlipAllBits<R> {
    dist: Bernoulli,
    rng: R,
}

impl<R> FlipAllBits<R> {
    // TODO: Should we take an `f64` probability instead of a `Bernoulli` and return a `Result<Self, BernoulliError>`?
    #[inline]
    #[must_use]
    pub fn new(dist: Bernoulli, rng: R) -> Self {
        Self { dist, rng }
    }
}

impl<P, B, E, R> Operator<P, Individual<B>, E> for FlipAllBits<R>
where
    B: Bits,
    E: Eval<P, B>,
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.mutate(solution, problem, eval)
    }
}

impl<P, B, E, R> Mutate<P, Individual<B>, E> for FlipAllBits<R>
where
    B: Bits,
    E: Eval<P, B>,
    R: Rng,
{
    #[inline]
    fn mutate(
        &mut self,
        solution: &mut Individual<B>,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        for idx in 0..solution.len() {
            if self.dist.sample(&mut self.rng) {
                solution.flip(idx).unwrap(); // PANICS: We know that the index is valid
            }
        }
        Ok(())
    }
}
