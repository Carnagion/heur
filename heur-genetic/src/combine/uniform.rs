use core::{
    error::Error,
    fmt::{self, Display, Formatter},
    iter,
    mem,
};

use rand::{
    Rng,
    distr::{Bernoulli, Distribution},
};

use heur_core::{
    Problem,
    op::Operator,
    solution::{IterMut, Population},
};

use crate::VecPopulation;

use super::Combine;

#[derive(Debug, Copy, Clone, PartialEq)]
#[must_use]
pub struct UniformCrossover<R> {
    dist: Bernoulli,
    rng: R,
}

impl<R> UniformCrossover<R> {
    pub fn new(dist: Bernoulli, rng: R) -> Self {
        Self { dist, rng }
    }
}

impl<P, R> Operator<P, VecPopulation<P>> for UniformCrossover<R>
where
    P: Problem,
    P::Solution: Population<Individual: for<'a> IterMut<'a>>,
    R: Rng,
{
    type Output = VecPopulation<P>;

    type Error = UniformCrossoverError;

    fn apply(
        &mut self,
        population: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        selected: VecPopulation<P>,
    ) -> Result<Self::Output, Self::Error> {
        self.combine(population, eval, problem, selected)
    }
}

impl<P, R> Combine<P> for UniformCrossover<R>
where
    P: Problem,
    P::Solution: Population<Individual: for<'a> IterMut<'a>>,
    R: Rng,
{
    fn combine(
        &mut self,
        _: &P::Solution,
        _: &mut P::Eval,
        _: &P,
        mut selected: VecPopulation<P>,
    ) -> Result<VecPopulation<P>, Self::Error> {
        // Ensure that we have an even number of parents so we can crossover every pair
        let selection_size = selected.len();
        if selection_size % 2 != 0 {
            return Err(UniformCrossoverError::OddSize { selection_size });
        }

        // TODO: Use `array_chunks_mut` when it gets stabilised
        for parents in selected.chunks_exact_mut(2) {
            let [left, right] = parents else {
                unreachable!() // PANICS: We have an even number of elements and ask for chunks of 2
            };

            // Crossover the two parents, producing offspring in-place
            for (left, right) in iter::zip(left.iter_mut(), right.iter_mut()) {
                if self.dist.sample(&mut self.rng) {
                    mem::swap(left, right);
                }
            }
        }

        Ok(selected)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UniformCrossoverError {
    OddSize { selection_size: usize },
}

impl Display for UniformCrossoverError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::OddSize { selection_size } => write!(
                formatter,
                "cannot combine {} individuals together - number of selected individuals must be \
                 a multiple of 2",
                selection_size,
            ),
        }
    }
}

impl Error for UniformCrossoverError {}
