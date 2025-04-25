use std::convert::Infallible;

use rand::{
    distr::{Bernoulli, Distribution},
    Rng,
};

use heur_core::{
    eval::Eval,
    op::{mutate::Mutate, Operator},
    solution::Individual,
};

use crate::Bits;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FlipBit<R> {
    rng: R,
}

impl<R> FlipBit<R> {
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
    fn mutate(
        &mut self,
        solution: &mut Individual<B>,
        _problem: &P,
        _eval: &mut E,
    ) -> Result<(), Self::Error> {
        // NOTE: We need to check that the solution is not empty, because `Rng::gen_range` panics on empty ranges.
        if !solution.is_empty() {
            let idx = self.rng.random_range(0..solution.len());
            solution.flip(idx).unwrap(); // PANICS: We know that the index is valid
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[must_use]
pub struct FlipAllBits<R> {
    dist: Bernoulli,
    rng: R,
}

impl<R> FlipAllBits<R> {
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
