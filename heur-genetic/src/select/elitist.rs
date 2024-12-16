use std::{cmp::Reverse, convert::Infallible};

use heur_core::{eval::Eval, op::Operator, solution::Population};

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

impl<P, S, E> Operator<P, S, E> for ElitistSelector
where
    S: Population<Individual: Clone>,
    E: Eval<P, S::Individual>,
{
    type Output = Vec<S::Individual>;

    type Error = Infallible;

    fn apply(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.select(population, problem, eval)
    }
}

impl<P, S, E> Select<P, S, E> for ElitistSelector
where
    S: Population<Individual: Clone>,
    E: Eval<P, S::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        let mut selected = Vec::with_capacity(self.selection_size);
        self.select_into(population, problem, eval, &mut selected)?;
        Ok(selected)
    }

    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        // Create a list of indices to individuals in the solution sorted by their objective values
        self.indices.extend(0..population.len());
        self.indices.sort_by_cached_key(|&idx| {
            // PANICS: The index is valid because it's between `0` and `population.len()`.
            let solution = population.get(idx).unwrap();

            // NOTE: We reverse the comparison order because we need the best (largest) objective values to be at the front.
            Reverse(eval.eval(solution, problem))
        });

        // Pick the `selection_size` best indivdiuals
        // NOTE: We don't check whether `selection_size <= population.len()`, so if `selection_size` is greater than the
        //       number of individuals available, we will invariably end up selecting repeated individuals, but this is fine.
        selected.clear();
        selected.reserve(self.selection_size);
        selected.extend(
            self.indices
                .iter()
                .cycle()
                .take(self.selection_size)
                .map(|&idx| population.get(idx).unwrap()) // PANICS: See above.
                .cloned(),
        );

        // Clear the indices so that the next time we select we have a blank state (but with a reusable allocation)
        self.indices.clear();

        Ok(())
    }
}
