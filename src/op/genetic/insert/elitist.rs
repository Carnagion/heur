use std::convert::Infallible;

use crate::{eval::Eval, op::Operator, solution::Population};

use super::Insert;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ElitistInserter {
    // NOTE: We store the indices as part of the struct itself to avoid re-allocating a new vec for them every time we
    //       need to insert individuals.
    indices: Vec<usize>,
}

impl ElitistInserter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
        }
    }
}

impl<P, S, E> Operator<P, S, E, Vec<S::Individual>> for ElitistInserter
where
    S: Population<Individual: Clone>,
    E: Eval<P, S::Individual>,
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
    S: Population<Individual: Clone>,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        // Create a list of indices to individuals in the solution sorted by their objective values
        self.indices.extend(0..population.len());
        self.indices.sort_by_cached_key(|&idx| {
            // PANICS: The index is valid because it's between `0` and `population.len()`.
            let solution = population.get(idx).unwrap();
            eval.eval(solution, problem)
        });

        for (idx, offspring) in self.indices.drain(..).zip(combined) {
            // PANICS: See above.
            let parent = population.get_mut(idx).unwrap();
            *parent = offspring;
        }

        Ok(())
    }
}
