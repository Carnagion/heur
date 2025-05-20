use core::convert::Infallible;

use rand::{
    Rng,
    distr::{Bernoulli, Distribution},
};

use heur_core::{Problem, op::Operator, solution::Individual};

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

impl<P, S, R> Operator<P> for FlipBit<R>
where
    P: Problem<Solution = Individual<S>>,
    S: Bits,
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        _: &mut P::Eval,
        _: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        // NOTE: We need to check that the solution is not empty, because `Rng::gen_range` panics on empty ranges.
        if !solution.is_empty() {
            let idx = self.rng.random_range(0..solution.len());
            let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
            solution.set(idx, !bit);
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

impl<P, S, R> Operator<P> for FlipAllBits<R>
where
    P: Problem<Solution = Individual<S>>,
    S: Bits,
    R: Rng,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        _: &mut P::Eval,
        _: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        for idx in 0..solution.len() {
            if self.dist.sample(&mut self.rng) {
                let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
                solution.set(idx, !bit);
            }
        }
        Ok(())
    }
}
