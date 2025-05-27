use core::{cmp::Reverse, convert::Infallible};

use alloc::vec::Vec;

use heur_core::{Problem, eval::Eval, op::Operator, solution::Population};

use super::Select;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct ElitistSelector {
    selection_size: usize,
    // NOTE: We store the indices as part of the struct itself to avoid re-allocating a new vec for them every time we
    //       need to select individuals.
    indices: Vec<usize>,
}

impl ElitistSelector {
    pub fn new(selection_size: usize) -> Self {
        Self {
            selection_size,
            indices: Vec::new(),
        }
    }
}

impl<P, S> Operator<P> for ElitistSelector
where
    P: Problem<Solution = S>,
    S: Population<Individual: Clone> + AsRef<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
{
    type Output = Vec<S::Individual>;

    type Error = Infallible;

    fn apply(
        &mut self,
        population: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        self.select(population, eval, problem)
    }
}

impl<P, S> Select<P> for ElitistSelector
where
    P: Problem<Solution = S>,
    S: Population<Individual: Clone> + AsRef<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
{
    fn select(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        let mut selected = Vec::with_capacity(self.selection_size);
        self.select_into(population, eval, problem, &mut selected)?;
        Ok(selected)
    }

    fn select_into(
        &mut self,
        population: &S,
        eval: &mut P::Eval,
        problem: &P,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        let population = population.as_ref();

        // Create a list of indices to individuals in the solution sorted by their objective values
        self.indices.extend(0..population.len());
        self.indices.sort_by_cached_key(|&idx| {
            let solution = &population[idx];

            // NOTE: We reverse the comparison order because we need the best (largest) objective values to be at the front.
            Reverse(eval.eval(solution, problem))
        });

        // Pick the `selection_size` best indivdiuals
        // NOTE: We don't check whether `selection_size <= population.len()`, so if `selection_size` is greater than the
        //       number of individuals available, we will invariably end up selecting repeated individuals, but this is fine.
        selected.clear();
        selected.extend(
            self.indices
                .iter()
                .cycle()
                .take(self.selection_size)
                .map(|&idx| &population[idx])
                .cloned(),
        );

        // Clear the indices so that the next time we select we have a blank state (but with a reusable allocation)
        self.indices.clear();

        Ok(())
    }
}
