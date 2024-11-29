use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    iter,
    mem,
};

use rand::{
    distributions::{Bernoulli, Distribution},
    Rng,
};

use crate::{eval::Eval, op::Operator, solution::Population};

use super::Combine;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UniformCrossover<R> {
    dist: Bernoulli,
    rng: R,
}

impl<P, S, E, R, T> Operator<P, S, E, Vec<S::Individual>> for UniformCrossover<R>
where
    S: Population<Individual: Clone>,
    for<'a> &'a mut S::Individual: IntoIterator<Item = &'a mut T>,
    E: Eval<P, S::Individual>,
    R: Rng,
{
    type Output = Vec<S::Individual>;

    type Error = UniformCrossoverError;

    
    fn apply(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.combine(population, problem, eval, selected)
    }
}

impl<P, S, E, R, T> Combine<P, S, E> for UniformCrossover<R>
where
    S: Population<Individual: Clone>,
    for<'a> &'a mut S::Individual: IntoIterator<Item = &'a mut T>,
    E: Eval<P, S::Individual>,
    R: Rng,
{
    fn combine(
        &mut self,
        _population: &S,
        _problem: &P,
        _eval: &mut E,
        mut selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        // Ensure that we have an even number of parents so we can crossover every pair
        let selection_size = selected.len();
        if selection_size % 2 != 0 {
            return Err(UniformCrossoverError::InvalidSize { selection_size });
        }

        // TODO: Use `array_chunks_mut` when it gets stabilised
        for parents in selected.chunks_exact_mut(2) {
            let [left, right] = parents else {
                unreachable!() // PANICS: We have an even number of elements and ask for chunks of 2
            };

            // Crossover the two parents, producing offspring in-place
            for (left, right) in iter::zip(left, right) {
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
    InvalidSize { selection_size: usize },
}

impl Display for UniformCrossoverError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize { selection_size } => write!(
                formatter,
                "cannot combine {} individuals together - number of selected individuals must be \
                 a multiple of 2",
                selection_size,
            ),
        }
    }
}

impl Error for UniformCrossoverError {}
