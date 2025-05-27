use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use alloc::vec::Vec;

use rand::{Rng, seq::IndexedRandom};

use heur_core::{Problem, eval::Eval, op::Operator, solution::Population};

use super::Select;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct TournamentSelector<R> {
    // TODO: Should these be `NonZeroUsize`?
    tournament_size: usize,
    selection_size: usize,
    rng: R,
}

impl<R> TournamentSelector<R> {
    pub fn new(tournament_size: usize, selection_size: usize, rng: R) -> Self {
        Self {
            tournament_size,
            selection_size,
            rng,
        }
    }
}

impl<P, S, R> Operator<P> for TournamentSelector<R>
where
    P: Problem<Solution = S>,
    // TODO: Should we use `IteratorRandom::choose_multiple` instead to work with solutions that don't impl `AsRef<[T]>`?
    S: Population<Individual: Clone> + AsRef<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
    R: Rng,
{
    type Output = Vec<S::Individual>;

    type Error = TournamentSelectError;

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

impl<P, S, R> Select<P> for TournamentSelector<R>
where
    P: Problem<Solution = S>,
    S: Population<Individual: Clone> + AsRef<[S::Individual]>,
    <P::Eval as Eval<P>>::Objective: Ord,
    R: Rng,
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

        // Ensure that we can run tournaments with `tournament_size` individuals
        if self.tournament_size > population.len() {
            return Err(TournamentSelectError::InvalidSize {
                tournament_size: self.tournament_size,
                population_size: population.len(),
            });
        }

        // Ensure that we can actually select individuals
        // NOTE: We early return here so that the compiler can remove bounds checks from each iteration below.
        if population.is_empty() || self.tournament_size == 0 {
            return Err(TournamentSelectError::NoSelection);
        }

        // Run tournaments `selection_size` times and select the best individual from each
        // NOTE: This does not guarantee that we won't select the same individual(s) multiple times. We also don't check
        //       whether `selection_size <= population.len()`, so in case `selection_size` is larger than the number of
        //       individuals available, we will invariably end up selecting repeated individuals, but this is fine.
        selected.clear();
        selected.reserve(self.selection_size);
        for _ in 0..self.selection_size {
            let winner = population
                .choose_multiple(&mut self.rng, self.tournament_size)
                .max_by_key(|solution| eval.eval(solution, problem))
                .cloned()
                .unwrap(); // PANICS: We have checked above that the population is not empty and `tournament_size > 0`.
            selected.push(winner);
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TournamentSelectError {
    InvalidSize {
        tournament_size: usize,
        population_size: usize,
    },
    NoSelection,
}

impl Display for TournamentSelectError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize {
                tournament_size,
                population_size,
            } => write!(
                formatter,
                "cannot select since tournament size ({}) is bigger than population size ({})",
                tournament_size, population_size,
            ),
            Self::NoSelection => write!(
                formatter,
                "cannot select since tournament size or population size is 0",
            ),
        }
    }
}

impl Error for TournamentSelectError {}
