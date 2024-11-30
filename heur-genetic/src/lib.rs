#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]

use heur_core::{eval::Eval, op::Hint, solution::Population};

pub mod select;
use select::Select;

pub mod combine;
use combine::Combine;

pub mod insert;
use insert::Insert;

impl<T, P, S, E> Select<P, S, E> for Hint<T, P, S, E>
where
    T: Select<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn select(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        self.as_mut().select(population, problem, eval)
    }

    fn select_into(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: &mut Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.as_mut()
            .select_into(population, problem, eval, selected)
    }
}

impl<T, P, S, E> Combine<P, S, E> for Hint<T, P, S, E, Vec<S::Individual>>
where
    T: Combine<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn combine(
        &mut self,
        population: &S,
        problem: &P,
        eval: &mut E,
        selected: Vec<S::Individual>,
    ) -> Result<Vec<S::Individual>, Self::Error> {
        self.as_mut().combine(population, problem, eval, selected)
    }
}

impl<T, P, S, E> Insert<P, S, E> for Hint<T, P, S, E, Vec<S::Individual>>
where
    T: Insert<P, S, E>,
    S: Population,
    E: Eval<P, S::Individual>,
{
    fn insert(
        &mut self,
        population: &mut S,
        problem: &P,
        eval: &mut E,
        combined: Vec<S::Individual>,
    ) -> Result<(), Self::Error> {
        self.as_mut().insert(population, problem, eval, combined)
    }
}
