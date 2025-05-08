use core::convert::Infallible;

use alloc::vec::Vec;

use heur_core::{eval::Eval, op::Operator, solution::Population};

use super::Insert;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct ElitistInserter {
    // NOTE: We store the indices as part of the struct itself to avoid re-allocating a new vec for them every time we
    //       need to insert individuals.
    indices: Vec<usize>,
}

impl ElitistInserter {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
        }
    }
}

impl<P, S, E> Operator<P, S, E, Vec<S::Individual>> for ElitistInserter
where
    S: Population<Individual: Clone> + AsMut<[S::Individual]>,
    E: Eval<P, S::Individual, Objective: Ord>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.insert(population, problem, eval, combined)
    }
}

impl<P, S, E> Insert<P, S, E> for ElitistInserter
where
    S: Population<Individual: Clone> + AsMut<[S::Individual]>,
    E: Eval<P, S::Individual, Objective: Ord>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        let population = population.as_mut();

        // Create a list of indices to individuals in the solution sorted by their objective values
        self.indices.extend(0..population.len());
        self.indices.sort_by_cached_key(|&idx| {
            let solution = &population[idx];
            eval.eval(solution, problem)
        });

        for (idx, offspring) in self.indices.drain(..).zip(combined) {
            let parent = &mut population[idx];
            *parent = offspring;
        }

        Ok(())
    }
}
