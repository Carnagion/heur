use std::convert::Infallible;

use rand::Rng;

use crate::{eval::Eval, op::Operator, solution::Population};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ElitistSelector<R> {
    selection_size: usize,
    rng: R,
    // NOTE: We store the indices as part of the struct itself to avoid re-allocating a new vec for them every time we
    //       need to select individuals.
    indices: Vec<usize>,
}

impl<R> ElitistSelector<R> {
    #[inline]
    #[must_use]
    pub fn new(selection_size: usize, rng: R) -> Self {
        Self {
            selection_size,
            rng,
            indices: Vec::new(),
        }
    }
}

impl<P, S, E, R> Operator<P, S, E> for ElitistSelector<R>
where
    S: Population<Individual: Clone>,
    E: Eval<P, S::Individual>,
    R: Rng,
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
        // Create a list of indices to individuals in the solution sorted by their objective values
        self.indices.extend(0..population.len());
        self.indices.sort_by_cached_key(|&idx| {
            // PANICS: The index is valid because it's between `0` and `population.len()`.
            let solution = population.get(idx).unwrap();
            eval.eval(solution, problem)
        });

        // Pick the `selection_size` best indivdiuals
        // NOTE: We don't check whether `selection_size <= population.len()`, so if `selection_size` is greater than the
        //       number of individuals available, we will invariably end up selecting repeated individuals, but this is fine.
        let mut selected = Vec::with_capacity(self.selection_size);
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

        Ok(selected)
    }
}
