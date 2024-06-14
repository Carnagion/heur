use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use rand::{seq::SliceRandom, Rng};

use crate::{
    eval::Eval,
    op::{population::Population, Operator},
};

use super::Select;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Tournament<R> {
    // TODO: Should these be `NonZeroUsize`?
    tournament_size: usize,
    selection_size: usize,
    rng: R,
}

impl<R> Tournament<R>
where
    R: Rng,
{
    #[inline]
    #[must_use]
    pub fn new(tournament_size: usize, selection_size: usize, rng: R) -> Self {
        Self {
            tournament_size,
            selection_size,
            rng,
        }
    }
}

impl<S, P, E, R> Operator<S, P, E> for Tournament<R>
where
    S: Population<Individual: Clone> + Deref<Target = [S::Individual]>,
    E: Eval<S::Individual, P>,
    R: Rng,
{
    type Output = Vec<S::Individual>;

    type Error = TournamentError;

    #[inline]
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

impl<S, P, E, R> Select<S, P, E> for Tournament<R>
where
    S: Population<Individual: Clone> + Deref<Target = [S::Individual]>,
    E: Eval<S::Individual, P>,
    R: Rng,
{
    #[inline]
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
        let population = &**population;

        // Ensure that we can run tournaments with `tournament_size` individuals
        if self.tournament_size > population.len() {
            return Err(TournamentError::InvalidSize {
                tournament_size: self.tournament_size,
                population_size: population.len(),
            });
        }

        // Ensure that we can actually select individuals
        // NOTE: We early return here so that the compiler can remove bounds checks from each iteration below.
        if population.is_empty() || self.tournament_size == 0 {
            return Err(TournamentError::NoSelection);
        }

        // Run tournaments `selection_size` times and select the best individual from each
        selected.clear();
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
pub enum TournamentError {
    InvalidSize {
        tournament_size: usize,
        population_size: usize,
    },
    NoSelection,
}

impl Display for TournamentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSize {
                tournament_size,
                population_size,
            } => write!(
                formatter,
                "tournament size ({}) is bigger than population size ({})",
                tournament_size, population_size
            ),
            Self::NoSelection => write!(formatter, "population contains no individuals to select"),
        }
    }
}

impl Error for TournamentError {}
