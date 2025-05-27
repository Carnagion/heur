use core::convert::Infallible;

use alloc::vec::Vec;

use heur_core::{Problem, eval::Eval, op::Operator, solution::Population};

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

impl<P, S> Operator<P, Vec<S::Individual>> for ElitistInserter
where
    P: Problem<Solution = S>,
    S: Population + AsMut<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        combined: Vec<S::Individual>,
    ) -> Result<Self::Output, Self::Error> {
        self.insert(population, eval, problem, combined)
    }
}

impl<P, S> Insert<P> for ElitistInserter
where
    P: Problem<Solution = S>,
    S: Population + AsMut<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
{
    fn insert(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
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
